//! Synthetic mutations preserve native claim IDs without widening clinical context.
use super::{civic_body, configure, run};
use crate::runner::{launch::Environment, network_stub::Stub};
use serde_json::{Value, json};

pub(super) fn claims() -> Result<(), String> {
    // Original invocation is the positive control for every mutation in this group.
    check(
        None,
        json!({"/result/candidates/0/sections/civic/data/profiles/0/statements/0/record/id":"EID1409"}),
    )?;
    for (path, value, field, absent) in [
        (
            "evidenceItems/nodes/0/evidenceLevel",
            json!(-1),
            "evidenceLevel",
            "statement/strength",
        ),
        (
            "evidenceItems/nodes/0/disease/name",
            json!(42),
            "disease.name",
            "statement",
        ),
        (
            "evidenceItems/nodes/0/evidenceDirection",
            json!([]),
            "evidenceDirection",
            "statement",
        ),
        (
            "assertions/nodes/0/ampLevel",
            json!("UNPUBLISHED_TIER"),
            "ampLevel",
            "statement",
        ),
    ] {
        let index = if path.starts_with("assertions") { 2 } else { 0 };
        let base =
            format!("/result/candidates/0/sections/civic/data/profiles/0/statements/{index}");
        let (omit_base, absent) = if absent == "statement/strength" {
            (format!("{base}/statement"), "strength")
        } else {
            (base.clone(), absent)
        };
        check(
            Some((path, value)),
            json!({
                format!("{base}/record/id"):if index == 2 {"AID7"} else {"EID1409"},
                omit_base:{"omits":[absent]},
                "/problems":{"count":1},"/problems/0/field":field
            }),
        )?;
    }
    check(
        Some(("evidenceItems/nodes/0/id", json!(0))),
        json!({
            "/result/candidates/0/sections/civic/data/profiles/0/statements":{"count":2},
            "/result/candidates/0/sections/civic/data/profiles/0/statements/0/record/id":"EID9851",
            "/problems":{"count":1},"/problems/0/field":"id"
        }),
    )?;
    Ok(())
}
pub(super) fn amp_and_group() -> Result<(), String> {
    check(
        None,
        json!({"/result/candidates/0/sections/civic/data/profiles/0/statements/2/therapy_interaction_type":"COMBINATION"}),
    )?;
    check(
        Some(("assertions/nodes/0/therapyInteractionType", Value::Null)),
        json!({
            "/result/candidates/0/sections/civic/data/profiles/0/statements/2/record/id":"AID7",
            "/result/candidates/0/sections/civic/data/profiles/0/statements/2":{"omits":["statement","therapy_interaction_type"]},
            "/problems":{"count":1},"/problems/0/field":"therapyInteractionType"
        }),
    )?;
    check(
        Some(("assertions/nodes/0/ampLevel", json!("NA"))),
        json!({
            "/result/candidates/0/sections/civic/data/profiles/0/statements/2/amp_level":"NA",
            "/result/candidates/0/sections/civic/data/profiles/0/statements/2/native/classification/label":"SENSITIVITYRESPONSE",
            "":{"omits":["problems"]}
        }),
    )?;
    Ok(())
}
fn check(mutation: Option<(&'static str, Value)>, expected: Value) -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let stub = Stub::for_sources(
        &["civicdb.org"],
        |_| "application/json",
        move |request| {
            let mut body: Value =
                serde_json::from_str(&civic_body(request)?).map_err(|e| e.to_string())?;
            if let Some((path, value)) = &mutation {
                *body
                    .pointer_mut(&format!("/data/molecularProfiles/nodes/0/{path}"))
                    .ok_or("missing mutation field")? = value.clone();
            }
            Ok(body.to_string())
        },
    )?;
    configure(&env, &stub)?;
    let value = run(
        &env,
        &[
            "get",
            "variant",
            "CA123643",
            "civic",
            "--disable-sources",
            "myvariant",
        ],
    )?;
    let step: crate::runner::Step =
        serde_json::from_value(json!({"expect":expected})).map_err(|e| e.to_string())?;
    let expectations = crate::runner::assertions::prepare(&step)?;
    crate::runner::assertions::check(
        &serde_json::to_vec(&value).map_err(|e| e.to_string())?,
        &step,
        &expectations,
    )?;
    stub.finish()?;
    Ok(())
}
