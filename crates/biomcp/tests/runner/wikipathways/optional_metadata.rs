use super::super::{configuration::invoke, schema::Registry};
use super::{Check, Environment, Value, catalog, fs, json, seed};

pub(super) fn check() -> Check {
    let root = super::super::launch::fixtures().join("adapter-values-native");
    let (search, mut get, id) = if root.is_dir() {
        (
            serde_json::from_str::<Value>(
                &fs::read_to_string(root.join("wiki-search.json")).map_err(|e| e.to_string())?,
            )
            .map_err(|e| e.to_string())?,
            serde_json::from_str::<Value>(
                &fs::read_to_string(root.join("wiki-get.json")).map_err(|e| e.to_string())?,
            )
            .map_err(|e| e.to_string())?,
            "WP382",
        )
    } else {
        (catalog(true, false), catalog(false, false), "WP1")
    };
    let expected_cited_in = get["pathwayInfo"]
        .as_array()
        .and_then(|rows| rows.iter().find(|r| r.get("id") == Some(&json!(id))))
        .and_then(|row| row.get("citedIn"))
        .cloned();
    let registry = Registry::embedded()?;
    let positive = Environment::new().map_err(|e| e.to_string())?;
    let replay = seed(&positive, search.clone(), get.clone(), 200)?;
    let mut globals = positive.globals(Some(&replay), false);
    globals.extend(["sync".into(), "wikipathways".into()]);
    invoke(&positive, globals, 0).map_err(|e| format!("positive native catalog sync: {e}"))?;
    let mut args = positive.globals(Some(&replay), false);
    args.extend(["get".into(), "pathway".into(), id.into()]);
    let control = invoke(&positive, args, 0)?;
    registry.validate(&control, &["get".into(), "pathway".into(), id.into()], true)?;
    if control.pointer("/result/pathway/id/id") != Some(&json!(id)) {
        return Err("positive WikiPathways identity missing".into());
    }
    if control.pointer("/result/pathway/cited_in") != expected_cited_in.as_ref() {
        return Err("valid native citation metadata changed".into());
    }
    let rows = get
        .get_mut("pathwayInfo")
        .and_then(Value::as_array_mut)
        .ok_or("native catalog rows")?;
    let row = rows
        .iter_mut()
        .find(|r| r.get("id") == Some(&json!(id)))
        .ok_or("native pathway row")?;
    row["name"] = json!({"bad":true});
    let changed = Environment::new().map_err(|e| e.to_string())?;
    let replay = seed(&changed, search, get, 200)?;
    let mut args = changed.globals(Some(&replay), false);
    args.extend(["sync".into(), "wikipathways".into()]);
    invoke(&changed, args, 0).map_err(|e| format!("mutated name must retain catalog: {e}"))?;
    let mut args = changed.globals(Some(&replay), false);
    args.extend(["get".into(), "pathway".into(), id.into()]);
    let answer = invoke(&changed, args, 0)?;
    registry.validate(&answer, &["get".into(), "pathway".into(), id.into()], true)?;
    if answer.pointer("/result/pathway/id/id") != Some(&json!(id))
        || answer.pointer("/result/pathway/name").is_some()
        || answer
            .pointer("/problems")
            .and_then(Value::as_array)
            .map(Vec::len)
            != Some(1)
        || answer.pointer("/problems/0/field") != Some(&json!("name"))
    {
        return Err("bad native name lost WikiPathways identity or field disclosure".into());
    }
    Ok(())
}
