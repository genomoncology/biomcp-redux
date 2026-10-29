//! Feature adverse-event-search: a native absence body requires its native HTTP status.
use super::{Check, args, both, environment};
use serde_json::json;
pub(super) fn optional_date() -> Check {
    let stub = super::Stub::for_sources(
        &["api.fda.gov"],
        |_| "application/json",
        |_| {
            let mut reply = super::body(5, 0);
            *reply
                .pointer_mut("/meta/last_updated")
                .ok_or("missing date fixture")? = json!("2024-02-30");
            Ok(reply.to_string())
        },
    )?;
    let env = environment(&stub)?;
    let value = both(&env, args(), 0)?;
    if value.pointer("/result/items/0/id") != Some(&json!("100"))
        || value.pointer("/result/source_updated").is_some()
        || value.pointer("/problems/0/field") != Some(&json!("meta.last_updated"))
        || value.pointer("/problems/0/code") != Some(&json!("value.unknown"))
    {
        return Err(format!(
            "optional source date discarded usable reports: {value}"
        ));
    }
    stub.finish()?;
    Ok(())
}
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
