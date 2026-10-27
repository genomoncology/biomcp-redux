//! Feature variant-identity: printed repeat keys retain their required assembly.
use crate::runner::{configuration::invoke, mcp, network_stub::Stub, schema::Registry};
use serde_json::{Value, json};
use std::io::Write;

const REPEAT: &str = "chr19:g.11106928AAG[1]";

pub(super) fn check() -> Result<(), String> {
    let stub = Stub::for_source("myvariant.info", "application/json", body)?;
    let env = super::source_environment(&stub)?;
    let args = ["get", "variant", "rs876657378"]
        .into_iter()
        .map(str::to_owned)
        .collect::<Vec<_>>();
    let card = invoke(&env, args.clone(), 0)?;
    Registry::embedded()?.validate(&card, &args, true)?;
    if mcp::call_with_globals(&env, &args, false, &[])? != card {
        return Err("repeat card differs over MCP".into());
    }
    identity(&card)?;
    // Ceilings are ceil(comparable bytes * 1.15), measured in j16521.
    bounded("rsID", &card, 757)?;
    let next = card
        .get("next")
        .and_then(Value::as_array)
        .ok_or("printed repeat key has no executable command with assembly")?;
    let expected = json!(["get", "variant", REPEAT, "--build", "GRCh37"]);
    if !next.iter().any(|step| step.get("args") == Some(&expected)) {
        return Err(format!(
            "printed repeat spelling has no exact assembly-qualified get: {next:?}"
        ));
    }
    for step in next {
        let args: Vec<String> = serde_json::from_value(
            step.get("args")
                .cloned()
                .ok_or("next command lacks arguments")?,
        )
        .map_err(|e| e.to_string())?;
        let followed = invoke(&env, args.clone(), 0)?;
        identity(&followed)?;
        if step.get("kind") == Some(&json!("disambiguate")) {
            bounded("repeat", &followed, 717)?;
        }
        if followed
            .get("next")
            .and_then(Value::as_array)
            .is_some_and(|steps| {
                steps
                    .iter()
                    .any(|step| step.get("args") == Some(&json!(args)))
            })
        {
            return Err("exact repeat get prints a self-loop".into());
        }
        Registry::embedded()?.validate(&followed, &args, true)?;
        if mcp::call_with_globals(&env, &args, false, &[])? != followed {
            return Err("printed repeat command differs over MCP".into());
        }
    }
    let requests = stub.finish()?;
    if requests.len() != 2
        || !requests.iter().any(|(_, request)| {
            request.starts_with("GET /v1/variant/chr19%3Ag.11106928AAG%5B1%5D?")
        })
    {
        return Err(format!(
            "repeat was rewritten or warm reads fetched again: {requests:?}"
        ));
    }
    Ok(())
}

fn bounded(label: &str, card: &Value, ceiling: usize) -> Result<(), String> {
    let full = serde_json::to_vec(card).map_err(|e| e.to_string())?.len();
    let comparable = crate::runner::source_terms::size(card, "")?;
    writeln!(
        std::io::stdout(),
        "repeat {label}: full={full}, comparable={comparable}"
    )
    .map_err(|e| e.to_string())?;
    if comparable > ceiling {
        return Err(format!(
            "repeat {label} exceeds measured budget: {comparable}/{ceiling}"
        ));
    }
    Ok(())
}

fn identity(card: &Value) -> Result<(), String> {
    if card.pointer("/result/resolution") != Some(&json!("resolved"))
        || card.pointer("/result/candidates/0/record/id") != Some(&json!(REPEAT))
        || card.pointer("/result/candidates/0/identity/ids") != Some(&json!(["rs876657378"]))
        || card.pointer("/result/candidates/0/identity/representations/0/expression/value")
            != Some(&json!(REPEAT))
    {
        return Err(format!("repeat identity changed: {card}"));
    }
    Ok(())
}

fn body(request: &str) -> Result<String, String> {
    // Explicit synthetic transport fixture; native source truth is captured separately.
    let hit = json!({"_id":REPEAT,"dbsnp":{"rsid":"rs876657378"}});
    Ok(if request.starts_with("GET /v1/query?") {
        json!({"total":1,"hits":[hit]})
    } else {
        hit
    }
    .to_string())
}
