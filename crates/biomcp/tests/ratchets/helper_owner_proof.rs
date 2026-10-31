//! Copied mapping and protocol helpers fail the source ownership contract.
use super::{Check, rejects};

#[test]
fn adapter_mapping_helpers_require_one_owner() -> Check {
    for declaration in [
        "fn bad() {}",
        "fn bad_reply() {}",
        "fn received_date() {}",
        "fn root() {}",
    ] {
        rejects(
            "crates/biomcp-adapters/src/example/map.rs",
            declaration,
            "copied helper",
        )?;
    }

    for declaration in ["fn problem() {}", "fn text() {}"] {
        rejects(
            "crates/biomcp-adapters/src/example/map.rs",
            declaration,
            "mapping helpers belong in mapped.rs",
        )?;
    }
    Ok(())
}
