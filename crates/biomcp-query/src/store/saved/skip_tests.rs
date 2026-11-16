//! Saved items picked by source, and stored values a newer build wrote that this build skips.
use super::tests::{check, melanoma, text};
use biomcp_types::Source;

/// A list holding BRAF as a gene under CIViC and UniProt.
fn sourced(path: &std::path::Path) -> super::SavedStore {
    let mut store = super::SavedStore::open(path).unwrap();
    let item = |source: Source| super::NewItem {
        entity: text("gene"),
        id: text("BRAF"),
        source: Some(source),
        note: None,
    };
    store
        .list_add(
            &text("genes"),
            &[item(Source::Civic), item(Source::UniProt)],
        )
        .unwrap();
    store
}

#[test]
fn source_picks_an_item() {
    let folder = tempfile::tempdir().unwrap();
    let mut store = sourced(&folder.path().join("saved.sqlite"));
    let on = |source| super::NoteTarget::Item {
        list: "genes",
        id: "BRAF",
        entity: Some("gene"),
        source,
    };
    assert_eq!(
        store.note(on(None), &text("x")).err(),
        Some(super::SavedError::Ambiguous),
        "one ID and entity under two sources needs a source"
    );
    let civic = store.note(on(Some("civic")), &text("v600e")).unwrap().unwrap();
    assert_eq!(
        civic.target.item.and_then(|item| item.source),
        Some(Source::Civic)
    );
}

#[test]
fn unknown_stored_values_are_skipped() {
    use biomcp_types::SavedSkipped;
    let folder = tempfile::tempdir().unwrap();
    let mut store = sourced(&folder.path().join("saved.sqlite"));
    let civic = super::NoteTarget::Item {
        list: "genes",
        id: "BRAF",
        entity: None,
        source: Some("civic"),
    };
    store.note(civic, &text("v600e")).unwrap().unwrap();
    // A newer 1.x build may store a note kind or item source this build does not know.
    store
        .connection
        .execute(
            "UPDATE saved_list_item SET source='future-source' WHERE source='uniprot'",
            [],
        )
        .unwrap();
    store
        .connection
        .execute(
            "INSERT INTO saved_note(id,project,kind,record,entity,item,source,text,created) \
             VALUES('future','default','future-kind','','','','','x',1)",
            [],
        )
        .unwrap();
    let skipped = |name: Option<&str>, field: &str, record: &str, value: &str| SavedSkipped {
        record: text(record),
        name: name.map(text),
        field: field.into(),
        value: text(value),
    };
    let item = skipped(Some("genes"), "source", "BRAF", "future-source");
    for shown in [
        store.show(None).unwrap(),
        store.show(Some("genes")).unwrap(),
    ] {
        assert_eq!(shown.lists[0].items.len(), 1, "the readable item stays");
        assert_eq!(shown.user_notes.len(), 1, "the readable note stays");
        assert!(shown.skipped.contains(&item));
    }
    let note = skipped(None, "kind", "future", "future-kind");
    assert_eq!(
        store.show(None).unwrap().skipped,
        [item.clone(), note.clone()],
        "each unreadable row is named, not a failure"
    );
    assert_eq!(
        store.list_remove("genes", &[text("KIT")]).unwrap().skipped,
        std::slice::from_ref(&item),
        "a list remove names the item it cannot read"
    );
    assert_eq!(store.list_add(&text("genes"), &[]).unwrap().skipped, [item]);
    assert_eq!(
        store.delete_note("future").unwrap(),
        Err(note),
        "an unreadable note is deleted by its ID"
    );
    assert_eq!(store.show(None).unwrap().skipped.len(), 1);
}

#[test]
fn unknown_snapshot_row_sources_are_skipped() {
    let folder = tempfile::tempdir().unwrap();
    let mut store = super::SavedStore::open(&folder.path().join("saved.sqlite")).unwrap();
    let search = melanoma(&mut store);
    store
        .record_check(&search, &check(&[("A", "1"), ("B", "1")], "s/1", true))
        .unwrap();
    store
        .connection
        .execute(
            "UPDATE saved_snapshot_row SET source='future-source' WHERE id='A'",
            [],
        )
        .unwrap();
    let mut rows = Vec::new();
    let kept = store.all_snapshots(&mut rows).unwrap();
    assert_eq!(kept[0].rows.len(), 1, "the readable row stays");
    assert_eq!(
        rows,
        [biomcp_types::SavedSkipped {
            record: text("A"),
            name: Some(text("Melanoma")),
            field: "source".into(),
            value: text("future-source"),
        }],
        "a snapshot row from a newer build is skipped"
    );
    assert!(
        !kept[0].complete,
        "a snapshot missing a row it cannot read is incomplete"
    );
    // A rolling deploy runs mixed builds: the unreadable row is not reported as gone, nor
    // as new when the next check reads it again.
    let report = store
        .record_check(&search, &check(&[("A", "1"), ("B", "1")], "s/1", true))
        .unwrap();
    assert_eq!(report.state, biomcp_types::ChangeState::Incomplete);
    assert!(report.new.is_empty() && report.gone.is_empty());
    store
        .connection
        .execute(
            "UPDATE saved_snapshot SET sources='[{\"source\":\"future-source\"}]'",
            [],
        )
        .unwrap();
    let mut skipped = Vec::new();
    let kept = store.snapshots(search.id.as_str(), &mut skipped).unwrap();
    assert!(
        kept.iter().all(|snapshot| !snapshot.complete) && skipped.len() == 3,
        "a snapshot missing a source it cannot read is incomplete: {skipped:?}"
    );
}

#[test]
fn a_note_on_an_unreadable_item_answers_its_skip() {
    let folder = tempfile::tempdir().unwrap();
    let mut store = sourced(&folder.path().join("saved.sqlite"));
    store
        .connection
        .execute(
            "UPDATE saved_list_item SET source='future-source' WHERE source='uniprot'",
            [],
        )
        .unwrap();
    let on = |source| super::NoteTarget::Item {
        list: "genes",
        id: "BRAF",
        entity: None,
        source,
    };
    let written = store.note(on(Some("future-source")), &text("x")).unwrap();
    let skip = written.expect_err("the item's stored source is unreadable");
    assert_eq!(
        (skip.name, &*skip.field, skip.value),
        (Some(text("genes")), "source", text("future-source")),
        "the note is kept and its reply names what this build cannot read"
    );
    assert!(store.note(on(Some("civic")), &text("x")).unwrap().is_ok());
}
