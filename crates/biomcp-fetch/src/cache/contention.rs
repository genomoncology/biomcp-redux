//! Real SQLite locking cannot be driven by the provider's virtual clock.
//! This inner concurrency contract checks bounded refusal and retained data;
//! outside-in concurrent replacement exercises the complete publication path.
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
    let admitted = std::time::Instant::now();
    assert!(matches!(
        cache.sql(deadline, replace),
        Err(CacheError::Busy)
    ));
    #[expect(
        clippy::print_stderr,
        reason = "Report measured SQLite admission without asserting machine speed"
    )]
    eprintln!(
        "SQLite held-writer admission: budget={:?}, elapsed={:?}; replay time unchanged",
        deadline.remaining(),
        admitted.elapsed()
    );
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

#[test]
fn required_write_reserves_before_snapshot_and_preserves_existing_reader() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("cache.sqlite");
    let cache = Cache::open_with_owners(&path, &[]).unwrap();
    let reader = Connection::open(&path).unwrap();
    reader
        .execute_batch(
            "CREATE TABLE admission(value TEXT); INSERT INTO admission VALUES('old'); BEGIN",
        )
        .unwrap();
    let value = |connection: &Connection| {
        connection
            .query_row("SELECT value FROM admission", [], |row| {
                row.get::<_, String>(0)
            })
            .unwrap()
    };
    assert_eq!(value(&reader), "old");
    let publisher = cache.owned_connection().unwrap();
    let transaction = super::write_transaction(&publisher).unwrap();
    assert_eq!(value(&transaction), "old");
    // This write occurs after the publisher's first read and before its first
    // mutation. Deferred admission lets it commit and invalidates that snapshot.
    let competitor = Connection::open(&path).unwrap();
    competitor.busy_timeout(Duration::ZERO).unwrap();
    let competing = competitor.execute("UPDATE admission SET value='competing'", []);
    transaction
        .execute("UPDATE admission SET value='new'", [])
        .expect("required writer must not upgrade a stale WAL snapshot");
    transaction.commit().unwrap();
    assert!(matches!(
        competing.map_err(super::db_error),
        Err(CacheError::Busy)
    ));
    assert_eq!(value(&reader), "old");
    reader.execute_batch("COMMIT").unwrap();
    assert_eq!(value(&reader), "new");
}
