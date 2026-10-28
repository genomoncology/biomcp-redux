//! Feature drug-label: tokenized discovery retains complete brand identity.
use super::super::network_stub::Stub;
use super::{Check, both, environment, field, identity, page, record};
use crate::runner::transcript::trial as transcript_trial;
use libtest_mimic::Trial;
use serde_json::{Value, json};

pub(super) fn trials() -> Vec<Trial> {
    [
        ("lowercase", "gleevec", "Gleevec", true),
        (
            "complete-two-word",
            "sarclisa escena",
            "SARCLISA  ESCENA",
            true,
        ),
        ("different-longer-brand", "Gleevec", "Gleevec Extra", false),
        ("generic-case", "daraxonrasib", "RASONQUE", true),
        ("partial-generic", "daraxon", "RASONQUE", false),
        ("different-brand", "Gleevec", "Herceptin", false),
    ]
    .into_iter()
    .map(|(name, typed, returned, found)| {
        transcript_trial(format!("drug-label/fallback-{name}"), move || {
            fallback(typed, returned, found).map_err(Into::into)
        })
    })
    .collect()
}
fn fallback(typed: &str, returned: &str, found: bool) -> Check {
    let mut id = identity(0)?;
    *id.get_mut("brand").ok_or("brand")? = json!(returned);
    let mut row = record(&id);
    row.get_mut("openfda")
        .and_then(Value::as_object_mut)
        .ok_or("openfda names")?
        .insert("generic_name".into(), json!(["DARAXONRASIB"]));
    let body = page(row).to_string();
    let encoded = typed.replace(' ', "%20");
    let exact = target(&format!(
        "openfda.brand_name.exact%3A%22{encoded}%22%20OR%20openfda.generic_name.exact%3A%22{encoded}%22"
    ));
    let tokenized = target(&format!(
        "openfda.brand_name%3A%22{encoded}%22%20OR%20openfda.generic_name%3A%22{encoded}%22"
    ));
    let tokenized_status = tokenized.clone();
    let stub = Stub::for_source_status(
        "api.fda.gov",
        "application/json",
        move |request| match request.lines().next() {
            Some(line) if line == exact => Ok(json!({"error": {
                "code":"NOT_FOUND", "message":"No matches found!"
            }})
            .to_string()),
            Some(line) if line == tokenized => Ok(body.clone()),
            _ => Err(format!("unexpected label request: {request}")),
        },
        move |request| {
            if request.lines().next() == Some(tokenized_status.as_str()) {
                200
            } else {
                404
            }
        },
    )?;
    let (env, stub) = environment(stub)?;
    let args = [
        "search",
        "drug",
        typed,
        "--source",
        "openfda-label",
        "--limit",
        "1",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect();
    let value = both(&env, args, if found { 0 } else { 1 })?;
    if found {
        if value.pointer("/result/items/0/id") != Some(field(&id, "set_id"))
            || value.pointer("/result/items/0/name") != Some(&json!(returned))
        {
            return Err("fallback lost complete brand identity".into());
        }
        let basis = if typed == "daraxonrasib" {
            "generic"
        } else {
            "brand"
        };
        if value.pointer("/result/items/0/matched_name") != Some(&json!(basis)) {
            return Err("fallback lost matched name basis".into());
        }
    } else if field(&value, "status") != "absent"
        || value.pointer("/result/items").is_some()
        || value.pointer("/result/total") != Some(&json!(1))
    {
        return Err("fallback accepted a different brand".into());
    }
    let requests = stub.finish()?;
    let expected = 3;
    if requests.len() != expected {
        return Err(format!(
            "fallback request count {}/{expected}",
            requests.len()
        ));
    }
    Ok(())
}
fn target(selector: &str) -> String {
    format!("GET /drug/label.json?search={selector}&limit=1&sort=effective_time%3Adesc HTTP/1.1")
}
