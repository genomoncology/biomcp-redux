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
        .ok_or("retained annual body")?;
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
        let stub = Stub::for_source("seer.cancer.gov", "application/json", move |_| {
            Ok(body.to_string())
        })?;
        let env = variant::source_environment(&stub)?;
        let value = invoke(&env, words.into_iter().chain(["--fresh"]), 0)?;
        if stub.finish()?.len() != 1 {
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
    }
    Ok(())
}
