use super::super::response_cases::checked_case;
use super::{Check, case};
use serde_json::json;

pub(super) fn uniprot() -> Check {
    let args = ["get", "gene", "CDKN2A", "protein"];
    let identity = "/result/items/0/sections/protein/data/0/accession";
    case(
        "gene/protein",
        &args,
        |_, _| {},
        json!({identity: "P42771"}),
        0,
    )?;
    let answer = checked_case(
        "gene/protein",
        &args,
        |source, record| {
            if source == "uniprot"
                && let Some(field) = record.pointer_mut("/exchanges/0/body/value/results/0/proteinDescription/recommendedName/fullName/value")
            {
                *field = json!(42);
            }
        },
        json!({identity: "P42771", "/problems": {"count": 1},
        "/problems/*/field": ["protein_name"]}),
        0,
    )?;
    if answer
        .pointer("/result/items/0/sections/protein/data/0/name")
        .is_some()
    {
        return Err("bad protein name survived in retained UniProt identity".into());
    }
    Ok(())
}

pub(super) fn reactome() -> Check {
    let args = ["get", "pathway", "R-HSA-5673001", "summary"];
    let identity = "/result/pathway/id/id";
    case(
        "pathway/native",
        &args,
        |_, _| {},
        json!({identity: "R-HSA-5673001"}),
        0,
    )?;
    let answer = checked_case(
        "pathway/native",
        &args,
        |source, record| {
            if source == "reactome"
                && record
                    .get("url")
                    .and_then(serde_json::Value::as_str)
                    .is_some_and(|u| u.ends_with("/data/query/R-HSA-5673001"))
                && let Some(field) = record.pointer_mut("/exchanges/0/body/value/displayName")
            {
                *field = json!(42);
            }
        },
        json!({identity: "R-HSA-5673001", "/problems": {"count": 1},
        "/problems/*/field": ["name"]}),
        0,
    )?;
    if answer.pointer("/result/pathway/name").is_some() {
        return Err("bad pathway name survived in retained Reactome identity".into());
    }
    Ok(())
}

pub(super) fn native_summation() -> Check {
    let text = format!(" \n{}\t ", "x".repeat(biodata::Text::MAX_CHARS + 1));
    let answer = checked_case(
        "pathway/native",
        &["get", "pathway", "R-HSA-5673001", "summary"],
        |source, record| {
            if source == "reactome"
                && record
                    .get("url")
                    .and_then(serde_json::Value::as_str)
                    .is_some_and(|u| u.ends_with("/data/query/R-HSA-5673001"))
                && let Some(field) = record.pointer_mut("/exchanges/0/body/value/summation/0/text")
            {
                *field = json!(text);
            }
        },
        json!({"/result/pathway/id/id": "R-HSA-5673001"}),
        0,
    )?;
    if answer.pointer("/result/pathway/summations/0/text") != Some(&json!(text)) {
        return Err("native summation text changed or gained typed-text limits".into());
    }
    Ok(())
}
