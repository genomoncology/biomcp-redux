//! P24 separates source-local empty, exact absence and source failure.
use super::{Environment, Stub, invoke, native, record, setup};
use libtest_mimic::Trial;
use serde_json::{Value, json};
const STATE_BYTES: usize = 6000;
pub(super) fn trials() -> Vec<Trial> {
    let mut rows = vec![Trial::test(
        "phenotype-identity/P24-synthetic-empty-absent-failure-states",
        || states().map_err(Into::into),
    )];
    rows.push(Trial::test(
        "phenotype-identity/synthetic-short-limit-keeps-native-page",
        || short_limit().map_err(Into::into),
    ));
    for source in ["hpo", "monarch"] {
        rows.push(Trial::test(
            format!("phenotype-identity/synthetic-disabled-{source}-fresh-before-cache"),
            move || disabled(source).map_err(Into::into),
        ));
    }
    rows
}
fn states() -> Result<(), String> {
    let mut observed = Vec::new();
    for kind in ["empty", "absent", "failure"] {
        let stub = Stub::for_source_status(
            "api-v3.monarchinitiative.org",
            "application/json",
            move |_| {
                Ok(if kind == "empty" {
                    json!({"limit":5,"offset":0,"total":0,"items":[]})
                } else {
                    json!({"error":"synthetic source error"})
                }
                .to_string())
            },
            move |_| {
                if kind == "absent" {
                    404
                } else if kind == "failure" {
                    500
                } else {
                    200
                }
            },
        )?;
        let env = setup(&stub)?;
        let args = if kind == "empty" {
            vec!["search", "phenotype", "synthetic empty"]
        } else {
            vec!["get", "phenotype", "HP:9999999", "--source", "monarch"]
        };
        let directory = env.root().join("state");
        let result = record(
            &env,
            &args,
            &directory,
            if kind == "failure" { 4 } else { 1 },
        )?;
        native::mcp(
            &directory,
            &args,
            &result,
            if kind == "failure" { 4 } else { 1 },
            Some(STATE_BYTES),
        )?;
        if kind == "empty" {
            if result.pointer("/page/total") != Some(&json!(0))
                || result.pointer("/result/items") != Some(&json!([]))
            {
                return Err(format!("successful empty search lost: {result}"));
            }
        } else if result.get("result").is_some() {
            return Err(format!("absence or failure invented a result: {result}"));
        }
        observed.push((
            result.get("status").cloned(),
            result.pointer("/sources/0/state").cloned(),
            result.pointer("/sources/0/reason").cloned(),
        ));
        if stub.finish()?.is_empty() {
            return Err("synthetic state made no request".into());
        }
    }
    if observed[0] == observed[1] || observed[0] == observed[2] || observed[1] == observed[2] {
        return Err(format!("P24 states collapsed: {observed:?}"));
    }
    if observed[0].0 != Some(json!("absent"))
        || observed[1].0 != Some(json!("absent"))
        || observed[2].0 != Some(json!("failed"))
    {
        return Err(format!("P24 statuses changed: {observed:?}"));
    }
    Ok(())
}
fn disabled(source: &str) -> Result<(), String> {
    let stub = Stub::for_source(
        if source == "hpo" {
            "ontology.jax.org"
        } else {
            "api-v3.monarchinitiative.org"
        },
        "application/json",
        |_| Ok(super::node().to_string()),
    )?;
    let env = setup(&stub)?;
    let words = ["get", "phenotype", "HP:0000001", "--source", source];
    let directory = env.root().join("warm");
    record(&env, &words, &directory, 0)?;
    let mut args = words.to_vec();
    args.extend(["--fresh", "--disable-sources", source]);
    let result = invoke(&env, &args, 4)?;
    if result.pointer("/sources/0/state") != Some(&json!("skipped"))
        || result.pointer("/sources/0/reason") != Some(&json!("disabled"))
        || result.get("result").is_some()
    {
        return Err(format!("disabled fresh leaked cache: {result}"));
    }
    native::mcp(&directory, &args, &result, 4, Some(STATE_BYTES))?;
    if stub.finish()?.len() != 1 {
        return Err("disabled fresh reached source".into());
    }
    // A new isolated environment also skips without requiring any populated cache.
    let cold = Environment::new().map_err(|e| e.to_string())?;
    let cold_result: Value = invoke(&cold, &args, 4)?;
    if cold_result.pointer("/sources/0/reason") != Some(&json!("disabled")) {
        return Err("cold disabled source touched cache".into());
    }
    Ok(())
}

fn short_limit() -> Result<(), String> {
    let stub = Stub::for_source(
        "api-v3.monarchinitiative.org",
        "application/json",
        |request| {
            if !request.contains("&limit=3&offset=0 ") {
                return Err(format!("short limit changed native request: {request}"));
            }
            Ok(json!({"limit":3,"offset":0,"total":0,"items":[]}).to_string())
        },
    )?;
    let env = setup(&stub)?;
    let args = ["search", "phenotype", "synthetic", "-l", "3"];
    let directory = env.root().join("alias");
    let result = record(&env, &args, &directory, 1)?;
    if result.pointer("/query/limit") != Some(&json!(3)) {
        return Err(format!("short limit lost: {result}"));
    }
    native::mcp(&directory, &args, &result, 1, Some(STATE_BYTES))?;
    if stub.finish()?.len() != 1 {
        return Err("short limit request count".into());
    }
    Ok(())
}
