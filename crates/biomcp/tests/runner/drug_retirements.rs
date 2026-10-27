//! Feature drug-retirements: migration never rewrites a query or contacts a source.
use super::{configuration::invoke, launch::Environment, mcp};
use libtest_mimic::Trial;
use serde_json::json;
pub(super) fn trials() -> Vec<Trial> {
    vec![Trial::test(
        "drug-retirements/fixed-guidance-before-io",
        || check().map_err(Into::into),
    )]
}
fn check() -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let listener = std::net::TcpListener::bind("127.0.0.1:0").map_err(|e| e.to_string())?;
    listener.set_nonblocking(true).map_err(|e| e.to_string())?;
    env.config(
        &json!({"proxy":format!("http://{}",listener.local_addr().map_err(|e|e.to_string())?)}),
    )
    .map_err(|e| e.to_string())?;
    for &(words, message) in CASES {
        let args = words
            .split_whitespace()
            .map(str::to_owned)
            .collect::<Vec<_>>();
        let value = invoke(&env, args.clone(), 2)?;
        if value.pointer("/error/code") != Some(&json!("usage.retired"))
            || value.pointer("/error/message") != Some(&json!(message))
            || value.get("sources").is_some()
        {
            return Err(format!("{words}: wrong migration guidance: {value}"));
        }
        if mcp::call_with_globals(&env, &args, true, &[])? != value {
            return Err(format!("{words}: CLI/MCP migration differs"));
        }
        let bytes = serde_json::to_vec(&value).map_err(|e| e.to_string())?.len();
        let measured = serde_json::to_vec(&json!({"schema":"biomcp.error/1","status":"usage", "error":{"code":"usage.retired","message":message}}))
            .map_err(|e| e.to_string())?
            .len();
        if bytes > measured * 115 / 100 {
            return Err(format!("{words}: migration exceeds size ceiling"));
        }
    }
    if listener.accept().is_ok() {
        return Err("migration opened a provider or download connection".into());
    }
    Ok(())
}
const CASES: &[(&str, &str)] = &[
    (
        "search drug trastuzumab --region who",
        "--region who is retired: use --source who-pq.",
    ),
    (
        "get drug BT-ON001 --region=who",
        "--region who is retired: use --source who-pq.",
    ),
    (
        "drug cell-lines imatinib --cell-line A --dataset B --limit 1 --offset 0",
        "drug cell-lines is retired: cell-line screening is outside the cancer-care drug slice.",
    ),
    (
        "cvx sync",
        "Vaccine identity lookup and sync are unavailable; search vaccine report summaries with `search adverse-event --source vaers`.",
    ),
    ("ema sync", "renamed; use: biomcp sync ema"),
    ("ddinter sync", "renamed; use: biomcp sync ddinter"),
    ("who sync", "renamed; use: biomcp sync who-pq"),
    (
        "search drug --query imatinib",
        "renamed; use: search drug <TEXT> --source <SOURCE>",
    ),
    (
        "search drug imatinib --mechanism kinase",
        "--mechanism is retired: semantic mechanism expansion is not supported; select a drug identity and read targets.",
    ),
    (
        "search drug imatinib --interactions warfarin",
        "--interactions is retired: use get drug <ID> interactions --source ddinter.",
    ),
    (
        "search drug imatinib --region US",
        "--region is retired: select an explicit --source.",
    ),
    (
        "search drug imatinib --product-type vaccines",
        "--product-type is retired: WHO prequalification is outside this question bank.",
    ),
    (
        "search drug imatinib --target ABL1",
        "drug search filters --target, --indication, --type, --atc and --pharm-class are not yet supported; select a \
        drug identity to read source sections.",
    ),
    (
        "search drug imatinib --indication cancer",
        "drug search filters --target, --indication, --type, --atc and --pharm-class are not yet supported; select a \
        drug identity to read source sections.",
    ),
    (
        "search drug imatinib --type Small",
        "drug search filters --target, --indication, --type, --atc and --pharm-class are not yet supported; select a \
        drug identity to read source sections.",
    ),
    (
        "search drug imatinib --atc L01",
        "drug search filters --target, --indication, --type, --atc and --pharm-class are not yet supported; select a \
        drug identity to read source sections.",
    ),
    (
        "search drug imatinib --pharm-class kinase",
        "drug search filters --target, --indication, --type, --atc and --pharm-class are not yet supported; select a \
        drug identity to read source sections.",
    ),
    (
        "gene drugs ABL1 --limit 1 --offset 0",
        "drug search filters --target, --indication, --type, --atc and --pharm-class are not yet supported; select a \
        drug identity to read source sections.",
    ),
    (
        "get drug --name trastuzumab",
        "--name is retired: use search drug <TEXT> --source <SOURCE> and select a native identity.",
    ),
    (
        "get drug CHEMBL941 --region EU",
        "--region is retired: select an explicit --source.",
    ),
    (
        "get drug 492dbdb2-077e-4064-bff3-372d6af0a7a2 --raw",
        "--raw is retired: use get drug <SPL-SET-ID> label --source openfda-label --label-section <NATIVE-HEADING>.",
    ),
    (
        "get drug CHEMBL941 safety",
        "safety is retired: use get drug <SPL-SET-ID> label --source openfda-label --label-section <NATIVE-HEADING>.",
    ),
    (
        "get drug CHEMBL941 shortage",
        "shortage is not yet supported in the cancer-care drug slice.",
    ),
    (
        "get drug CHEMBL941 all",
        "all is not supported: select one named source section.",
    ),
    (
        "get drug CHEMBL941 cell_lines",
        "drug cell-lines is retired: cell-line screening is outside the cancer-care drug slice.",
    ),
    (
        "drug interactions 1744 --limit 1 --offset 0",
        "renamed; use: get drug <ID> interactions --source ddinter --section-limit <N> --section-offset <N>",
    ),
];
