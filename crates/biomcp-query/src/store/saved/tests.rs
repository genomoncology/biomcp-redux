//! Saved store migrations, round trips and newer-file refusal.
use super::{
    COMPATIBILITY, MIGRATIONS, NAME_CHARS, NOTE_CHARS, NewItem, NoteTarget, SavedError, SavedStore,
    migrations::only_adds,
};
use biodata::Text;

pub(super) fn text(value: &str) -> Text {
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
        .unwrap()
        .lists
        .remove(0);
    let ids: Vec<_> = list.items.iter().map(|i| i.reference.id.as_str()).collect();
    assert_eq!(ids, ["BRAF", "NRAS", "KIT"], "set insert in added order");
    assert_eq!(list.name.as_str(), "Genes", "the first spelling stays");
    let removed = store
        .list_remove("GENES", &[text("NRAS"), text("TP53")])
        .unwrap();
    let list = &removed.lists[0];
    assert_eq!(list.items.len(), 2, "one item removed");
    assert_eq!(removed.missing, [text("TP53")], "not held");
    let unchanged = store.list_remove("genes", &[text("NRAS")]).unwrap();
    assert_eq!(
        unchanged.lists[0].modified, list.modified,
        "nothing removed leaves modified"
    );
    assert_eq!(unchanged.missing, [text("NRAS")], "now missing");
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

fn genes(path: &std::path::Path) {
    let mut store = SavedStore::open(path).unwrap();
    store
        .list_add(
            &text("genes"),
            &[NewItem {
                entity: text("gene"),
                id: text("BRAF"),
                source: None,
                note: None,
            }],
        )
        .unwrap();
}

#[test]
fn file_with_later_additive_migrations_opens() {
    let folder = tempfile::tempdir().unwrap();
    let path = folder.path().join("saved.sqlite");
    genes(&path);
    let later = i64::try_from(MIGRATIONS.len() + 2).unwrap();
    let connection = rusqlite::Connection::open(&path).unwrap();
    connection
        .execute_batch(
            "ALTER TABLE saved_list ADD COLUMN color TEXT NOT NULL DEFAULT ''; \
             CREATE TABLE saved_later(id TEXT PRIMARY KEY);",
        )
        .unwrap();
    connection
        .execute("UPDATE meta SET value=?1 WHERE key='schema'", [later])
        .unwrap();
    let mut store = SavedStore::open(&path).expect("a later 1.x file opens");
    let list = store
        .list_add(
            &text("genes"),
            &[NewItem {
                entity: text("gene"),
                id: text("KIT"),
                source: None,
                note: None,
            }],
        )
        .unwrap();
    assert_eq!(
        list.lists[0].items.len(),
        2,
        "an older build writes the later file"
    );
    let schema: i64 = connection
        .query_row("SELECT value FROM meta WHERE key='schema'", [], |row| {
            row.get(0)
        })
        .unwrap();
    assert_eq!(
        schema, later,
        "an older build never lowers the migration count"
    );
}

#[test]
fn newer_file_is_refused_untouched() {
    let folder = tempfile::tempdir().unwrap();
    let path = folder.path().join("saved.sqlite");
    genes(&path);
    let newer = COMPATIBILITY + 1;
    let connection = rusqlite::Connection::open(&path).unwrap();
    connection
        .execute(
            "INSERT INTO meta(key,value) VALUES('compatibility',?1)",
            [newer],
        )
        .unwrap();
    let state = |connection: &rusqlite::Connection| -> (i64, i64, i64) {
        connection
            .query_row(
                "SELECT (SELECT value FROM meta WHERE key='compatibility'), \
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
    assert_eq!(
        SavedStore::open(&path).err(),
        Some(SavedError::Newer),
        "a file needing a newer reader is refused"
    );
    let after: i64 = connection
        .query_row("PRAGMA data_version", [], |row| row.get(0))
        .unwrap();
    assert_eq!(after, changes, "no other connection wrote the refused file");
    assert_eq!(
        state(&connection),
        before,
        "refused file keeps its key, tables and items"
    );
    assert_eq!(before.0, newer, "the newer key stays");
}

pub(super) fn check(rows: &[(&str, &str)], schema: &str, complete: bool) -> super::NewSnapshot {
    super::NewSnapshot {
        total: rows.len() as u64,
        complete,
        row_schema: text(schema),
        sources: Vec::new(),
        rows: rows
            .iter()
            .map(|(id, fingerprint)| super::SnapshotRow {
                entity: text("trial"),
                id: text(id),
                source: None,
                fingerprint: text(fingerprint),
            })
            .collect(),
    }
}

fn ids(refs: &[biomcp_types::SavedRef]) -> Vec<&str> {
    refs.iter().map(|r| r.id.as_str()).collect()
}

pub(super) fn melanoma(store: &mut SavedStore) -> biomcp_types::SavedSearch {
    let args = vec!["search".to_owned(), "trial".to_owned()];
    store
        .save_search(&text("Melanoma"), &args, "biomcp.trial-search/1", "00")
        .unwrap()
}

#[test]
fn snapshots_compare_and_keep_two() {
    use biomcp_types::ChangeState;
    let folder = tempfile::tempdir().unwrap();
    let mut store = SavedStore::open(&folder.path().join("saved.sqlite")).unwrap();
    let search = melanoma(&mut store);
    let first = store
        .record_check(&search, &check(&[("A", "1"), ("B", "1")], "s/1", true))
        .unwrap();
    assert_eq!(first.state, ChangeState::First, "nothing to compare yet");
    assert_eq!(
        Some(first.next_due),
        first.to.map(|to| to.taken + super::FLOOR_SECONDS),
        "due an hour after the check"
    );
    let same = store
        .record_check(&search, &check(&[("A", "1"), ("B", "1")], "s/1", true))
        .unwrap();
    assert_eq!(same.state, ChangeState::Unchanged, "same IDs and rows");
    let changed = store
        .record_check(&search, &check(&[("B", "2"), ("C", "1")], "s/1", true))
        .unwrap();
    assert_eq!(changed.state, ChangeState::Changed, "rows differ");
    assert_eq!(
        (ids(&changed.new), ids(&changed.gone), ids(&changed.changed)),
        (vec!["C"], vec!["A"], vec!["B"]),
        "new, gone and changed by ID and fingerprint"
    );
    assert_eq!(
        store
            .snapshots(search.id.as_str(), &mut Vec::new())
            .unwrap()
            .len(),
        2,
        "two newest kept"
    );
    let renamed = store
        .record_check(&search, &check(&[("B", "2"), ("C", "3")], "s/2#rows", true))
        .unwrap();
    let again = store
        .record_check(&search, &check(&[("B", "2"), ("C", "4")], "s/3#rows", true))
        .unwrap();
    assert_eq!(
        (renamed.state, again.state, ids(&again.changed)),
        (ChangeState::Rebaselined, ChangeState::Changed, vec!["C"]),
        "a new schema ID with the same row digest compares rows instead of rebaselining"
    );
    let rebaselined = store
        .record_check(&search, &check(&[("X", "9")], "s/2", true))
        .unwrap();
    assert_eq!(
        rebaselined.state,
        ChangeState::Rebaselined,
        "row schema moved"
    );
    assert_eq!(
        (
            ids(&rebaselined.new),
            ids(&rebaselined.gone),
            ids(&rebaselined.changed)
        ),
        (vec!["X"], vec!["B", "C"], Vec::<&str>::new()),
        "a rebaseline lists new and gone by ID but no changed rows"
    );
    let incomplete = store
        .record_check(&search, &check(&[("Y", "9")], "s/2", false))
        .unwrap();
    assert_eq!(incomplete.state, ChangeState::Incomplete, "over the cap");
    assert!(
        incomplete.new.is_empty() && incomplete.gone.is_empty(),
        "only totals compare"
    );
}

#[test]
fn checks_inside_the_floor_are_not_due_and_delete_takes_snapshots() {
    use biomcp_types::ChangeState;
    let folder = tempfile::tempdir().unwrap();
    let mut store = SavedStore::open(&folder.path().join("saved.sqlite")).unwrap();
    let search = melanoma(&mut store);
    assert!(
        super::not_due(&search, None, super::now()).is_none(),
        "a search never checked is due"
    );
    store
        .record_check(&search, &check(&[("A", "1")], "s/1", true))
        .unwrap();
    let checked = store.searches(Some("melanoma")).unwrap().remove(0);
    let newest = store
        .snapshots(search.id.as_str(), &mut Vec::new())
        .unwrap();
    let not_due = super::not_due(&checked, newest.first(), super::now()).unwrap();
    assert_eq!(not_due.state, ChangeState::NotDue, "inside the floor");
    assert!(
        super::not_due(&checked, newest.first(), not_due.next_due).is_none(),
        "due at next_due"
    );
    assert_eq!(
        store.all_snapshots(&mut Vec::new()).unwrap().len(),
        1,
        "export lists kept"
    );
    store.delete(Some("melanoma")).unwrap();
    assert!(
        store.all_snapshots(&mut Vec::new()).unwrap().is_empty(),
        "delete takes snapshots"
    );
    let rows: i64 = store
        .connection
        .query_row("SELECT count(*) FROM saved_snapshot_row", [], |row| {
            row.get(0)
        })
        .unwrap();
    assert_eq!(rows, 0, "and their rows");
}

/// A list holding BRAF as a gene and a protein and KIT as a gene, and one saved search.
fn noted(path: &std::path::Path) -> SavedStore {
    let mut store = SavedStore::open(path).unwrap();
    let item = |entity: &str, id: &str| NewItem {
        entity: text(entity),
        id: text(id),
        source: None,
        note: None,
    };
    store
        .list_add(
            &text("genes"),
            &[
                item("gene", "BRAF"),
                item("protein", "BRAF"),
                item("gene", "KIT"),
            ],
        )
        .unwrap();
    let args = vec!["search".to_owned(), "trial".to_owned()];
    store
        .save_search(&text("trials"), &args, "biomcp.trial-search/1", "00")
        .unwrap();
    store
}

fn on_item<'a>(id: &'a str, entity: Option<&'a str>) -> NoteTarget<'a> {
    NoteTarget::Item {
        list: "genes",
        id,
        entity,
        source: None,
    }
}

#[test]
fn notes_are_content_addressed_and_limited() {
    use biomcp_types::NoteTargetKind;
    let folder = tempfile::tempdir().unwrap();
    let mut store = noted(&folder.path().join("saved.sqlite"));
    let first = store
        .note(NoteTarget::Named("GENES"), &text("watch"))
        .unwrap()
        .unwrap();
    let again = store
        .note(NoteTarget::Named("genes"), &text("watch"))
        .unwrap()
        .unwrap();
    assert_eq!(first, again, "the same note again changes nothing");
    assert_eq!(first.id.as_str().len(), 32, "16 bytes of the hash in hex");
    assert_eq!(first.target.kind, NoteTargetKind::List);
    let other = store
        .note(NoteTarget::Named("trials"), &text("watch"))
        .unwrap()
        .unwrap();
    assert_ne!(other.id, first.id, "the target is part of the ID");
    assert_eq!(other.target.kind, NoteTargetKind::Search);
    assert_eq!(
        store.note(on_item("BRAF", None), &text("v600e")).err(),
        Some(SavedError::Ambiguous),
        "an ID held under two entities needs one"
    );
    let gene = store
        .note(on_item("BRAF", Some("gene")), &text("v600e"))
        .unwrap()
        .unwrap();
    assert_eq!(
        gene.target.item.as_ref().map(|r| r.entity.as_str()),
        Some("gene")
    );
    assert_eq!(
        store.note(on_item("TP53", None), &text("x")).err(),
        Some(SavedError::ItemUnknown)
    );
    assert_eq!(
        store.note(NoteTarget::Named("nothing"), &text("x")).err(),
        Some(SavedError::Unknown)
    );
    let project = store
        .note(NoteTarget::Project, &text("tumor board"))
        .unwrap()
        .unwrap();
    assert_eq!(project.target.name, None);
    let longest = "n".repeat(NOTE_CHARS);
    store
        .note(NoteTarget::Project, &text(&longest))
        .expect("a note at the limit is kept")
        .unwrap();
    assert_eq!(
        store
            .note(NoteTarget::Project, &text(&format!("{longest}n")))
            .err(),
        Some(SavedError::TooLong)
    );
    let long = text(&"x".repeat(NAME_CHARS + 1));
    assert_eq!(
        store.list_add(&long, &[]).err(),
        Some(SavedError::TooLong),
        "the store refuses a long name itself"
    );
}

#[test]
fn notes_follow_their_targets() {
    let folder = tempfile::tempdir().unwrap();
    let mut store = noted(&folder.path().join("saved.sqlite"));
    store
        .note(NoteTarget::Named("genes"), &text("watch"))
        .unwrap()
        .unwrap();
    let search = store
        .note(NoteTarget::Named("trials"), &text("watch"))
        .unwrap()
        .unwrap();
    let gene = store
        .note(on_item("BRAF", Some("gene")), &text("v600e"))
        .unwrap()
        .unwrap();
    store
        .note(on_item("KIT", None), &text("gist"))
        .unwrap()
        .unwrap();
    store
        .note(NoteTarget::Project, &text("board"))
        .unwrap()
        .unwrap();
    assert_eq!(store.show(None).unwrap().user_notes.len(), 5);
    assert_eq!(
        store.show(Some("genes")).unwrap().user_notes.len(),
        3,
        "a list shows its own and its items' notes"
    );
    store.list_remove("genes", &[text("KIT")]).unwrap();
    assert_eq!(
        store.show(Some("genes")).unwrap().user_notes.len(),
        2,
        "removing an item removes its notes"
    );
    assert_eq!(
        store.delete_note(gene.id.as_str()).unwrap(),
        Ok(gene.clone())
    );
    assert_eq!(
        store.delete_note(gene.id.as_str()).err(),
        Some(SavedError::Unknown)
    );
    let deleted = store.delete(Some("trials")).unwrap();
    assert_eq!(
        deleted.user_notes,
        [search],
        "a deleted search takes its notes"
    );
    let deleted = store.delete(None).unwrap();
    assert_eq!(
        deleted.user_notes.len(),
        2,
        "the list note and the project note"
    );
    assert_eq!(store.show(None).unwrap(), Default::default());
}
