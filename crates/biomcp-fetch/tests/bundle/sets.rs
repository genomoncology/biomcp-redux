//! Complete-set publication is a public transport concurrency and parser contract.
use super::support;
use biomcp_fetch::{
    BundlePart, BundleSet, Fetch, FetchConfig, Find, Freshness, KeyKind, KeyUse, Keys, Quota, Rate,
    Row, RowFormat, SourceKey,
};
use serde::Deserialize;
use std::{marker::PhantomData, time::Duration};
#[derive(Debug, Deserialize)]
struct Native {
    name: String,
}
impl Row for Native {
    fn keys(&self) -> Keys {
        Keys {
            exact: vec![(KeyKind::Id, self.name.clone().into())],
            words: vec![self.name.clone().into()],
        }
    }
}
static QUOTA: Quota = Quota::new(
    SourceKey::declare("bundle-set-contract"),
    Rate::per_second(1000),
    None,
    4,
);
static PARTS: &[BundlePart] = &[
    BundlePart {
        name: "A",
        path: "/A",
    },
    BundlePart {
        name: "B",
        path: "/B",
    },
    BundlePart {
        name: "D",
        path: "/D",
    },
    BundlePart {
        name: "H",
        path: "/H",
    },
    BundlePart {
        name: "L",
        path: "/L",
    },
    BundlePart {
        name: "P",
        path: "/P",
    },
    BundlePart {
        name: "R",
        path: "/R",
    },
    BundlePart {
        name: "V",
        path: "/V",
    },
];
fn declared(server: &support::Server) -> BundleSet<Native> {
    BundleSet {
        spec: server.spec(&QUOTA, KeyUse::None),
        name: "synthetic-complete-set",
        parts: PARTS,
        format: RowFormat::Csv { banner_lines: 0 },
        fresh: Freshness::Days(7),
        row: PhantomData,
    }
}
fn cached(path: &std::path::Path) -> Fetch {
    Fetch::open(FetchConfig {
        cache: Some(path.to_owned()),
        ..Default::default()
    })
    .unwrap()
}
fn names(fetch: &Fetch, set: &BundleSet<Native>, word: &str) -> Vec<String> {
    let (rows, info) = fetch
        .leg(set.spec, fetch.deadline(Duration::from_secs(3)))
        .stored_set_rows(set, Find::Words(word))
        .unwrap();
    assert_eq!(info.rows, 8);
    assert!(
        info.content_revision
            .as_deref()
            .is_some_and(|s| s.starts_with("sha256:"))
    );
    rows.into_iter().map(|r| r.value.name).collect()
}
#[test]
fn complete_set_readers_see_one_revision_and_failed_parts_preserve_it() {
    let server = support::Server::new();
    let set = declared(&server);
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("cache.sqlite");
    let writer = cached(&path);
    let reader = cached(&path);
    std::thread::scope(|scope| {
        let call = scope.spawn(|| writer.sync_set(&set));
        for part in PARTS {
            server
                .next()
                .respond(200, "", &format!("name\nOLD {}\n", part.name));
        }
        assert_eq!(call.join().unwrap().unwrap().rows, 8);
    });
    assert_eq!(names(&reader, &set, "OLD").len(), 8);
    std::thread::scope(|scope| {
        let call = scope.spawn(|| writer.sync_set(&set));
        for (i, part) in PARTS.iter().enumerate() {
            let request = server.next();
            assert!(request.text.starts_with(&format!("GET {} ", part.path)));
            if i == 7 {
                assert_eq!(names(&reader, &set, "OLD").len(), 8);
                assert!(names(&reader, &set, "NEW").is_empty());
            }
            request.respond(200, "", &format!("name\nNEW {}\n", part.name));
        }
        call.join().unwrap().unwrap();
    });
    assert!(names(&reader, &set, "OLD").is_empty());
    assert_eq!(names(&reader, &set, "NEW").len(), 8);
    for malformed in [false, true] {
        std::thread::scope(|scope| {
            let call = scope.spawn(|| writer.sync_set(&set));
            for (i, part) in PARTS.iter().enumerate() {
                let request = server.next();
                if i == 3 {
                    if malformed {
                        request.respond(200, "", "wrong\nmissing-name\n");
                    } else {
                        request.respond(404, "", "missing partition");
                    }
                    break;
                }
                request.respond(200, "", &format!("name\nBROKEN {}\n", part.name));
            }
            assert!(call.join().unwrap().is_err());
        });
        assert_eq!(names(&reader, &set, "NEW").len(), 8);
        assert!(names(&reader, &set, "BROKEN").is_empty());
    }
    // Every read above is stored-only. No unresponded read request exists.
    assert!(server.requests.try_recv().is_err());
}

#[test]
fn older_positional_writers_invalidate_computed_revision_on_replace_update_and_delete() {
    let server = support::Server::new();
    let set = declared(&server);
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("cache.sqlite");
    let fetch = cached(&path);
    for sql in [
        "INSERT OR REPLACE INTO bundle VALUES('synthetic-complete-set',1,8,0,NULL,NULL)",
        "UPDATE bundle SET bytes=2 WHERE name='synthetic-complete-set'",
        "DELETE FROM bundle WHERE name='synthetic-complete-set'",
    ] {
        std::thread::scope(|scope| {
            let call = scope.spawn(|| fetch.sync_set(&set));
            for part in PARTS {
                server
                    .next()
                    .respond(200, "", &format!("name\nOLD {}\n", part.name));
            }
            call.join().unwrap().unwrap();
        });
        let db = fetch.cache_access().unwrap().connect().unwrap();
        let recursive: bool = db
            .pragma_query_value(None, "recursive_triggers", |r| r.get(0))
            .unwrap();
        assert!(
            !recursive,
            "proof must use ordinary SQLite trigger settings"
        );
        assert_eq!(
            db.query_row("SELECT count(*) FROM bundle_content", [], |r| r
                .get::<_, u32>(0))
                .unwrap(),
            1
        );
        db.execute(sql, []).unwrap();
        assert_eq!(
            db.query_row("SELECT count(*) FROM bundle_content", [], |r| r
                .get::<_, u32>(0))
                .unwrap(),
            0
        );
        if !sql.starts_with("DELETE") {
            let (_, info) = fetch
                .leg(set.spec, fetch.deadline(Duration::from_secs(3)))
                .stored_set_rows(&set, Find::Words("OLD"))
                .unwrap();
            assert!(info.content_revision.is_none());
        }
    }
}
