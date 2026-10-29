//! Malformed requests cross the real CLI with no admitted provider requests.
use super::{Value, gwas_budgets, wire};
pub(super) fn run() -> Result<(), String> {
    let stub = super::super::network_stub::Stub::for_source(
        "www.ebi.ac.uk",
        "application/json",
        wire::live_body,
    )?;
    let env = super::super::variant::source_environment(&stub)?;
    for args in inputs() {
        let expected = if args.first() == Some(&"search") {
            "search gwas"
        } else {
            "get gwas"
        };
        let value = gwas_budgets::invoke(
            "gwas_search/usage/run/1",
            &env,
            args.iter().map(|a| (*a).to_owned()).collect(),
            2,
        )?;
        let message = value
            .pointer("/error/message")
            .and_then(Value::as_str)
            .ok_or("usage message")?;
        if !message.contains(expected) || value.get("sources").is_some() {
            return Err(format!("missing route guidance for {args:?}: {value}"));
        }
        if serde_json::to_vec(&value).map_err(|e| e.to_string())?.len() > 1800 {
            return Err("usage response exceeds 1800 bytes".into());
        }
    }
    if !stub.finish()?.is_empty() {
        return Err("malformed input made a request".into());
    }
    Ok(())
}
fn inputs() -> Vec<Vec<&'static str>> {
    vec![
        vec!["search", "gwas"],
        vec!["search", "gwas", "--include-child-traits"],
        vec!["search", "gwas", "--extended-geneset"],
        vec!["search", "gwas", "--study", "GCSTwrong"],
        vec!["search", "gwas", "--variant", "RS2981582"],
        vec!["search", "gwas", "--pubmed", "0"],
        vec!["search", "gwas", "--trait-id", "EFO/0000305"],
        vec!["search", "gwas", "--gene", "FGFR2", "--limit", "0"],
        vec!["search", "gwas", "--gene", "FGFR2", "--limit", "51"],
        vec!["search", "gwas", "--gene", "FGFR2", "--offset", "-1"],
        vec!["search", "gwas", "--gene", "FGFR2", "--sort", "p_value"],
        vec!["get", "gwas", "6114"],
        vec!["get", "gwas", "association:0"],
        vec!["get", "gwas", "association:-1"],
        vec!["get", "gwas", "GCSTwrong"],
        vec!["get", "gwas", "GCST000035", "loci"],
        vec!["get", "gwas", "association:6114", "associations"],
        vec!["get", "gwas", "association:6114", "ancestries"],
        vec![
            "get",
            "gwas",
            "GCST000035",
            "ancestries",
            "--section-limit",
            "0",
        ],
        vec![
            "get",
            "gwas",
            "GCST000035",
            "ancestries",
            "--section-limit",
            "51",
        ],
        vec![
            "get",
            "gwas",
            "GCST000035",
            "ancestries",
            "--section-offset",
            "-1",
        ],
    ]
}
