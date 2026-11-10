//! An exact threshold cannot count associations it could not compare.
use super::{Environment, Value, body, call, json, pairs, seed, url};

pub(super) fn run() -> Result<(), String> {
    for mode in ["missing", "invalid", "refused", "all-missing"] {
        let env = Environment::new().map_err(|error| error.to_string())?;
        let args = ["search", "gwas", "--pubmed", "17529967", "--p-value", "1e-200"];
        let mut page = body(&url(&pairs()), 0, 3);
        let rows = page.pointer_mut("/_embedded/associations").and_then(Value::as_array_mut)
            .ok_or("association fixture")?;
        for (index, row) in rows.iter_mut().enumerate() {
            if mode == "all-missing" || index == 1 {
                let row = row.as_object_mut().ok_or("association object")?;
                match mode {
                    "refused" => { row.insert("association_id".into(), json!("invalid")); }
                    "invalid" => { row.insert("pvalue_mantissa".into(), json!(0)); }
                    _ => { row.remove("pvalue_mantissa"); row.remove("pvalue_exponent"); }
                }
            }
        }
        let replay = seed(&env, &args, page, 200, 0)?;
        let value = call("gwas_search/uncertainty", &env, &replay, &args, 0)?;
        check(&value, mode)?;
        let pool = value.pointer("/page/pool/id").and_then(Value::as_str).ok_or("retained pool")?;
        let mut explicit = args.to_vec();
        explicit.extend(["--pool", pool]);
        check(&call("gwas_search/uncertainty/reload", &env, &replay, &explicit, 0)?, mode)?;
    }
    Ok(())
}

fn check(value: &Value, mode: &str) -> Result<(), String> {
    let count = value.pointer("/result/items").and_then(Value::as_array).map_or(0, Vec::len);
    if value.pointer("/status") != Some(&json!("partial"))
        || value.pointer("/result/qualifying_total").is_some()
        || count != if mode == "all-missing" { 0 } else { 2 }
        || value.pointer("/problems").and_then(Value::as_array).is_none_or(Vec::is_empty)
    {
        return Err(format!("{mode}: unknown membership claimed exact: {value}"));
    }
    Ok(())
}
