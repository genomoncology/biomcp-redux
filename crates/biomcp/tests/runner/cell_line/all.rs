//! Feature cell-line-all: bounded independent first pages with visible source outcomes.
use super::{
    Environment, Stub, Trial, Value, canonical, chembl, fixture, fs, invoke, json, path, sections,
    setup,
};
use crate::runner::transcript::trial as transcript_trial;
mod controls;
const MEASURE: bool = false;
use super::chembl::surface;
pub(super) fn trials() -> Vec<Trial> {
    let mut trials = vec![transcript_trial(
        "cell-line-all/bounded-first-pages",
        || native().map_err(Into::into),
    )];
    trials.extend(controls::trials());
    trials
}
fn reply(request: &str) -> Result<String, String> {
    let target = request.split_whitespace().nth(1).ok_or("target")?;
    if target.starts_with("/chembl/") {
        return chembl::reply(request);
    }
    if target.starts_with("/release-info?") {
        return fixture("portable-cellosaurus-cellosaurus-release.json");
    }
    for id in ["CVCL_2119", "CVCL_0004"] {
        if canonical(target)
            == canonical(&format!(
                "/cell-line/{id}?format=json&fields=ac%2Cid%2Csy%2Cox%2Cdi%2Cca%2Csx%2Cag%2Cdr%2Cvar%2Cprob%2Ccaution%2Cdt"
            ))
        {
            return fixture(&format!("portable-cellosaurus-cell-{id}.json"));
        }
    }
    Err(format!("unproved all request: {target}"))
}
fn stub() -> Result<Stub, String> {
    Stub::for_sources(
        &["api.cellosaurus.org", "www.ebi.ac.uk"],
        |_| "application/json",
        reply,
    )
}
fn native() -> Result<(), String> {
    let stub = stub()?;
    let env = setup(&stub)?;
    for id in ["CVCL_2119", "CVCL_0004"] {
        let args = ["get", "cell-line", id, "all"];
        let phase = format!("all/native/{id}");
        let value = parity(&phase, &env, &args, 0)?;
        for section in ["names", "identity", "xrefs", "variants", "chembl"] {
            path(
                &value,
                &format!("/result/sections/{section}/state"),
                json!("data"),
            )?;
        }
        facts(id, &value)?;
        sections::markdown(
            &phase,
            &env,
            &args,
            &[
                "--replay".into(),
                env.root()
                    .join("recordings")
                    .join(&phase)
                    .to_str()
                    .ok_or("path")?
                    .into(),
            ],
            &value,
            MEASURE,
        )?;
    }
    let requests = stub.finish()?;
    if requests.len() != 8 || requests.iter().any(|(_, r)| r.contains("/assay.json")) {
        return Err(format!("all fanout changed: {}", requests.len()));
    }
    Ok(())
}

fn facts(id: &str, value: &Value) -> Result<(), String> {
    let native: Value =
        serde_json::from_str(&fixture(&format!("portable-cellosaurus-cell-{id}.json"))?)
            .map_err(|e| e.to_string())?;
    let row = native
        .pointer("/Cellosaurus/cell-line-list/0")
        .ok_or("native row")?;
    for section in ["names", "identity", "xrefs", "variants"] {
        let page = json!({"result": value.pointer(&format!("/result/sections/{section}/data")).ok_or("section data")?});
        sections::expected::page(
            row,
            section,
            &page,
            0,
            if section == "variants" { 3 } else { 5 },
        )?;
    }
    chembl::native_facts(
        &json!({"result":value.pointer("/result/sections/chembl/data").ok_or("chembl data")?}),
        id,
    )?;
    path(value, "/result/release/version", json!("57.0"))?;
    path(
        value,
        "/result/sections/chembl/data/release/version",
        json!("ChEMBL_37"),
    )?;
    path(
        value,
        "/result/sections/chembl/data/release/updated",
        json!("2026-05-01"),
    )?;
    if value.pointer("/result/release/updated")
        == value.pointer("/result/sections/chembl/data/release/updated")
    {
        return Err("source dates were conflated".into());
    }
    let actions = value.get("next").and_then(Value::as_array).ok_or("next")?;
    if !actions.iter().any(|action| {
        action
            .get("args")
            .and_then(Value::as_array)
            .is_some_and(|args| {
                args.contains(&json!("xrefs")) && args.contains(&json!("--section-offset"))
            })
    }) {
        return Err("all lost xrefs continuation".into());
    }
    Ok(())
}

fn parity(phase: &str, env: &Environment, args: &[&str], exit: i32) -> Result<Value, String> {
    let value = surface::parity(phase, env, args, exit, MEASURE)?;
    let raw = fs::read(env.root().join("mcp-raw").join(phase).join("mcp.stdout"))
        .map_err(|e| e.to_string())?;
    surface::measure_bytes(phase, "mcp-rpc", raw.len(), MEASURE)?;
    Ok(value)
}
