//! Retained native CIViC gene values cross the command boundary independently.
use serde_json::json;

pub(super) fn values() -> Result<(), Box<dyn std::error::Error>> {
    for (field, absent, invalid) in [
        ("link", "url", json!(42)),
        ("variants.totalCount", "variant_count", json!(-1)),
    ] {
        crate::runner::response_cases::checked_case(
            "gene/civic",
            &["get", "gene", "ERBB2", "civic"],
            |source, record| {
                if source == "civic"
                    && record.pointer("/exchanges/0/body/value/data/g0/id") == Some(&json!(20))
                    && let Some(value) = record.pointer_mut(&format!(
                        "/exchanges/0/body/value/data/g0/{}",
                        field.replace('.', "/")
                    ))
                {
                    *value = invalid.clone();
                }
            },
            json!({
                "/result/items/0/sections/civic/data/id":20,
                "/result/items/0/sections/civic/data":{"omits":[absent]},
                "/problems":{"count":1},"/problems/0/field":field,"/problems/0/record":"20"
            }),
            0,
        )?;
    }
    Ok(())
}

pub(super) fn identities() -> Result<(), Box<dyn std::error::Error>> {
    for (field, invalid) in [("id", json!(0)), ("entrezId", json!(-1))] {
        crate::runner::response_cases::checked_case(
            "gene/civic",
            &["get", "gene", "ERBB2", "civic"],
            |source, record| {
                if source == "civic"
                    && record.pointer("/exchanges/0/body/value/data/g0/id") == Some(&json!(20))
                    && let Some(value) =
                        record.pointer_mut(&format!("/exchanges/0/body/value/data/g0/{field}"))
                {
                    *value = invalid.clone();
                }
            },
            json!({
                "/result/items/0/answer/gene/symbol":"ERBB2",
                "/result/items/0/answer/gene/hgnc_id":"HGNC:3430",
                "/result/items/0/sections/civic/state":"empty",
                "/result/items/0/sections/civic":{"omits":["record","data"]},
                "/problems":{"count":1},"/problems/0/field":field
            }),
            0,
        )?;
    }
    Ok(())
}
