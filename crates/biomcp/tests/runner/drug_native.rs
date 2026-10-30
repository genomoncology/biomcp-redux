//! Feature drug-native-sources: retained native bodies delivered locally through real record and replay.
//! Original source dates and request identities remain in fixture provenance; delivery is not a live provider check.
use super::{configuration::invoke, launch::Environment, network_stub::Stub};
use crate::runner::transcript::trial as transcript_trial;
use libtest_mimic::Trial;
use serde::Deserialize;
use serde_json::{Map, Value, json};
use std::{fs, path::PathBuf};
type Check = Result<(), String>;
#[derive(Clone, Deserialize)]
struct Case {
    name: String,
    args: Vec<String>,
    requests: Vec<Reply>,
    expect: Map<String, Value>,
    missing: Vec<String>,
    exit: i32,
    max_bytes: usize,
    max_comparable: usize,
    max_tokens: usize,
    max_comparable_tokens: usize,
}
#[derive(Clone, Deserialize)]
struct Reply {
    file: String,
    request: String,
    json: Option<Value>,
}
pub(super) fn fixture_root() -> PathBuf {
    crate::runner::launch::fixtures().join("fixtures/drug/native")
}
fn plan() -> Result<Vec<Case>, String> {
    serde_json::from_str(&crate::runner::launch::fixture_text(
        "fixtures/drug/native/plan.json",
    )?)
    .map_err(|e| e.to_string())
}
pub(super) fn trials() -> Vec<Trial> {
    let mut trials: Vec<Trial> = match plan() {
        Ok(cases) => cases
            .into_iter()
            .map(|case| {
                transcript_trial(format!("drug-native-sources/{}", case.name), move || {
                    check(case.clone()).map_err(Into::into)
                })
            })
            .collect(),
        Err(error) => vec![transcript_trial(
            "drug-native-sources/fixture-plan",
            move || Err(error.clone().into()),
        )],
    };
    if let Some(root) = std::env::var_os("BIOMCP_CHEMBL_DEFAULT_RECORDING") {
        trials.push(transcript_trial(
            "drug-native-sources/chembl-default-retained",
            move || default_retained(PathBuf::from(&root)).map_err(Into::into),
        ));
    }
    trials
}
fn default_retained(root: PathBuf) -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let args = [
        "search",
        "drug",
        "axicabtagene ciloleucel",
        "--source",
        "chembl",
    ]
    .map(str::to_owned)
    .to_vec();
    let globals = env.globals(Some(&root), false);
    let mut cli = globals.clone();
    cli.extend(args.clone());
    let value = invoke(&env, cli, 0)?;
    super::schema::Registry::embedded()?.validate(&value, &args, true)?;
    if super::mcp::call_with_globals(&env, &args, false, &globals)? != value {
        return Err("retained ChEMBL default CLI/MCP mismatch".into());
    }
    for (pointer, expected) in [
        ("/query/limit", json!(10)),
        ("/query/source", json!("chembl")),
        ("/page/returned", json!(1)),
        ("/page/total", json!(1)),
        ("/result/items/0/id", json!("CHEMBL3989989")),
    ] {
        if value.pointer(pointer) != Some(&expected) {
            return Err(format!(
                "retained ChEMBL default changed {pointer}: {value}"
            ));
        }
    }
    let full = serde_json::to_vec(&value).map_err(|e| e.to_string())?.len();
    let comparable = super::source_terms::size(&value, "")?;
    if full > 756 || comparable > 633 {
        return Err(format!(
            "retained ChEMBL default exceeds measured ceiling: {full}/{comparable}"
        ));
    }
    use std::io::Write;
    writeln!(
        std::io::stdout(),
        "CHEMBL_DEFAULT_REPLAY full={full} comparable={comparable} caps=756/633"
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}
fn peer(replies: Vec<Reply>) -> Result<Stub, String> {
    let host = if replies.first().is_some_and(|r| r.json.is_some()) {
        "api.platform.opentargets.org"
    } else {
        "www.ebi.ac.uk"
    };
    Stub::for_source_status(
        host,
        "application/json",
        move |request| {
            let (header, body) = request
                .split_once("\r\n\r\n")
                .ok_or("missing HTTP header boundary")?;
            for reply in &replies {
                if header.lines().next() != Some(reply.request.as_str()) {
                    continue;
                }
                if let Some(expected) = &reply.json {
                    let actual: Value = serde_json::from_str(body).map_err(|e| e.to_string())?;
                    if &actual != expected {
                        continue;
                    }
                }
                return fs::read_to_string(fixture_root().join(&reply.file))
                    .map_err(|e| e.to_string());
            }
            Err(format!("undeclared native request: {request}"))
        },
        |_| 200,
    )
}
fn check(case: Case) -> Check {
    let stub = peer(case.requests.clone())?;
    let env = Environment::new().map_err(|e| e.to_string())?;
    let certificate = env.root().join("ca.pem");
    stub.retain(&env);
    fs::write(&certificate, &stub.certificate).map_err(|e| e.to_string())?;
    env.config(&json!({"ca_bundle":certificate,"proxy":format!("http://{}",stub.address)}))
        .map_err(|e| e.to_string())?;
    let recording = env.root().join("recording");
    fs::create_dir(&recording).map_err(|e| e.to_string())?;
    let mut action = case.args.clone();
    action.push("--fresh".into());
    let mut args = action.clone();
    args.extend(["--record".into(), recording.to_string_lossy().into()]);
    let value = invoke(&env, args, case.exit)?;
    super::schema::Registry::embedded()?.validate(&value, &case.args, true)?;
    assertions(&case, &value)?;
    let globals = vec!["--replay".into(), recording.to_string_lossy().into()];
    let mut replay = action.clone();
    replay.extend(globals.clone());
    if invoke(&env, replay, case.exit)? != value {
        return Err("CLI replay changed native response".into());
    }
    if super::mcp::call_with_globals(&env, &action, case.exit >= 2, &globals)? != value {
        return Err("MCP replay changed native response".into());
    }
    let requests = stub.finish()?;
    if requests.len() != case.requests.len() {
        return Err(format!(
            "native request count changed: {} / {}",
            requests.len(),
            case.requests.len()
        ));
    }
    measure(&case, &value)
}
fn assertions(case: &Case, value: &Value) -> Check {
    for (pointer, expected) in &case.expect {
        if value.pointer(pointer) != Some(expected) {
            return Err(format!(
                "{}: {pointer}: expected {expected}, got {:?}",
                case.name,
                value.pointer(pointer)
            ));
        }
    }
    for pointer in &case.missing {
        if value.pointer(pointer).is_some() {
            return Err(format!("{}: unexpected {pointer}", case.name));
        }
    }
    if case.args.first().is_some_and(|arg| arg == "search")
        && let Some(items) = value.pointer("/result/items").and_then(Value::as_array)
    {
        let next = value.get("next").and_then(Value::as_array).ok_or("next")?;
        for (index, item) in items.iter().enumerate() {
            let step = next.get(index).ok_or("candidate get was dropped")?;
            if step["kind"] != "disambiguate" || step.pointer("/args/2") != item.get("id") {
                return Err("candidate gets lost native order or moved behind paging".into());
            }
        }
    }
    let page = value.get("page");
    if let Some(page) = page {
        let offset = page["offset"].as_u64().ok_or("offset")?;
        let returned = page["returned"].as_u64().ok_or("returned")?;
        let total = page["total"].as_u64().ok_or("total")?;
        let next = value.get("next").and_then(Value::as_array);
        let page_next = next.and_then(|items| items.iter().find(|n| n["kind"] == "page"));
        if returned > 0 && offset + returned < total {
            let args = page_next
                .and_then(|p| p["args"].as_array())
                .ok_or_else(|| format!("missing page continuation: {}", value))?;
            for flag in ["--source", "--name-selector", "--section-limit"] {
                if let Some(index) = case.args.iter().position(|s| s == flag) {
                    let actual = args
                        .iter()
                        .position(|s| s == flag)
                        .and_then(|i| args.get(i + 1))
                        .ok_or("lost page selector")?;
                    if actual
                        != case
                            .args
                            .get(index + 1)
                            .ok_or("missing case selector value")?
                    {
                        return Err(format!("changed next {flag}"));
                    }
                }
            }
            let flag = if case.args.first().is_some_and(|s| s == "search") {
                "--offset"
            } else {
                "--section-offset"
            };
            let actual = args
                .iter()
                .position(|s| s == flag)
                .and_then(|i| args.get(i + 1))
                .and_then(Value::as_str)
                .ok_or("missing next offset")?;
            if actual != (offset + returned).to_string() {
                return Err("incorrect next offset".into());
            }
        } else if page_next.is_some() {
            return Err("unexpected next page".into());
        }
    }
    Ok(())
}
fn measure(case: &Case, value: &Value) -> Check {
    let bytes = serde_json::to_vec(value).map_err(|e| e.to_string())?.len();
    let comparable = super::source_terms::size(value, "")?;
    if bytes > case.max_bytes
        || comparable > case.max_comparable
        || bytes.div_ceil(4) > case.max_tokens
        || comparable.div_ceil(4) > case.max_comparable_tokens
    {
        return Err(format!(
            "{}: native output budget exceeded {bytes}/{} full, {comparable}/{} comparable",
            case.name, case.max_bytes, case.max_comparable
        ));
    }
    use std::io::Write;
    writeln!(
        std::io::stdout(),
        "NATIVE_SIZE {} {bytes} {comparable}",
        case.name
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}
