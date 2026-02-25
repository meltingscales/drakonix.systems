use rusqlite::{Connection, Result};
use serde::Serialize;
use std::sync::{Arc, Mutex};

const MAX_ENTRIES: usize = 1000;

#[derive(Clone)]
pub struct HoneypotDb {
    conn: Arc<Mutex<Connection>>,
}

#[derive(Serialize, utoipa::ToSchema)]
pub struct HoneypotHit {
    pub id: i64,
    pub slug: String,
    pub ip: String,
    pub timestamp: String,
    pub headers: String, // raw JSON blob
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
        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    pub async fn log_hit(&self, slug: String, ip: String, headers_json: String) {
        let conn = Arc::clone(&self.conn);
        let timestamp = chrono::Utc::now().to_rfc3339();
        tokio::task::spawn_blocking(move || {
            let conn = conn.lock().unwrap();
            let _ = conn.execute(
                "INSERT INTO honeypot_hits (slug, ip, timestamp, headers) VALUES (?1, ?2, ?3, ?4)",
                rusqlite::params![slug, ip, timestamp, headers_json],
            );
            // Rotate: delete oldest rows beyond MAX_ENTRIES
            let count: usize = conn
                .query_row("SELECT COUNT(*) FROM honeypot_hits", [], |r| r.get(0))
                .unwrap_or(0);
            if count > MAX_ENTRIES {
                let _ = conn.execute(
                    "DELETE FROM honeypot_hits WHERE id IN \
                     (SELECT id FROM honeypot_hits ORDER BY id ASC LIMIT ?1)",
                    rusqlite::params![count - MAX_ENTRIES],
                );
            }
        })
        .await
        .ok();
    }

    pub async fn get_recent_hits(&self) -> Vec<HoneypotHit> {
        let conn = Arc::clone(&self.conn);
        tokio::task::spawn_blocking(move || {
            let conn = conn.lock().unwrap();
            let mut stmt = conn
                .prepare(
                    "SELECT id, slug, ip, timestamp, headers \
                     FROM honeypot_hits ORDER BY id DESC LIMIT 1000",
                )
                .ok()?;
            let hits = stmt
                .query_map([], |row| {
                    Ok(HoneypotHit {
                        id: row.get(0)?,
                        slug: row.get(1)?,
                        ip: row.get(2)?,
                        timestamp: row.get(3)?,
                        headers: row.get(4)?,
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
}
