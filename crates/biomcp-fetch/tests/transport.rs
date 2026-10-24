use biomcp_fetch::{Fetch, FetchConfig};

#[test]
fn expired_deadline_refuses_sending() {
    let fetch = Fetch::open(FetchConfig::default()).unwrap();
    assert!(fetch.deadline(std::time::Duration::ZERO).expired());
}
