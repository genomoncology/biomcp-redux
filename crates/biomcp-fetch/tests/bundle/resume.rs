use super::*;

#[test]
fn complete_part_recovers_from_an_unsatisfiable_resume() {
    let server = support::Server::new();
    let bundle = declared(&server);
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("cache.sqlite");
    let fetch = cached(&path);
    let invalid = "symbol\tsymbol\nBRAF\tkinase\n";
    assert!(matches!(
        sync_reply(&fetch, &bundle, &server, 200, "ETag: \"one\"\r\n", invalid),
        Err(biomcp_fetch::RequestError::Source(error))
            if error.reason == biomcp_types::Reason::BadReply
    ));
    // Recreate a complete part retained by an older version after parsing failed.
    let parts = path.with_extension("parts");
    let legacy = parts.join(".tmpABC416");
    std::fs::write(&legacy, invalid).unwrap();
    let db = rusqlite::Connection::open(&path).unwrap();
    let prior: Option<String> = db
        .query_row("SELECT part FROM bundle_sync", [], |row| row.get(0))
        .ok();
    if let Some(prior) = prior {
        std::fs::remove_file(prior).unwrap();
    }
    db.execute(
        "INSERT OR REPLACE INTO bundle_sync VALUES('genes',NULL,0,?1,?2,?3,?4,?3)",
        rusqlite::params![
            format!("{}/genes", server.base),
            legacy.to_str().unwrap(),
            invalid.len() as i64,
            "\"one\""
        ],
    )
    .unwrap();
    std::thread::scope(|scope| {
        let call = scope.spawn(|| fetch.sync(&bundle));
        let request = server.next();
        let headers = request.text.to_ascii_lowercase();
        assert!(headers.contains(&format!("range: bytes={}-", invalid.len())));
        assert!(headers.contains("if-match: \"one\""));
        request.respond(416, "", "");
        let request = server.next();
        let headers = request.text.to_ascii_lowercase();
        assert!(!headers.contains("range:"));
        assert!(!headers.contains("if-match:"));
        request.respond(200, "ETag: \"one\"\r\n", TSV);
        let info = call.join().unwrap().unwrap();
        assert_eq!(info.rows, 1);
        assert_eq!(info.downloaded_bytes, TSV.len() as u64);
    });
    assert_eq!(std::fs::read_dir(parts).unwrap().count(), 0);
}

#[test]
fn a_completed_body_with_a_bad_header_is_discarded_before_the_next_sync() {
    let server = support::Server::new();
    let bundle = declared(&server);
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("cache.sqlite");
    let fetch = cached(&path);
    assert!(
        sync_reply(
            &fetch,
            &bundle,
            &server,
            200,
            "ETag: \"one\"\r\n",
            "symbol\tsymbol\nBRAF\tkinase\n",
        )
        .is_err()
    );
    assert_eq!(
        std::fs::read_dir(path.with_extension("parts"))
            .unwrap()
            .count(),
        0
    );
    let db = rusqlite::Connection::open(&path).unwrap();
    assert_eq!(
        db.query_row("SELECT count(*) FROM bundle_sync", [], |row| {
            row.get::<_, u32>(0)
        })
        .unwrap(),
        0
    );
    std::thread::scope(|scope| {
        let call = scope.spawn(|| fetch.sync(&bundle));
        let request = server.next();
        let headers = request.text.to_ascii_lowercase();
        assert!(!headers.contains("range:"));
        assert!(!headers.contains("if-match:"));
        request.respond(200, "ETag: \"one\"\r\n", TSV);
        assert_eq!(call.join().unwrap().unwrap().rows, 1);
    });
}
