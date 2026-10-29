//! Synthetic OncoKB gene replies keep native literals and independent card values.
use serde_json::{Value, json};
use std::fs;

pub(super) fn classifications() -> Result<(), String> {
    case(
        None,
        Some(json!({
            "/result/items/0/answer/gene/symbol":"TP53",
            "/result/items/0/oncokb/gene_type":"TSG",
            "/result/items/0/oncokb/gene_type_code":"TSG",
            "":{"omits":["problems"]}
        })),
    )?;
    for (invalid, expected) in [
        (
            json!(" TSG "),
            json!({
                "/result/items/0/oncokb/gene_type":" TSG ",
                "/result/items/0/oncokb":{"omits":["gene_type_code"]}
            }),
        ),
        (
            json!(" FUTURE_NATIVE_ROLE "),
            json!({
                "/result/items/0/oncokb/gene_type":" FUTURE_NATIVE_ROLE ",
                "/result/items/0/oncokb":{"omits":["gene_type_code"]}
            }),
        ),
        (
            json!("FUTURE_NATIVE_ROLE"),
            json!({
                "/result/items/0/oncokb/gene_type":"FUTURE_NATIVE_ROLE",
                "/result/items/0/oncokb":{"omits":["gene_type_code"]}
            }),
        ),
        (
            json!(42),
            json!({
                "/result/items/0/oncokb":{"omits":["gene_type","gene_type_code"]}
            }),
        ),
    ] {
        let mut expect = expected.as_object().ok_or("expected object")?.clone();
        expect.extend(
            json!({
                "/result/items/0/answer/gene/symbol":"TP53",
                "/result/items/0/sections/oncokb/data/background":"Synthetic background.",
                "/status":"partial","/problems":{"count":1},"/problems/0/field":"geneType"
            })
            .as_object()
            .ok_or("expected object")?
            .clone(),
        );
        case(Some(("/0/geneType", invalid)), Some(Value::Object(expect)))?;
    }
    Ok(())
}
pub(super) fn version() -> Result<(), String> {
    case(None, None)?;
    case(
        Some(("/dataVersion/version", json!(42))),
        Some(json!({
            "/result/items/0/answer/gene/symbol":"TP53",
            "/result/items/0/oncokb/gene_type_code":"TSG",
            "/result/items/0/oncokb":{"omits":["version"]},
            "/result/items/0/sections/oncokb/data":{"omits":["version"]},
            "/status":"partial","/problems":{"count":1},"/problems/0/field":"dataVersion.version"
        })),
    )
}
fn case(mutation: Option<(&str, Value)>, expected: Option<Value>) -> Result<(), String> {
    let directory = tempfile::tempdir().map_err(|e| e.to_string())?;
    let capture = directory.path().join("recordings");
    crate::runner::response_cases::copy_capture(
        "gene/oncokb-synthetic/data",
        &capture,
        |source, record| {
            if source == "oncokb"
                && let Some((field, value)) = &mutation
                && let Some(target) = record.pointer_mut(&format!("/exchanges/0/body/value{field}"))
            {
                *target = value.clone();
            }
        },
    )
    .map_err(|e| e.to_string())?;
    let original = crate::runner::launch::fixtures().join("cases/gene/sections/oncokb-data.yaml");
    let mut case: Value = serde_json::from_slice(&fs::read(original).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    *case.get_mut("replay").ok_or("missing original replay")? = json!(capture);
    if let Some(expected) = expected {
        *case
            .pointer_mut("/steps/0/expect")
            .ok_or("missing original step")? = expected;
    }
    let path = directory.path().join("case.yaml");
    fs::write(&path, serde_json::to_vec(&case).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    crate::runner::run_case(&path, &crate::runner::schema::Registry::embedded()?)?;
    Ok(())
}
