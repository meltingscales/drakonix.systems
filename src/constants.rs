/// Maximum number of honeypot hits to retain in the database.
/// Oldest entries are pruned automatically after each insert.
pub const HONEYPOT_MAX_ENTRIES: usize = 50_000;
