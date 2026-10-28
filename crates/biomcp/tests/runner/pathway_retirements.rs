//! Feature pathway-retirements: fixed migration guidance precedes parsing and I/O.
use super::launch::Environment;
use crate::runner::pathway_budgets as budgets;
use crate::runner::transcript::trial as transcript_trial;
use libtest_mimic::Trial;
use serde_json::json;
const TEXT: &str = "use: search pathway <text>";
const CLASS: &str =
    "--type is retired: the accepted pathway class is enforced and needs no filter.";
const KEGG: &str = "KEGG pathways are dropped; use a Reactome R-HSA-… or WikiPathways WP… ID.";
const ENRICH: &str = "Gene-set enrichment is dropped; an agent can call g:Profiler itself.";
const DRUGS: &str = "use: get pathway ID genes; reverse gene-to-drug search is not supported; select a drug identity and request its explicit source sections.";
const ARTICLES: &str =
    "use: search article <text>; a matching title does not establish pathway article linkage.";
const TRIALS: &str = "use: search trial --gene <gene>; pathway-name trial search is retired.";
const LIST: &str = "use: help search pathway; help get pathway";
pub(super) fn trials() -> Vec<Trial> {
    cases()
        .into_iter()
        .map(|(args, message)| {
            transcript_trial(format!("pathway-retirements/{args}"), move || {
                check(&args, message).map_err(Into::into)
            })
        })
        .collect()
}
fn cases() -> Vec<(String, &'static str)> {
    let mut cases = vec![
        ("search pathway -q MAPK".into(), TEXT),
        ("search pathway --query MAPK".into(), TEXT),
        ("search pathway --query=MAPK".into(), TEXT),
        ("search pathway -l 1 --query MAPK".into(), TEXT),
        ("search pathway -l1 --query MAPK".into(), TEXT),
        ("search pathway MAPK --type Pathway".into(), CLASS),
        ("search pathway MAPK --type=Pathway".into(), CLASS),
        ("get pathway hsa04151".into(), KEGG),
        ("get pathway path:hsa04151".into(), KEGG),
        ("get pathway map04151".into(), KEGG),
        ("get pathway ko04151".into(), KEGG),
        ("get pathway mmu04151".into(), KEGG),
        ("get pathway R-HSA-5673001 enrichment".into(), ENRICH),
        (
            "get pathway R-HSA-5673001 --section enrichment".into(),
            ENRICH,
        ),
        ("enrich BRAF,KRAS".into(), ENRICH),
        ("enrich BRAF,KRAS -l 1".into(), ENRICH),
        ("enrich BRAF,KRAS --limit 1".into(), ENRICH),
        ("list pathway".into(), LIST),
    ];
    for (helper, message) in [("drugs", DRUGS), ("articles", ARTICLES), ("trials", TRIALS)] {
        for option in ["", " -l 1", " --limit 1", " --offset 1"] {
            cases.push((format!("pathway {helper} R-HSA-5673001{option}"), message));
        }
    }
    for source in ["ctgov", "nci"] {
        cases.push((
            format!("pathway trials R-HSA-5673001 --source {source}"),
            TRIALS,
        ));
    }
    cases
}
fn check(words: &str, message: &str) -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let listener = std::net::TcpListener::bind("127.0.0.1:0").map_err(|e| e.to_string())?;
    listener.set_nonblocking(true).map_err(|e| e.to_string())?;
    env.config(
        &json!({"proxy":format!("http://{}", listener.local_addr().map_err(|e|e.to_string())?)}),
    )
    .map_err(|e| e.to_string())?;
    let args: Vec<String> = words.split_whitespace().map(str::to_owned).collect();
    let value = budgets::invoke("runner/pathway_retirements/check/1", &env, args.clone(), 2)?;
    if value.pointer("/error/code") != Some(&json!("usage.retired"))
        || value.pointer("/error/message") != Some(&json!(message))
        || value.get("sources").is_some()
    {
        return Err(format!("{words}: wrong fixed guidance: {value}"));
    }
    if budgets::mcp("runner/pathway_retirements/check/2", &env, &args, true, &[])? != value {
        return Err(format!("{words}: CLI/MCP guidance differs"));
    }
    if serde_json::to_vec(&value).map_err(|e| e.to_string())?.len() > 400 {
        return Err(format!("{words}: response exceeds its size ceiling"));
    }
    if listener.accept().is_ok() {
        return Err("retirement contacted a source".into());
    }
    Ok(())
}
