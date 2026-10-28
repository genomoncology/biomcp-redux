//! Feature protein-domains-complexes: synthetic returned identities are dropped before caching.
use super::super::mcp;
use super::{Stub, Trial, Value, call, json, setup};
pub(super) fn trials() -> Vec<Trial> {
    ["accession", "isoform", "ipr", "cpx"]
        .into_iter()
        .map(|kind| {
            Trial::test(
                format!("protein-domains-complexes/synthetic-wrong-{kind}-not-cached"),
                move || run(kind).map_err(Into::into),
            )
        })
        .collect()
}
fn run(kind: &'static str) -> Result<(), String> {
    let stub = Stub::for_sources(
        &["rest.uniprot.org", "www.ebi.ac.uk"],
        move |_| {
            if kind == "isoform" {
                "text/plain"
            } else {
                "application/json"
            }
        },
        move |request| {
            if kind == "isoform" {
                return Ok(">sp|P38398|SYNTHETIC_HUMAN Synthetic OX=9606 GN=SYN\nACDEFG\n".into());
            }
            if kind == "ipr" {
                return Ok(json!(
                    {"count":1,"next":null,"results":[{"metadata":{"accession":"CPX-715","name":"Synthetic wrong IPR identity",
                    "source_database":"interpro","type":"family"},"proteins":[{"accession":"p38398","organism":"9606","entry_protein_locations":[]}]}]}
                )
                .to_string());
            }
            let mut value: Value = serde_json::from_str(&if kind == "cpx" {
                super::complexes::reply(request)?
            } else {
                super::cards::reply(request)?
            })
            .map_err(|e| e.to_string())?;
            if kind == "cpx" {
                *value.get_mut("complexAc").ok_or("CPX identity")? = json!("CPX-999");
            } else {
                *value
                    .get_mut("primaryAccession")
                    .ok_or("UniProt accession")? = json!("P00533");
            }
            Ok(value.to_string())
        },
    )?;
    let env = setup(&stub)?;
    let args = match kind {
        "isoform" => vec!["protein", "sequence", "P38398-2"],
        "ipr" => vec!["protein", "domains", "P38398"],
        "cpx" => vec![
            "protein",
            "complex-members",
            "P38398",
            "--complex",
            "CPX-715",
        ],
        _ => vec!["get", "protein", "P38398"],
    };
    for _ in 0..2 {
        let value = call(&env, &args, 4)?;

        rejected(&value, kind)?;
    }
    let value = mcp::call_with_globals(
        &env,
        &args.iter().map(|s| (*s).to_owned()).collect::<Vec<_>>(),
        true,
        &[],
    )?;

    rejected(&value, kind)?;
    if stub.finish()?.len() != 3 {
        return Err("wrong returned identity entered cache".into());
    }
    Ok(())
}
fn rejected(value: &Value, kind: &str) -> Result<(), String> {
    if value.get("status") != Some(&json!("failed"))
        || value.pointer("/sources/0/reason") != Some(&json!("bad_reply"))
        || value
            .get("problems")
            .and_then(Value::as_array)
            .is_none_or(Vec::is_empty)
        || value.pointer("/result/items/0/header").is_some()
        || (kind != "accession" && value.get("result").is_some())
    {
        return Err(format!(
            "wrong {kind} identity was not dropped with a problem: {value}"
        ));
    }
    let (cli_ceiling, mcp_ceiling) = identity_ceilings(kind)?;
    super::card_failures::bounded(value, cli_ceiling, mcp_ceiling)
}

// Complete synthetic envelopes measured in j15006, with fifteen percent headroom.
fn identity_ceilings(kind: &str) -> Result<(usize, usize), String> {
    Ok(match kind {
        "accession" => (566, 565),
        "isoform" => (511, 510),
        "ipr" => (525, 524),
        "cpx" => (585, 584),
        _ => return Err("missing measured protein case ceiling".into()),
    })
}
