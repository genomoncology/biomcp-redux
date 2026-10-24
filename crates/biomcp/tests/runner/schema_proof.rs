use libtest_mimic::Trial;
use serde_json::{Value, json};

pub(super) fn trials() -> Vec<Trial> {
    vec![Trial::test("runner/validates-results-before-assertions", || {
        results().map_err(Into::into)
    })]
}

fn results() -> Result<(), String> {
    let good = json!({"schema":"biomcp.error/1", "status":"usage",
        "error":{"code":"usage.schema","message":"expected schema <known artifact id>"}});
    let args = vec!["unknown".into()];
    super::schema::validate(&good, &args, false)?;
    let mut rows = vec![json!({}), json!({"schema":42}),
        json!({"schema":"missing/1"}), json!({"$id":"invented/1"})];
    let mut bad_code = good.clone();
    *bad_code.pointer_mut("/error/code").ok_or("missing code")? = json!("bad");
    rows.push(bad_code);
    let mut missing = good;
    missing.as_object_mut().ok_or("not object")?.remove("status");
    rows.push(missing);
    for value in rows {
        rejects(&value, &args, false)?;
    }
    Ok(())
}

fn rejects(value: &Value, args: &[String], success: bool) -> Result<(), String> {
    match super::schema::validate(value, args, success) {
        Err(error) if error.starts_with("schema validation:") => Ok(()),
        result => Err(format!("expected schema validation failure; actual {result:?}")),
    }
}
