//! A controlled synthetic legacy-store copy preserves bundles and erases withheld bytes.
#![expect(
    clippy::unwrap_used,
    clippy::disallowed_methods,
    clippy::disallowed_types,
    reason = "Isolated synthetic SQLite repair witness"
)]
use biomcp_query::store::scrub_copy;
use rusqlite::{Connection, params};
use std::{fs, time::Duration};
const HIDDEN: &str = "WITHHELD_LEGACY_BODY_9af8e210";
const ALLOWED: &str = "PERMITTED_BODY_28cb431a";

fn fixture(path: &std::path::Path) -> Connection {
    let db = Connection::open(path).unwrap();
    db.execute_batch("PRAGMA journal_mode=WAL; PRAGMA secure_delete=OFF;
        CREATE TABLE meta(owner TEXT PRIMARY KEY, version INTEGER NOT NULL);
        INSERT INTO meta VALUES ('biomcp-fetch',5),('biomcp-query',6);
        CREATE TABLE reply(key BLOB PRIMARY KEY,source TEXT,url TEXT,body BLOB,status INTEGER DEFAULT 200);
        CREATE TABLE bundle(name TEXT PRIMARY KEY,rows INTEGER);
        CREATE TABLE bundle_row(bundle TEXT,n INTEGER,row TEXT);
        INSERT INTO bundle VALUES ('study',1);
        INSERT INTO bundle_row VALUES ('study',0,'{\"study_id\":\"synthetic-study\"}');").unwrap();
    db.execute_batch(include_str!("../src/store/schema.sql"))
        .unwrap();
    let rows = serde_json::json!([
        {"pmid":1001,"section":"RESULTS","text":ALLOWED},
        {"pmid":1002,"section":"RESULTS","text":HIDDEN},
        {"pmid":1003,"section":"RESULTS","text":HIDDEN}
    ]);
    reply(
        &db,
        1,
        "litsense",
        "https://www.ncbi.nlm.nih.gov/litsense",
        rows.to_string().as_bytes(),
    );
    for (key, id, license) in [(2, 1001, "CC BY"), (3, 1002, "CC BY NC")] {
        let metadata =
            serde_json::json!({"pmcid":format!("PMC{id}"),"pmid":id,"license_code":license});
        reply(
            &db,
            key,
            "pmc-oa",
            &format!("https://example/PMC{id}.1.json"),
            metadata.to_string().as_bytes(),
        );
    }
    reply(
        &db,
        4,
        "pmc-oa",
        "https://example/PMC1002.1/PMC1002.1.xml",
        HIDDEN.as_bytes(),
    );
    reply(&db, 5, "pubmed", "https://example/summary", br#"{"result":{"uids":["1003"],"1003":{"uid":"1003","title":"Unlicensed abstract stays"}}}"#);
    let abstracts = format!(
        "<PubmedArticleSet><PubmedArticle><MedlineCitation><PMID>1002</PMID><Article><Abstract><AbstractText>{HIDDEN}</AbstractText></Abstract></Article></MedlineCitation></PubmedArticle><PubmedArticle><MedlineCitation><PMID>1003</PMID><Article><Abstract><AbstractText>UNKNOWN_LICENSE_ABSTRACT_STAYS</AbstractText></Abstract></Article></MedlineCitation></PubmedArticle></PubmedArticleSet>"
    );
    reply(
        &db,
        6,
        "pubmed",
        "https://example/efetch",
        abstracts.as_bytes(),
    );
    db.execute(
        "INSERT INTO pool VALUES ('p',x'01',1,9999999999,3,'[]')",
        [],
    )
    .unwrap();
    for (rank, id, text) in [(0, 1001, ALLOWED), (1, 1002, HIDDEN), (2, 1003, HIDDEN)] {
        let item = serde_json::json!({"key":{"namespace":"pubmed","id":id.to_string()},"pmid":id.to_string(),"sentence":{"text":text,"section":"RESULTS"}});
        db.execute(
            "INSERT INTO pool_item(pool,rank,identity,item) VALUES ('p',?1,?2,?3)",
            params![rank, id.to_string(), item.to_string()],
        )
        .unwrap();
    }
    // A deleted large row leaves withheld text in free pages as well as the WAL.
    reply(
        &db,
        9,
        "litsense",
        "https://example/deleted",
        HIDDEN.repeat(600).as_bytes(),
    );
    db.execute("DELETE FROM reply WHERE key=?1", [vec![9_u8]])
        .unwrap();
    db
}
fn reply(db: &Connection, key: u8, source: &str, url: &str, body: &[u8]) {
    db.execute(
        "INSERT INTO reply(key,source,url,body) VALUES (?1,?2,?3,?4)",
        params![vec![key], source, url, body],
    )
    .unwrap();
}

#[test]
fn scrub_copy_removes_withheld_disk_bytes_and_preserves_rank_and_study_lookup() {
    let dir = tempfile::tempdir().unwrap();
    let original = dir.path().join("original.sqlite");
    let destination = dir.path().join("checked.sqlite");
    let db = fixture(&original);
    let before = fs::read(&original).unwrap();
    let wal = original.with_extension("sqlite-wal");
    let before_wal = fs::read(&wal).unwrap();
    assert!(
        before_wal
            .windows(HIDDEN.len())
            .any(|value| value == HIDDEN.as_bytes())
    );
    scrub_copy(&original, &destination, Duration::from_secs(20)).unwrap();
    assert_eq!(before, fs::read(&original).unwrap());
    assert_eq!(before_wal, fs::read(&wal).unwrap());
    let copy = Connection::open(&destination).unwrap();
    let study: String = copy
        .query_row(
            "SELECT row FROM bundle_row WHERE bundle='study' AND n=0",
            [],
            |row| row.get(0),
        )
        .unwrap();
    assert!(study.contains("synthetic-study"));
    let items: Vec<String> = copy
        .prepare("SELECT item FROM pool_item ORDER BY rank")
        .unwrap()
        .query_map([], |row| row.get(0))
        .unwrap()
        .collect::<Result<_, _>>()
        .unwrap();
    assert_eq!(items.len(), 3);
    assert!(items[0].contains(ALLOWED));
    for (item, id) in items.iter().zip([1001, 1002, 1003]) {
        assert!(item.contains(&id.to_string()));
    }
    assert!(!items[1].contains(HIDDEN));
    assert!(!items[2].contains(HIDDEN));
    let abstract_metadata: Vec<u8> = copy
        .query_row("SELECT body FROM reply WHERE key=?1", [vec![5_u8]], |row| {
            row.get(0)
        })
        .unwrap();
    assert!(
        String::from_utf8(abstract_metadata)
            .unwrap()
            .contains("Unlicensed abstract stays")
    );
    let abstracts: Vec<u8> = copy
        .query_row("SELECT body FROM reply WHERE key=?1", [vec![6_u8]], |row| {
            row.get(0)
        })
        .unwrap();
    assert!(
        String::from_utf8(abstracts)
            .unwrap()
            .contains("UNKNOWN_LICENSE_ABSTRACT_STAYS")
    );
    drop(copy);
    for suffix in ["", "-wal", "-shm", "-journal"] {
        let path = dir.path().join(format!("checked.sqlite{suffix}"));
        if let Ok(bytes) = fs::read(path) {
            assert!(
                !bytes
                    .windows(HIDDEN.len())
                    .any(|value| value == HIDDEN.as_bytes())
            );
        }
    }
    assert!(scrub_copy(&original, &destination, Duration::from_secs(20)).is_err());
    assert_eq!(before_wal, fs::read(&wal).unwrap());
    drop(db);
}

#[test]
fn failed_scrub_discards_only_its_new_copy_and_leaves_existing_destination_untouched() {
    let dir = tempfile::tempdir().unwrap();
    let original = dir.path().join("original.sqlite");
    let destination = dir.path().join("copy.sqlite");
    let db = fixture(&original);
    db.execute("UPDATE reply SET body=x'ff' WHERE key=?1", [vec![1_u8]])
        .unwrap();
    assert!(scrub_copy(&original, &destination, Duration::from_secs(20)).is_err());
    assert!(!destination.exists());
    fs::write(&destination, b"existing operator file").unwrap();
    assert!(scrub_copy(&original, &destination, Duration::from_secs(20)).is_err());
    assert_eq!(fs::read(&destination).unwrap(), b"existing operator file");
    let original_body: Vec<u8> = db
        .query_row("SELECT body FROM reply WHERE key=?1", [vec![1_u8]], |row| {
            row.get(0)
        })
        .unwrap();
    assert_eq!(original_body, vec![255]);
}
