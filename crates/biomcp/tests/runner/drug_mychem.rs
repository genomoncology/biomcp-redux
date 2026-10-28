//! Feature drug-mychem: native replies captured 2026-10-25, delivered by a local TLS peer.
//! Imatinib card fixture is a labeled identity subset. Full native replies stay in source evidence.
use super::{configuration::invoke, launch::Environment, network_stub::Stub};
use crate::runner::transcript::trial as transcript_trial;
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
        transcript_trial(format!("drug-mychem/{name}"), move || {
            check(n).map_err(Into::into)
        })
    })
    .collect();
    trials.push(transcript_trial(
        "drug-mychem/reject-wrong-identity-before-cache",
        || wrong_identity().map_err(Into::into),
    ));
    trials.push(transcript_trial(
        "drug-mychem/component-policy-before-request-and-projection",
        || component_policy().map_err(Into::into),
    ));
    trials.push(transcript_trial(
        "drug-mychem/complete-xrefs-page-walk",
        || xrefs_walk().map_err(Into::into),
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
    stub.retain(&env);
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
    let bound = [3532, 2875, 1724, 1853, 1262, 1169, 1652, 958, 516]
        .get(n)
        .copied()
        .ok_or("budget index")?;
    use std::io::Write;
    writeln!(
        std::io::stdout(),
        "drug native case {n}: full {bytes} bytes/{} tokens; comparable {comparable} bytes/{} tokens",
        bytes.div_ceil(4),
        comparable.div_ceil(4)
    )
    .map_err(|e| e.to_string())?;
    let measured_full: [usize; 9] = [3822, 3251, 2250, 2443, 1848, 1767, 2398, 1795, 1199];
    let full_bound = (*measured_full.get(n).ok_or("full budget index")? * 115).div_ceil(100);
    if comparable > bound || bytes > full_bound {
        return Err(format!(
            "case {n} exceeds measured comparable output budget: {comparable}/{bound}"
        ));
    }
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
            && value
                .pointer("/result/alternate_names")
                .and_then(Value::as_array)
                .is_none_or(|names| names.len() != 29)
        {
            return Err("identity card lost native aliases".into());
        }
        if value.pointer("/result/cross_references").is_some()
            || value.pointer("/result/parents").is_some()
            || value.pointer("/result/products").is_some()
        {
            return Err("identity card expanded native link scope".into());
        }
        let expected = if n == 6 {
            json!([{"source":"mychem", "record":"KTUFNOKKBVMGRW-UHFFFAOYSA-N", "field":"drugcentral.xrefs.pdb_chem_id", "code":"value.unknown"}])
        } else {
            json!([])
        };
        let problems = value.get("problems").cloned().unwrap_or_else(|| json!([]));
        if problems != expected {
            return Err(format!(
                "native rejected reference lost its disclosure: {value}"
            ));
        }
        if next(value, "section")?.is_none() {
            return Err("identity card lacks xrefs action".into());
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
    stub.retain(&env);
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
    stub.retain(&env);
    std::fs::write(&cert, &stub.certificate).map_err(|e| e.to_string())?;
    env.config(&json!({"ca_bundle":cert,"proxy":format!("http://{}",stub.address),"disable_sources":["chembl"]}))
        .map_err(|e| e.to_string())?;
    let args = [
        "get",
        "drug",
        "KTUFNOKKBVMGRW-UHFFFAOYSA-N",
        "xrefs",
        "--source",
        "mychem",
    ]
    .map(str::to_owned)
    .to_vec();
    let value = invoke(&env, args.clone(), 0)?;
    let rows = value
        .pointer("/result/xrefs/data/records")
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

fn xrefs_walk() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let stub = Stub::for_source("mychem.info", "application/json", |request| {
        if !request.starts_with("GET /v1/chem/KTUFNOKKBVMGRW-UHFFFAOYSA-N?") {
            return Err("unexpected link request".into());
        }
        Ok(REPLIES.get(6).ok_or("fixture")?.to_string())
    })?;
    let cert = env.root().join("ca.pem");
    stub.retain(&env);
    std::fs::write(&cert, &stub.certificate).map_err(|e| e.to_string())?;
    env.config(&json!({"ca_bundle":cert,"proxy":format!("http://{}",stub.address)}))
        .map_err(|e| e.to_string())?;
    let card = invoke(
        &env,
        [
            "get",
            "drug",
            "KTUFNOKKBVMGRW-UHFFFAOYSA-N",
            "--source",
            "mychem",
        ]
        .map(str::to_owned)
        .to_vec(),
        0,
    )?;
    let next = next(&card, "section")?.ok_or("identity card must offer explicit xrefs action")?;
    let date = card
        .pointer("/result/retrieved")
        .ok_or("card retrieval date")?
        .clone();
    let links = walk_pages(&env, next, &date)?;
    if links.len() != 135
        || !links.iter().any(|r| {
            r.pointer("/identity/id") == Some(&json!("CHEMBL941"))
                && r.pointer("/attribution/source") == Some(&json!("chembl"))
        })
    {
        return Err("xref walk lost native links".into());
    }
    let applications = links
        .iter()
        .filter(|r| r.get("relation") == Some(&json!("ndc.applicationnumber")))
        .collect::<Vec<_>>();
    if applications.len() != 32 {
        return Err("native NDC product/application contexts were merged".into());
    }
    for product in [
        "82009-087_8d48cd1c-474c-49f9-8e70-aa7525bb1087",
        "82009-086_8d48cd1c-474c-49f9-8e70-aa7525bb1087",
    ] {
        if !applications.iter().any(|r| {
            r.pointer("/identity/id") == Some(&json!("ANDA212773"))
                && r.pointer("/attribution/record") == Some(&json!(product))
        }) {
            return Err("an application lost its source product record".into());
        }
    }
    for (n, row) in links.iter().enumerate() {
        if links.iter().take(n).any(|r| r == row) {
            return Err("duplicate xref in walk".into());
        }
    }
    custom_page(&env, &links, &date)?;
    if stub.finish()?.len() != 1 {
        return Err("xref pages did not reuse the same original native document".into());
    }
    Ok(())
}
fn next(value: &Value, kind: &str) -> Result<Option<Vec<String>>, String> {
    value
        .get("next")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .find(|row| row.get("kind") == Some(&json!(kind)))
        .map(|row| {
            serde_json::from_value(row.get("args").cloned().ok_or("next args")?)
                .map_err(|e| e.to_string())
        })
        .transpose()
}

fn walk_pages(
    env: &Environment,
    mut next: Vec<String>,
    date: &Value,
) -> Result<Vec<Value>, String> {
    let mut links = Vec::new();
    for page in 0..14 {
        let value = invoke(env, next.clone(), 0)?;
        super::schema::Registry::embedded()?.validate(&value, &next, true)?;
        if value.pointer("/result/identity/id") != Some(&json!("KTUFNOKKBVMGRW-UHFFFAOYSA-N"))
            || value.pointer("/result/xrefs/data/retrieved") != Some(date)
            || value.pointer("/result/xrefs/data/total") != Some(&json!(135))
        {
            return Err("xref identity, native link total or retrieval date changed".into());
        }
        let rows = value
            .pointer("/result/xrefs/data/records")
            .and_then(Value::as_array)
            .ok_or("xref rows")?;
        if rows.len() > 10 {
            return Err("xref default page exceeded limit".into());
        }
        links.extend(rows.iter().cloned());
        if (page == 0 || page == 13)
            && super::mcp::call_with_globals(env, &next, false, &[])? != value
        {
            return Err("xref MCP parity".into());
        }
        measure_links(page, &value)?;
        if page == 13 && self::next(&value, "page")?.is_some() {
            return Err("xref walk advertised a page past its exact total".into());
        }
        match self::next(&value, "page")? {
            Some(args) => next = args,
            None => break,
        }
    }
    Ok(links)
}
fn custom_page(env: &Environment, links: &[Value], date: &Value) -> Check {
    let args = [
        "get",
        "drug",
        "KTUFNOKKBVMGRW-UHFFFAOYSA-N",
        "xrefs",
        "--source",
        "mychem",
        "--section-limit",
        "7",
        "--section-offset",
        "1",
    ]
    .map(str::to_owned)
    .to_vec();
    let custom = invoke(env, args, 0)?;
    let expected = links.get(1..8).ok_or("native custom page")?;
    if custom.pointer("/result/xrefs/data/records") != Some(&json!(expected))
        || custom.pointer("/result/xrefs/data/retrieved") != Some(date)
    {
        return Err("explicit xref selectors changed identity or rows".into());
    }
    let continuation = self::next(&custom, "page")?.ok_or("custom continuation")?;
    if !continuation
        .windows(2)
        .any(|w| w == ["--section-limit", "7"])
        || !continuation
            .windows(2)
            .any(|w| w == ["--section-offset", "8"])
    {
        return Err("xref continuation lost explicit selectors".into());
    }
    Ok(())
}

fn measure_links(page: usize, value: &Value) -> Check {
    let max_bytes = [
        3979, 3925, 3991, 4061, 4623, 4581, 4571, 4632, 4590, 4566, 4628, 4586, 4545, 2667,
    ]
    .get(page)
    .copied()
    .ok_or("xref page budget")?;
    let full = serde_json::to_vec(value).map_err(|e| e.to_string())?.len();
    let comparable = super::source_terms::size(value, "")?;
    let measured_full = [
        4422, 4375, 4432, 4493, 4982, 4945, 4936, 4989, 4953, 4932, 4986, 4949, 4914, 3281,
    ]
    .get(page)
    .copied()
    .ok_or("xref full page budget")?;
    let max_full = (measured_full * 115_usize).div_ceil(100);
    if comparable > max_bytes || full > max_full {
        return Err(format!(
            "xref page {page} exceeds measured output budget: full {full}/{max_full}, comparable {comparable}/{max_bytes}"
        ));
    }
    use std::io::Write;
    writeln!(
        std::io::stdout(),
        "drug xrefs page {page}: full {full}, comparable {comparable}"
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}
