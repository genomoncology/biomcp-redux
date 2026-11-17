//! Annual source paging advances through a refused row without changing native positions.
use super::super::{configuration::invoke, launch, network_stub::Stub, schema::Registry, variant};
use super::{Check, save};
use serde_json::{Value, json};
use std::path::Path;
pub(super) fn seer(directory: &Path, registry: &Registry) -> Check {
    let fixture = launch::fixture_text(
        "replies/disease/seer-synthetic/seer/6034e658f1d299da7d80b7b51bc8f9751bd9ce674eaa5b4f328a617e4c81c671.json",
    )?;
    let envelope: Value = serde_json::from_str(&fixture).map_err(|e| e.to_string())?;
    let body = envelope
        .pointer("/exchanges/0/body/value")
        .and_then(Value::as_str)
        .ok_or("retained annual JSON string body")?;
    let body: Value = serde_json::from_str(body).map_err(|error| error.to_string())?;
    let words = [
        "get",
        "disease",
        "55",
        "survival",
        "--source",
        "seer",
        "--sex",
        "3",
        "--race",
        "1",
        "--age-range",
        "1",
        "--dataset",
        "4",
        "--interval",
        "5",
        "--section-limit",
        "1",
    ];
    for refused in [false, true] {
        let mut body = body.clone();
        if refused {
            let series = body
                .get_mut("data")
                .and_then(Value::as_object_mut)
                .and_then(|data| data.values_mut().next())
                .ok_or("retained annual series")?;
            *series
                .pointer_mut("/data_series/0/0")
                .ok_or("retained annual year")? = json!({"bad_shape":true});
        }
        let stub = seer_stub(&body)?;
        let env = variant::source_environment(&stub)?;
        let value = invoke(
            &env,
            words
                .into_iter()
                .chain(["--fresh"])
                .map(str::to_owned)
                .collect(),
            0,
        )?;
        if stub.finish()?.len() != 2 {
            return Err("annual source request count changed".into());
        }
        registry.validate(&value, &words.map(str::to_owned), true)?;
        save(
            directory,
            if refused {
                "seer-refused-window"
            } else {
                "seer-window-positive"
            },
            &value,
        )?;
        annual_window(&value, refused)?;
    }
    Ok(())
}

/// The native endpoint returns a JSON string containing the document; since 0720 an
/// annual read also fetches the site label catalog.
fn seer_stub(body: &Value) -> Result<Stub, String> {
    let catalog: Value = serde_json::from_str(&launch::fixture_text(
        "replies/disease/seer-synthetic/seer/223a28d053d7e4bbf1a87ce337a5b800d2839356b691fa24ee0df85d7d52be5c.json",
    )?)
    .map_err(|e| e.to_string())?;
    let catalog = catalog
        .pointer("/exchanges/0/body/value")
        .ok_or("retained label catalog body")?
        .to_string();
    let body = json!(body.to_string()).to_string();
    Stub::for_source("seer.cancer.gov", "application/json", move |request| {
        Ok(if request.contains("get_var_formats.php") {
            catalog.clone()
        } else {
            body.clone()
        })
    })
}

fn annual_window(value: &Value, refused: bool) -> Check {
    if value.pointer("/page/total") != Some(&json!(2)) {
        return Err("annual native total changed".into());
    }
    let next = value
        .get("next")
        .and_then(Value::as_array)
        .and_then(|next| {
            next.iter()
                .find(|next| next.get("kind") == Some(&json!("page")))
        });
    if !next
        .and_then(|next| next.get("args"))
        .and_then(Value::as_array)
        .is_some_and(|args| {
            args.windows(2)
                .any(|pair| pair == [json!("--section-offset"), json!("1")])
        })
    {
        return Err("annual native window did not advance".into());
    }
    if refused
        && (value.pointer("/page/returned") != Some(&json!(0))
            || !value
                .get("problems")
                .and_then(Value::as_array)
                .is_some_and(|problems| {
                    problems
                        .iter()
                        .any(|problem| problem.get("field") == Some(&json!("year")))
                }))
    {
        return Err("refused annual identity did not retain its problem".into());
    }
    Ok(())
}

pub(super) fn study(directory: &Path, registry: &Registry) -> Check {
    for (section, identity) in [
        ("profiles", "molecularProfileId"),
        ("sample-lists", "sampleListId"),
    ] {
        let mut row = json!({
            "studyId":"synthetic_0", "molecularAlterationType":"MRNA_EXPRESSION",
            "datatype":"CONTINUOUS", "name":"Synthetic profile",
            "description":"Native profile description", "patientLevel":false,
            "showProfileInAnalysisTab":true, "category":"all_cases_in_study", "sampleCount":1
        });
        row.as_object_mut()
            .ok_or("study row object")?
            .insert(identity.into(), json!("native_1"));
        let mut bad = row.clone();
        bad.as_object_mut()
            .ok_or("study refused row object")?
            .insert(identity.into(), json!({"bad_shape":true}));
        let stub = study_stub(json!([bad, row]))?;
        let env = variant::source_environment(&stub)?;
        let words = [
            "get",
            "study",
            "synthetic_0",
            section,
            "--section-limit",
            "1",
            "--fresh",
        ];
        let first = invoke(&env, words.map(str::to_owned).to_vec(), 0)?;
        registry.validate(&first, &words.map(str::to_owned), true)?;
        save(
            directory,
            &format!("study-{section}-refused-window"),
            &first,
        )?;
        if first.pointer("/page/total") != Some(&json!(2))
            || first.pointer("/page/returned") != Some(&json!(0))
        {
            return Err(format!(
                "{section}: refused identity shifted native page positions"
            ));
        }
        let continuation = first
            .get("next")
            .and_then(Value::as_array)
            .and_then(|steps| {
                steps
                    .iter()
                    .find(|step| step.get("kind") == Some(&json!("page")))
            })
            .and_then(|step| step.get("args"))
            .and_then(Value::as_array)
            .ok_or("study native continuation")?;
        if !continuation
            .windows(2)
            .any(|pair| pair == [json!("--section-offset"), json!("1")])
        {
            return Err("study native offset changed".into());
        }
        let args = words
            .into_iter()
            .chain(["--section-offset", "1"])
            .map(str::to_owned)
            .collect();
        let second = invoke(&env, args, 0)?;
        save(directory, &format!("study-{section}-next-window"), &second)?;
        if second.pointer("/page/offset") != Some(&json!(1))
            || second.pointer("/page/returned") != Some(&json!(1))
            || second.pointer(&format!("/result/items/0/{identity}")) != Some(&json!("native_1"))
        {
            return Err("study native second row moved".into());
        }
        let requests = if section == "sample-lists" { 3 } else { 2 };
        if stub.finish()?.len() != requests {
            return Err(format!("{section}: study source request count changed"));
        }
    }
    Ok(())
}

/// Since 0724 each returned sample list reads its own count.
fn study_stub(body: Value) -> Result<Stub, String> {
    let count = json!({"sampleListId":"native_1", "studyId":"synthetic_0", "sampleCount":1});
    Stub::for_source("www.cbioportal.org", "application/json", move |request| {
        Ok(if request.contains("/sample-lists/native_1") {
            count.to_string()
        } else {
            body.to_string()
        })
    })
}

pub(super) fn chembl(directory: &Path, registry: &Registry) -> Check {
    let case = super::native_case("015.json")?;
    let fixture = launch::fixture_text("fixtures/drug/native/015.json")?;
    let original: Value = serde_json::from_str(&fixture).map_err(|error| error.to_string())?;
    for refused in [false, true] {
        let mut body = original.clone();
        *body
            .pointer_mut("/page_meta/total_count")
            .ok_or("native mechanism total")? = json!(2);
        if refused {
            *body
                .pointer_mut("/mechanisms/0/mec_id")
                .ok_or("native mechanism identity")? = json!({"bad_shape": true});
        }
        let mutation = super::Mutation {
            file: "015.json",
            pointer: "",
            field: "mechanism-native-window",
            absent: "",
            present: "",
        };
        let value = super::acquire(&case, Some((&mutation, body)), registry)?;
        save(
            directory,
            if refused {
                "chembl-refused-window"
            } else {
                "chembl-window-positive"
            },
            &value,
        )?;
        let returned = u32::from(!refused);
        if value.pointer("/page/total") != Some(&json!(2))
            || value.pointer("/page/returned") != Some(&json!(returned))
            || value.pointer("/sources/0/retrieved_count") != Some(&json!(returned))
        {
            return Err("ChEMBL native total or accepted identity count changed".into());
        }
        if !value
            .get("next")
            .and_then(Value::as_array)
            .is_some_and(|steps| {
                steps.iter().any(|step| {
                    step.get("kind") == Some(&json!("page"))
                        && step
                            .get("args")
                            .and_then(Value::as_array)
                            .is_some_and(|args| {
                                args.windows(2)
                                    .any(|pair| pair == [json!("--section-offset"), json!("1")])
                            })
                })
            })
        {
            return Err("ChEMBL refused identity removed native continuation".into());
        }
        if refused
            && !value
                .get("problems")
                .and_then(Value::as_array)
                .is_some_and(|problems| {
                    problems
                        .iter()
                        .any(|problem| problem.get("field") == Some(&json!("mec_id")))
                })
        {
            return Err("ChEMBL refused identity lost its Problem".into());
        }
    }
    Ok(())
}
