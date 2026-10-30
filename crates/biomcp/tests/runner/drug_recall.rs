//! Feature drug-recall: native search, paging, exact identity and source field problems.
use super::{configuration::invoke, maude::environment, mcp, network_stub::Stub};
use crate::runner::transcript::trial as transcript_trial;
use libtest_mimic::Trial;
use serde_json::{Value, json};
pub(super) fn trials() -> Vec<Trial> {
    let mut trials = super::drug_recall_capture::trials();
    trials.extend(vec![
        transcript_trial("drug-recall/empty-search-is-complete", || {
            empty_search().map_err(Into::into)
        }),
        transcript_trial("drug-recall/native-filters-and-paging", || {
            search().map_err(Into::into)
        }),
        transcript_trial("drug-recall/native-card-and-problem", || {
            card().map_err(Into::into)
        }),
        transcript_trial("drug-recall/invalid-selectors-before-io", || {
            invalid().map_err(Into::into)
        }),
    ]);
    trials
}
fn row() -> Value {
    json!({"recall_number":"D-0001-2024","classification":"Class II","status":"Ongoing",
        "recalling_firm":"Synthetic maker","product_description":"Published product",
        "recall_initiation_date":"20240102","report_date":"20240203",
        "reason_for_recall":"Published reason","distribution_pattern":"Published distribution",
        "code_info":"Published code","voluntary_mandated":"Voluntary: Firm Initiated",
        "openfda":{"brand_name":["Harmonized brand"],"generic_name":["Harmonized generic"]}})
}
fn envelope(row: Value, offset: u32, total: u32) -> String {
    json!({"meta":{"results":{"skip":offset,"limit":1,"total":total}},"results":[row]}).to_string()
}
fn empty_search() -> Result<(), String> {
    let stub = Stub::for_source("api.fda.gov", "application/json", |_| {
        Ok(json!({"meta":{"results":{"skip":0,"limit":10,"total":0}},"results":[]}).to_string())
    })?;
    let env = environment(&stub)?;
    let command = ["search", "drug", "missing", "--source", "fda-drug-recall"]
        .map(str::to_owned)
        .to_vec();
    let value = invoke(&env, command, 0)?;
    if value.pointer("/status") != Some(&json!("complete"))
        || value.pointer("/page/total") != Some(&json!(0))
        || value.pointer("/sources/0/state") != Some(&json!("ok"))
    {
        return Err(format!(
            "successful empty recall search lost its answer: {value}"
        ));
    }
    if stub.finish()?.len() != 1 {
        return Err("empty recall search fetched again".into());
    }
    Ok(())
}
fn search() -> Result<(), String> {
    for (args, selector) in [
        (
            vec!["Synthetic name"],
            "(product_description:\"Synthetic name\" OR openfda.brand_name:\"Synthetic name\" OR openfda.generic_name:\"Synthetic name\")",
        ),
        (
            vec!["--firm", "Synthetic maker"],
            "recalling_firm:\"Synthetic maker\"",
        ),
        (
            vec!["--classification", "Class II"],
            "classification:\"Class II\"",
        ),
        (vec!["--status", "Ongoing"], "status:\"Ongoing\""),
        (
            vec!["--reason", "Published reason"],
            "reason_for_recall:\"Published reason\"",
        ),
        (
            vec![
                "--initiated-from",
                "2024-01-01",
                "--initiated-to",
                "2024-12-31",
            ],
            "recall_initiation_date:[20240101 TO 20241231]",
        ),
    ] {
        let encoded = encode(selector);
        let stub = Stub::for_source("api.fda.gov", "application/json", move |request| {
            let offset = if request.contains("&skip=1&") { 1 } else { 0 };
            let expected = format!(
                "GET /drug/enforcement.json?search={encoded}&limit=1&skip={offset}&sort=recall_initiation_date%3Adesc HTTP/1.1"
            );
            if request.lines().next() != Some(expected.as_str()) {
                return Err(format!("wrong recall selector: {request}"));
            }
            Ok(envelope(row(), offset, 2))
        })?;
        let env = environment(&stub)?;
        let mut command = vec!["search", "drug"];
        command.extend(args);
        command.extend(["--source", "fda-drug-recall", "--limit", "1"]);
        let command: Vec<String> = command.into_iter().map(str::to_owned).collect();
        let cli = invoke(&env, command.clone(), 0)?;
        let tool = mcp::call_with_globals(&env, &command, false, &[])?;
        if cli != tool
            || cli.pointer("/result/items/0/recall_number") != Some(&json!("D-0001-2024"))
            || cli.pointer("/result/count_unit") != Some(&json!("recall-records"))
            || cli.pointer("/page/total") != Some(&json!(2))
            || cli.to_string().contains("Harmonized")
        {
            return Err(format!("native recall page differs: {cli} / {tool}"));
        }
        let step = cli
            .get("next")
            .and_then(Value::as_array)
            .and_then(|steps| steps.iter().find(|s| s.get("kind") == Some(&json!("page"))))
            .and_then(|s| s.get("args"))
            .and_then(Value::as_array)
            .ok_or("missing recall continuation")?;
        let step: Vec<String> = step
            .iter()
            .map(|v| v.as_str().unwrap_or_default().to_owned())
            .collect();
        let page = invoke(&env, step, 0)?;
        if page.pointer("/page/offset") != Some(&json!(1)) {
            return Err(format!("lost recall paging: {page}"));
        }
        if stub.finish()?.len() != 2 {
            return Err("recall paging request count differs".into());
        }
    }
    Ok(())
}
fn encode(s: &str) -> String {
    s.bytes()
        .map(|b| {
            if b.is_ascii_alphanumeric() || b"-._~".contains(&b) {
                char::from(b).to_string()
            } else {
                format!("%{b:02X}")
            }
        })
        .collect()
}
fn card() -> Result<(), String> {
    for bad in [false, true] {
        let stub = Stub::for_source("api.fda.gov", "application/json", move |request| {
            if request.lines().next()
                != Some(
                    "GET /drug/enforcement.json?search=recall_number.exact%3A%22D-0001-2024%22&limit=1&skip=0 HTTP/1.1",
                )
            {
                return Err(format!("wrong recall number lookup: {request}"));
            }
            let mut row = row();
            if bad {
                let fields = row
                    .as_object_mut()
                    .ok_or("synthetic recall is not an object")?;
                fields.insert("report_date".into(), json!("20240230"));
                fields.insert("code_info".into(), json!(["wrong shape"]));
            }
            Ok(envelope(row, 0, 1))
        })?;
        let env = environment(&stub)?;
        let args = ["get", "drug", "D-0001-2024", "--source", "fda-drug-recall"]
            .map(str::to_owned)
            .to_vec();
        let cli = invoke(&env, args.clone(), 0)?;
        let tool = mcp::call_with_globals(&env, &args, false, &[])?;
        if cli != tool
            || cli.pointer("/result/header/initiated") != Some(&json!("2024-01-02"))
            || cli.pointer("/result/reason") != Some(&json!("Published reason"))
            || cli.pointer("/result/header/product_description")
                != Some(&json!("Published product"))
            || cli.pointer("/status") != Some(&json!(if bad { "partial" } else { "complete" }))
            || cli.to_string().contains("Harmonized")
        {
            return Err(format!("native recall card differs: {cli} / {tool}"));
        }
        if bad
            && (cli.pointer("/result/header/reported").is_some()
                || cli.pointer("/result/code_info").is_some()
                || cli
                    .get("problems")
                    .and_then(Value::as_array)
                    .is_none_or(|p| p.len() != 2))
        {
            return Err(format!("bad recall field lacked a problem: {cli}"));
        }
        if stub.finish()?.len() != 1 {
            return Err("warm recall get requested again".into());
        }
    }
    Ok(())
}
fn invalid() -> Result<(), String> {
    let env = super::launch::Environment::new().map_err(|e| e.to_string())?;
    for args in [
        vec!["search", "drug", "--source", "mychem"],
        vec![
            "search", "drug", "name", "--source", "mychem", "--firm", "maker",
        ],
        vec![
            "search",
            "drug",
            "--source",
            "fda-drug-recall",
            "--classification",
            "severe",
        ],
        vec![
            "search",
            "drug",
            "--source",
            "fda-drug-recall",
            "--initiated-from",
            "2024",
        ],
        vec![
            "search",
            "drug",
            "--source",
            "fda-drug-recall",
            "--initiated-from",
            "2024-12-31",
            "--initiated-to",
            "2024-01-01",
        ],
        vec!["get", "drug", "Z-0001-2024", "--source", "fda-drug-recall"],
        vec![
            "get",
            "drug",
            "D-0001-2024",
            "label",
            "--source",
            "fda-drug-recall",
            "--label-section",
            "purpose",
        ],
    ] {
        let args: Vec<String> = args.into_iter().map(str::to_owned).collect();
        let cli = invoke(&env, args.clone(), 2)?;
        let tool = mcp::call_with_globals(&env, &args, true, &[])?;
        if cli != tool {
            return Err(format!("recall refusal differs: {cli} / {tool}"));
        }
    }
    Ok(())
}
