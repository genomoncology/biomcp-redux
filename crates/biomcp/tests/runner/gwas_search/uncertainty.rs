//! An exact threshold cannot count associations it could not compare.
use super::{Environment, Value, body, call, json, pairs, seed, url};

pub(super) fn run() -> Result<(), String> {
    for mode in ["missing", "invalid", "refused", "all-missing"] {
        let env = Environment::new().map_err(|error| error.to_string())?;
        let args = [
            "search",
            "gwas",
            "--pubmed",
            "17529967",
            "--p-value",
            "1e-200",
        ];
        let mut page = body(&url(&pairs()), 0, 3);
        let rows = page
            .pointer_mut("/_embedded/associations")
            .and_then(Value::as_array_mut)
            .ok_or("association fixture")?;
        for (index, row) in rows.iter_mut().enumerate() {
            if mode == "all-missing" || index == 1 {
                let row = row.as_object_mut().ok_or("association object")?;
                match mode {
                    "refused" => {
                        row.insert("association_id".into(), json!("invalid"));
                    }
                    "invalid" => {
                        row.insert("pvalue_mantissa".into(), json!(0));
                    }
                    _ => {
                        row.remove("pvalue_mantissa");
                        row.remove("pvalue_exponent");
                    }
                }
            }
        }
        let replay = seed(&env, &args, page, 200, 0)?;
        let value = call("gwas_search/uncertainty", &env, &replay, &args, 0)?;
        check(&value, mode, if mode == "all-missing" { 0 } else { 2 })?;
        let pool = value
            .pointer("/page/pool/id")
            .and_then(Value::as_str)
            .ok_or("retained pool")?;
        if mode == "missing" {
            let db = rusqlite::Connection::open(env.root().join("case-cache/replay.sqlite"))
                .map_err(|error| error.to_string())?;
            db.execute("UPDATE trial_snapshot SET payload=json_set(payload,'$.dropped',json('false')) WHERE id=?1", [pool])
                .map_err(|error| error.to_string())?;
        }
        let mut explicit = args.to_vec();
        explicit.extend(["--pool", pool]);
        check(
            &call(
                "gwas_search/uncertainty/reload",
                &env,
                &replay,
                &explicit,
                0,
            )?,
            mode,
            if mode == "all-missing" { 0 } else { 2 },
        )?;
    }
    later_page()
}

fn later_page() -> Result<(), String> {
    let stub = crate::runner::network_stub::Stub::for_source(
        "www.ebi.ac.uk",
        "application/json",
        |request| {
            let mut value: Value = serde_json::from_str(&super::wire::live_body(request)?)
                .map_err(|error| error.to_string())?;
            if value.pointer("/page/number") == Some(&json!(0)) {
                let row = value
                    .pointer_mut("/_embedded/associations/1")
                    .and_then(Value::as_object_mut)
                    .ok_or("first page row")?;
                row.remove("pvalue_mantissa");
                row.remove("pvalue_exponent");
            }
            Ok(value.to_string())
        },
    )?;
    let env = crate::runner::variant::source_environment(&stub)?;
    let first = super::invoke(
        &env,
        [
            "search",
            "gwas",
            "--pubmed",
            "17529967",
            "--p-value",
            "1e-200",
        ],
        0,
    )?;
    check(&first, "first page", 2)?;
    let next = crate::runner::action(&first, "page")?;
    let last = super::invoke(&env, next, 0)?;
    check(&last, "clean terminal page", 3)?;
    if last.pointer("/page/truncated") != Some(&json!(false)) || stub.finish()?.len() != 2 {
        return Err(format!("uncertain walk did not terminate: {last}"));
    }
    Ok(())
}

fn check(value: &Value, mode: &str, expected: usize) -> Result<(), String> {
    let count = value
        .pointer("/result/items")
        .and_then(Value::as_array)
        .map_or(0, Vec::len);
    if value.pointer("/status") != Some(&json!("partial"))
        || value.pointer("/result/qualifying_total").is_some()
        || count != expected
        || value
            .pointer("/problems")
            .and_then(Value::as_array)
            .is_none_or(Vec::is_empty)
    {
        return Err(format!("{mode}: unknown membership claimed exact: {value}"));
    }
    Ok(())
}
