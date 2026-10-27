//! Feature study-catalog: synchronize the publisher's JSON representation.
use super::{rows, super::{configuration::invoke, launch::Environment, network_stub::Stub}};
use libtest_mimic::Trial;
use serde_json::json;
use std::fs;

pub(super) fn trials() -> Vec<Trial> {
    vec![Trial::test("study-catalog/sync-negotiates-json", || {
        sync().map_err(Into::into)
    }),
    Trial::test("pathway-failures/sync-negotiates-json", || {
        document("wikipathways", "www.wikipathways.org", "application/json", "pathway/examples-sync/wikipathways", 2).map_err(Into::into)
    }),
    Trial::test("diagnostic-retirements/sync-negotiates-html", || {
        document("fda-cdx", "www.fda.gov", "text/html", "diagnostic/fda-cdx-synthetic/fda-cdx", 1).map_err(Into::into)
    })]
}
fn accepts_json(request: &str) -> bool {
    request.lines().any(|line| line.eq_ignore_ascii_case("accept: application/json"))
}
fn sync() -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let stub = Stub::for_source_status(
        "www.cbioportal.org",
        "application/json",
        |_| Ok(rows().to_string()),
        |request| if accepts_json(request) { 200 } else { 406 },
    )?;
    let ca = env.root().join("ca.pem");
    fs::write(&ca, &stub.certificate).map_err(|e| e.to_string())?;
    env.config(&json!({"proxy":format!("http://{}",stub.address),"ca_bundle":ca}))
        .map_err(|e| e.to_string())?;
    let result = invoke(&env, vec!["sync".into(), "cbioportal-studies".into()], 0);
    let requests = stub.finish()?;
    if requests.len() != 1 {
        return Err(format!("catalog sync made {} physical requests", requests.len()));
    }
    let request = &requests.first().ok_or("missing catalog request")?.1;
    if !accepts_json(request) {
        return Err(format!("catalog requested a rejected representation: {request}; result: {result:?}"));
    }
    let value = result?;
    if value.pointer("/result/rows") != Some(&json!(12)) {
        return Err(format!("catalog sync did not publish the JSON rows: {value}"));
    }
    let value = invoke(&env, vec!["search".into(), "study".into(), "atlas".into(), "--offline".into()], 0)?;
    if value.pointer("/page/total") != Some(&json!(12)) {
        return Err(format!("synchronized catalog is unavailable offline: {value}"));
    }
    Ok(())
}

fn document(source: &str, host: &'static str, media: &'static str, fixture: &str, count: usize) -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let mut replies = Vec::new();
    for file in fs::read_dir(super::super::launch::fixtures().join("replies").join(fixture)).map_err(|e| e.to_string())? {
        let file = file.map_err(|e| e.to_string())?;
        let value: serde_json::Value = serde_json::from_slice(&fs::read(file.path()).map_err(|e| e.to_string())?).map_err(|e| e.to_string())?;
        let url = value.get("url").and_then(serde_json::Value::as_str).ok_or("missing fixture URL")?;
        let path = url.strip_prefix(&format!("https://{host}")).ok_or("wrong fixture host")?;
        let body = value.pointer("/exchanges/0/body/value").and_then(serde_json::Value::as_str).ok_or("missing fixture body")?;
        replies.push((format!("GET {path} "), body.to_owned()));
    }
    let stub = Stub::for_source_status(host, media, move |request| {
        replies.iter().find(|(path, _)| request.starts_with(path)).map(|(_, body)| body.clone()).ok_or_else(|| format!("unexpected document request: {request}"))
    }, move |request| if request.lines().any(|line| line.eq_ignore_ascii_case(&format!("accept: {media}"))) {200} else {406})?;
    let ca = env.root().join("ca.pem");
    fs::write(&ca, &stub.certificate).map_err(|e| e.to_string())?;
    env.config(&json!({"proxy":format!("http://{}",stub.address),"ca_bundle":ca})).map_err(|e| e.to_string())?;
    let result = invoke(&env, vec!["sync".into(), source.into()], 0);
    let requests = stub.finish()?;
    result?;
    if requests.len() != count || requests.iter().any(|(_, request)| !request.lines().any(|line| line.eq_ignore_ascii_case(&format!("accept: {media}")))) {
        return Err(format!("{source} did not request its {count} {media} documents: {requests:?}"));
    }
    Ok(())
}
