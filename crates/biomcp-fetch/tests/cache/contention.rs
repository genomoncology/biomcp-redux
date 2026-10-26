//! Cache-resilience regressions through public source legs.
use super::*;

#[derive(serde::Deserialize)]
struct Absence {
    absent: bool,
}
impl Reply for Absence {
    fn check(&self) -> Result<(), BadReply> {
        if self.absent { Ok(()) } else { Err(BadReply) }
    }
    fn check_status(&self, status: u16) -> Result<(), BadReply> {
        if status == 404 {
            self.check()
        } else {
            Err(BadReply)
        }
    }
}

#[test]
fn busy_read_is_a_miss_before_the_live_deadline() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("cache.sqlite");
    let server = Server::new();
    let spec = spec(&server, KeyUse::None);
    let fetch = Fetch::open(config(&path)).unwrap();
    let endpoint = spec.get("/answer").fresh(Freshness::Hours(1));
    let deadline = fetch.deadline(Duration::from_secs(5));
    let access = fetch.cache_access().unwrap();
    let report = thread::scope(|scope| {
        let worker = access
            .admit(deadline, || {
                let worker = scope.spawn(|| {
                    let leg = fetch.leg(spec, deadline);
                    assert!(leg.json::<Answer>(&endpoint, &[]).unwrap().answer);
                    leg.finish()
                });
                // Observing HTTP while the cache is still held proves the read did
                // not spend the source budget waiting for the cache connection.
                server
                    .requests
                    .recv_timeout(Duration::from_secs(10))
                    .unwrap()
                    .ok();
                Ok(worker)
            })
            .unwrap();
        worker.join().unwrap()
    });
    assert_eq!(report.cache_problems, [CacheError::Busy]);
    assert!(report.failures.is_empty());
    assert!(!report.stale);
}

#[test]
fn negative_admission_and_json_absence_removal_keep_the_source_answer() {
    for json_absence in [false, true] {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("cache.sqlite");
        let server = Server::new();
        let spec = spec(&server, KeyUse::None);
        let fetch = Fetch::open(config(&path)).unwrap();
        let mut endpoint = spec.get("/answer").fresh(Freshness::Hours(1));
        exchange(&server, 200, r#"{"answer":true}"#, || {
            get(&fetch, spec, &endpoint, &[])
        })
        .unwrap();
        if json_absence {
            endpoint = endpoint.json_absence();
        }
        let connection = Connection::open(&path).unwrap();
        connection.execute_batch("BEGIN IMMEDIATE").unwrap();
        let (result, report) = exchange(&server, 404, r#"{"absent":true}"#, || {
            let leg = fetch
                .leg(spec, fetch.deadline(Duration::from_millis(500)))
                .fresh(true);
            let result = if json_absence {
                leg.json::<Absence>(&endpoint, &[])
                    .map(|reply| reply.absent)
            } else {
                leg.json::<Answer>(&endpoint, &[]).map(|reply| reply.answer)
            };
            (result, leg.finish())
        });
        if json_absence {
            assert!(result.unwrap());
        } else {
            assert_eq!(source(result.unwrap_err()).reason, Reason::NotFound);
        }
        assert_eq!(report.cache_problems, [CacheError::Busy]);
        assert!(report.failures.is_empty());
        assert!(!report.stale);
    }
}

#[test]
fn failed_invalidation_removal_cannot_reuse_an_unchecked_stale_entry() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("cache.sqlite");
    let server = Server::new();
    let spec = spec(&server, KeyUse::None);
    let fetch = Fetch::open(config(&path)).unwrap();
    let endpoint = spec.get("/answer").fresh(Freshness::Hours(1));
    exchange(&server, 200, r#"{"answer":true}"#, || {
        get(&fetch, spec, &endpoint, &[])
    })
    .unwrap();
    let connection = Connection::open(&path).unwrap();
    connection
        .execute(
            "UPDATE reply SET body=?1, fresh_until=0",
            [b"{}".as_slice()],
        )
        .unwrap();
    connection.execute_batch("BEGIN IMMEDIATE").unwrap();
    let (result, report) = exchange(&server, 400, "rejected", || {
        let leg = fetch.leg(spec, fetch.deadline(Duration::from_secs(2)));
        let result = leg.json::<Answer>(&endpoint, &[]);
        (result, leg.finish())
    });
    assert_eq!(source(result.unwrap_err()).reason, Reason::Rejected);
    assert_eq!(report.cache_problems, [CacheError::Busy]);
    assert!(!report.stale);
}
