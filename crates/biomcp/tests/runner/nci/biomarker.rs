//! Feature biomarker-nci: retained native pages and conservative MCP projections.
use super::super::{launch::Environment, nci::invoke, response_cases::copy_capture};
use libtest_mimic::Trial;
use serde_json::{Value, json};
type Check = Result<(), String>;
pub(super) fn trials() -> Vec<Trial> {
    vec![
        Trial::test("biomarker/help-means-concepts", || help().map_err(Into::into)),
        Trial::test("biomarker/native-snapshot-pages", || {
            pages().map_err(Into::into)
        }),
        Trial::test("biomarker/bounded-failures", || {
            failures().map_err(Into::into)
        }),
        Trial::test("biomarker/research-keeps-native-rows", || {
            research().map_err(Into::into)
        }),
    ]
}
fn environment() -> Result<Environment, String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    env.config(&json!({"keys":{"nci_api_key":"synthetic-nci-stored-key"}}))
        .map_err(|e| e.to_string())?;
    Ok(env)
}
fn query() -> Vec<String> {
    [
        "search",
        "biomarker",
        "--source",
        "nci",
        "--name",
        "PD-L1",
        "--limit",
        "2",
    ]
    .map(str::to_owned)
    .to_vec()
}
fn pages() -> Check {
    let env = environment()?;
    let replay = super::super::launch::fixtures().join("replies/biomarker/native");
    let mut args = query();
    args.extend(env.globals(Some(&replay), false));
    let mut answer = invoke(&env, &args, 0)?;
    let mut rows = Vec::new();
    for _ in 0..19 {
        if answer.pointer("/page/total") != Some(&json!(37)) {
            return Err("native total changed while paging".into());
        }
        rows.extend(
            answer
                .pointer("/result/items")
                .and_then(Value::as_array)
                .ok_or("missing biomarker headers")?
                .iter()
                .cloned(),
        );
        let next = answer
            .get("next")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .find(|s| s.get("kind") == Some(&json!("page")));
        let Some(next) = next else {
            break;
        };
        let mut args: Vec<String> =
            serde_json::from_value(next.get("args").cloned().ok_or("page args")?)
                .map_err(|e| e.to_string())?;
        if !args.iter().any(|s| s == "--pool") {
            return Err("page lost its snapshot".into());
        }
        args.extend(env.globals(None, true));
        answer = invoke(&env, &args, 0)?;
    }
    let path = replay.join("nci");
    let mut expected = None;
    for file in std::fs::read_dir(path).map_err(|e| e.to_string())? {
        let file = file.map_err(|e| e.to_string())?;
        let record: Value =
            serde_json::from_slice(&std::fs::read(file.path()).map_err(|e| e.to_string())?)
                .map_err(|e| e.to_string())?;
        if record
            .get("url")
            .and_then(Value::as_str)
            .is_some_and(|s| s.ends_with("name=PD-L1"))
        {
            expected = Some(record.pointer("/exchanges/0/body/value/data").and_then(Value::as_array).ok_or("native rows")?.iter().map(|row| json!({"name":row["name"],"codes":row["codes"],"type":row["type"],"count":row["count"]})).collect::<Vec<_>>());
        }
    }
    if expected.as_ref() != Some(&rows) {
        return Err("offline pages changed, skipped or repeated native headers".into());
    }
    Ok(())
}
fn failures() -> Check {
    for kind in ["overflow", "bucket", "bucket-500", "cut", "wrong-code"] {
        let env = environment()?;
        let replay = env.root().join("reply");
        copy_capture("biomarker/native", &replay, |_, record| {
            mutate(record, kind)
        })
        .map_err(|e| e.to_string())?;
        let mut args = if kind == "wrong-code" {
            ["get", "biomarker", "C96022"].map(str::to_owned).to_vec()
        } else {
            query()
        };
        args.extend(env.globals(Some(&replay), false));
        let answer = invoke(&env, &args, if kind == "overflow" { 2 } else { 4 })?;
        if kind == "overflow" {
            if answer.pointer("/error/code") != Some(&json!("usage.too_broad"))
                || !answer
                    .pointer("/error/message")
                    .and_then(Value::as_str)
                    .is_some_and(|s| s.contains("--name"))
            {
                return Err("overflow lost narrowing refusal".into());
            }
        } else {
            let reason = if kind == "bucket-500" {
                "unavailable"
            } else {
                "bad_reply"
            };
            if answer.pointer("/sources/0/reason") != Some(&json!(reason))
                || answer.get("page").is_some()
                || answer.get("result").is_some()
            {
                return Err(format!(
                    "{kind} became an empty or selected result: {answer}"
                ));
            }
        }
    }
    Ok(())
}
fn mutate(record: &mut Value, kind: &str) {
    if let Some(exchange) = record.pointer_mut("/exchanges/0") {
        match kind {
            "overflow" => {
                exchange["body"] = json!({"kind":"text","value":"x".repeat(1024*1024+1)});
            }
            "bucket" | "bucket-500" => {
                exchange["body"] = json!({"kind":"json","value":{"total":0,"data":[],"error":{"type":"too_many_buckets_exception","reason":"synthetic bucket limit"}}});
                if kind == "bucket-500" {
                    exchange["status"] = json!(500);
                }
            }
            "wrong-code" => {
                exchange["body"]["value"]["data"][0]["codes"] = json!(["C1"]);
            }
            _ => {
                exchange["body"]["value"]["total"] = json!(999);
            }
        }
    }
}
fn research() -> Check {
    let env = environment()?;
    let replay = super::super::launch::fixtures().join("replies/biomarker/native");
    let globals = env.globals(Some(&replay), false);
    let mut args = query();
    args.extend_from_slice(&globals);
    let cli = invoke(&env, &args, 0)?;
    let main = super::super::research::call(&env, "biomcp", json!({"args":query()}), &globals)?;
    if main != cli {
        return Err("main MCP tool changed the biomarker envelope".into());
    }
    let result = super::super::research::call(
        &env,
        "search",
        json!({"query":"biomarker --source nci --name PD-L1 --limit 2"}),
        &globals,
    )?;
    if result.get("results").is_some()
        || result.pointer("/metadata/omitted_rows") != Some(&json!(2))
        || result.pointer("/metadata/envelope") != Some(&cli)
    {
        return Err("research wrapper invented citations or lost biomarker rows".into());
    }
    for (capture, omitted) in [
        ("synthetic-candidates", "card"),
        ("synthetic-bad-value", "parent_ids"),
    ] {
        let env = environment()?;
        let replay = super::super::launch::fixtures()
            .join("replies/biomarker")
            .join(capture);
        let globals = env.globals(Some(&replay), false);
        let answer = super::super::research::call(
            &env,
            "biomcp",
            json!({"args":["get","biomarker","C96022"]}),
            &globals,
        )?;
        let pointer = if omitted == "card" {
            "/result/card"
        } else {
            "/result/card/parent_ids"
        };
        if answer.pointer(pointer).is_some() {
            return Err("ambiguous or malformed value acquired a selected result".into());
        }
    }
    Ok(())
}

fn help() -> Check {
    let env = environment()?;
    let answer = invoke(&env, &["help".into(), "search".into(), "biomarker".into()], 0)?;
    let text = answer.pointer("/result/commands/0/description").and_then(Value::as_str).ok_or("missing biomarker help")?;
    for field in ["codes", "name", "type", "trial count", "synonyms", "parent_ids", "ancestor_ids", "eligibility_criterion", "diagnostic --biomarker", "trial --variant"] {
        if !text.contains(field) { return Err(format!("biomarker help omits {field}")); }
    }
    Ok(())
}
