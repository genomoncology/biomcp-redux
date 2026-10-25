//! Feature drug-mychem: native replies captured 2026-10-25, delivered by a local TLS peer.
//! Imatinib card fixture is a labeled identity subset. Full native replies stay in source evidence.
use super::{configuration::invoke, launch::Environment, network_stub::Stub};
use libtest_mimic::Trial;
use serde_json::{Value, json};
type Check = Result<(), String>;
pub(super) fn trials() -> Vec<Trial> {
    let mut trials: Vec<_> = [
        "imatinib-default",
        "trastuzumab-default",
        "deruxtecan-default",
        "axi-cel-default",
        "imatinib-first-page",
        "imatinib-second-page",
        "imatinib-exact-card",
        "axi-cel-exact-card",
        "quoted-empty",
    ]
    .into_iter()
    .enumerate()
    .map(|(n, name)| {
        Trial::test(format!("drug-mychem/{name}"), move || {
            check(n).map_err(Into::into)
        })
    })
    .collect();
    trials.push(Trial::test(
        "drug-mychem/reject-wrong-identity-before-cache",
        || wrong_identity().map_err(Into::into),
    ));
    trials.push(Trial::test(
        "drug-mychem/component-policy-before-request-and-projection",
        || component_policy().map_err(Into::into),
    ));
    trials
}
const REPLIES: [&str; 9] = [
    include_str!("../../../../tests/fixtures/drug/mychem/001.json"),
    include_str!("../../../../tests/fixtures/drug/mychem/002.json"),
    include_str!("../../../../tests/fixtures/drug/mychem/003.json"),
    include_str!("../../../../tests/fixtures/drug/mychem/004.json"),
    include_str!("../../../../tests/fixtures/drug/mychem/005.json"),
    include_str!("../../../../tests/fixtures/drug/mychem/006.json"),
    include_str!("../../../../tests/fixtures/drug/mychem/007.json"),
    include_str!("../../../../tests/fixtures/drug/mychem/008.json"),
    include_str!("../../../../tests/fixtures/drug/mychem/009.json"),
];
fn check(n: usize) -> Check {
    let plan: Vec<Value> = serde_json::from_str(include_str!(
        "../../../../tests/fixtures/drug/mychem/plan.json"
    ))
    .map_err(|e| e.to_string())?;
    let selected = plan.get(n).ok_or("plan index")?.clone();
    let expected = selected["request"]
        .as_str()
        .ok_or("request")?
        .replace('+', "%20");
    let body = REPLIES.get(n).ok_or("fixture index")?.to_string();
    let stub = Stub::for_source_status(
        "mychem.info",
        "application/json",
        move |request| {
            if request.lines().next() != Some(expected.as_str()) {
                return Err(format!("unexpected request: {request}"));
            }
            Ok(body.clone())
        },
        |_| 200,
    )?;
    let env = Environment::new().map_err(|e| e.to_string())?;
    let cert = env.root().join("ca.pem");
    std::fs::write(&cert, &stub.certificate).map_err(|e| e.to_string())?;
    env.config(&json!({"ca_bundle":cert,"proxy":format!("http://{}",stub.address)}))
        .map_err(|e| e.to_string())?;
    let args = arguments(n)?;
    let exit = if n == 8 { 1 } else { 0 };
    let value = invoke(&env, args.clone(), exit)?;
    super::schema::Registry::embedded()?.validate(&value, &args, true)?;
    let mcp = super::mcp::call_with_globals(&env, &args, false, &[])?;
    if value != mcp {
        return Err("CLI and MCP differ".into());
    }
    verify(n, &value, &selected)?;
    let requests = stub.finish()?;
    if requests.len() != 1 {
        return Err(format!(
            "expected one request and one warm cache hit: {}",
            requests.len()
        ));
    }
    let bytes = serde_json::to_vec(&value).map_err(|e| e.to_string())?.len();
    let comparable = super::source_terms::size(&value, "")?;
    println!(
        "drug native case {n}: full {bytes} bytes/{} tokens; comparable {comparable} bytes/{} tokens",
        bytes.div_ceil(4),
        comparable.div_ceil(4)
    );
    Ok(())
}
fn arguments(n: usize) -> Result<Vec<String>, String> {
    let args = match n {
        0 => vec!["search", "drug", "imatinib"],
        1 => vec!["search", "drug", "trastuzumab"],
        2 => vec!["search", "drug", "fam-trastuzumab deruxtecan"],
        3 => vec!["search", "drug", "axicabtagene ciloleucel"],
        4 => vec!["search", "drug", "imatinib", "--limit", "2"],
        5 => vec![
            "search", "drug", "imatinib", "--limit", "2", "--offset", "2",
        ],
        6 => vec![
            "get",
            "drug",
            "KTUFNOKKBVMGRW-UHFFFAOYSA-N",
            "--source",
            "mychem",
        ],
        7 => vec!["get", "drug", "CHEMBL3989989", "--source", "mychem"],
        8 => vec!["search", "drug", "\"biomcp-nonexistent-drug-0116-control\""],
        _ => return Err("case index".into()),
    };
    Ok(args.into_iter().map(str::to_owned).collect())
}
fn verify(n: usize, value: &Value, selected: &Value) -> Check {
    if n == 6 || n == 7 {
        if value.pointer("/result/header/id") != selected.pointer("/ids/0") {
            return Err("exact returned identity changed".into());
        }
        if n == 6
            && !value
                .pointer("/result/cross_references")
                .and_then(Value::as_array)
                .is_some_and(|rows| {
                    rows.iter().any(|row| {
                        row.pointer("/identity/id") == Some(&json!("CHEMBL941"))
                            && row.pointer("/attribution/source") == Some(&json!("chembl"))
                    })
                })
        {
            return Err("missing attributed CHEMBL941".into());
        }
        for key in [
            "approvals",
            "regulatory",
            "targets",
            "label",
            "interactions",
        ] {
            if value.pointer(&format!("/result/{key}")).is_some() {
                return Err(format!("identity card expanded {key}"));
            }
        }
    } else {
        let ids = value
            .pointer("/result/items")
            .and_then(Value::as_array)
            .map(|items| items.iter().map(|i| i["id"].clone()).collect::<Vec<_>>())
            .unwrap_or_default();
        if json!(ids) != selected["ids"]
            || value.pointer("/page/total") != selected.get("total")
            || value.pointer("/query/source") != Some(&json!("mychem"))
        {
            return Err(format!("native page changed: {value}"));
        }
        if n == 1 && value.pointer("/result/items/0/name").is_some() {
            return Err("invented name on ID-only candidate".into());
        }
    }
    Ok(())
}
fn wrong_identity() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let stub = Stub::for_source("mychem.info", "application/json", |request| {
        if !request.starts_with("GET /v1/chem/CHEMBL941?") {
            return Err("unexpected identity request".into());
        }
        Ok("{\"_id\":\"different-native-document\"}".into())
    })?;
    let cert = env.root().join("ca.pem");
    std::fs::write(&cert, &stub.certificate).map_err(|e| e.to_string())?;
    env.config(&json!({"ca_bundle":cert,"proxy":format!("http://{}",stub.address)}))
        .map_err(|e| e.to_string())?;
    let args = ["get", "drug", "CHEMBL941", "--source", "mychem"]
        .map(str::to_owned)
        .to_vec();
    let value = invoke(&env, args.clone(), 4)?;
    if value["status"] != "failed"
        || value.pointer("/sources/0/reason") != Some(&json!("bad_reply"))
        || value["result"] != Value::Null
    {
        return Err(format!("wrong identity admitted: {value}"));
    }
    let mcp = super::mcp::call_with_globals(&env, &args, true, &[])?;
    if value != mcp {
        return Err("failed identity differs over MCP".into());
    }
    if stub.finish()?.len() != 2 {
        return Err("wrong identity was cached or triggered a fallback".into());
    }
    Ok(())
}
fn component_policy() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let stub = Stub::for_source("mychem.info", "application/json", |request| {
        if request.contains("%2Cchembl.") || request.contains("fields=chembl.") {
            return Err("disabled component entered request fields".into());
        }
        Ok(REPLIES.get(6).ok_or("fixture")?.to_string())
    })?;
    let cert = env.root().join("ca.pem");
    std::fs::write(&cert, &stub.certificate).map_err(|e| e.to_string())?;
    env.config(&json!({"ca_bundle":cert,"proxy":format!("http://{}",stub.address),"disable_sources":["chembl"]})).map_err(|e|e.to_string())?;
    let args = [
        "get",
        "drug",
        "KTUFNOKKBVMGRW-UHFFFAOYSA-N",
        "--source",
        "mychem",
    ]
    .map(str::to_owned)
    .to_vec();
    let value = invoke(&env, args.clone(), 0)?;
    let rows = value
        .pointer("/result/cross_references")
        .and_then(Value::as_array)
        .ok_or("crossrefs")?;
    if rows
        .iter()
        .any(|r| r.pointer("/attribution/source") == Some(&json!("chembl")))
        || !rows
            .iter()
            .any(|r| r.pointer("/attribution/source") == Some(&json!("drugbank")))
    {
        return Err("component projection leaked or removed unrelated identities".into());
    }
    let source = value["sources"]
        .as_array()
        .and_then(|v| v.iter().find(|s| s["source"] == "chembl"))
        .ok_or("component outcome")?;
    if source["state"] != "skipped"
        || source["reason"] != "disabled"
        || source["use"] != "attribution"
    {
        return Err("component policy outcome".into());
    }
    if super::mcp::call_with_globals(&env, &args, false, &[])? != value {
        return Err("component MCP parity".into());
    }
    if stub.finish()?.len() != 1 {
        return Err("component request cache identity".into());
    }
    Ok(())
}
