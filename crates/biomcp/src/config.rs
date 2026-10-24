//! The process argument boundary.
use std::ffi::OsString;

#[expect(
    clippy::disallowed_methods,
    reason = "Arguments enter only at the command boundary"
)]
pub(crate) fn arguments() -> Vec<OsString> {
    std::env::args_os().skip(1).collect()
}
