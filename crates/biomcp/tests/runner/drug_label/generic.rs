//! Feature drug-label: complete generic discovery preserves the native brand.
use super::{Check, both, field, identity, page, record, setup};
use crate::runner::transcript::trial as transcript_trial;
use libtest_mimic::Trial;
use serde_json::{Value, json};

fn search(typed: &str, basis: &str) -> Check {
    // Synthetic identity and prose; names reproduce the reported public label fields.
    let mut id = identity(0)?;
    *id.get_mut("brand").ok_or("brand")? = json!("RASONQUE");
    let mut row = record(&id);
    row.get_mut("openfda")
        .and_then(Value::as_object_mut)
        .ok_or("openfda names")?
        .insert("generic_name".into(), json!(["DARAXONRASIB"]));
    let selector = format!(
        "openfda.brand_name.exact%3A%22{typed}%22%20OR%20openfda.generic_name.exact%3A%22{typed}%22"
    );
    let (env, stub) = setup(page(row), selector, 200)?;
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
    let value = both(&env, args, 0)?;
    if value.pointer("/result/items/0/name") != Some(&json!("RASONQUE"))
        || value.pointer("/result/items/0/id") != Some(field(&id, "set_id"))
        || value.pointer("/result/items/0/matched_name") != Some(&json!(basis))
    {
        return Err("complete native name or match basis lost".into());
    }
    if stub.finish()?.len() != 1 {
        return Err("exact name made an extra request".into());
    }
    Ok(())
}
