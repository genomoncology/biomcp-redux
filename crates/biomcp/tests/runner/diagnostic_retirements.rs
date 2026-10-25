//! Feature diagnostic-retirements: fixed guidance never repairs source queries.
use super::{configuration::invoke, launch::Environment, mcp};
use libtest_mimic::Trial;
use serde_json::json;
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
    let measured = serde_json::to_vec(&json!({"schema":"biomcp.error/1","status":"usage",
        "error":{"code":code,"message":message}}))
    .map_err(|e| e.to_string())?
    .len();

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
        "search diagnostic --source who-ivd --marker HPV",
        "WHO discovery, type and marker search are deferred; use: get diagnostic PRODUCT-ID --source who-ivd",
    ),
    (
        "get diagnostic GTR000006692.3",
        "choose --source gtr or fda-cdx for search; choose --source gtr, who-ivd, fda-pma or fda-510k for exact get",
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
        "choose --source gtr or fda-cdx for search; choose --source gtr, who-ivd, fda-pma or fda-510k for exact get",
    ),
    (
        "get diagnostic X --source all",
        "choose --source gtr or fda-cdx for search; choose --source gtr, who-ivd, fda-pma or fda-510k for exact get",
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
        "WHO discovery, type and marker search are deferred; use: get diagnostic PRODUCT-ID --source who-ivd",
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
