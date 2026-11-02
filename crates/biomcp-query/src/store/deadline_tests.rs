//! Query-owned connections must use the transport's real versus replay lock policy.
//! This inner contract inspects SQLite state unavailable through the CLI; the
//! held-store-lock installed case owns the reader and sync behavior.
use super::Store;
use biomcp_fetch::{Fetch, FetchConfig, Mode};
use std::time::Duration;

#[test]
fn query_open_and_prepare_keep_virtual_sqlite_waits_zero_and_live_waits_bounded()
-> Result<(), Box<dyn std::error::Error>> {
    for replay in [false, true] {
        let directory = tempfile::tempdir()?;
        let fetch = Fetch::open(FetchConfig {
            mode: if replay {
                Mode::Replay {
                    dir: directory.path().into(),
                }
            } else {
                Mode::default()
            },
            cache: Some(directory.path().join("cache.sqlite")),
            cache_owners: vec![super::OWNER],
            ..FetchConfig::default()
        })?;
        let store = Store::open(
            fetch.cache_access()?,
            fetch.deadline(Duration::from_secs(120)),
        )?;
        for prepare in [false, true] {
            if prepare {
                store.prepare()?;
            }
            let milliseconds: u64 =
                store
                    .connection
                    .query_row("PRAGMA busy_timeout", [], |row| row.get(0))?;
            if replay {
                assert_eq!(
                    milliseconds, 0,
                    "a virtual deadline cannot drive a real SQLite wait"
                );
            } else {
                assert!(
                    milliseconds > 0 && milliseconds <= 120_000,
                    "live SQLite waits must retain the remaining real budget: {milliseconds}"
                );
            }
        }
    }
    Ok(())
}
