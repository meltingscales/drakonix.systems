use crate::constants::HONEYPOT_MAX_ENTRIES;
use rusqlite::{Connection, Result};
use serde::Serialize;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct HoneypotDb {
    conn: Arc<Mutex<Connection>>,
}

#[derive(Serialize, utoipa::ToSchema)]
pub struct HoneypotHit {
    pub id: i64,
    pub slug: String,
    pub ip: String,
    pub country: String, // ISO 3166-1 alpha-2, e.g. "US"
    pub org: String,     // ASN + org name, e.g. "AS14061 DigitalOcean, LLC"
    pub timestamp: String,
    pub headers: String, // raw JSON blob
    pub body: String,    // raw request body (empty for GET)
}

/// Slim hit record (no headers/body blobs) for charts/heatmaps/stats that need
/// the whole dataset at once — the full `HoneypotHit` is only fetched a page
/// at a time via `get_hits_page`.
#[derive(Serialize, utoipa::ToSchema)]
pub struct HoneypotHitLight {
    pub slug: String,
    pub ip: String,
    pub country: String,
    pub org: String,
    pub timestamp: String,
}

/// Columns that server-side sorting/searching is allowed to touch.
/// Whitelisted because the column name is interpolated into SQL (rusqlite
/// can't bind identifiers) — never pass an unvalidated string here.
pub const SORTABLE_COLUMNS: &[&str] = &["id", "slug", "ip", "country", "org", "timestamp"];

pub struct HitsPage {
    pub rows: Vec<HoneypotHit>,
    pub total: i64,
    pub filtered: i64,
}

impl HoneypotDb {
    pub fn new(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS honeypot_hits (
                id        INTEGER PRIMARY KEY AUTOINCREMENT,
                slug      TEXT NOT NULL,
                ip        TEXT NOT NULL,
                timestamp TEXT NOT NULL,
                headers   TEXT NOT NULL
            );",
        )?;
        // Migrations: add columns introduced after initial schema
        let _ = conn.execute_batch(
            "ALTER TABLE honeypot_hits ADD COLUMN country TEXT NOT NULL DEFAULT '';",
        );
        let _ = conn.execute_batch(
            "ALTER TABLE honeypot_hits ADD COLUMN org TEXT NOT NULL DEFAULT '';",
        );
        let _ = conn.execute_batch(
            "ALTER TABLE honeypot_hits ADD COLUMN body TEXT NOT NULL DEFAULT '';",
        );
        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    pub async fn log_hit(&self, slug: String, ip: String, headers_json: String, body: String, country: String, org: String) {
        let conn = Arc::clone(&self.conn);
        let timestamp = chrono::Utc::now().to_rfc3339();
        tokio::task::spawn_blocking(move || {
            let conn = conn.lock().unwrap();
            let _ = conn.execute(
                "INSERT INTO honeypot_hits (slug, ip, country, org, timestamp, headers, body) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                rusqlite::params![slug, ip, country, org, timestamp, headers_json, body],
            );
            // Rotate: delete oldest rows beyond HONEYPOT_MAX_ENTRIES
            let count: usize = conn
                .query_row("SELECT COUNT(*) FROM honeypot_hits", [], |r| r.get(0))
                .unwrap_or(0);
            if count > HONEYPOT_MAX_ENTRIES {
                let _ = conn.execute(
                    "DELETE FROM honeypot_hits WHERE id IN \
                     (SELECT id FROM honeypot_hits ORDER BY id ASC LIMIT ?1)",
                    rusqlite::params![count - HONEYPOT_MAX_ENTRIES],
                );
            }
        })
        .await
        .ok();
    }

    /// Slim rows (no headers/body) across the whole retained dataset — used for
    /// charts/heatmaps/top-stats, which need every row but not the heavy blobs.
    pub async fn get_stats_hits(&self) -> Vec<HoneypotHitLight> {
        let conn = Arc::clone(&self.conn);
        tokio::task::spawn_blocking(move || {
            let conn = conn.lock().unwrap();
            let mut stmt = conn
                .prepare(
                    "SELECT slug, ip, country, org, timestamp \
                     FROM honeypot_hits ORDER BY id DESC LIMIT ?1",
                )
                .ok()?;
            let hits = stmt
                .query_map(rusqlite::params![HONEYPOT_MAX_ENTRIES], |row| {
                    Ok(HoneypotHitLight {
                        slug: row.get(0)?,
                        ip: row.get(1)?,
                        country: row.get(2)?,
                        org: row.get(3)?,
                        timestamp: row.get(4)?,
                    })
                })
                .ok()?
                .filter_map(|r| r.ok())
                .collect();
            Some(hits)
        })
        .await
        .ok()
        .flatten()
        .unwrap_or_default()
    }

    /// One page of full hit rows (including headers/body) for the hits table,
    /// optionally filtered by a case-insensitive substring `search` across
    /// slug/ip/country/org, and sorted by a whitelisted column.
    pub async fn get_hits_page(
        &self,
        offset: i64,
        limit: i64,
        search: Option<String>,
        sort_col: &'static str,
        sort_dir: &'static str,
    ) -> HitsPage {
        let conn = Arc::clone(&self.conn);
        tokio::task::spawn_blocking(move || {
            let conn = conn.lock().unwrap();

            let total: i64 = conn
                .query_row("SELECT COUNT(*) FROM honeypot_hits", [], |r| r.get(0))
                .unwrap_or(0);

            let where_clause = if search.is_some() {
                "WHERE slug LIKE ?1 OR ip LIKE ?1 OR country LIKE ?1 OR org LIKE ?1"
            } else {
                ""
            };

            let filtered: i64 = if let Some(term) = &search {
                let pattern = format!("%{term}%");
                conn.query_row(
                    &format!("SELECT COUNT(*) FROM honeypot_hits {where_clause}"),
                    rusqlite::params![pattern],
                    |r| r.get(0),
                )
                .unwrap_or(0)
            } else {
                total
            };

            let query = format!(
                "SELECT id, slug, ip, country, org, timestamp, headers, body \
                 FROM honeypot_hits {where_clause} \
                 ORDER BY {sort_col} {sort_dir} LIMIT ?{n} OFFSET ?{n_plus_1}",
                n = if search.is_some() { 2 } else { 1 },
                n_plus_1 = if search.is_some() { 3 } else { 2 },
            );
            let mut stmt = match conn.prepare(&query) {
                Ok(s) => s,
                Err(_) => return HitsPage { rows: vec![], total, filtered },
            };

            let map_row = |row: &rusqlite::Row| {
                Ok(HoneypotHit {
                    id: row.get(0)?,
                    slug: row.get(1)?,
                    ip: row.get(2)?,
                    country: row.get(3)?,
                    org: row.get(4)?,
                    timestamp: row.get(5)?,
                    headers: row.get(6)?,
                    body: row.get(7)?,
                })
            };

            let rows: Vec<HoneypotHit> = if let Some(term) = &search {
                let pattern = format!("%{term}%");
                stmt.query_map(rusqlite::params![pattern, limit, offset], map_row)
            } else {
                stmt.query_map(rusqlite::params![limit, offset], map_row)
            }
            .map(|iter| iter.filter_map(|r| r.ok()).collect())
            .unwrap_or_default();

            HitsPage { rows, total, filtered }
        })
        .await
        .unwrap_or_else(|_| HitsPage { rows: vec![], total: 0, filtered: 0 })
    }
}
