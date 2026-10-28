//! Feature adverse-event-retirements: migration never rewrites a query or contacts a source.
use super::{configuration::invoke, launch::Environment, mcp};
use libtest_mimic::Trial;
use serde_json::json;
pub(super) fn trials() -> Vec<Trial> {
    vec![
        Trial::test("adverse-event-retirements/fixed-guidance-before-io", || {
            check().map_err(Into::into)
        }),
        Trial::test(
            "adverse-event-retirements/option-and-literal-boundaries",
            || boundaries().map_err(Into::into),
        ),
    ]
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
        "search adverse-event MMR --source cvx",
        "Vaccine identity lookup and sync are unavailable; search vaccine report summaries with `search adverse-event --source vaers`.",
    ),
    (
        "search adverse-event MMR --source=cvx",
        "Vaccine identity lookup and sync are unavailable; search vaccine report summaries with `search adverse-event --source vaers`.",
    ),
    (
        "get adverse-event 03 --source cvx",
        "Vaccine identity lookup and sync are unavailable; search vaccine report summaries with `search adverse-event --source vaers`.",
    ),
    (
        "get adverse-event 03 --source=cvx",
        "Vaccine identity lookup and sync are unavailable; search vaccine report summaries with `search adverse-event --source vaers`.",
    ),
    (
        "search adverse-event text --source=vaers",
        "supported vaccines are MMR and COVID19, or one exact native influenza code; use search \
            adverse-event influenza --source vaers to list influenza codes and labels",
    ),
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
        "--outcome is retired; use --serious death or --serious hospitalization for report flags and \
            reactions for native outcomes; the old hospitalization and disability mappings were wrong",
    ),
    (
        "search adverse-event text --outcome hospitalization",
        "--outcome is retired; use --serious death or --serious hospitalization for report flags and \
            reactions for native outcomes; the old hospitalization and disability mappings were wrong",
    ),
    (
        "search adverse-event text --outcome disability",
        "--outcome is retired; use --serious death or --serious hospitalization for report flags and \
            reactions for native outcomes; the old hospitalization and disability mappings were wrong",
    ),
    (
        "search adverse-event text --outcome recovered",
        "--outcome is retired; use --serious death or --serious hospitalization for report flags and \
            reactions for native outcomes; the old hospitalization and disability mappings were wrong",
    ),
    (
        "search adverse-event text --outcome any",
        "--outcome is retired; use --serious death or --serious hospitalization for report flags and \
            reactions for native outcomes; the old hospitalization and disability mappings were wrong",
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
        "only --serious any, --serious death and --serious hospitalization are supported",
    ),
    (
        "search adverse-event text --serious=ANY",
        "only --serious any, --serious death and --serious hospitalization are supported",
    ),
    (
        "search adverse-event text --source maude --serious any",
        "this adverse-event filter is not supported",
    ),
    (
        "search adverse-event text --serious true",
        "only --serious any, --serious death and --serious hospitalization are supported",
    ),
    (
        "search adverse-event text --serious disability",
        "only --serious any, --serious death and --serious hospitalization are supported",
    ),
    (
        "search adverse-event text --serious life-threatening",
        "only --serious any, --serious death and --serious hospitalization are supported",
    ),
    (
        "search adverse-event text --serious other",
        "only --serious any, --serious death and --serious hospitalization are supported",
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
    (
        "search adverse-event text --outcome=fatal",
        "--outcome is retired; use --serious death or --serious hospitalization for report flags and \
            reactions for native outcomes; the old hospitalization and disability mappings were wrong",
    ),
    (
        "search adverse-event text --outcome=hospitalisation",
        "--outcome is retired; use --serious death or --serious hospitalization for report flags and \
            reactions for native outcomes; the old hospitalization and disability mappings were wrong",
    ),
    (
        "search adverse-event text --outcome=disabling",
        "--outcome is retired; use --serious death or --serious hospitalization for report flags and \
            reactions for native outcomes; the old hospitalization and disability mappings were wrong",
    ),
    (
        "search adverse-event text --serious=hospitalisation",
        "only --serious any, --serious death and --serious hospitalization are supported",
    ),
    (
        "search adverse-event text --serious=lifethreatening",
        "only --serious any, --serious death and --serious hospitalization are supported",
    ),
    (
        "search adverse-event text --serious=disabling",
        "only --serious any, --serious death and --serious hospitalization are supported",
    ),
    (
        "search adverse-event text --serious=congenital",
        "only --serious any, --serious death and --serious hospitalization are supported",
    ),
    (
        "search adverse-event text --serious=congenital_anomaly",
        "only --serious any, --serious death and --serious hospitalization are supported",
    ),
    (
        "search adverse-event text --count=reaction",
        "this adverse-event filter is not supported",
    ),
    (
        "search adverse-event text --count=reactionmeddrapt",
        "this adverse-event filter is not supported",
    ),
    (
        "search adverse-event text --count=patient.reaction.reactionmeddrapt",
        "this adverse-event filter is not supported",
    ),
    (
        "search adverse-event text --count=patient.reaction.reactionmeddrapt.exact",
        "this adverse-event filter is not supported",
    ),
    (
        "search adverse-event text --count=medicinalproduct",
        "this adverse-event filter is not supported",
    ),
    (
        "search adverse-event text --count=medicinalproduct.exact",
        "this adverse-event filter is not supported",
    ),
    (
        "search adverse-event text --count=openfda.generic_name",
        "this adverse-event filter is not supported",
    ),
    (
        "search adverse-event text --count=openfda.generic_name.exact",
        "this adverse-event filter is not supported",
    ),
    (
        "search adverse-event text --count=openfda.brand_name",
        "this adverse-event filter is not supported",
    ),
    (
        "search adverse-event text --count=openfda.brand_name.exact",
        "this adverse-event filter is not supported",
    ),
    (
        "search adverse-event text --count=patientsex",
        "this adverse-event filter is not supported",
    ),
    (
        "search adverse-event text --count=patientonsetage",
        "this adverse-event filter is not supported",
    ),
    (
        "search adverse-event text --count=serious",
        "this adverse-event filter is not supported",
    ),
    (
        "search adverse-event text --count=seriousnessdeath",
        "this adverse-event filter is not supported",
    ),
    (
        "search adverse-event text --count=seriousnesshospitalization",
        "this adverse-event filter is not supported",
    ),
    (
        "search adverse-event text --count=seriousnesslifethreatening",
        "this adverse-event filter is not supported",
    ),
    (
        "search adverse-event text --count=seriousnessdisabling",
        "this adverse-event filter is not supported",
    ),
    (
        "search adverse-event text --count=seriousnesscongenitalanomali",
        "this adverse-event filter is not supported",
    ),
    (
        "search adverse-event text --count=seriousnessother",
        "this adverse-event filter is not supported",
    ),
    (
        "search adverse-event text --count=reactionoutcome",
        "this adverse-event filter is not supported",
    ),
    (
        "search adverse-event text --count=primarysource.qualification",
        "this adverse-event filter is not supported",
    ),
    (
        "search adverse-event text --count=total",
        "this adverse-event filter is not supported",
    ),
    (
        "search adverse-event text --count=meta.total",
        "this adverse-event filter is not supported",
    ),
    (
        "search adverse-event text --count=results.total",
        "this adverse-event filter is not supported",
    ),
    (
        "search adverse-event text --count=meta.results.total",
        "this adverse-event filter is not supported",
    ),
    (
        "search adverse-event text --serious=any --source=maude",
        "this adverse-event filter is not supported",
    ),
    (
        "search adverse-event text --serious=death --source=maude",
        "this adverse-event filter is not supported",
    ),
    (
        "search adverse-event text --serious=injury --source=maude",
        "this adverse-event filter is not supported",
    ),
    (
        "search adverse-event text --type=faers",
        "renamed; use: --source faers",
    ),
    (
        "search adverse-event text --type=device",
        "use: search adverse-event <TEXT> --source maude",
    ),
    (
        "search adverse-event text --type=devices",
        "use: search adverse-event <TEXT> --source maude",
    ),
    (
        "search adverse-event text --type=maude",
        "use: search adverse-event <TEXT> --source maude",
    ),
    (
        "search adverse-event text --type=recall",
        "recalls are outside this entity",
    ),
    (
        "search adverse-event text --type=recalls",
        "recalls are outside this entity",
    ),
    (
        "search adverse-event text --type=enforcement",
        "recalls are outside this entity",
    ),
    (
        "get adverse-event 100 --section=outcomes",
        "use the card seriousness flags and the reactions section",
    ),
];

// Tokens owned by other options or literal input must never become retirements.
fn boundaries() -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let globals = vec!["--offline".to_owned()];
    for words in [
        "search adverse-event text --limit=--count",
        "search adverse-event text --offset --outcome",
        "get adverse-event 100 --snapshot --json",
        "search adverse-event text --type=unsupported",
        "search adverse-event text --type",
        "search adverse-event text --type=vaers",
        "search adverse-event text --counted",
    ] {
        let mut args = globals.clone();
        args.extend(words.split_whitespace().map(str::to_owned));
        let value = invoke(&env, args.clone(), 2)?;
        if value.pointer("/error/code") != Some(&json!("usage.arguments")) {
            return Err(format!(
                "{words}: literal or unsupported selector retired: {value}"
            ));
        }
        if mcp::call_with_globals(&env, &args, true, &[])? != value {
            return Err(format!("{words}: CLI/MCP boundary differs"));
        }
        boundary_size(&value)?;
    }
    for (suffix, text, reaction) in [
        (vec!["--", "--count"], "--count", None),
        (vec!["--", "--outcome"], "--outcome", None),
        (vec!["--", "--json"], "--json", None),
        (
            vec!["--", "drug adverse-events"],
            "drug adverse-events",
            None,
        ),
        (
            vec!["--reaction=--count", "literal"],
            "literal",
            Some("--count"),
        ),
        (
            vec!["--reaction", "outcomes", "literal"],
            "literal",
            Some("outcomes"),
        ),
    ] {
        let mut args = globals.clone();
        args.extend(["search", "adverse-event"].map(str::to_owned));
        args.extend(suffix.into_iter().map(str::to_owned));
        let value = invoke(&env, args.clone(), 4).map_err(|e| format!("{args:?}: {e}"))?;
        if value.pointer("/query/text") != Some(&json!(text))
            || value.pointer("/query/reaction") != reaction.map(|r| json!(r)).as_ref()
            || value.pointer("/sources/0/reason") != Some(&json!("offline"))
        {
            return Err(format!("literal input changed: {value}"));
        }
        if mcp::call_with_globals(&env, &args, true, &[])? != value {
            return Err("literal query differs through MCP".into());
        }
        boundary_size(&value)?;
    }
    Ok(())
}

fn boundary_size(value: &serde_json::Value) -> Result<(), String> {
    let full = serde_json::to_vec(value).map_err(|e| e.to_string())?.len();
    let comparable = super::source_terms::size(value, "")?;
    // j13144: argument errors 143/143; offline literal queries at most 327/212.
    let (full_limit, comparable_limit) = if value.get("error").is_some() {
        (165, 165)
    } else {
        (377, 244)
    };
    if full > full_limit || comparable > comparable_limit {
        return Err(format!(
            "literal boundary exceeded measured ceiling: {full}/{comparable}"
        ));
    }
    Ok(())
}
