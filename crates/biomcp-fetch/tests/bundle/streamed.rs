//! Streamed gzip XML parts keep only the rows a source emits, under their own decoded bound.
use super::*;
use biomcp_fetch::{BundlePart, BundleSet};
use std::io::{BufRead, Write};

#[derive(Debug, Deserialize)]
struct Item {
    symbol: String,
}
impl Row for Item {
    /// Each `<i>` line is one row; `<i/>` is a rejected element; any other line is skipped.
    fn stream_xml(
        xml: &mut dyn BufRead,
        emit: &mut dyn FnMut(Option<String>),
    ) -> Result<(), biomcp_fetch::BadReply> {
        for line in xml.lines() {
            let line = line.map_err(|_| biomcp_fetch::BadReply)?;
            if line == "<i/>" {
                emit(None);
            } else if let Some(symbol) = line
                .strip_prefix("<i>")
                .and_then(|l| l.strip_suffix("</i>"))
            {
                emit(Some(format!("{{\"symbol\":\"{symbol}\"}}")));
            }
        }
        Ok(())
    }
    fn keys(&self) -> Keys {
        Keys {
            exact: vec![(KeyKind::Symbol, self.symbol.clone().into())],
            words: Vec::new(),
        }
    }
}
fn gzip(body: &[u8]) -> Vec<u8> {
    let mut encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::fast());
    encoder.write_all(body).unwrap();
    encoder.finish().unwrap()
}
fn set(server: &support::Server, max_decoded_bytes: u64) -> BundleSet<Item> {
    let parts: &'static [BundlePart] = Box::leak(Box::new([BundlePart {
        name: "xml",
        path: "/all.xml.gz",
        format: Some(RowFormat::StreamedXml {
            max_bytes: 1024,
            max_decoded_bytes,
        }),
    }]));
    BundleSet {
        spec: server.spec(&QUOTA, KeyUse::None),
        name: "streamed-xml",
        parts,
        format: RowFormat::Tsv,
        fresh: Freshness::Days(1),
        row: PhantomData,
    }
}
fn reply(server: &support::Server, body: &[u8]) {
    let mut bytes = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        body.len()
    )
    .into_bytes();
    bytes.extend_from_slice(body);
    server.next().bytes(bytes);
}
const XML: &str = "<all>\n<i>BRAF</i>\n<skip>ignored</skip>\n<i/>\n<i>EGFR</i>\n</all>\n";

#[test]
fn streamed_part_keeps_emitted_rows_and_refuses_decoded_overflow() {
    let server = support::Server::new();
    let dir = tempfile::tempdir().unwrap();
    let fetch = cached(&dir.path().join("cache.sqlite"));
    let body = gzip(XML.as_bytes());
    let exact = set(&server, XML.len() as u64);
    std::thread::scope(|scope| {
        let call = scope.spawn(|| fetch.sync_set(&exact));
        reply(&server, &body);
        let info = call.join().unwrap().unwrap();
        assert_eq!(info.rows, 2);
        assert_eq!(info.rejections.len(), 1);
    });
    let short = set(&server, XML.len() as u64 - 1);
    std::thread::scope(|scope| {
        let call = scope.spawn(|| fetch.sync_set(&short));
        reply(&server, &body);
        assert!(call.join().unwrap().is_err(), "accepted decoded overflow");
    });
    let leg = fetch.leg(exact.spec, fetch.deadline(Duration::from_secs(2)));
    let (rows, _) = leg
        .stored_set_rows(&exact, Find::Exact(KeyKind::Symbol, "EGFR"))
        .unwrap();
    assert_eq!(rows.len(), 1, "a refused refresh replaced the prior set");
}

#[test]
fn a_stored_set_over_the_held_ceiling_copies_from_an_older_cache() {
    let server = support::Server::new();
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("cache.sqlite");
    let fetch = cached(&path);
    let set = set(&server, XML.len() as u64);
    std::thread::scope(|scope| {
        let call = scope.spawn(|| fetch.sync_set(&set));
        reply(&server, &gzip(XML.as_bytes()));
        call.join().unwrap().unwrap();
    });
    drop(fetch);
    // GTR's real set records about 245 MB, most of it a streamed part.
    let size = 245_000_000_u32;
    let database = rusqlite::Connection::open(&path).unwrap();
    database
        .execute(
            "UPDATE bundle SET bytes=?1 WHERE name='streamed-xml'",
            [size],
        )
        .unwrap();
    drop(database);
    let next = biomcp_fetch::Fetch::open(biomcp_fetch::FetchConfig {
        cache: Some(dir.path().join("next.sqlite")),
        cache_predecessors: vec![path],
        ..Default::default()
    })
    .unwrap();
    let leg = next.leg(set.spec, next.deadline(Duration::from_secs(2)));
    let (rows, info) = leg
        .stored_set_rows(&set, Find::Exact(KeyKind::Symbol, "BRAF"))
        .unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(info.bytes, u64::from(size));
}
