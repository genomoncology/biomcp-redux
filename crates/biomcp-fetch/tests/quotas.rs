//! Public quota declaration boundaries and shared provider behavior.
use biomcp_fetch::{Quota, Rate, SourceKey};

#[test]
fn quota_declarations_reject_empty_invalid_and_zero_limits() {
    for key in ["", "Upper", "space here", "slash/name", "é"] {
        assert!(std::panic::catch_unwind(|| SourceKey::declare(key)).is_err());
    }
    for (rate, burst) in [(0, 1), (1, 0), (0, 0)] {
        assert!(std::panic::catch_unwind(|| Rate::new(rate, burst)).is_err());
    }
    assert!(std::panic::catch_unwind(|| Quota::new(SourceKey::declare("valid-1"), Rate::per_second(1), None, 0)).is_err());
    const VALID: Quota = Quota::new(SourceKey::declare("valid-1"), Rate::new(u16::MAX, u16::MAX), Some(Rate::per_second(1)), u8::MAX);
    assert!(format!("{VALID:?}").contains("valid-1"));
}
