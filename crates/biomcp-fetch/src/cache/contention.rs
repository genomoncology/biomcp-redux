//! Real SQLite locking cannot be driven by the provider's virtual clock.
//! This inner concurrency contract checks bounded refusal and retained data;
//! outside-in concurrent replacement exercises the complete publication path.
use super::{Cache, CacheError, Connection, Duration};
use crate::{Deadline, deadline::VirtualClock};

#[test]
#[expect(
    clippy::disallowed_methods,
    reason = "Join the sibling SQL probe before releasing its isolated cache"
)]
fn discretionary_read_waits_for_sibling_sql() {
    let directory = tempfile::tempdir().unwrap();
    let cache = Cache::open_with_owners(&directory.path().join("cache.sqlite"), &[]).unwrap();
    let owner = cache.0.lock().unwrap();
    let (started, starting) = std::sync::mpsc::channel();
    std::thread::scope(|scope| {
        let reader = scope.spawn(|| {
            started.send(()).unwrap();
            cache.try_sql(
                Deadline::start(Duration::from_secs(5), None),
                |connection| {
                    let busy: u64 = connection
                        .query_row("PRAGMA busy_timeout", [], |row| row.get(0))
                        .unwrap();
                    assert_eq!(busy, 0, "external SQLite contention must remain immediate");
                    Ok(42)
                },
            )
        });
        starting.recv().unwrap();
        drop(owner);
        assert_eq!(reader.join().unwrap(), Ok(42));
    });
}

#[test]
#[expect(
    clippy::disallowed_types,
    reason = "Bind the held-lock probe to the command cancellation flag"
)]
fn discretionary_read_refuses_expired_or_cancelled_local_wait() {
    use std::sync::{Arc, atomic::AtomicBool};
    let directory = tempfile::tempdir().unwrap();
    let cache = Cache::open_with_owners(&directory.path().join("cache.sqlite"), &[]).unwrap();
    let _owner = cache.0.lock().unwrap();
    let cancelled = Arc::new(AtomicBool::new(true));
    for deadline in [
        Deadline::start(Duration::ZERO, None),
        Deadline::start(Duration::from_secs(5), None).with_cancellation(Some(&cancelled)),
    ] {
        assert_eq!(
            cache.try_sql(deadline, |_| -> Result<(), CacheError> {
                panic!("expired local wait reached SQL")
            }),
            Err(CacheError::Busy)
        );
    }
}

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
