//! The saved-work schema, one statement per step, applied in order.
//!
//! Append only. A statement may create a table, create an index or add a column
//! with a default, so any 1.x build reads any 1.x file (sdlc/planning/saved-work.md,
//! Compatibility). The `migrations_only_add` test enforces this.

/// Schema statements; a file's version is how many of them it has run.
pub const MIGRATIONS: &[&str] = &[
    "CREATE TABLE meta(key TEXT PRIMARY KEY, value INTEGER NOT NULL)",
    "CREATE TABLE saved_search(id TEXT PRIMARY KEY, project TEXT NOT NULL DEFAULT 'default', \
     name TEXT NOT NULL, args TEXT NOT NULL, schema TEXT NOT NULL, request_key TEXT NOT NULL, \
     created INTEGER NOT NULL, checked INTEGER)",
    "CREATE UNIQUE INDEX saved_search_name ON saved_search(project, name COLLATE NOCASE)",
    "CREATE TABLE saved_list(id TEXT PRIMARY KEY, project TEXT NOT NULL DEFAULT 'default', \
     name TEXT NOT NULL, created INTEGER NOT NULL, updated INTEGER NOT NULL)",
    "CREATE UNIQUE INDEX saved_list_name ON saved_list(project, name COLLATE NOCASE)",
    "CREATE TABLE saved_list_item(list TEXT NOT NULL, entity TEXT NOT NULL, id TEXT NOT NULL, \
     source TEXT NOT NULL DEFAULT '', note TEXT, added INTEGER NOT NULL, \
     PRIMARY KEY(list, entity, id, source))",
    "CREATE TABLE saved_snapshot(id TEXT PRIMARY KEY, search TEXT NOT NULL, taken INTEGER NOT NULL, \
     total INTEGER NOT NULL, complete INTEGER NOT NULL, row_schema TEXT NOT NULL, \
     sources TEXT NOT NULL DEFAULT '[]')",
    "CREATE INDEX saved_snapshot_search ON saved_snapshot(search, taken)",
    "CREATE TABLE saved_snapshot_row(snapshot TEXT NOT NULL, position INTEGER NOT NULL, \
     entity TEXT NOT NULL, id TEXT NOT NULL, source TEXT NOT NULL DEFAULT '', \
     fingerprint TEXT NOT NULL, PRIMARY KEY(snapshot, position))",
    "CREATE TABLE saved_note(id TEXT PRIMARY KEY, project TEXT NOT NULL DEFAULT 'default', \
     kind TEXT NOT NULL, record TEXT NOT NULL DEFAULT '', entity TEXT NOT NULL DEFAULT '', \
     item TEXT NOT NULL DEFAULT '', source TEXT NOT NULL DEFAULT '', text TEXT NOT NULL, \
     created INTEGER NOT NULL)",
    "CREATE INDEX saved_note_record ON saved_note(record, kind, item)",
];

/// Whether one migration statement only adds: a table, an index, or a column with a default.
#[cfg(test)]
pub(super) fn only_adds(statement: &str) -> bool {
    let words: Vec<String> = statement
        .split_whitespace()
        .map(str::to_ascii_uppercase)
        .collect();
    let text = words.join(" ");
    let single = !text.trim_end_matches(';').contains(';');
    let forbidden = [
        "DROP", "RENAME", "DELETE", "UPDATE", "INSERT", "REPLACE", "SELECT", "PRAGMA",
    ]
    .iter()
    .any(|word| {
        words
            .iter()
            .any(|w| w.trim_matches(|c: char| !c.is_ascii_alphabetic()) == *word)
    });
    let kind = text.starts_with("CREATE TABLE ")
        || text.starts_with("CREATE INDEX ")
        || text.starts_with("CREATE UNIQUE INDEX ")
        || (text.starts_with("ALTER TABLE ")
            && text.contains(" ADD COLUMN ")
            && text.contains(" DEFAULT "));
    single && !forbidden && kind
}
