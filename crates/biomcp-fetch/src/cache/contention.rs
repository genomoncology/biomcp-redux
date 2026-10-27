//! Real SQLite locking cannot be driven by the provider's virtual clock.
//! This inner concurrency contract checks bounded refusal and retained data;
//! outside-in concurrent replacement exercises the complete publication path.
#![expect(
    clippy::unwrap_used,
    reason = "A failed SQLite contract must fail the test"
)]
use super::{Cache, CacheError, Connection, Duration};
use crate::{Deadline, deadline::VirtualClock};

#[test]
fn replay_admission_preserves_rows_under_an_external_writer() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("cache.sqlite");
    let cache = Cache::open_with_owners(&path, &[]).unwrap();
    let owner = Connection::open(&path).unwrap();
    owner
        .execute_batch("CREATE TABLE admission(value TEXT); INSERT INTO admission VALUES('old'); BEGIN IMMEDIATE")
        .unwrap();
    let clock = VirtualClock::default();
    let deadline = Deadline::start(Duration::from_millis(5), Some(&clock));
    let replace = |connection: &Connection| {
        connection
            .execute("UPDATE admission SET value='new'", [])
            .map_err(super::db_error)
    };
    // The external transaction remains held until the attempt returns. There
    // is no timer that releases it and no assertion about machine speed.
    assert!(matches!(
        cache.sql(deadline, replace),
        Err(CacheError::Busy)
    ));
    assert!(
        !deadline.expired(),
        "disk admission must not advance replay time"
    );
    assert_eq!(
        owner
            .query_row("SELECT value FROM admission", [], |row| row
                .get::<_, String>(0))
            .unwrap(),
        "old"
    );
    owner.execute_batch("ROLLBACK").unwrap();
    assert_eq!(cache.sql(deadline, replace).unwrap(), 1);
    assert_eq!(
        owner
            .query_row("SELECT value FROM admission", [], |row| row
                .get::<_, String>(0))
            .unwrap(),
        "new"
    );
    assert!(matches!(
        cache.sql(Deadline::start(Duration::ZERO, Some(&clock)), replace),
        Err(CacheError::Busy)
    ));
}
