//! Feature source-controls: drug declarations and admission use the shared registry.
use super::{configuration::invoke, launch::Environment};
use libtest_mimic::Trial;
use serde_json::{Value, json};
type Check = Result<(), String>;
pub(super) fn trials() -> Vec<Trial> {
    vec![Trial::test("drug-registry/publisher-declarations", || declarations().map_err(Into::into))]
}
fn declarations() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let health = invoke(&env, vec!["health".into(), "--offline".into()], 0)?;
    let rows = health.pointer("/result/sources").and_then(Value::as_array).ok_or("missing health")?;
    for (name, usage, url) in [
        ("mychem", "attribution", "https://docs.mychem.info/en/latest/doc/data_source.html"),
        ("chembl", "attribution", "https://chembl.github.io/chembl-licensing/"),
        ("opentargets", "open", "https://platform-docs.opentargets.org/licence"),
        ("civic", "open", "https://civicdb.org/releases/licensing"),
        ("openfda-label", "open", "https://open.fda.gov/terms/"),
        ("drugsfda", "open", "https://open.fda.gov/terms/"),
        ("ema", "attribution", "https://www.ema.europa.eu/en/about-us/about-website/legal-notice"),
        ("ddinter", "noncommercial", "https://ddinter.scbdd.com/terms/"),
    ] {
        let row = rows.iter().find(|r| r["source"] == name).ok_or_else(|| format!("missing {name}"))?;
        if row["use"] != usage || row["terms_url"] != url || row["disabled"] != false {
            return Err(format!("wrong declaration: {row}"));
        }
        if !row["terms"].as_str().is_some_and(|s| !s.is_empty()) { return Err("missing terms".into()); }
    }
    for name in ["who", "fda-orphan", "dgidb", "pharmacodb", "cvx"] {
        if rows.iter().any(|r| r["source"] == json!(name)) { return Err(format!("dropped {name} declared")); }
    }
    Ok(())
}
