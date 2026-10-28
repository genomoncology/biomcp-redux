//! Feature pgx-retirements: fixed guidance, no query repair or phase inference.
use crate::runner::transcript::trial as transcript_trial;
use super::{configuration::invoke, launch::Environment, mcp};
use libtest_mimic::Trial;
use serde_json::json;
pub(super) fn trials() -> Vec<Trial> {
    let mut trials = CASES
        .iter()
        .enumerate()
        .map(|(index, &(words, message))| {
            transcript_trial(
                format!("pgx-retirements/fixed-guidance-{index}"),
                move || check(words, message).map_err(Into::into),
            )
        })
        .collect::<Vec<_>>();
    trials.push(transcript_trial(
        "pgx-retirements/option-values-stay-literal",
        || boundaries().map_err(Into::into),
    ));
    trials
}
fn check(words: &str, message: &str) -> Result<(), String> {
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
    if value.pointer("/error/code") != Some(&json!("usage.retired"))
        || value.pointer("/error/message") != Some(&json!(message))
        || value.get("sources").is_some()
    {
        return Err(format!("{words}: wrong migration guidance: {value}"));
    }
    if mcp::call_with_globals(&env, &args, true, &[])? != value {
        return Err(format!("{words}: CLI/MCP guidance differs"));
    }
    let measured = serde_json::to_vec(&json!({"schema":"biomcp.error/1","status":"usage",
        "error":{"code":"usage.retired","message":message}}))
    .map_err(|e| e.to_string())?
    .len();

    if serde_json::to_vec(&value).map_err(|e| e.to_string())?.len() > (measured * 115).div_ceil(100)
    {
        return Err("guidance exceeds size ceiling".into());
    }
    if listener.accept().is_ok() {
        return Err("unsupported PGx request opened a source connection".into());
    }
    Ok(())
}
const CASES: &[(&str, &str)] = &[
    (
        "get pgx --gene DPYD recommendations --source cpic --limit 1",
        "use: --section-limit",
    ),
    (
        "get pgx DPYD interactions --source cpic",
        "use: search pgx --gene <SYMBOL> --source cpic; get pgx --gene <SYMBOL> --source cpic",
    ),
    (
        "get pgx DPYD frequencies --source cpic",
        "frequency pages are deferred because no harvested question needs them",
    ),
    (
        "get pgx DPYD all --source cpic",
        "name one PGx section per request",
    ),
    (
        "search pgx DPYD --source cpic",
        "use: search pgx --gene <SYMBOL> --source cpic",
    ),
    (
        "get pgx DPYD --source cpic",
        "use: get pgx --gene <SYMBOL> [recommendations|guidelines|diplotype|annotations] --source <SOURCE>",
    ),
    (
        "search pgx --gene DPYD --source cpic -d text",
        "drug-text and guideline-text search are dropped; use search pgx --gene <SYMBOL> --source cpic",
    ),
    (
        "search pgx --gene DPYD --source cpic --drug text",
        "drug-text and guideline-text search are dropped; use search pgx --gene <SYMBOL> --source cpic",
    ),
    (
        "search pgx --gene DPYD --source cpic --evidence text",
        "drug-text and guideline-text search are dropped; use search pgx --gene <SYMBOL> --source cpic",
    ),
    (
        "search pgx --gene DPYD --source cpic --pgx-testing text",
        "this PGx selector is not yet supported; --drug-id is supported only on recommendations",
    ),
    (
        "search pgx --gene DPYD --source cpic --population text",
        "this PGx selector is not yet supported; --drug-id is supported only on recommendations",
    ),
    (
        "search pgx --gene DPYD --source cpic --lookup-key text",
        "this PGx selector is not yet supported; --drug-id is supported only on recommendations",
    ),
    (
        "search pgx --gene DPYD --source cpic --drug-id text",
        "this PGx selector is not yet supported; --drug-id is supported only on recommendations",
    ),
    (
        "get pgx --gene DPYD guidelines --source cpic --drug-id RxNorm:10324",
        "this PGx selector is not yet supported; --drug-id is supported only on recommendations",
    ),
    (
        "get pgx --gene DPYD interactions --source cpic",
        "use: search pgx --gene <SYMBOL> --source cpic; get pgx --gene <SYMBOL> --source cpic",
    ),
    (
        "get pgx --gene DPYD frequencies --source cpic",
        "frequency pages are deferred because no harvested question needs them",
    ),
    (
        "get pgx --gene DPYD all --source cpic",
        "name one PGx section per request",
    ),
    (
        "get pgx --gene DPYD --full --source cpic",
        "name one PGx section per request",
    ),
    (
        "get pgx --gene DPYD recommendations guidelines --source cpic",
        "name one PGx section per request",
    ),
    (
        "get pgx --gene DPYD recommendations --source cpic -l 1",
        "use: --section-limit",
    ),
    (
        "get pgx --gene DPYD recommendations --source cpic --offset 1",
        "use: --section-offset",
    ),
    ("search pgx --gene DPYD --source cpic -l 1", "use: --limit"),
    ("get pgx --gene DPYD --source cpic -j", "use: --format json"),
    (
        "get pgx --gene DPYD --source cpic --json",
        "use: --format json",
    ),
    (
        "search pgx --gene DPYD",
        "choose --source cpic for search, the gene card, recommendations, guidelines or diplotype; annotations requires --source clinpgx",
    ),
    (
        "get pgx --gene DPYD",
        "choose --source cpic for search, the gene card, recommendations, guidelines or diplotype; annotations requires --source clinpgx",
    ),
    (
        "get pgx --gene DPYD annotations",
        "choose --source cpic for search, the gene card, recommendations, guidelines or diplotype; annotations requires --source clinpgx",
    ),
    (
        "search pgx --gene DPYD --source clinpgx",
        "choose --source cpic for search, the gene card, recommendations, guidelines or diplotype; annotations requires --source clinpgx",
    ),
    (
        "get pgx --gene DPYD --source clinpgx",
        "choose --source cpic for search, the gene card, recommendations, guidelines or diplotype; annotations requires --source clinpgx",
    ),
    (
        "get pgx --gene DPYD recommendations --source clinpgx",
        "choose --source cpic for search, the gene card, recommendations, guidelines or diplotype; annotations requires --source clinpgx",
    ),
    (
        "get pgx --gene DPYD guidelines --source clinpgx",
        "choose --source cpic for search, the gene card, recommendations, guidelines or diplotype; annotations requires --source clinpgx",
    ),
    (
        "get pgx --gene DPYD diplotype --diplotype *1/*2A --source clinpgx",
        "choose --source cpic for search, the gene card, recommendations, guidelines or diplotype; annotations requires --source clinpgx",
    ),
    (
        "get pgx --gene DPYD annotations --source cpic",
        "choose --source cpic for search, the gene card, recommendations, guidelines or diplotype; annotations requires --source clinpgx",
    ),
    (
        "get pgx --gene NUDT15 diplotype --variants c.415C>T,c.50_55dup --source cpic",
        "unphased variants cannot select a diplotype; --diplotype requires an unchanged declared expression",
    ),
    (
        "get pgx --gene TPMT diplotype --variants c.460G>A,c.719A>G --source cpic",
        "unphased variants cannot select a diplotype; --diplotype requires an unchanged declared expression",
    ),
];

fn boundaries() -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    for words in [
        "--disable-sources cpic search pgx --gene interactions --source cpic",
        "--disable-sources cpic get pgx --gene frequencies --source cpic",
        "--disable-sources cpic get pgx --gene DPYD diplotype --diplotype frequencies --source cpic",
        "--disable-sources cpic get pgx --gene DPYD recommendations --drug-id RxNorm:10324 --source cpic",
        "--disable-sources clinpgx get pgx --annotation-id PA100001 --source clinpgx",
    ] {
        let args = words
            .split_whitespace()
            .map(str::to_owned)
            .collect::<Vec<_>>();
        let value = invoke(&env, args, 4)?;
        if value.pointer("/sources/0/reason") != Some(&json!("disabled")) {
            return Err(format!("literal selector became retirement: {value}"));
        }
    }
    Ok(())
}

pub(super) fn bounded(name: &str, value: &serde_json::Value) -> Result<(), String> {
    use std::io::Write;
    let bytes = serde_json::to_vec(value).map_err(|e| e.to_string())?.len();
    writeln!(std::io::stdout(), "{name}: {bytes}").map_err(|e| e.to_string())?;
    let baseline: usize = match name {
        "CPIC pairs" => 1047,
        "CPIC card" => 859,
        "CPIC view" => 1255,
        "CPIC table" => 1365,
        "CPIC missing-second-gene" => 1018,
        "CPIC guidelines" => 942,
        "CPIC guidelines-many" => 1529,
        "CPIC diplotype" => 970,
        "CPIC empty" => 570,
        "CPIC wrong-gene" | "CPIC status" | "CPIC count" | "CPIC unknown-field" | "CPIC range"
        | "CPIC missing-range" | "CPIC broken" => 579,
        "CPIC wrong-drug" => 669,
        "CPIC truncated-card" => 538,
        "CPIC truncated-diplotype" => 495,
        "CPIC 429" => 511,
        "CPIC 500" => 510,
        "CPIC timeout" => 475,
        "ClinPGx clinical" => 1451,
        "ClinPGx default" => 2711,
        "ClinPGx wide" => 4015,
        "ClinPGx guideline" => 916,
        "ClinPGx label" => 904,
        "ClinPGx annotation" => 1235,
        "ClinPGx empty" => 569,
        "ClinPGx 429" => 584,
        "ClinPGx 500" => 583,
        "ClinPGx broken" | "ClinPGx status" | "ClinPGx unknown-field" | "ClinPGx wrong-gene" => 655,
        "ClinPGx wrong-accession" => 534,
        "ClinPGx timeout" => 468,
        _ => return Err(format!("missing PGx output measurement: {name}")),
    };
    if bytes > (baseline * 115).div_ceil(100) {
        return Err(format!("{name}: output exceeds measured ceiling"));
    }
    Ok(())
}
