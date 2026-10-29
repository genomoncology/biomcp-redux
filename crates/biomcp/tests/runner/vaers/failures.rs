use super::{Check, environment, words};
use crate::runner::{configuration::invoke, launch::Environment, network_stub::Stub};
use serde_json::{Value, json};
pub(super) fn run() -> Check {
    let stub = Stub::for_source("wonder.cdc.gov", "application/xml", |_| {
        Ok("<synthetic-broken>".into())
    })?;
    let env = environment(&stub)?;
    let original = env.root().join("recording");
    let mut args = words("MMR");
    args.extend(["--record".into(), original.to_string_lossy().into_owned()]);
    invoke(&env, args, 4)?;
    if stub.finish()?.len() != 1 {
        return Err("broken table retried".into());
    }
    for (fault, reason) in [
        ("403", "rejected"),
        ("429", "rate-limited"),
        ("500", "unavailable"),
        ("timeout", "deadline-exceeded"),
        ("broken", "bad-reply"),
    ] {
        let dir = env.root().join(fault);
        crate::runner::adverse_event_failures::mutate(&original, &dir, fault)?;
        let replay = Environment::new().map_err(|e| e.to_string())?;
        let globals = replay.globals(Some(&dir), false);
        let mut args = globals.clone();
        args.extend(words("MMR"));
        let value = invoke(&replay, args, 4)?;
        crate::runner::schema::Registry::embedded()?.validate(&value, &words("MMR"), true)?;
        assert_failed(&value, reason)?;
        super::budget::check(&value)?;
        if crate::runner::mcp::call_with_globals(&replay, &words("MMR"), true, &globals)? != value {
            return Err("failure differs through MCP".into());
        }
    }
    Ok(())
}
fn assert_failed(value: &Value, reason: &str) -> Check {
    if value.get("status") != Some(&json!("failed"))
        || value.get("result").is_some()
        || value.pointer("/sources/0/source") != Some(&json!("vaers"))
        || value.pointer("/sources/0/state") != Some(&json!("failed"))
        || value.pointer("/sources/0/terms_url")
            != Some(&json!("https://wonder.cdc.gov/datause.html"))
        || value.pointer("/sources/0/reason") != Some(&json!(reason))
        || value.pointer("/sources/0/use") != Some(&json!("restricted"))
        || value.pointer("/sources/0/terms") != Some(&json!("CDC WONDER data use restrictions"))
    {
        return Err(format!("failed native source became an answer: {value}"));
    }
    Ok(())
}
