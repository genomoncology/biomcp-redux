//! Feature adverse-event-search: a native absence body requires its native HTTP status.
use super::{Check, args, both, environment};
use serde_json::json;
pub(super) fn run() -> Check {
    let stub = super::Stub::for_source_status(
        "api.fda.gov",
        "application/json",
        |_| Ok(r#"{"error":{"code":"NOT_FOUND","message":"No matches found!"}}"#.into()),
        |_| 200,
    )?;
    let env = environment(&stub)?;
    let value = both(&env, args(), 4)?;
    if value.get("result").is_some()
        || value.pointer("/sources/0/reason") != Some(&json!("bad-reply"))
    {
        return Err("HTTP 200 error body became report absence".into());
    }
    if stub.finish()?.len() != 2 {
        return Err("invalid 200 body entered ordinary cache".into());
    }
    Ok(())
}
