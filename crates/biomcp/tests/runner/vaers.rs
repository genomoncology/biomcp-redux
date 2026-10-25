//! Feature vaers-summary: explicit vaccine selection and failed source outcomes.
use super::{configuration::invoke, launch::Environment, network_stub::Stub};
use libtest_mimic::Trial;
use serde_json::{Value, json};
mod fixture;
mod failures;
mod rejections;
mod navigation;
type Check = Result<(), String>;

pub(super) fn trials() -> Vec<Trial> {
    vec![
        Trial::test("vaers-summary/rejected-filters-and-influenza-before-io", || rejections::run().map_err(Into::into)),
        Trial::test("vaers-summary/synthetic-403-429-500-timeout-broken", || failures::run().map_err(Into::into)),
        Trial::test("vaers-summary/independent-tables-retained-walk-and-suppression", || navigation::run().map_err(Into::into)),
        Trial::test("vaers-summary/unsupported-vaccine-before-io", || {
            unsupported().map_err(Into::into)
        }),
        Trial::test("vaers-summary/broken-xml-is-not-zero-reports", || {
            broken().map_err(Into::into)
        }),
    ]
}

fn words(text: &str) -> Vec<String> {
    ["search", "adverse-event", text, "--source", "vaers"]
        .into_iter()
        .map(str::to_owned)
        .collect()
}

fn unsupported() -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let listener = std::net::TcpListener::bind("127.0.0.1:0").map_err(|e| e.to_string())?;
    listener.set_nonblocking(true).map_err(|e| e.to_string())?;
    env.config(&json!({"proxy":format!("http://{}",listener.local_addr().map_err(|e|e.to_string())?)}))
        .map_err(|e| e.to_string())?;
    for text in ["HPV9", "MMR vaccine", "COVID", "165"] {
        let args = words(text);
        let value = invoke(&env, args.clone(), 2)?;
        let message = value.pointer("/error/message").and_then(|v| v.as_str()).ok_or("argument message")?;
        if !["MMR", "COVID19"].iter().all(|name| message.contains(name)) {
            return Err(format!("unsupported vaccine lacks exact supported codes: {value}"));
        }
        if super::mcp::call_with_globals(&env, &args, true, &[])? != value {
            return Err("unsupported vaccine differs through MCP".into());
        }
    }
    if listener.accept().is_ok() {
        return Err("unsupported vaccine opened a source connection".into());
    }
    Ok(())
}

fn broken() -> Result<(), String> {
    let stub = Stub::for_source("wonder.cdc.gov", "application/xml", |_| {
        Ok("<synthetic-broken>".into())
    })?;
    let env = Environment::new().map_err(|e| e.to_string())?;
    let ca = env.root().join("ca.pem");
    std::fs::write(&ca, &stub.certificate).map_err(|e| e.to_string())?;
    env.config(&json!({"ca_bundle":ca,"proxy":format!("http://{}",stub.address)}))
        .map_err(|e| e.to_string())?;
    let args = words("MMR");
    let value = invoke(&env, args.clone(), 4)?;
    super::schema::Registry::embedded()?.validate(&value, &args, true)?;
    if value.get("status") != Some(&json!("failed"))
        || value.get("result").is_some()
        || value.pointer("/sources/0/source") != Some(&json!("vaers"))
        || value.pointer("/sources/0/reason") != Some(&json!("bad_reply"))
        || value.pointer("/sources/0/terms_url") != Some(&json!("https://wonder.cdc.gov/datause.html"))
    {
        return Err(format!("broken XML became an aggregate answer: {value}"));
    }
    if stub.finish()?.len() != 1 {
        return Err("broken XML source was retried".into());
    }
    Ok(())
}

fn environment(stub: &Stub) -> Result<Environment, String> {
    let env = Environment::new().map_err(|e|e.to_string())?;
    let ca = env.root().join("ca.pem");
    std::fs::write(&ca, &stub.certificate).map_err(|e|e.to_string())?;
    env.config(&json!({"ca_bundle":ca,"proxy":format!("http://{}",stub.address)})).map_err(|e|e.to_string())?;
    Ok(env)
}
fn both(env: &Environment, args: Vec<String>, exit: i32) -> Result<Value, String> {
    let value = invoke(env, args.clone(), exit)?;
    super::schema::Registry::embedded()?.validate(&value, &args, true)?;
    if super::mcp::call_with_globals(env, &args, exit >= 2, &[])? != value { return Err("VAERS CLI/MCP mismatch".into()); }
    Ok(value)
}
