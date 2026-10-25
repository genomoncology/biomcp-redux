use super::{Check, Environment, Value, invoke, json};
use crate::runner::launch::fixtures;
pub(super) fn run() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let replay = fixtures().join("replies/diagnostic/pma-synthetic");
    let mut statements = String::new();
    let mut offset = 0u64;
    loop {
        let mut args = env.globals(Some(&replay), false);
        args.extend([
            "get".into(),
            "diagnostic".into(),
            "P150013".into(),
            "regulatory".into(),
            "--source".into(),
            "fda-pma".into(),
            "--section-offset".into(),
            offset.to_string(),
        ]);
        let value = invoke(&env, args, 0)?;
        let rows = value
            .pointer("/result/items")
            .and_then(Value::as_array)
            .ok_or("regulatory fragments")?;
        for row in rows {
            statements.push_str(
                row.get("text")
                    .and_then(Value::as_str)
                    .ok_or("fragment text")?,
            );
        }
        offset += rows.len() as u64;
        if offset
            >= value
                .pointer("/page/total")
                .and_then(Value::as_u64)
                .ok_or("total")?
        {
            break;
        }
        if rows.is_empty() {
            return Err("statement continuation did not advance".into());
        }
    }
    if statements != format!("Synthetic statement. {}", "x\"\\\n".repeat(2500)) {
        return Err("mechanical fragments did not reconstruct the whole native statement".into());
    }
    let mut identities = Vec::new();
    for offset in [0, 2, 4, 6] {
        let mut args = env.globals(Some(&replay), false);
        args.extend([
            "get".into(),
            "diagnostic".into(),
            "P150013".into(),
            "supplements".into(),
            "--source".into(),
            "fda-pma".into(),
            "--section-limit".into(),
            "2".into(),
            "--section-offset".into(),
            offset.to_string(),
        ]);
        let value = invoke(&env, args, 0)?;
        let rows = value
            .pointer("/result/items")
            .and_then(Value::as_array)
            .ok_or("supplement headers")?;
        for row in rows {
            identities.push(
                row.get("supplement_number")
                    .and_then(Value::as_str)
                    .ok_or("supplement identity")?
                    .to_owned(),
            );
        }
        let steps = value
            .get("next")
            .and_then(Value::as_array)
            .map(Vec::as_slice)
            .unwrap_or(&[]);
        let gets: Vec<_> = steps
            .iter()
            .filter(|s| s.get("kind") == Some(&json!("disambiguate")))
            .collect();
        if gets.len() != rows.len() {
            return Err(format!("not all selectable supplements retained: {value}"));
        }
    }
    if identities != ["S020", "S021", "S022", "S023", "S024", "S025", "S026"] {
        return Err(format!("supplement walk: {identities:?}"));
    }
    Ok(())
}
