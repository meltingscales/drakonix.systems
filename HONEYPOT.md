# Honeypot System

The markov-babble honeypot traps scrapers and bots by streaming ~10 MB of
slow garbage HTML at ~10 KB/s. Every hit is logged to SQLite and visualised
on a live dashboard.

---

## How it works

1. Honeypot URLs (e.g. `/api/markov-babble/<slug>/gen`) are injected as
   hidden links into every page — invisible to real users but followed by
   scrapers.
2. When a bot hits the endpoint it receives a slow-streamed wall of Markov-
   chain text full of more honeypot links, trapping it in a loop.
3. 1-in-100 requests get **schizo-rng mode**: random chaos bytes streamed
   instead of HTML.
4. Every hit is logged asynchronously (fire-and-forget) — the response to
   the bot is never delayed by logging.
5. A **catch-all fallback** catches every other unmatched path (e.g.
   `/wp-login.php`, `/.env`, `/phpmyadmin/`) — logs the full URL string
   (path + query string) and request body, then returns a plain 404.

---

## Architecture

### Endpoints

```
GET /api/markov-babble/:slug/gen   — slow markov stream (trap loop)
ANY /*                             — catch-all fallback (log + 404)
```

`markov_babble_honeypot` and `catch_all_honeypot` in `src/handlers.rs`.

On each hit it spawns a background task that:
1. Calls `ipinfo.io/{ip}/country` and `ipinfo.io/{ip}/org` **concurrently**
   via `tokio::join!`.
2. Writes the result to SQLite via `HoneypotDb::log_hit`.

### Database

SQLite file at `honeypot.db` (override with `HONEYPOT_DB_PATH` env var).

```sql
CREATE TABLE honeypot_hits (
    id        INTEGER PRIMARY KEY AUTOINCREMENT,
    slug      TEXT NOT NULL,             -- markov slug OR full path+query for catch-all hits
    ip        TEXT NOT NULL,
    country   TEXT NOT NULL DEFAULT '',  -- ISO 3166-1 alpha-2, e.g. "US"
    org       TEXT NOT NULL DEFAULT '',  -- ASN + name, e.g. "AS14061 DigitalOcean, LLC"
    timestamp TEXT NOT NULL,             -- RFC 3339 UTC
    headers   TEXT NOT NULL,             -- JSON object of request headers
    body      TEXT NOT NULL DEFAULT ''   -- raw request body, up to 64 KB (empty for GETs)
);
```

New columns are added via `ALTER TABLE … ADD COLUMN` migrations that run
on startup and are silently ignored if the column already exists — so the
schema is forwards-compatible with existing databases.

### Rolling cap

`HONEYPOT_MAX_ENTRIES = 50_000` (defined in `src/constants.rs`).

After every insert the row count is checked; if it exceeds the cap the
oldest rows are deleted. This keeps the `.db` file at a predictable steady-
state size (~40–100 MB depending on header payload size).

### IP enrichment cache

Both the country and org lookups are cached in-process:

```rust
pub type CountryCache = Arc<RwLock<HashMap<String, String>>>;
pub type OrgCache     = Arc<RwLock<HashMap<String, String>>>;
```

Each unique IP is looked up **at most once** per server lifetime. Cache
misses make a `GET` request to ipinfo.io with a 3-second timeout; errors
store an empty string (also cached, so bad IPs don't keep retrying).

---

## API

All endpoints are Swagger-documented at `/swagger-ui/`.

| Method | Path | Description |
|--------|------|-------------|
| `GET` | `/api/markov-babble/:slug/gen` | Honeypot stream endpoint (markov babble trap) |
| `ANY` | `/*` | Catch-all fallback — logs path+query+body, returns 404 |
| `GET` | `/api/honeypot/hits` | Up to 50,000 most-recent hits as JSON |
| `GET` | `/api/honeypot/config` | Configuration constants (e.g. `max_entries`) |

---

## Dashboard

```
/services/honeypot-dummies
```

Linked from the `~/services/` navbar dropdown. Loads all data from
`/api/honeypot/hits` and `/api/honeypot/config` on page load with no
server-side templating — pure client-side JS.

### Visualisations

| Widget | Description |
|--------|-------------|
| **52-week heatmap** | GitHub-style calendar, one cell per day, coloured by hit count. Rich tooltip with top IPs + flag emojis. |
| **Hourly breakdown** | 14-day × 24-hour grid, pageable with prev/next. Colour scale is relative to the current page's max. Tooltip shows date, hour range, hit count, countries, and top orgs. |
| **24h distribution** | Aggregate bar chart of hits by UTC hour across all stored time — useful for spotting scheduled bots. |
| **Top IPs** | Horizontal bar chart, top 10, linked to ipinfo.io. |
| **Top slugs** | Horizontal bar chart, top 10. |
| **Top countries** | Horizontal bar chart, top 10. |
| **Hits table** | Full scrollable table with live filter, collapsible header JSON, and collapsible body. |

### Controls

- **Filter** — live text search across all table columns (IP, slug, country, org, timestamp).
- **Auto-refresh** — interval selector (off / 15 s / 30 s / 1 min / 5 min). Re-fetches and re-renders all widgets; preserves the active filter.

### Flag emojis

Country codes are converted to flag emojis client-side using Unicode
regional indicator symbols — no lookup table required:

```js
function countryFlag(code) {
  return [...code.toUpperCase()].map(c =>
    String.fromCodePoint(0x1F1E6 + c.charCodeAt(0) - 65)
  ).join("");
}
```

---

## Files

| File | Purpose |
|------|---------|
| `src/constants.rs` | `HONEYPOT_MAX_ENTRIES` — single source of truth for the DB cap |
| `src/honeypot_db.rs` | `HoneypotDb` struct, schema, migrations, `log_hit`, `get_recent_hits` |
| `src/handlers.rs` | `markov_babble_honeypot`, `lookup_country`, `lookup_org`, dashboard + API handlers |
| `src/main.rs` | `AppState` fields, route registration, cache + client init |
| `templates/honeypot_dummies.html` | Page skeleton — toolbar, root div, stylesheet + script tags |
| `static/js/honeypot_dummies.js` | All client-side rendering, data fetching, auto-refresh, filter |
| `static/css/honeypot_dummies.css` | Styles for all dashboard widgets and tooltip |

---

## Configuration

| Env var | Default | Description |
|---------|---------|-------------|
| `HONEYPOT_DB_PATH` | `honeypot.db` | Path to the SQLite database file |
| `MARKOV_STREAM_SPEED_MULTIPLIER` | `1.0` | Stream speed multiplier (higher = faster, useful for testing) |
