//! Feature adverse-event-retirements: migration never rewrites a query or contacts a source.
use super::{configuration::invoke, launch::Environment, mcp};
use libtest_mimic::Trial;
use serde_json::json;
pub(super) fn trials() -> Vec<Trial> {
    vec![Trial::test(
        "adverse-event-retirements/fixed-guidance-before-io",
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
        let measured = serde_json::to_vec(&json!({"schema":"biomcp.error/1","status":"usage",
            "error":{"code":"usage.retired","message":message}}))
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
        "search adverse-event -d text",
        "use: search adverse-event <TEXT>; text stays literal",
    ),
    (
        "search adverse-event --drug text",
        "use: search adverse-event <TEXT>; text stays literal",
    ),
    (
        "search adverse-event text --outcome death",
        "--outcome is retired; use --serious death or --serious hospitalization for report flags and reactions for native outcomes; the old hospitalization and disability mappings were wrong",
    ),
    (
        "search adverse-event text --outcome hospitalization",
        "--outcome is retired; use --serious death or --serious hospitalization for report flags and reactions for native outcomes; the old hospitalization and disability mappings were wrong",
    ),
    (
        "search adverse-event text --outcome disability",
        "--outcome is retired; use --serious death or --serious hospitalization for report flags and reactions for native outcomes; the old hospitalization and disability mappings were wrong",
    ),
    (
        "search adverse-event text --outcome recovered",
        "--outcome is retired; use --serious death or --serious hospitalization for report flags and reactions for native outcomes; the old hospitalization and disability mappings were wrong",
    ),
    (
        "search adverse-event text --outcome any",
        "--outcome is retired; use --serious death or --serious hospitalization for report flags and reactions for native outcomes; the old hospitalization and disability mappings were wrong",
    ),
    (
        "search adverse-event text --date-from 2024-01-01",
        "renamed; use: --received-from",
    ),
    (
        "search adverse-event text --since 2024-01-01",
        "renamed; use: --received-from",
    ),
    (
        "search adverse-event text --date-to 2024-12-31",
        "renamed; use: --received-to",
    ),
    (
        "search adverse-event text --until 2024-12-31",
        "renamed; use: --received-to",
    ),
    (
        "search adverse-event text --serious ",
        "only --serious death and --serious hospitalization are supported",
    ),
    (
        "search adverse-event text --serious any",
        "only --serious death and --serious hospitalization are supported",
    ),
    (
        "search adverse-event text --serious disability",
        "only --serious death and --serious hospitalization are supported",
    ),
    (
        "search adverse-event text --serious life-threatening",
        "only --serious death and --serious hospitalization are supported",
    ),
    (
        "search adverse-event text --serious other",
        "only --serious death and --serious hospitalization are supported",
    ),
    (
        "search adverse-event text --suspect-only",
        "this adverse-event filter is not supported",
    ),
    (
        "search adverse-event text --suspect-only=anything",
        "this adverse-event filter is not supported",
    ),
    (
        "search adverse-event text --suspect-only anything",
        "this adverse-event filter is not supported",
    ),
    (
        "search adverse-event text --sex",
        "this adverse-event filter is not supported",
    ),
    (
        "search adverse-event text --sex=anything",
        "this adverse-event filter is not supported",
    ),
    (
        "search adverse-event text --sex anything",
        "this adverse-event filter is not supported",
    ),
    (
        "search adverse-event text --age-min",
        "this adverse-event filter is not supported",
    ),
    (
        "search adverse-event text --age-min=anything",
        "this adverse-event filter is not supported",
    ),
    (
        "search adverse-event text --age-min anything",
        "this adverse-event filter is not supported",
    ),
    (
        "search adverse-event text --age-max",
        "this adverse-event filter is not supported",
    ),
    (
        "search adverse-event text --age-max=anything",
        "this adverse-event filter is not supported",
    ),
    (
        "search adverse-event text --age-max anything",
        "this adverse-event filter is not supported",
    ),
    (
        "search adverse-event text --reporter",
        "this adverse-event filter is not supported",
    ),
    (
        "search adverse-event text --reporter=anything",
        "this adverse-event filter is not supported",
    ),
    (
        "search adverse-event text --reporter anything",
        "this adverse-event filter is not supported",
    ),
    (
        "search adverse-event text --manufacturer",
        "this adverse-event filter is not supported",
    ),
    (
        "search adverse-event text --manufacturer=anything",
        "this adverse-event filter is not supported",
    ),
    (
        "search adverse-event text --manufacturer anything",
        "this adverse-event filter is not supported",
    ),
    (
        "search adverse-event text --product-code",
        "this adverse-event filter is not supported",
    ),
    (
        "search adverse-event text --product-code=anything",
        "this adverse-event filter is not supported",
    ),
    (
        "search adverse-event text --product-code anything",
        "this adverse-event filter is not supported",
    ),
    (
        "search adverse-event text --count",
        "this adverse-event filter is not supported",
    ),
    (
        "search adverse-event text --count=anything",
        "this adverse-event filter is not supported",
    ),
    (
        "search adverse-event text --count anything",
        "this adverse-event filter is not supported",
    ),
    (
        "search adverse-event text --type faers",
        "renamed; use: --source faers",
    ),
    (
        "search adverse-event text --type device",
        "use: search adverse-event <TEXT> --source maude",
    ),
    (
        "search adverse-event text --type devices",
        "use: search adverse-event <TEXT> --source maude",
    ),
    (
        "search adverse-event text --type maude",
        "use: search adverse-event <TEXT> --source maude",
    ),
    (
        "search adverse-event text --type recall",
        "recalls are outside this entity",
    ),
    (
        "search adverse-event text --type recalls",
        "recalls are outside this entity",
    ),
    (
        "search adverse-event text --type enforcement",
        "recalls are outside this entity",
    ),
    (
        "search adverse-event text --classification 1",
        "recalls are outside this entity",
    ),
    (
        "search adverse-event text --source all",
        "choose --source faers or --source maude",
    ),
    (
        "search adverse-event --device text",
        "use: search adverse-event <TEXT> --source maude",
    ),
    ("search adverse-event text -l 1", "use: --limit"),
    (
        "get adverse-event 100 --version 1",
        "multi-version get is unproven",
    ),
    (
        "search adverse-event text --source maude --serious death",
        "this adverse-event filter is not supported",
    ),
    (
        "drug adverse-events text",
        "use: search adverse-event <TEXT>; text stays literal",
    ),
    (
        "get adverse-event 100 outcomes",
        "use the card seriousness flags and the reactions section",
    ),
    (
        "get adverse-event 100 --section outcomes",
        "use the card seriousness flags and the reactions section",
    ),
    ("get adverse-event 100 concomitant", "renamed; use: drugs"),
    (
        "get adverse-event 100 --section concomitant",
        "renamed; use: drugs",
    ),
    (
        "get adverse-event 100 guidance",
        "choose a named adverse-event section",
    ),
    (
        "get adverse-event 100 --section guidance",
        "choose a named adverse-event section",
    ),
    (
        "get adverse-event 100 all",
        "choose a named adverse-event section",
    ),
    (
        "get adverse-event 100 --section all",
        "choose a named adverse-event section",
    ),
    ("search adverse-event text -j", "use: --format json"),
    ("-j get adverse-event 100", "use: --format json"),
    ("search adverse-event text --json", "use: --format json"),
    ("--json get adverse-event 100", "use: --format json"),
];
