//! Exact threshold behavior through the command and its emitted continuation.
use super::*;
pub(super) fn walk() -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let first_url = url(&pairs());
    let replay = seed(&env, &first_url, body(&first_url, 0, 6), 200, 0)?;
    let first = call(&env, &replay, &["search", "gwas", "--pubmed", "17529967", "--p-value", "4e-254"], 0)?;
    if first.pointer("/result/items").and_then(Value::as_array).map(Vec::len) != Some(3)
        || first.pointer("/result/qualifying_total") != Some(&Value::Null)
        || first.pointer("/next/0/kind") != Some(&json!("page"))
    {
        return Err(format!("inclusive first page must keep unknown qualifying total: {first}"));
    }
    Ok(())
}
