//! Optional FAERS values retain identity and independent native sections.
use super::{Check, Environment, args, both, setup};
use serde_json::{Value, json};

pub(super) fn input(body: &mut Value, mode: &str) -> Check {
    if mode == "empty-collections" {
        for field in ["drug", "reaction"] {
            *body
                .pointer_mut(&format!("/results/0/patient/{field}"))
                .ok_or(field)? = json!([]);
        }
        return Ok(());
    }
    let Some((kind, field)) = mode.split_once('-') else {
        return Ok(());
    };
    if !matches!(kind, "missing" | "null" | "invalid") {
        return Ok(());
    }
    let (path, key) = match field {
        "patient" => ("/results/0", "patient"),
        "drugs" => ("/results/0/patient", "drug"),
        "reactions" => ("/results/0/patient", "reaction"),
        _ => return Err(format!("unknown optional field: {mode}")),
    };
    let object = body
        .pointer_mut(path)
        .and_then(Value::as_object_mut)
        .ok_or(path)?;
    if kind == "missing" {
        object.remove(key);
    } else {
        object.insert(
            key.into(),
            if kind == "null" {
                Value::Null
            } else {
                json!(false)
            },
        );
    }
    Ok(())
}
pub(super) fn run(mode: &str) -> Check {
    let (env, stub) = setup(mode)?;
    let value = both(&env, args(None), 0)?;
    let patient = !mode.ends_with("-patient");
    let drugs = if !patient || mode.ends_with("-drugs") || mode == "empty-collections" {
        0
    } else {
        3
    };
    let reactions = if !patient || mode.ends_with("-reactions") || mode == "empty-collections" {
        0
    } else {
        1
    };
    if value.get("status") != Some(&json!("complete"))
        || value.pointer("/sources/0/state") != Some(&json!("ok"))
        || value.pointer("/sources/0/source") != Some(&json!("faers"))
        || value.pointer("/sources/0/reason").is_some()
        || value.pointer("/result/header/id") != Some(&json!("100"))
        || value.pointer("/result/header/version") != Some(&json!("1"))
        || value.pointer("/result/reporter/country") != Some(&json!("US"))
        || value.pointer("/result/patient").is_some() != patient
        || value.pointer("/result/patient/age")
            != patient.then(|| json!({"value":42.5,"unit":"801"})).as_ref()
        || value.pointer("/result/drug_count") != Some(&json!(drugs))
        || value.pointer("/result/reaction_count") != Some(&json!(reactions))
    {
        return Err(format!("optional value lost report facts: {mode}: {value}"));
    }
    let problems = if mode.starts_with("invalid-") {
        let field = if !patient {
            "patient"
        } else if drugs == 0 {
            "patient.drug"
        } else {
            "patient.reaction"
        };
        Some(json!([{"source":"faers","record":"100","field":field,"code":"value.unknown"}]))
    } else {
        None
    };
    if value.get("problems") != problems.as_ref() {
        return Err(format!("optional value problem changed: {mode}: {value}"));
    }
    optional_sections(&env, &value, drugs, reactions, problems.as_ref())?;
    if stub.finish()?.len() != 1 {
        return Err("optional report or snapshot bypassed cache".into());
    }
    Ok(())
}

fn optional_sections(
    env: &Environment,
    value: &Value,
    drugs: usize,
    reactions: usize,
    problems: Option<&Value>,
) -> Check {
    let snapshot = value
        .pointer("/result/snapshot")
        .and_then(Value::as_str)
        .ok_or("snapshot")?;
    for (section, total) in [("drugs", drugs), ("reactions", reactions)] {
        let mut words = args(Some(section));
        words.extend(["--snapshot".into(), snapshot.into()]);
        let page = both(env, words, 0)?;
        let native = match section {
            "drugs" => ("medicinalproduct", "Synthetic A"),
            _ => ("term/label", "Synthetic reaction"),
        };
        if page.get("status") != Some(&json!("complete"))
            || page.pointer("/sources/0/state") != Some(&json!("ok"))
            || page.pointer(&format!("/result/{section}/total")) != Some(&json!(total))
            || (total > 0
                && page.pointer(&format!("/result/{section}/records/0/{}", native.0))
                    != Some(&json!(native.1)))
            || page
                .pointer(&format!("/result/{section}/records"))
                .and_then(Value::as_array)
                .map_or(0, Vec::len)
                != total
            || page.get("problems") != problems
        {
            return Err(format!("optional section changed: {section}: {page}"));
        }
    }
    Ok(())
}
