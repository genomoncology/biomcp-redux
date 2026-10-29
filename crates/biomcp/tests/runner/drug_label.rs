//! Feature drug-label: captured identity subsets with explicitly synthetic label prose.
//! Whole native label bodies remain outside Git in the reviewed source evidence.
use super::{configuration::invoke, launch::Environment, network_stub::Stub};
use crate::runner::transcript::trial as transcript_trial;
use libtest_mimic::Trial;
use serde_json::{Value, json};
type Check = Result<(), String>;
mod capture;
mod errors;
mod fallback;
mod generic;
mod pagination;
mod walk;
pub(super) fn trials() -> Vec<Trial> {
    let mut trials = Vec::new();
    for n in 0..3 {
        trials.push(transcript_trial(
            format!("drug-label/brand-{n}"),
            move || search(n).map_err(Into::into),
        ));
    }
    trials.push(transcript_trial(
        "drug-label/native-brand-not-found",
        || absent().map_err(Into::into),
    ));
    for n in 3..9 {
        trials.push(transcript_trial(
            format!("drug-label/spl-card-{n}"),
            move || card(n).map_err(Into::into),
        ));
    }
    trials.push(transcript_trial("drug-label/five-native-headings", || {
        headings().map_err(Into::into)
    }));
    trials.extend(fallback::trials());
    trials.extend(generic::trials());
    trials.extend(pagination::trials());
    trials.extend(walk::trials());
    trials.extend(errors::trials());
    trials.extend(capture::trials());
    trials
}
fn identity(n: usize) -> Result<Value, String> {
    let rows: Vec<Value> = serde_json::from_str(&crate::runner::launch::fixture_text(
        "fixtures/drug/openfda/identities.json",
    )?)
    .map_err(|e| e.to_string())?;
    rows.get(n).cloned().ok_or("identity index".into())
}
fn record(identity: &Value) -> Value {
    json!(
        {"id":field(identity,"id"),"set_id":field(identity,"set_id"),"version":field(identity,"version"),"effective_time":field(identity,
        "effective_time"),"openfda":{"brand_name":[field(identity,"brand")],"spl_set_id":[field(identity,"set_id")]},
        "boxed_warning":["Synthetic warning fixture."],"dosage_and_administration":[" Synthetic dose fixture.\n"],
        "drug_interactions":["Synthetic interaction fixture."],"indications_and_usage":["Synthetic indication fixture."],
        "warnings_and_cautions":["Synthetic caution fixture."]}
    )
}
fn setup(body: Value, selector: String, status: u16) -> Result<(Environment, Stub), String> {
    setup_raw(body.to_string(), selector, status)
}
fn setup_raw(body: String, selector: String, status: u16) -> Result<(Environment, Stub), String> {
    let expected = format!(
        "GET /drug/label.json?search={selector}&limit=1&sort=effective_time%3Adesc HTTP/1.1"
    );
    let stub = Stub::for_source_status(
        "api.fda.gov",
        "application/json",
        move |request| {
            if request.lines().next() != Some(expected.as_str()) {
                return Err(format!("unexpected request: {request}"));
            }
            Ok(body.clone())
        },
        move |_| status,
    )?;
    environment(stub)
}
fn environment(stub: Stub) -> Result<(Environment, Stub), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let cert = env.root().join("ca.pem");
    stub.retain(&env);
    std::fs::write(&cert, &stub.certificate).map_err(|e| e.to_string())?;
    env.config(&json!({"ca_bundle":cert,"proxy":format!("http://{}",stub.address)}))
        .map_err(|e| e.to_string())?;
    Ok((env, stub))
}
fn page(record: Value) -> Value {
    json!({"meta":{"results":{"skip":0,"limit":1,"total":1}},"results":[record]})
}
fn get_args(identity: &Value) -> Result<Vec<String>, String> {
    Ok(vec![
        "get".into(),
        "drug".into(),
        field(identity, "set_id").as_str().ok_or("set")?.into(),
        "--source".into(),
        "openfda-label".into(),
    ])
}
fn get_setup(identity: &Value, record: Value) -> Result<(Environment, Stub), String> {
    setup(
        page(record),
        format!(
            "openfda.spl_set_id%3A%22{}%22",
            field(identity, "set_id").as_str().ok_or("set")?
        ),
        200,
    )
}
fn both(env: &Environment, args: Vec<String>, exit: i32) -> Result<Value, String> {
    let value = invoke(env, args.clone(), exit)?;
    super::schema::Registry::embedded()?.validate(&value, &args, true)?;
    let mcp = super::mcp::call_with_globals(env, &args, exit >= 2, &[])?;
    if value != mcp {
        return Err("CLI and MCP differ".into());
    }
    measure(&value)?;
    Ok(value)
}
fn measure(value: &Value) -> Check {
    let bytes = serde_json::to_vec(value).map_err(|e| e.to_string())?.len();
    let comparable = super::source_terms::size(value, "")?;
    // Peak retained full-body replay is 7661 comparable bytes; allow 15% growth.
    if comparable > 8811 {
        return Err(format!(
            "label exceeds measured comparable budget {comparable}/8811"
        ));
    }
    if bytes > 8192 {
        return Err(format!("label exceeds full allocation {bytes}/8192"));
    }
    use std::io::Write;
    writeln!(
        std::io::stdout(),
        "drug-label: full {bytes} bytes/{} tokens, comparable {comparable} bytes/{} tokens",
        bytes.div_ceil(4),
        comparable.div_ceil(4)
    )
    .map_err(|e| e.to_string())
}
fn search(n: usize) -> Check {
    let id = identity(n)?;
    let brand = field(&id, "brand").as_str().ok_or("brand")?;
    let (env, stub) = setup(
        page(record(&id)),
        format!(
            "openfda.brand_name.exact%3A%22{brand}%22%20OR%20openfda.generic_name.exact%3A%22{brand}%22"
        ),
        200,
    )?;
    let args = vec![
        "search",
        "drug",
        brand,
        "--source",
        "openfda-label",
        "--limit",
        "1",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect();
    let value = both(&env, args, 0)?;
    budget(&value, if n == 1 { 647 } else { 643 })?;
    if value.pointer("/result/items/0/id") != Some(field(&id, "set_id"))
        || value.pointer("/result/items/0/version") != Some(field(&id, "version"))
    {
        return Err("native label identity or revision changed".into());
    }
    if stub.finish()?.len() != 1 {
        return Err("warm label search made another request".into());
    }
    Ok(())
}
fn absent() -> Check {
    let stub = Stub::for_source_status(
        "api.fda.gov",
        "application/json",
        |request| {
            let selector = request.lines().next().ok_or("request")?;
            if !["openfda.brand_name.exact", "openfda.brand_name"].iter().any(|field| {
                let generic = field.replace("brand_name", "generic_name");
                let names = format!("{field}%3A%22Tagrisso%22%20OR%20{generic}%3A%22Tagrisso%22");
                selector == format!(
                    "GET /drug/label.json?search={names}&limit=1&sort=effective_time%3Adesc HTTP/1.1"
                )
            }) {
                return Err(format!("unexpected absence request: {request}"));
            }
            Ok(json!({"error":{"code":"NOT_FOUND","message":"No matches found!"}}).to_string())
        },
        |_| 404,
    )?;
    let (env, stub) = environment(stub)?;
    let args = vec![
        "search",
        "drug",
        "Tagrisso",
        "--source",
        "openfda-label",
        "--limit",
        "1",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect();
    let value = both(&env, args, 1)?;
    budget(&value, 262)?;
    if field(&value, "status") != "absent"
        || value.pointer("/result/total") != Some(&json!(0))
        || value.pointer("/sources/0/state") != Some(&json!("ok"))
    {
        return Err("native NOT_FOUND became a failed source or a clinical answer".into());
    }
    if stub.finish()?.len() != 4 {
        return Err("native absence fallback count changed".into());
    }
    Ok(())
}
fn card(n: usize) -> Check {
    let id = identity(n)?;
    let (env, stub) = get_setup(&id, record(&id))?;
    let value = both(&env, get_args(&id)?, 0)?;
    let measured = [1390, 1388, 1392, 1386, 1384, 1386]
        .get(n - 3)
        .copied()
        .ok_or("card budget")?;
    budget(&value, measured)?;
    if value.pointer("/result/header/id") != Some(field(&id, "set_id"))
        || value.pointer("/result/version") != Some(field(&id, "version"))
        || value.pointer("/result/header/identity_level") != Some(&json!("spl_set"))
    {
        return Err("SPL card identity changed".into());
    }
    if field(&value, "result").get("label").is_some()
        || field(&value, "result").get("text").is_some()
        || value.pointer("/result/retrieved").is_none()
    {
        return Err("card leaks label text or loses retrieval metadata".into());
    }
    if stub.finish()?.len() != 1 {
        return Err("warm card made another request".into());
    }
    Ok(())
}
fn headings() -> Check {
    let id = identity(4)?;
    let record = record(&id);
    let (env, stub) = get_setup(&id, record.clone())?;
    for (index, heading) in biomcp_adapters::openfda_label::HEADINGS
        .into_iter()
        .enumerate()
    {
        let mut args = get_args(&id)?;
        args.extend(["label".into(), "--label-section".into(), heading.into()]);
        let value = both(&env, args, 0)?;
        let measured = [1208, 1232, 1220, 1227, 1224]
            .get(index)
            .copied()
            .ok_or("heading budget")?;
        budget(&value, measured)?;
        if field(&value, "schema") != "biomcp.drug-label/3"
            || value.pointer("/result/label/data/text") != record.pointer(&format!("/{heading}/0"))
            || value.pointer("/result/label/data/retrieved").is_none()
        {
            return Err(format!("native heading or text changed: {heading}"));
        }
    }
    if stub.finish()?.len() != 1 {
        return Err("cached field selections fetched the label again".into());
    }
    Ok(())
}

fn field<'a>(value: &'a Value, key: &str) -> &'a Value {
    value.get(key).unwrap_or(&Value::Null)
}

fn budget(value: &Value, measured: usize) -> Check {
    let actual = super::source_terms::size(value, "")?;
    let bound = (measured * 115).div_ceil(100);
    if actual > bound {
        return Err(format!(
            "label case exceeds measured comparable budget {actual}/{bound}"
        ));
    }
    Ok(())
}
