//! Contract: closed projections preserve released indexing values and reject malformed shapes.
use super::projection::Projection;
use crate::runner::transcript::trial as transcript_trial;
use libtest_mimic::Trial;
use serde_json::{Value, json};

pub(super) fn trial() -> Trial {
    transcript_trial("mcp-parity/released-indexing-projections", || {
        check().map_err(Into::into)
    })
}
fn project(field: &str, raw: &Value) -> Result<Value, String> {
    let projection: Projection = serde_json::from_value(json!({
        "article_indexing":"/indexing","field":field
    }))
    .map_err(|error| error.to_string())?;
    projection.select(&json!({"indexing":raw}))
}
fn check() -> Result<(), String> {
    let affiliations = (0..12)
        .map(|i| json!({"text":format!("Affiliation {i}")}))
        .collect::<Vec<_>>();
    let raw = json!({"authors":[
        {"affiliations":affiliations,"orcid":"0000-0001-0000-0001"},
        {"affiliations":[],"orcid":"0000-0001-0000-0001"},
        {"affiliations":[],"orcid":"0000-0001-0000-0002"},
        {"affiliations":[]}
    ],"mesh_headings":[{"descriptor":{"text":"Source descriptor","major_topic":true},
        "qualifiers":[{"text":"Source qualifier","major_topic":false}]}]});
    for (field, expected) in [
        ("affiliations_total", json!(12)),
        (
            "affiliations_items",
            json!(
                (0..10)
                    .map(|i| format!("Affiliation {i}"))
                    .collect::<Vec<_>>()
            ),
        ),
        (
            "orcids",
            json!(["0000-0001-0000-0001", "0000-0001-0000-0002"]),
        ),
        (
            "mesh",
            json!([{"descriptor":"Source descriptor","major":true,
            "qualifiers":[{"name":"Source qualifier","major":false}]}]),
        ),
    ] {
        if project(field, &raw)? != expected {
            return Err(format!("released indexing values changed for {field}"));
        }
    }
    for field in ["affiliations_items", "orcids", "mesh"] {
        if project(field, &json!({"authors":[],"mesh_headings":[]}))? != json!([]) {
            return Err("empty native array was not preserved".into());
        }
    }
    for (field, raw) in [
        ("affiliations_total", json!({"authors":null})),
        (
            "affiliations_items",
            json!({"authors":[{"affiliations":[{"text":false}]}]}),
        ),
        ("orcids", json!({"authors":[{"orcid":null}]})),
        (
            "mesh",
            json!({"mesh_headings":[{"descriptor":{"text":"x","major_topic":"Y"},"qualifiers":[]}]}),
        ),
    ] {
        if project(field, &raw).is_ok() {
            return Err(format!("malformed released {field} accepted"));
        }
    }
    for shape in [
        json!({"article_indexing":"/indexing","field":"invented"}),
        json!({"article_indexing":"/indexing","field":"mesh","expected":[]}),
    ] {
        if serde_json::from_value::<Projection>(shape).is_ok() {
            return Err("indexing projection accepted invented facts".into());
        }
    }
    Ok(())
}
