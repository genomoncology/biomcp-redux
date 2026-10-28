use super::{Environment, Stub, annotation, args, invoke, record, replay, setup};
use crate::runner::transcript::trial as transcript_trial;
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::fs;
pub(super) fn trials() -> Vec<Trial> {
    vec![
        transcript_trial(
            "disease-mydisease/no-borrowed-definition-or-consider-parent",
            || missing().map_err(Into::into),
        ),
        transcript_trial("disease-mydisease/unsupported-before-io", || {
            unsupported().map_err(Into::into)
        }),
        transcript_trial(
            "disease-mydisease/unproven-identity-never-enters-cache",
            || wrong().map_err(Into::into),
        ),
        transcript_trial("disease-mydisease/failures-retain-source-and-terms", || {
            failures().map_err(Into::into)
        }),
        transcript_trial("disease-mydisease/timeout-retains-source-and-terms", || {
            timeout().map_err(Into::into)
        }),
        transcript_trial("disease-mydisease/disabled-projections-and-cache", || {
            disabled().map_err(Into::into)
        }),
    ]
}
fn missing_reply(request: &str) -> Result<String, String> {
    let obsolete = request.contains("MONDO%3A0007607");
    let id = if obsolete {
        "MONDO:0007607"
    } else {
        "MONDO:0800444"
    };
    let mut mondo = json!({"mondo":id,"label":"Synthetic native primary label"});
    if obsolete {
        super::insert(&mut mondo, "is_obsolete", json!(true))?;
        super::insert(
            &mut mondo,
            "consider",
            json!(["MONDO:0800444", "MONDO:0800445"]),
        )?;
    }
    Ok(json!(
        {"_id":id,"mondo":mondo,"disease_ontology":{"doid":"DOID:3910","name":"Different component",
        "def":"This must never become the MONDO definition."}}
    )
    .to_string())
}
fn missing() -> Result<(), String> {
    let stub = Stub::for_source("mydisease.info", "application/json", missing_reply)?;
    let env = setup(&stub)?;
    for id in ["MONDO:0800444", "MONDO:0007607"] {
        let directory = env.root().join(id.replace(':', "-"));
        let words = ["get", "disease", id];
        let value = record(&env, &words, &directory, 0)?;
        if value.pointer("/result/definition").is_some() {
            return Err("borrowed a component definition".into());
        }
        let mut expected = json!({"/result/replaced_by":null});
        if id == "MONDO:0007607" {
            super::insert(&mut expected, "/result/obsolete", json!(true))?;
            super::insert(
                &mut expected,
                "/result/consider",
                json!(["MONDO:0800444", "MONDO:0800445"]),
            )?;
        }
        replay(
            &directory,
            json!(
                [{"run":words,"mcp":words,"expect":expected},{"run":["get","disease",id,"hierarchy"],"mcp":["get","disease",id,"hierarchy"],
                "exit":1,"expect":{"/status":"absent","/page/total":0,}}]
            ),
        )?;
    }
    if stub.finish()?.len() != 2 {
        return Err("card made an enrichment request".into());
    }
    Ok(())
}
fn unsupported() -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    for words in [
        vec!["get", "disease", "NCIT:C3512"],
        vec!["get", "disease", "MONDO:bad"],
        vec!["get", "disease", "lung cancer"],
        vec!["get", "disease", "MONDO:0005061", "genes"],
        vec!["get", "disease", "MONDO:0005061", "--section-limit", "0"],
    ] {
        let value = invoke(&env, &args(&words), 2)?;
        if value.get("sources").is_some() {
            return Err("invalid request reached source handling".into());
        }
    }
    Ok(())
}
fn wrong() -> Result<(), String> {
    for malformed_alias in [false, true] {
        let stub = Stub::for_source_status(
            "mydisease.info",
            "application/json",
            move |_| {
                let mut value = annotation();
                *value
                    .pointer_mut("/mondo/xrefs/doid")
                    .ok_or("missing xref")? = if malformed_alias {
                    json!(["MONDO:0000001"])
                } else {
                    json!(["DOID:9999"])
                };
                Ok(value.to_string())
            },
            |_| 200,
        )?;
        let env = setup(&stub)?;
        let id = if malformed_alias {
            "MONDO:0000001"
        } else {
            "DOID:3910"
        };
        for _ in 0..2 {
            let value = invoke(&env, &args(&["get", "disease", id]), 4)?;
            if value.get("result").is_some()
                || value.pointer("/problems/0/field").and_then(Value::as_str) != Some("record")
            {
                return Err(format!("unproved identity admitted: {value}"));
            }
        }
        if stub.finish()?.len() != 2 {
            return Err("wrong record entered cache".into());
        }
    }
    Ok(())
}
fn failures() -> Result<(), String> {
    for (status, body) in [(429, "{}"), (500, "{}"), (200, "broken JSON")] {
        let stub = Stub::for_source_status(
            "mydisease.info",
            "application/json",
            move |_| Ok(body.into()),
            move |_| status,
        )?;
        let env = setup(&stub)?;
        let directory = env.root().join("failure");
        let words = ["get", "disease", "MONDO:0005061"];
        record(&env, &words, &directory, 4)?;
        stub.finish()?;
        replay(
            &directory,
            json!(
                [{"run":words,"mcp":words,"exit":4,"is_error":true,"expect":{"/status":"failed","/sources/0/source":"mydisease",
                "/sources/0/state":"failed","/sources/0/terms":"MyDisease source-specific terms"}}]
            ),
        )?;
    }
    Ok(())
}
fn timeout() -> Result<(), String> {
    let stub = Stub::for_source("mydisease.info", "application/json", super::reply)?;
    let env = setup(&stub)?;
    let directory = env.root().join("timeout");
    let words = ["get", "disease", "DOID:3910"];
    record(&env, &words, &directory, 0)?;
    stub.finish()?;
    for file in fs::read_dir(directory.join("mydisease")).map_err(|e| e.to_string())? {
        let file = file.map_err(|e| e.to_string())?;
        let mut value: Value =
            serde_json::from_slice(&fs::read(file.path()).map_err(|e| e.to_string())?)
                .map_err(|e| e.to_string())?;
        *value
            .pointer_mut("/exchanges/0/delay_ms")
            .ok_or("missing delay")? = json!(20000);
        fs::write(
            file.path(),
            serde_json::to_vec(&value).map_err(|e| e.to_string())?,
        )
        .map_err(|e| e.to_string())?;
    }
    replay(
        &directory,
        json!(
            [{"run":words,"mcp":words,"exit":4,"is_error":true,"expect":{"/status":"failed","/sources/0/reason":"deadline_exceeded",
            "/sources/0/terms":"MyDisease source-specific terms"}}]
        ),
    )?;
    Ok(())
}
fn disabled() -> Result<(), String> {
    let stub = Stub::for_source_status(
        "mydisease.info",
        "application/json",
        |request| {
            let mut value = annotation();
            if request.contains("fields=mondo ") {
                value
                    .as_object_mut()
                    .ok_or("missing record")?
                    .remove("disease_ontology");
            } else if !request.contains("fields=mondo%2Cdisease_ontology ") {
                return Err("unexpected component projection".into());
            }
            Ok(value.to_string())
        },
        |_| 200,
    )?;
    let env = setup(&stub)?;
    invoke(&env, &args(&["get", "disease", "MONDO:0005061"]), 0)?;
    let value = invoke(
        &env,
        &args(&[
            "get",
            "disease",
            "MONDO:0005061",
            "--disable-sources",
            "doid",
        ]),
        0,
    )?;
    if value
        .pointer("/result/components")
        .and_then(Value::as_array)
        .is_none_or(|v| v.len() != 1)
    {
        return Err("disabled component survived projection".into());
    }
    for disabled in ["mydisease", "mondo,doid"] {
        let value = invoke(
            &env,
            &args(&[
                "get",
                "disease",
                "MONDO:0005061",
                "--disable-sources",
                disabled,
            ]),
            4,
        )?;
        if value.pointer("/sources/0/reason").and_then(Value::as_str) != Some("disabled")
            || value.get("result").is_some()
        {
            return Err("disabled source reused a cached annotation".into());
        }
    }
    if stub.finish()?.len() != 2 {
        return Err("disabled projections made unexpected requests".into());
    }
    Ok(())
}
