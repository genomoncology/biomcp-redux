//! Feature phenotype-sections: source-owned text and direct link disclosure.
use super::{invoke, setup, Stub};
use libtest_mimic::Trial;
use serde_json::{Value, json};
pub(super) fn trials() -> Vec<Trial> {
    vec![Trial::test("phenotype-sections/retained-local-pages-and-executable-card-actions", || pages().map_err(Into::into))]
}
fn pages() -> Result<(), String> {
    let stub = Stub::for_source("ontology.jax.org", "application/json", |request| {
        if !request.starts_with("GET /api/hp/terms/HP%3A0000001 ") { return Err(format!("wrong entity: {request}")); }
        Ok(json!({"id":"HP:0000001","name":"Synthetic term","synonyms":["First","Second","Third","Fourth","Fifth","Sixth"],"xrefs":[]}).to_string())
    })?;
    let env = setup(&stub)?;
    let card = invoke(&env, &["get", "phenotype", "HP:0000001"], 0)?;
    let words = step(&card, "section", 0)?;
    let words = words.iter().map(String::as_str).collect::<Vec<_>>();
    let page = invoke(&env, &words, 0)?;
    if page.pointer("/page/returned") != Some(&json!(5)) || page.pointer("/page/total") != Some(&json!(6)) || page.pointer("/result/items/0/value") != Some(&json!("First")) || page.pointer("/result/items/0/relationship").is_some() { return Err(format!("source rows or five-row default changed: {page}")); }
    let words = step(&page, "page", 0)?;
    let words = words.iter().map(String::as_str).collect::<Vec<_>>();
    let terminal = invoke(&env, &words, 0)?;
    if terminal.pointer("/page/offset") != Some(&json!(5)) || terminal.pointer("/result/items/0/value") != Some(&json!("Sixth")) || terminal.get("next").is_some() { return Err(format!("local terminal changed: {terminal}")); }
    if stub.finish()?.len() != 1 { return Err("section walk refetched retained entity".into()); }
    Ok(())
}
fn step(value: &Value, kind: &str, index: usize) -> Result<Vec<String>, String> {
    value.get("next").and_then(Value::as_array).ok_or("missing next")?.iter().filter(|row| row.get("kind").and_then(Value::as_str) == Some(kind)).nth(index).and_then(|row| row.get("args")).and_then(Value::as_array).ok_or_else(|| format!("missing {kind} step: {value}"))?.iter().map(|word| word.as_str().map(String::from).ok_or("step word".into())).collect()
}
