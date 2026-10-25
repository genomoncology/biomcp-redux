//! Feature drug-label: captured identity subsets with explicitly synthetic label prose.
//! Whole native label bodies remain outside Git in the reviewed source evidence.
use super::{configuration::invoke, launch::Environment, network_stub::Stub};
use libtest_mimic::Trial;
use serde_json::{Value, json};
type Check = Result<(), String>;
mod walk;
pub(super) fn trials() -> Vec<Trial> {
    let mut trials = Vec::new();
    for n in 0..3 {
        trials.push(Trial::test(format!("drug-label/brand-{n}"), move || {
            search(n).map_err(Into::into)
        }));
    }
    trials.push(Trial::test("drug-label/native-brand-not-found", || {
        absent().map_err(Into::into)
    }));
    for n in 3..9 {
        trials.push(Trial::test(format!("drug-label/spl-card-{n}"), move || {
            card(n).map_err(Into::into)
        }));
    }
    trials.push(Trial::test("drug-label/five-native-headings", || {
        headings().map_err(Into::into)
    }));
    trials.extend(walk::trials());
    trials
}
fn identity(n: usize) -> Result<Value, String> {
    let rows: Vec<Value> = serde_json::from_str(include_str!(
        "../../../../tests/fixtures/drug/openfda/identities.json"
    ))
    .map_err(|e| e.to_string())?;
    rows.get(n).cloned().ok_or("identity index".into())
}
fn record(identity: &Value) -> Value {
    json!({"id":identity["id"],"set_id":identity["set_id"],"version":identity["version"],"effective_time":identity["effective_time"],"openfda":{"brand_name":[identity["brand"]],"spl_set_id":[identity["set_id"]]},"boxed_warning":["Synthetic warning fixture."],"dosage_and_administration":[" Synthetic dose fixture.\n"],"drug_interactions":["Synthetic interaction fixture."],"indications_and_usage":["Synthetic indication fixture."],"warnings_and_cautions":["Synthetic caution fixture."]})
}
fn setup(body: Value, selector: String, status: u16) -> Result<(Environment, Stub), String> {
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
            Ok(body.to_string())
        },
        move |_| status,
    )?;
    let env = Environment::new().map_err(|e| e.to_string())?;
    let cert = env.root().join("ca.pem");
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
        identity["set_id"].as_str().ok_or("set")?.into(),
        "--source".into(),
        "openfda-label".into(),
    ])
}
fn get_setup(identity: &Value, record: Value) -> Result<(Environment, Stub), String> {
    setup(
        page(record),
        format!(
            "openfda.spl_set_id%3A%22{}%22",
            identity["set_id"].as_str().ok_or("set")?
        ),
        200,
    )
}
fn both(env: &Environment, args: Vec<String>, exit: i32) -> Result<Value, String> {
    let value = invoke(env, args.clone(), exit)?;
    super::schema::Registry::embedded()?.validate(&value, &args, true)?;
    let mcp = super::mcp::call_with_globals(env, &args, false, &[])?;
    if value != mcp {
        return Err("CLI and MCP differ".into());
    }
    measure(&value)?;
    Ok(value)
}
fn measure(value: &Value) -> Check {
    let bytes = serde_json::to_vec(value).map_err(|e| e.to_string())?.len();
    let comparable = super::source_terms::size(value, "")?;
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
    let brand = id["brand"].as_str().ok_or("brand")?;
    let (env, stub) = setup(
        page(record(&id)),
        format!("openfda.brand_name.exact%3A%22{brand}%22"),
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
    if value.pointer("/result/items/0/id") != Some(&id["set_id"])
        || value.pointer("/result/items/0/version") != Some(&id["version"])
    {
        return Err("native label identity or revision changed".into());
    }
    if stub.finish()?.len() != 1 {
        return Err("warm label search made another request".into());
    }
    Ok(())
}
fn absent() -> Check {
    let (env, stub) = setup(
        json!({"error":{"code":"NOT_FOUND","message":"No matches found!"}}),
        "openfda.brand_name.exact%3A%22Tagrisso%22".into(),
        404,
    )?;
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
    if value["status"] != "absent"
        || value.pointer("/result/total") != Some(&json!(0))
        || value.pointer("/sources/0/state") != Some(&json!("ok"))
    {
        return Err("native NOT_FOUND became a failed source or a clinical answer".into());
    }
    if stub.finish()?.len() != 2 {
        return Err("native absence retry count changed".into());
    }
    Ok(())
}
fn card(n: usize) -> Check {
    let id = identity(n)?;
    let (env, stub) = get_setup(&id, record(&id))?;
    let value = both(&env, get_args(&id)?, 0)?;
    if value.pointer("/result/header/id") != Some(&id["set_id"])
        || value.pointer("/result/version") != Some(&id["version"])
        || value.pointer("/result/header/identity_level") != Some(&json!("spl_set"))
    {
        return Err("SPL card identity changed".into());
    }
    if value["result"].get("label").is_some()
        || value["result"].get("text").is_some()
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
    for heading in biomcp_adapters::openfda_label::HEADINGS {
        let mut args = get_args(&id)?;
        args.extend(["label".into(), "--label-section".into(), heading.into()]);
        let value = both(&env, args, 0)?;
        if value["schema"] != "biomcp.drug_label/3"
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
