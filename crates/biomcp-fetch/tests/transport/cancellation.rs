use super::*;

#[test]
fn cancellation_reaches_local_checkpoints_without_stopping_another_call() {
    let shared = Fetch::open(FetchConfig::default()).unwrap();
    let (call, cancellation) = shared.cancellable().unwrap();
    let (other, _) = shared.cancellable().unwrap();
    let deadline = call.deadline(Duration::from_secs(30));
    let local = deadline.local();
    let command = call.command_deadline(Duration::from_secs(30));
    cancellation.cancel();
    assert!(deadline.expired());
    assert!(local.expired());
    assert!(command.deadline().expired());
    assert!(!other.deadline(Duration::from_secs(30)).expired());
    assert!(!shared.deadline(Duration::from_secs(30)).expired());
}
