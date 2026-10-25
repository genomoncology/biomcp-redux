//! Feature source-controls: adverse event publisher declarations precede dispatch.
use super::{configuration::invoke, launch::Environment};
use libtest_mimic::Trial;
use serde_json::Value;
type Check = Result<(), String>;
pub(super) fn trials() -> Vec<Trial> {
    ["faers", "maude"].into_iter().map(|source| {
        Trial::test(format!("adverse-event-registry/{source}-declaration"), move || {
            declaration(source).map_err(Into::into)
        })
    }).collect()
}
fn declaration(source: &str) -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let health = invoke(&env, vec!["health".into(), "--offline".into()], 0)?;
    let row = health.pointer("/result/sources").and_then(Value::as_array)
        .and_then(|rows| rows.iter().find(|row| row["source"] == source))
        .ok_or_else(|| format!("missing {source}"))?;
    if row["usage"] != "open" || row["disabled"] != false || row["terms_url"] != "https://open.fda.gov/terms/" {
        return Err(format!("wrong publisher: {row}"));
    }
    let (name, url) = if source == "faers" {
        ("meddra", "https://admin.meddra.org/sites/default/files/page/documents/000198_statement_on_meddra_data_sharing_1_1.pdf")
    } else { ("gmdn", "https://open.fda.gov/license/") };
    let component = row["components"].as_array().and_then(|rows| rows.iter().find(|row| row["name"] == name)).ok_or("missing component")?;
    if component["disabled"] != false || component["terms"]["usage"] != "restricted" || component["terms"]["url"] != url {
        return Err(format!("wrong component: {component}"));
    }
    Ok(())
}
