//! The process argument boundary.
use std::ffi::OsString;

#[expect(clippy::disallowed_methods, reason = "Arguments enter only at the command boundary")]
pub(crate) fn schema_name() -> Option<OsString> {
    let mut arguments = std::env::args_os().skip(1);
    let command = arguments.next()?;
    let name = arguments.next()?;
    (command == "schema" && arguments.next().is_none()).then_some(name)
}
