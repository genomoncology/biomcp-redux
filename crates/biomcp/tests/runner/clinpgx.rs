//! Feature clinpgx-annotations: independent retained collections through CLI and MCP.
mod rows;
use super::{configuration::invoke, launch::Environment, network_stub::Stub};
use libtest_mimic::Trial;
use serde_json::{Value, json};
pub(super) fn trials() -> Vec<Trial> {
    vec![Trial::test(
        "clinpgx-annotations/guideline-local-page",
        || scenario().map_err(Into::into),
    )]
}
fn scenario() -> Result<(), String> {
    let body = json!({"data": rows::references(), "status":"success"});
    let expected = body.pointer("/data/0").ok_or("synthetic row")?.clone();
    let stub = Stub::for_sources(
        &["api.clinpgx.org"],
        |_| "application/json",
        move |request| {
            if !request.starts_with("GET /v1/data/guidelineAnnotation?")
                || !request.contains("relatedGenes.symbol=DPYD")
                || !request.contains("view=min")
            {
                return Err(format!("wrong native route: {request}"));
            }
            Ok(body.to_string())
        },
    )?;
    let env = Environment::new().map_err(|e| e.to_string())?;
    let ca = env.root().join("ca.pem");
    std::fs::write(&ca, &stub.certificate).map_err(|e| e.to_string())?;
    env.config(&json!({"ca_bundle":ca,"proxy":format!("http://{}",stub.address)}))
        .map_err(|e| e.to_string())?;
    let args = [
        "get",
        "pgx",
        "--gene",
        "DPYD",
        "annotations",
        "--source",
        "clinpgx",
        "--kind",
        "guideline",
        "--section-limit",
        "1",
    ]
    .map(str::to_owned)
    .to_vec();
    let first = invoke(&env, args.clone(), 0)?;
    if first.pointer("/result/items/0") != Some(&expected)
        || first.pointer("/result/page/total") != Some(&json!(2))
        || first.pointer("/result/page/total_kind") != Some(&json!("observed"))
    {
        return Err(format!("collection or observed scope changed: {first}"));
    }
    let mcp = super::mcp::call_with_globals(&env, &args, false, &[])?;
    if mcp != first {
        return Err("CLI/MCP differs".into());
    }
    let next = first
        .get("next")
        .and_then(Value::as_array)
        .and_then(|v| v.iter().find(|v| v.get("kind") == Some(&json!("page"))))
        .and_then(|v| v.get("args"))
        .and_then(Value::as_array)
        .ok_or("page step")?
        .iter()
        .map(|v| {
            v.as_str()
                .map(str::to_owned)
                .ok_or("page argument".to_owned())
        })
        .collect::<Result<Vec<_>, _>>()?;
    let second = invoke(&env, next, 0)?;
    if second.pointer("/result/items/0/id") != Some(&json!("PA100002"))
        || second.pointer("/result/page/offset") != Some(&json!(1))
    {
        return Err("local page skipped row".into());
    }
    if stub.finish()?.len() != 1 {
        return Err("local page or warm transport opened provider request".into());
    }
    Ok(())
}
