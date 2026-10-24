use libtest_mimic::Trial;
use serde_json::{Value, json};

pub(crate) fn trials() -> Vec<Trial> {
    vec![
        Trial::test("runner/validates-results-before-assertions", || {
            results().map_err(Into::into)
        }),
        Trial::test("runner/validates-raw-artifact-request", || {
            raw_artifacts().map_err(Into::into)
        }),
        Trial::test("runner/resolves-only-registered-schema-references", || {
            references().map_err(Into::into)
        }),
    ]
}

fn results() -> Result<(), String> {
    let good = json!({"schema":"biomcp.error/1", "status":"usage",
        "error":{"code":"usage.schema","message":"expected schema <known artifact id>"}});
    let args = vec!["unknown".into()];
    super::schema::validate(&good, &args, false)?;
    let mut rows = vec![json!({}), json!({"schema":42}),
        json!({"schema":"missing/1"}), json!({"$id":"invented/1"})];
    let mut bad_code = good.clone();
    *bad_code.pointer_mut("/error/code").ok_or("missing code")? = json!("bad");
    rows.push(bad_code);
    let mut missing = good;
    missing.as_object_mut().ok_or("not object")?.remove("status");
    rows.push(missing);
    for value in rows {
        rejects(&value, &args, false)?;
    }
    let long_id = json!({"schema": "🚫".repeat(10_000)});
    let diagnostic = super::schema::validate(&long_id, &args, false).err()
        .ok_or("long unknown identity accepted")?;
    if diagnostic.chars().count() > 240 {
        return Err("unbounded schema identity diagnostic".into());
    }
    Ok(())
}

fn raw_artifacts() -> Result<(), String> {
    let artifact = |id: &str| -> Result<Value, String> {
        let (_, text) = super::schema::ARTIFACTS.iter().find(|(name, _)| *name == id)
            .ok_or("missing artifact")?;
        serde_json::from_str(text).map_err(|error| error.to_string())
    };
    let source = artifact("biodata.source/1")?;
    let args = vec!["schema".into(), "biodata.source/1".into()];
    super::schema::validate(&source, &args, true)?;
    rejects(&source, &args, false)?;
    rejects(&source, &["unknown".into()], true)?;
    rejects(&source, &["schema".into(), "missing/1".into()], true)?;
    rejects(&artifact("biomcp.error/1")?, &args, true)?;
    let mut changed = source.clone();
    changed.as_object_mut().ok_or("not object")?.insert("description".into(), json!("changed"));
    rejects(&changed, &args, true)?;
    let mut wrong_id = source;
    wrong_id.as_object_mut().ok_or("not object")?.insert("$id".into(), json!("invented/1"));
    rejects(&wrong_id, &args, true)
}

fn references() -> Result<(), String> {
    let parent = json!({"$id":"fixture.parent/1", "type":"object", "required":["schema", "child"],
        "properties":{"schema":{"const":"fixture.parent/1"},
            "child":{"$ref":"../fixture.child/1#/$defs/Count"}}});
    let child = json!({"$id":"fixture.child/1", "$defs":{"Count":{"type":"integer","minimum":3}}});
    let registry = super::schema::Registry::new([
        ("fixture.parent/1".into(), parent.clone()),
        ("fixture.child/1".into(), child),
    ].into_iter().collect())?;
    registry.validate(&json!({"schema":"fixture.parent/1", "child":3}), &[], true)?;
    if registry.validate(&json!({"schema":"fixture.parent/1", "child":2}), &[], true).is_ok() {
        return Err("cross-artifact constraint ignored".into());
    }
    for reference in ["https://unregistered.invalid/schema", "file:///unregistered/schema.json"] {
        let mut external = parent.clone();
        *external.pointer_mut("/properties/child/$ref").ok_or("missing ref")? = json!(reference);
        if super::schema::Registry::new([("fixture.parent/1".into(), external)].into_iter().collect()).is_ok() {
            return Err("unregistered reference accepted".into());
        }
    }
    Ok(())
}

fn rejects(value: &Value, args: &[String], success: bool) -> Result<(), String> {
    match super::schema::validate(value, args, success) {
        Err(error) if error.starts_with("schema validation:") => Ok(()),
        result => Err(format!("expected schema validation failure; actual {result:?}")),
    }
}
