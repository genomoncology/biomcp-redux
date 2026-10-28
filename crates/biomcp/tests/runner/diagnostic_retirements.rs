//! Feature diagnostic-retirements: fixed guidance never repairs source queries.
use super::{configuration::invoke, launch::Environment, mcp};
use libtest_mimic::Trial;
use serde_json::json;
// j19098 measured the largest value-free typed argument error envelope.
const USAGE_BYTES: usize = 252;
pub(super) fn trials() -> Vec<Trial> {
    CASES
        .iter()
        .enumerate()
        .map(|(i, &(words, message))| {
            Trial::test(
                format!("diagnostic-retirements/fixed-guidance-{i}"),
                move || check(words, message, "usage.retired").map_err(Into::into),
            )
        })
        .chain([Trial::test("diagnostic-retirements/literal-values", || {
            boundaries().map_err(Into::into)
        })])
        .collect()
}
fn check(words: &str, message: &str, code: &str) -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let listener = std::net::TcpListener::bind("127.0.0.1:0").map_err(|e| e.to_string())?;
    listener.set_nonblocking(true).map_err(|e| e.to_string())?;
    env.config(
        &json!({"proxy":format!("http://{}",listener.local_addr().map_err(|e|e.to_string())?)}),
    )
    .map_err(|e| e.to_string())?;
    let args = words
        .split_whitespace()
        .map(str::to_owned)
        .collect::<Vec<_>>();
    let value = invoke(&env, args.clone(), 2)?;
    if value.pointer("/error/code") != Some(&json!(code))
        || value.pointer("/error/message") != Some(&json!(message))
        || value.get("sources").is_some()
    {
        return Err(format!("{words}: wrong migration guidance: {value}"));
    }
    if mcp::call_with_globals(&env, &args, true, &[])? != value {
        return Err(format!("{words}: CLI/MCP guidance differs"));
    }
    let measured = if code == "usage.arguments" {
        USAGE_BYTES
    } else {
        serde_json::to_vec(&json!({"schema":"biomcp.error/1","status":"usage",
        "error":{"code":code,"message":message}}))
        .map_err(|e| e.to_string())?
        .len()
    };

    if serde_json::to_vec(&value).map_err(|e| e.to_string())?.len() > (measured * 115).div_ceil(100)
    {
        return Err("guidance exceeds size ceiling".into());
    }
    if listener.accept().is_ok() {
        return Err("unsupported diagnostic request opened a source connection".into());
    }
    Ok(())
}
fn boundaries() -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    for words in [
        "--disable-sources gtr search diagnostic --gene all --source gtr",
        "--disable-sources gtr search diagnostic --gene regulatory --source gtr",
        "--disable-sources fda-cdx search diagnostic --biomarker=--full --source fda-cdx",
    ] {
        let value = invoke(
            &env,
            words.split_whitespace().map(str::to_owned).collect(),
            4,
        )?;
        if value.pointer("/sources/0/reason") != Some(&json!("disabled")) {
            return Err(format!("selector value became retirement: {value}"));
        }
    }
    for words in [
        "search diagnostic --source gtr --device-id K173492",
        "search diagnostic --source gtr --marker HIV",
        "search diagnostic --source who-ivd --gene BRCA1",
        "search diagnostic --source who-ivd --biomarker HIV",
        "search diagnostic --source who-ivd --marker ...",
        "search diagnostic --source who-ivd --manufacturer ...",
        "search diagnostic --source fda-cdx --gene BRCA1",
        "get diagnostic P150013 --source gtr",
        "get diagnostic K173492 --source fda-pma",
        "get diagnostic P150013 --source fda-510k",
        "get diagnostic P150013 --source fda-cdx",
    ] {
        check(
            words,
            "expected a supported command with checked arguments",
            "usage.arguments",
        )?;
    }
    Ok(())
}
const CASES: &[(&str, &str)] = &[
    (
        "get diagnostic GTR000006692.3 --source gtr --section all",
        "name one diagnostic section per request",
    ),
    (
        "get diagnostic GTR000006692.3 --source gtr --full",
        "name one diagnostic section per request",
    ),
    (
        "search diagnostic --source who-ivd --disease HIV",
        "use: search diagnostic --source who-ivd --marker TEXT",
    ),
    (
        "get diagnostic GTR000006692.3",
        "choose --source gtr, who-ivd, fda-cdx or fda-device-recall for search; choose --source gtr, who-ivd, fda-pma, fda-510k or fda-device-recall for exact get",
    ),
    (
        "--json get diagnostic GTR000006692.3 --source gtr",
        "use: --format json",
    ),
    (
        "get diagnostic GTR000006692.3 --source gtr --no-cache",
        "use: --fresh to bypass cached replies",
    ),
    (
        "search diagnostic --gene BRCA1",
        "choose --source gtr, who-ivd, fda-cdx or fda-device-recall for search; choose --source gtr, who-ivd, fda-pma, fda-510k or fda-device-recall for exact get",
    ),
    (
        "get diagnostic X --source all",
        "choose --source gtr, who-ivd, fda-cdx or fda-device-recall for search; choose --source gtr, who-ivd, fda-pma, fda-510k or fda-device-recall for exact get",
    ),
    (
        "search diagnostic --source gtr --gene BRCA1 -g BRCA1",
        "use: --gene",
    ),
    (
        "search diagnostic --source gtr --gene BRCA1 -t Clinical",
        "use: --type",
    ),
    (
        "search diagnostic --source gtr --gene BRCA1 -l 1",
        "use: --limit",
    ),
    (
        "search diagnostic --source gtr --gene BRCA1 -d cancer",
        "use: --condition-id MedGen:CUI with --source gtr; use an exact product card with --source who-ivd",
    ),
    (
        "search diagnostic --source gtr --gene BRCA1 --disease cancer",
        "use: --condition-id MedGen:CUI with --source gtr; use an exact product card with --source who-ivd",
    ),
    (
        "search diagnostic --source gtr --gene BRCA1 --manufacturer lab",
        "manufacturer and laboratory selectors are deferred",
    ),
    (
        "search diagnostic --source gtr --gene BRCA1 --laboratory lab",
        "manufacturer and laboratory selectors are deferred",
    ),
    (
        "search diagnostic --source gtr --gene BRCA1 --full",
        "name one diagnostic section per request",
    ),
    (
        "search diagnostic --source gtr --gene BRCA1 -j",
        "use: --format json",
    ),
    (
        "search diagnostic --source gtr --gene BRCA1 --json",
        "use: --format json",
    ),
    (
        "get diagnostic GTR000006692.3 all --source gtr",
        "name one diagnostic section per request",
    ),
    (
        "get diagnostic GTR000006692.3 regulatory --source gtr",
        "use: get diagnostic ID --source fda-pma; search diagnostic --source fda-cdx --device-id ID",
    ),
    (
        "get diagnostic 0466-046-00 regulatory --source who-ivd",
        "use: get diagnostic ID --source fda-pma; search diagnostic --source fda-cdx --device-id ID",
    ),
    (
        "search diagnostic --source who-ivd --type text",
        "WHO type search is deferred; use: search diagnostic --source who-ivd --marker TEXT",
    ),
    (
        "search diagnostic --source fda-pma --query text",
        "free-text PMA search is deferred; use: get diagnostic P-NUMBER --source fda-pma",
    ),
    (
        "search diagnostic --source fda-510k --query text",
        "free-text 510(k) search is deferred; use: get diagnostic K-NUMBER --source fda-510k",
    ),
    ("gtr sync", "use: biomcp sync gtr"),
    ("who-ivd sync", "use: biomcp sync who-ivd"),
    ("list diagnostic", "use: biomcp help diagnostic"),
];

// Diagnostic mechanism cases share one measured envelope bound per view/outcome.
pub(super) fn bounded(value: &serde_json::Value) -> Result<(), String> {
    use std::io::Write;
    let bytes = serde_json::to_vec(value).map_err(|e| e.to_string())?.len();
    let schema = value
        .get("schema")
        .and_then(serde_json::Value::as_str)
        .ok_or("missing schema")?;
    let kind = value
        .pointer("/result/kind")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("");
    let status = value
        .get("status")
        .and_then(serde_json::Value::as_str)
        .ok_or("missing status")?;
    // Temporary byte measurement for the newly introduced source; replace with its observed fixed ceiling.
    if value.pointer("/query/source") == Some(&json!("fda-device-recall")) {
        writeln!(std::io::stdout(), "recall {schema}/{kind}/{status}: {bytes}").map_err(|e| e.to_string())?;
        return Ok(());
    }
    // Full compact synthetic maxima measured by j13382.
    let baseline: usize = match (schema, kind, status) {
        // New WHO search view, full compact maximum measured by j13465.
        ("biomcp.diagnostic-page/2", "", _)
            if value.pointer("/query/source") == Some(&json!("who-ivd")) =>
        {
            2249
        }
        ("biomcp.diagnostic-card/1", "", "absent") => 281,
        ("biomcp.diagnostic-card/1", "", "failed") => 436,
        ("biomcp.diagnostic-card/1", "", "partial") => 393,
        ("biomcp.diagnostic-card/1", "clearance", "complete") => 725,
        ("biomcp.diagnostic-card/1", "gtr_test", "complete") => 1519,
        ("biomcp.diagnostic-card/1", "gtr_test", "partial") => 1559,
        ("biomcp.diagnostic-card/1", "who_ivd_product", "complete") => 2578,
        ("biomcp.diagnostic-card/1", "who_ivd_product", "partial") => 2576,
        ("biomcp.diagnostic-companion/1", "", "complete") => 7879,
        ("biomcp.diagnostic-companion/1", "", "failed") => 410,
        ("biomcp.diagnostic-companion/1", "", "partial") => 7035,
        ("biomcp.diagnostic-genes/2", "", "complete") => 867,
        ("biomcp.diagnostic-page/2", "", "failed") => 379,
        ("biomcp.diagnostic-regulatory/1", "", "complete") => 7260,
        ("biomcp.diagnostic-supplements/1", "", "complete") => 1414,
        ("biomcp.error/1", "", "usage") => USAGE_BYTES,
        ("biomcp.sync/5", "", "complete") => 348,
        ("biomcp.sync/5", "", "failed") => 337,
        _ => {
            return Err(format!(
                "unmeasured diagnostic envelope: {schema}/{kind}/{status}"
            ));
        }
    };
    writeln!(
        std::io::stdout(),
        "diagnostic {schema}/{kind}/{status}: {bytes}"
    )
    .map_err(|e| e.to_string())?;
    if bytes > (baseline * 115).div_ceil(100) {
        return Err(format!(
            "diagnostic output exceeds measured envelope ceiling: {bytes}"
        ));
    }
    Ok(())
}
