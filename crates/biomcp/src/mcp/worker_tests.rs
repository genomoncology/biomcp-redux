//! A public request cannot intentionally break a worker invariant.
#[test]
fn panicking_worker_completes_with_internal_error() {
    let outcome: std::io::Result<()> = super::execute(|| panic!("injected worker failure"));
    assert!(outcome.is_err(), "panic did not become an internal error");
}
