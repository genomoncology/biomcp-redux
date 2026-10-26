//! Source failures cannot become a successful empty text collection.
use super::{invoke, native, record, setup, Stub};
use serde_json::{Value, json};
use std::fs;
pub(super) fn run() -> Result<(), String> {
    for kind in 0..5 {
        let stub = Stub::for_source_status("ontology.jax.org", "application/json", move |_| {
            Ok(match kind {
                2 => "{broken".into(),
                3 => json!({"id":"HP:0000002","name":"Other","synonyms":[]}).to_string(),
                _ => json!({"id":"HP:0000001","name":"Term","synonyms":[]}).to_string(),
            })
        }, move |_| match kind { 0 => 429, 1 => 500, _ => 200 })?;
        let env = setup(&stub)?;
        let words = ["get","phenotype","HP:0000001","synonyms"];
        let directory = env.root().join("section-failure");
        record(&env, &words, &directory, if kind == 4 { 1 } else { 4 })?;
        stub.finish()?;
        if kind == 4 { delay(&directory)?; }
        let mut args = words.to_vec(); args.extend(["--replay",directory.to_str().ok_or("replay path")?]);
        let result = invoke(&env,&args,4)?;
        if result.get("status") != Some(&json!("failed")) || result.get("result").is_some() || result.pointer("/sources/0/terms").is_none() || result.pointer("/sources/0/source") != Some(&json!("hpo")) { return Err(format!("section failure lost source or became absence: {result}")); }
        native::mcp(&directory,&words,&result,4,None)?;
    }
    Ok(())
}
fn delay(directory: &std::path::Path) -> Result<(), String> {
    for entry in fs::read_dir(directory.join("hpo")).map_err(|e|e.to_string())? {
        let entry=entry.map_err(|e|e.to_string())?;
        let mut row:Value=serde_json::from_slice(&fs::read(entry.path()).map_err(|e|e.to_string())?).map_err(|e|e.to_string())?;
        *row.pointer_mut("/exchanges/0/delay_ms").ok_or("delay")?=json!(20000);
        fs::write(entry.path(),serde_json::to_vec(&row).map_err(|e|e.to_string())?).map_err(|e|e.to_string())?;
    }
    Ok(())
}
