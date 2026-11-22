//! Default typed tool discovery and its advertised schemas against recorded replies.
use super::super::super::{
    launch,
    mcp::{exchange_with_globals, request},
    response_cases,
};
use launch::Environment;
use serde_json::{Value, json};

pub(super) type Validators<'a> = std::collections::BTreeMap<&'a str, jsonschema::Validator>;

pub(super) fn check<'a>(env: &Environment, tools: &'a [Value]) -> Result<Validators<'a>, String> {
    let validators = reply_validators(tools)?;
    replay_replies(env, &validators)?;
    Ok(validators)
}

pub(super) fn validate(
    validators: &Validators<'_>,
    tool: &str,
    label: &str,
    reply: &Value,
) -> Result<(), String> {
    validators
        .get(tool)
        .ok_or_else(|| format!("{label}: tool schema missing"))?
        .validate(reply)
        .map_err(|error| format!("{label}: output schema rejected reply: {error}"))
}

fn reply_validators(tools: &[Value]) -> Result<Validators<'_>, String> {
    tools
        .iter()
        .map(|tool| {
            let name = tool
                .get("name")
                .and_then(Value::as_str)
                .ok_or("tool name")?;
            let schema = tool
                .get("outputSchema")
                .ok_or_else(|| format!("{name}: missing default outputSchema"))?;
            let validator = jsonschema::validator_for(schema)
                .map_err(|error| format!("{name}: invalid outputSchema: {error}"))?;
            Ok((name, validator))
        })
        .collect()
}

fn replay_replies(env: &Environment, validators: &Validators<'_>) -> Result<(), String> {
    for (name, args, capture) in [
        (
            "trial",
            json!({"action":"search","search":{
                "condition":"melanoma","intervention":"pembrolizumab"
            }}),
            Some("trial/intervention"),
        ),
        ("help", json!({"command":"search trial"}), None),
        ("biomcp", json!({"args":["help","search","trial"]}), None),
        (
            "search",
            json!({"query":"trial --condition melanoma --intervention pembrolizumab"}),
            Some("trial/intervention"),
        ),
        (
            "fetch",
            json!({"id":"trial NCT02296125"}),
            Some("trial/get/NCT02296125"),
        ),
    ] {
        let replay = response_cases::replay(env, capture, None)?;
        let globals = env.globals(replay.as_deref(), capture.is_none());
        let frame = request(
            json!(1),
            "tools/call",
            json!({"name":name,"arguments":args}),
        );
        let frames = exchange_with_globals(env, &[frame], &globals)?;
        let mut reply = frames
            .first()
            .and_then(|frame| frame.pointer("/result/structuredContent"))
            .cloned()
            .ok_or_else(|| format!("{name}: missing replayed reply"))?;
        let schema = validators
            .get(name)
            .ok_or_else(|| format!("{name}: tool schema missing"))?;
        schema
            .validate(&reply)
            .map_err(|error| format!("{name}: output schema rejected replayed reply: {error}"))?;
        if name == "trial" {
            *reply
                .pointer_mut("/result/items/0/title")
                .ok_or("missing replayed trial title")? = json!({"wrong_type":true});
            if schema.is_valid(&reply) {
                return Err("trial search schema accepted an object as a row title".into());
            }
        }
    }
    Ok(())
}
