//! Feature source-controls: disease services and carried components share publisher policy.
use super::{configuration::invoke, launch::Environment};
use libtest_mimic::Trial;
use serde_json::Value;
type Check = Result<(), String>;
pub(super) fn trials() -> Vec<Trial> {
    vec![Trial::test("disease-registry/publisher-declarations", || declarations().map_err(Into::into))]
}
fn declarations() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let health = invoke(&env, vec!["health".into(), "--offline".into()], 0)?;
    let rows = health.pointer("/result/sources").and_then(Value::as_array).ok_or("missing health")?;
    for (name, usage, url) in [
        ("ols", "attribution", "https://www.ebi.ac.uk/about/terms-of-use/"),
        ("mydisease", "attribution", "https://docs.mydisease.info/en/latest/doc/data.html"),
        ("monarch", "attribution", "https://monarch-app.monarchinitiative.org/Licensing/"),
        ("seer", "open", "https://seer.cancer.gov/about/using-website.html"),
    ] {
        let row = rows.iter().find(|r| r["source"] == name).ok_or_else(|| format!("missing {name}"))?;
        if row["usage"] != usage || row["terms_url"] != url || row["disabled"] != false || row["terms"].as_str().is_none_or(str::is_empty) {
            return Err(format!("wrong declaration: {row}"));
        }
    }
    for name in ["hpo", "ontotree", "medlineplus", "umls", "disgenet", "reporter"] {
        if rows.iter().any(|r| r["source"] == name) { return Err(format!("dropped route declared: {name}")); }
    }
    Ok(())
}
