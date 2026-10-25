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
    trials.push(Trial::test("drug-mychem/complete-xrefs-page-walk",||xrefs_walk().map_err(Into::into)));
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
    let expected = selected
        .get("request")
        .and_then(Value::as_str)
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
    let bound = [3005,2348,1724,1853,1262,1169,0,1720,516].get(n).copied().ok_or("budget index")?;
    if bound != 0 && comparable > bound { return Err(format!("case {n} exceeds measured comparable output budget: {comparable}/{bound}")); }
    use std::io::Write;
    writeln!(std::io::stdout(),
        "drug native case {n}: full {bytes} bytes/{} tokens; comparable {comparable} bytes/{} tokens",
        bytes.div_ceil(4),
        comparable.div_ceil(4)
    ).map_err(|e|e.to_string())?;
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
            .map(|items| {
                items
                    .iter()
                    .map(|i| i.get("id").cloned().unwrap_or(Value::Null))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        if Some(&json!(ids)) != selected.get("ids")
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
    if value.get("status") != Some(&json!("failed"))
        || value.pointer("/sources/0/reason") != Some(&json!("bad_reply"))
        || value.get("result").is_some_and(|v| !v.is_null())
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
    let source = value
        .get("sources")
        .and_then(Value::as_array)
        .and_then(|v| v.iter().find(|s| s.get("source") == Some(&json!("chembl"))))
        .ok_or("component outcome")?;
    if source.get("state") != Some(&json!("skipped"))
        || source.get("reason") != Some(&json!("disabled"))
        || source.get("use") != Some(&json!("attribution"))
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

fn xrefs_walk()->Check {
    let env=Environment::new().map_err(|e|e.to_string())?;
    let stub=Stub::for_source("mychem.info","application/json",|request| {
        if !request.starts_with("GET /v1/chem/KTUFNOKKBVMGRW-UHFFFAOYSA-N?") {return Err("unexpected link request".into());}
        Ok(REPLIES.get(6).ok_or("fixture")?.to_string())
    })?;
    let cert=env.root().join("ca.pem");std::fs::write(&cert,&stub.certificate).map_err(|e|e.to_string())?;
    env.config(&json!({"ca_bundle":cert,"proxy":format!("http://{}",stub.address)})).map_err(|e|e.to_string())?;
    let card=invoke(&env,["get","drug","KTUFNOKKBVMGRW-UHFFFAOYSA-N","--source","mychem"].map(str::to_owned).to_vec(),0)?;
    let mut next=next(&card,"section")?.ok_or("identity card must offer explicit xrefs action")?;
    let date=card.pointer("/result/retrieved").ok_or("card retrieval date")?.clone();
    let mut links=Vec::new();
    for page in 0..12 {
        let value=invoke(&env,next.clone(),0)?;
        super::schema::Registry::embedded()?.validate(&value,&next,true)?;
        if value.pointer("/result/identity/id")!=Some(&json!("KTUFNOKKBVMGRW-UHFFFAOYSA-N")) || value.pointer("/result/xrefs/data/retrieved")!=Some(&date) || value.pointer("/result/xrefs/data/total")!=Some(&json!(112)) {return Err("xref identity, native link total or retrieval date changed".into());}
        let rows=value.pointer("/result/xrefs/data/records").and_then(Value::as_array).ok_or("xref rows")?;
        if rows.len()>10 {return Err("xref default page exceeded limit".into());}
        links.extend(rows.iter().cloned());
        if page==0 || page==11 { if super::mcp::call_with_globals(&env,&next,false,&[])?!=value {return Err("xref MCP parity".into());} }
        let full=serde_json::to_vec(&value).map_err(|e|e.to_string())?.len();let comparable=super::source_terms::size(&value,"")?;
        use std::io::Write;writeln!(std::io::stdout(),"drug xrefs page {page}: full {full}, comparable {comparable}").map_err(|e|e.to_string())?;
        match self::next(&value,"page")? {Some(args)=>next=args,None=>break,}
    }
    if links.len()!=112 || !links.iter().any(|r|r.pointer("/identity/id")==Some(&json!("CHEMBL941")) && r.pointer("/attribution/source")==Some(&json!("chembl"))) {return Err("xref walk lost native links".into());}
    for (n,row) in links.iter().enumerate() { if links.iter().take(n).any(|r|r==row) {return Err("duplicate xref in walk".into());} }
    if stub.finish()?.len()!=1 {return Err("xref pages did not reuse the same original native document".into());}
    Ok(())
}
fn next(value:&Value,kind:&str)->Result<Option<Vec<String>>,String> {
    value.get("next").and_then(Value::as_array).into_iter().flatten().find(|row|row.get("kind")==Some(&json!(kind))).map(|row|serde_json::from_value(row.get("args").cloned().ok_or("next args")?).map_err(|e|e.to_string())).transpose()
}
