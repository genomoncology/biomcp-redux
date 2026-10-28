//! Feature disease-monarch: evidence presence follows either native reference input.
use super::{association, invoke, setup};
use crate::runner::network_stub::Stub;
use serde_json::json;

pub(super) fn run() -> Result<(), String> {
    for (ids, links, expected) in [
        (None, None, None),
        (Some(json!(null)), Some(json!(null)), None),
        (Some(json!([])), None, None),
        (None, Some(json!([])), None),
        (
            Some(json!(["ECO:0000501"])),
            None,
            Some(json!([{"id":"ECO:0000501"}])),
        ),
        (
            None,
            Some(json!([{"id":"ECO:0000501","label":"Synthetic evidence"}])),
            Some(json!([{"id":"ECO:0000501","label":"Synthetic evidence"}])),
        ),
    ] {
        for (entity, id, section) in [
            ("disease", "MONDO:0019003", "phenotypes"),
            ("phenotype", "HP:0001250", "genes"),
        ] {
            let ids = ids.clone();
            let links = links.clone();
            let stub = Stub::for_source_status(
                "api-v3.monarchinitiative.org",
                "application/json",
                move |_| {
                    let mut row = association();
                    let fields = row.as_object_mut().ok_or("missing synthetic row")?;
                    fields.remove("has_evidence");
                    if let Some(value) = ids.clone() {
                        fields.insert("has_evidence".into(), value);
                    }
                    if let Some(value) = links.clone() {
                        fields.insert("has_evidence_links".into(), value);
                    }
                    if entity == "phenotype" {
                        fields.insert("subject_taxon".into(), json!("NCBITaxon:9606"));
                        fields.insert("object".into(), json!("HP:0001250"));
                    } else {
                        fields.insert("subject".into(), json!("MONDO:0019003"));
                        fields.insert("object".into(), json!("HP:0001250"));
                    }
                    Ok(json!({"limit":1,"offset":0,"total":1,"items":[row]}).to_string())
                },
                |_| 200,
            )?;
            let env = setup(&stub)?;
            let result = invoke(
                &env,
                &["get", entity, id, section, "--source", "monarch"],
                0,
            )?;
            if result.pointer("/result/items/0/association/evidence") != expected.as_ref() {
                return Err(format!("native evidence presence changed: {result}"));
            }
            if stub.finish()?.len() != 1 {
                return Err("unexpected evidence request count".into());
            }
        }
    }
    Ok(())
}
