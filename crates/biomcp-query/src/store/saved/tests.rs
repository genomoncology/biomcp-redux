//! Saved store migrations, round trips and newer-file refusal.
use super::{MIGRATIONS, NewItem, SavedError, SavedStore, migrations::only_adds};
use biodata::Text;

fn text(value: &str) -> Text {
    Text::new(value).unwrap()
}

#[test]
fn migrations_only_add() {
    for statement in MIGRATIONS {
        assert!(
            only_adds(statement),
            "migration does more than add: {statement}"
        );
    }
    for statement in [
        "DROP TABLE saved_search",
        "ALTER TABLE saved_list RENAME TO old_list",
        "ALTER TABLE saved_list RENAME COLUMN name TO title",
        "ALTER TABLE saved_list DROP COLUMN updated",
        "ALTER TABLE saved_list ADD COLUMN color TEXT",
        "CREATE TABLE copy AS SELECT * FROM saved_list",
        "CREATE TABLE a(x INTEGER); DROP TABLE saved_list",
        "DELETE FROM saved_list",
        "UPDATE meta SET value=0",
        "INSERT INTO meta(key,value) VALUES('schema',0)",
        "PRAGMA journal_mode=WAL",
    ] {
        assert!(!only_adds(statement), "accepted: {statement}");
    }
    assert!(
        only_adds("ALTER TABLE saved_list ADD COLUMN color TEXT NOT NULL DEFAULT ''"),
        "a column with a default is accepted"
    );
}

#[test]
fn lists_and_searches_round_trip() {
    let folder = tempfile::tempdir().unwrap();
    let path = folder.path().join("saved.sqlite");
    let mut store = SavedStore::open(&path).unwrap();
    let args = vec!["search".to_owned(), "trial".to_owned()];
    let search = store
        .save_search(&text("Melanoma"), &args, "biomcp.trial-search/1", "00")
        .unwrap();
    let again = store
        .save_search(&text("melanoma"), &args, "biomcp.trial-search/1", "00")
        .unwrap();
    assert_eq!(
        search, again,
        "the same words under the same name change nothing"
    );
    assert_eq!(search.id.as_str().len(), 32, "16 random bytes in hex");
    assert_eq!(
        store.check_search_name("MELANOMA", &["search".to_owned()]),
        Err(SavedError::Taken),
        "other words under a held name are refused"
    );
    let item = |id: &str| NewItem {
        entity: text("gene"),
        id: text(id),
        source: None,
        note: None,
    };
    assert_eq!(
        store.list_add(&text("melanoma"), &[item("BRAF")]).err(),
        Some(SavedError::Taken),
        "a list cannot take a search's name"
    );
    store
        .list_add(&text("Genes"), &[item("BRAF"), item("NRAS")])
        .unwrap();
    let list = store
        .list_add(&text("genes"), &[item("BRAF"), item("KIT")])
        .unwrap();
    let ids: Vec<_> = list.items.iter().map(|i| i.reference.id.as_str()).collect();
    assert_eq!(ids, ["BRAF", "NRAS", "KIT"], "set insert in added order");
    assert_eq!(list.name.as_str(), "Genes", "the first spelling stays");
    let list = store.list_remove("GENES", &[text("NRAS")]).unwrap();
    assert_eq!(list.items.len(), 2, "one item removed");
    drop(store);
    let mut store = SavedStore::open(&path).unwrap();
    let all = store.show(None).unwrap();
    assert_eq!(
        (all.searches.len(), all.lists.len()),
        (1, 1),
        "reopened file keeps both"
    );
    assert_eq!(
        store.show(Some("nothing")).err(),
        Some(SavedError::Unknown),
        "unknown name"
    );
    let deleted = store.delete(Some("melanoma")).unwrap();
    assert_eq!(deleted.searches.len(), 1, "deleted search returned");
    let deleted = store.delete(None).unwrap();
    assert_eq!(deleted.lists.len(), 1, "delete all returns the rest");
    assert_eq!(
        store.show(None).unwrap(),
        Default::default(),
        "nothing left"
    );
}

#[test]
fn newer_file_is_refused_untouched() {
    let folder = tempfile::tempdir().unwrap();
    let path = folder.path().join("saved.sqlite");
    let mut store = SavedStore::open(&path).unwrap();
    store
        .list_add(&text("genes"), &[NewItem { entity: text("gene"), id: text("BRAF"), source: None, note: None }])
        .unwrap();
    drop(store);
    let newer = i64::try_from(MIGRATIONS.len() + 1).unwrap();
    let connection = rusqlite::Connection::open(&path).unwrap();
    connection
        .execute("UPDATE meta SET value=?1 WHERE key='schema'", [newer])
        .unwrap();
    let state = |connection: &rusqlite::Connection| -> (i64, i64, i64) {
        connection
            .query_row(
                "SELECT (SELECT value FROM meta WHERE key='schema'), \
                 (SELECT count(*) FROM sqlite_master), (SELECT count(*) FROM saved_list_item)",
                [],
                |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
            )
            .unwrap()
    };
    let before = state(&connection);
    let changes: i64 = connection
        .query_row("PRAGMA data_version", [], |row| row.get(0))
        .unwrap();
    assert_eq!(SavedStore::open(&path).err(), Some(SavedError::Newer), "newer file refused");
    let after: i64 = connection
        .query_row("PRAGMA data_version", [], |row| row.get(0))
        .unwrap();
    assert_eq!(after, changes, "no other connection wrote the refused file");
    assert_eq!(state(&connection), before, "refused file keeps its version, tables and items");
    assert_eq!(before.0, newer, "the newer version stays");
}
