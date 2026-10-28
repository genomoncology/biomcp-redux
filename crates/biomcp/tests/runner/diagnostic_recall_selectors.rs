//! Feature diagnostic-device-recall: literal selectors, native record paging and rejected optional fields.
use super::super::super::{
    configuration::invoke,
    launch::{Environment, arguments},
    maude::environment,
    mcp,
    network_stub::Stub,
};
use libtest_mimic::Trial;
use serde_json::{Value, json};
pub(super) fn trials() -> Vec<Trial> {
    vec![
        Trial::test(
            "diagnostic-device-recall/native-selectors-and-replay",
            || selectors().map_err(Into::into),
        ),
        Trial::test(
            "diagnostic-device-recall/record-paging-and-field-problems",
            || paging().map_err(Into::into),
        ),
    ]
}
fn encode(value: &str) -> String {
    value
        .bytes()
        .map(|b| {
            if b.is_ascii_alphanumeric() || b"-._~".contains(&b) {
                char::from(b).to_string()
            } else {
                format!("%{b:02X}")
            }
        })
        .collect()
}
fn row(number: &str) -> Value {
    json!({"product_res_number":number,"res_event_number":"101","recalling_firm":"Synthetic maker",
        "product_code":"ABC","recall_status":"Ongoing","event_date_initiated":"2024-01-02"})
}
fn replay(
    env: &Environment,
    args: &[String],
    recording: &std::path::Path,
    exit: i32,
) -> Result<Value, String> {
    let globals = ["--record".into(), recording.to_string_lossy().into_owned()];
    let original = invoke(env, arguments(args, &globals), exit)?;
    let fresh = Environment::new().map_err(|e| e.to_string())?;
    let globals = fresh.globals(Some(recording), false);
    let cli = invoke(&fresh, arguments(args, &globals), exit)?;
    let tool = mcp::call_with_globals(&fresh, args, exit >= 2, &globals)?;
    if cli != tool || cli.get("result") != original.get("result") {
        return Err(format!(
            "recall replay differs: {original} / {cli} / {tool}"
        ));
    }
    Ok(original)
}
fn selectors() -> Result<(), String> {
    for (flags, expression, absent) in CASES {
        let request = format!(
            "GET /device/recall.json?search={}&limit=1&skip=0&sort=event_date_initiated%3Adesc HTTP/1.1",
            encode(expression)
        );
        let empty = *absent;
        let stub = Stub::for_source("api.fda.gov", "application/json", move |req| {
            if req.lines().next() != Some(request.as_str()) {
                return Err(format!("wrong native recall selector: {req}"));
            }
            Ok(
                json!({"meta":{"results":{"skip":0,"limit":1,"total":if empty {0}else{1}}},
                "results":if empty {vec![]}else{vec![row("Z-0001-2024")]}})
                .to_string(),
            )
        })?;
        let env = environment(&stub)?;
        let mut args = [
            "search",
            "diagnostic",
            "--source",
            "fda-device-recall",
            "--limit",
            "1",
        ]
        .map(str::to_owned)
        .to_vec();
        args.extend(flags.iter().map(|v| (*v).to_owned()));
        let value = replay(&env, &args, &env.root().join("recording"), i32::from(empty))?;
        if value.pointer("/page/total") != Some(&json!(if empty { 0 } else { 1 }))
            || value.pointer("/result/count_unit") != Some(&json!("recall_records"))
        {
            return Err(format!("wrong native count: {value}"));
        }
        if stub.finish()?.len() != 1 {
            return Err("recall replay opened network".into());
        }
    }
    Ok(())
}
fn paging() -> Result<(), String> {
    let stub = Stub::for_source("api.fda.gov", "application/json", |req| {
        let line = req.lines().next().ok_or("request")?;
        let skip = if line.contains("&skip=0&") {
            0
        } else if line.contains("&skip=1&") {
            1
        } else {
            return Err(format!("wrong paging request: {line}"));
        };
        if !line
            .starts_with("GET /device/recall.json?search=product_code%3A%22ABC%22&limit=1&skip=")
        {
            return Err(format!("paging lost exact product code: {line}"));
        }
        let mut record = row(if skip == 0 {
            "Z-0001-2024"
        } else {
            "Z-0002-2024"
        });
        *record
            .get_mut("event_date_initiated")
            .ok_or("missing synthetic date")? = json!("2024-02-30");
        Ok(
            json!({"meta":{"results":{"skip":skip,"limit":1,"total":2}},"results":[record]})
                .to_string(),
        )
    })?;
    let env = environment(&stub)?;
    let args = [
        "search",
        "diagnostic",
        "--source",
        "fda-device-recall",
        "--product-code",
        "ABC",
        "--limit",
        "1",
    ]
    .map(str::to_owned)
    .to_vec();
    let first = invoke(&env, args, 0)?;
    if first.pointer("/page/total") != Some(&json!(2))
        || first.get("status") != Some(&json!("partial"))
        || first.pointer("/result/items/0/header/initiated").is_some()
        || first.pointer("/problems/0/field") != Some(&json!("event_date_initiated"))
        || first.pointer("/problems/0/code") != Some(&json!("value.unknown"))
    {
        return Err(format!("bad native date/count lost: {first}"));
    }
    let next = first
        .get("next")
        .and_then(Value::as_array)
        .and_then(|steps| steps.iter().find(|v| v.get("kind") == Some(&json!("page"))))
        .and_then(|v| v.get("args"))
        .ok_or("missing next recall page")?;
    let args: Vec<String> = serde_json::from_value(next.clone()).map_err(|e| e.to_string())?;
    let second = invoke(&env, args.clone(), 0)?;
    let tool = mcp::call_with_globals(&env, &args, false, &[])?;
    if second != tool
        || second.pointer("/result/items/0/header/recall_number") != Some(&json!("Z-0002-2024"))
        || second.pointer("/page/total") != Some(&json!(2))
        || second.pointer("/page/offset") != Some(&json!(1))
    {
        return Err(format!(
            "event grouping or paging lost records: {second} / {tool}"
        ));
    }
    if stub.finish()?.len() != 2 {
        return Err("paging opened extra HTTP requests".into());
    }
    Ok(())
}
const CASES: &[(&[&str], &str, bool)] = &[
    (&["--product-code", "ABC"], "product_code:\"ABC\"", false),
    (
        &["--product-code", "UNKNOWNCODE"],
        "product_code:\"UNKNOWNCODE\"",
        true,
    ),
    (
        &["--initiated-from", "2024-01-01"],
        "event_date_initiated:[2024-01-01 TO *]",
        false,
    ),
    (
        &["--initiated-to", "2024-12-31"],
        "event_date_initiated:[* TO 2024-12-31]",
        false,
    ),
    (
        &[
            "--initiated-from",
            "2024-01-01",
            "--initiated-to",
            "2024-12-31",
        ],
        "event_date_initiated:[2024-01-01 TO 2024-12-31]",
        false,
    ),
    (
        &[
            "--manufacturer",
            "Synthetic maker",
            "--product-code",
            "ABC",
            "--initiated-from",
            "2024-01-01",
        ],
        "recalling_firm:\"Synthetic maker\" AND product_code:\"ABC\" AND event_date_initiated:[2024-01-01 TO *]",
        false,
    ),
];
