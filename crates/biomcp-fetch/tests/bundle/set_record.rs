//! A multi-file sync records and replays parts larger than the ordinary record limits (0906).
use super::*;
use biomcp_fetch::{BundlePart, BundleSet};

#[derive(Debug, Deserialize)]
struct Named {
    name: String,
}
impl Row for Named {
    fn keys(&self) -> Keys {
        Keys {
            exact: vec![(KeyKind::Id, self.name.clone().into())],
            words: Vec::new(),
        }
    }
}
static PARTS: &[BundlePart] = &[
    BundlePart {
        format: None,
        name: "small",
        path: "/small",
    },
    BundlePart {
        format: None,
        name: "big",
        path: "/big",
    },
];
const SMALL: &str = "name\nSMALL\n";

/// A set records each part as a sidecar under the set's own ceiling, so a part over the ordinary
/// 16 MiB record file limit, or over the ordinary 50 MiB binary reply limit, records and replays.
fn records_and_replays_large_part(big_bytes: usize) {
    let server = support::Server::<()>::new();
    let set = BundleSet::<Named> {
        spec: server.spec(&QUOTA, KeyUse::None),
        name: "synthetic-large-set",
        parts: PARTS,
        format: RowFormat::Csv { banner_lines: 0 },
        fresh: Freshness::Days(7),
        row: PhantomData,
    };
    // Quoted rows with doubled quotes grow when stored as JSON text.
    let line = format!("\"{}\"\n", "\"\"".repeat(32 * 1024));
    let mut big = String::from("name\n");
    while big.len() < big_bytes {
        big.push_str(&line);
    }
    let rows = ((big.len() - 5) / line.len() + 1) as u64;
    let dir = tempfile::tempdir().unwrap();
    let capture = dir.path().join("capture");
    let record = Fetch::open(FetchConfig {
        cache: Some(dir.path().join("record.sqlite")),
        mode: biomcp_fetch::Mode::Record {
            dir: capture.clone(),
        },
        ..Default::default()
    })
    .unwrap();
    std::thread::scope(|scope| {
        let call = scope.spawn(|| record.sync_set(&set));
        server.next().respond(200, "", SMALL);
        server.next().respond(200, "", &big);
        assert_eq!(
            call.join().unwrap().unwrap().rows,
            rows,
            "A recorded set must publish every row of both parts"
        );
    });
    drop(record);
    let replay = Fetch::open(FetchConfig {
        cache: Some(dir.path().join("replay.sqlite")),
        mode: biomcp_fetch::Mode::Replay { dir: capture },
        ..Default::default()
    })
    .unwrap();
    let info = replay.sync_set(&set).unwrap();
    assert_eq!(
        (info.rows, info.bytes),
        (rows, (big.len() + SMALL.len()) as u64),
        "The replayed set must carry every recorded row and byte"
    );
    assert!(
        server.requests.try_recv().is_err(),
        "Replay must not reach the network"
    );
}
#[test]
fn multi_file_sync_records_and_replays_a_part_over_sixteen_mib() {
    records_and_replays_large_part(17 * 1024 * 1024);
}
#[test]
fn multi_file_sync_records_and_replays_a_part_over_the_binary_reply_limit() {
    records_and_replays_large_part(51 * 1024 * 1024);
}
