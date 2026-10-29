//! A provider route failure is distinct from its exact structured no-entry reply.
use super::{Check, Environment, call, failed, fixtures, uncached};
use serde_json::json;
pub(super) fn run() -> Check {
    let template = Environment::new().map_err(|e| e.to_string())?;
    let root = fixtures::seed(&template)?;
    let empty = json!(
        {"code":404,"reason":"NOT_FOUND","url":"http://reactome.org/ContentService/search/query",
        "messages":["No entries found for query: Synthetic"],"targets":null}
    );
    let mut foreign = empty.clone();
    *foreign.get_mut("messages").ok_or("messages")? = json!(["No entries found for query: Other"]);
    for body in [json!({"message":"Not found"}), foreign] {
        fixtures::replace(&root, "/search/query", &body.to_string(), 404, 0)?;
        let env = Environment::new().map_err(|e| e.to_string())?;
        let value = call(
            "pathway_failures/absence/run/1",
            &env,
            &root,
            &["search", "pathway", "Synthetic"],
            4,
        )?;
        failed(&value, "bad-reply")?;
        uncached(&env, "/search/query")?;
    }
    fixtures::replace(&root, "/search/query", &empty.to_string(), 404, 0)?;
    let env = Environment::new().map_err(|e| e.to_string())?;
    let value = call(
        "pathway_failures/absence/run/2",
        &env,
        &root,
        &["search", "pathway", "Synthetic"],
        1,
    )?;
    if value.get("status") != Some(&json!("absent"))
        || value.pointer("/result/page/total") != Some(&json!(0))
    {
        return Err(format!(
            "exact structured source absence did not remain empty: {value}"
        ));
    }
    Ok(())
}
