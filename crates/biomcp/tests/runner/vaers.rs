//! Feature vaers-summary: explicit vaccine selection and failed source outcomes.
use super::{configuration::invoke, launch::Environment, network_stub::Stub};
use crate::runner::transcript::trial as transcript_trial;
use libtest_mimic::Trial;
use serde_json::{Value, json};

mod budget;

mod fixture;

mod navigation;

type Check = Result<(), String>;

pub(super) fn trials() -> Vec<Trial> {
    // Keep dynamic state, source bindings or wire checks that YAML cannot express.
    vec![transcript_trial(
        crate::runner::required_contract::VAERS_TABLES.0,
        || navigation::run().map_err(Into::into),
    )]
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
    env.config(
        &json!({"proxy":format!("http://{}",listener.local_addr().map_err(|e|e.to_string())?)}),
    )
    .map_err(|e| e.to_string())?;
    for text in [
        "HPV9",
        "MMR vaccine",
        "COVID",
        "165",
        "FLU3",
        "FLUX",
        "FLU( H1N1 )",
    ] {
        let args = words(text);
        let value = invoke(&env, args.clone(), 2)?;
        let message = value
            .pointer("/error/message")
            .and_then(|v| v.as_str())
            .ok_or("argument message")?;
        if !["MMR", "COVID19"].iter().all(|name| message.contains(name)) {
            return Err(format!(
                "unsupported vaccine lacks exact supported codes: {value}"
            ));
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

fn environment(stub: &Stub) -> Result<Environment, String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let ca = env.root().join("ca.pem");
    stub.retain(&env);
    std::fs::write(&ca, &stub.certificate).map_err(|e| e.to_string())?;
    env.config(&json!({"ca_bundle":ca,"proxy":format!("http://{}",stub.address)}))
        .map_err(|e| e.to_string())?;
    Ok(env)
}
fn both(env: &Environment, args: Vec<String>, exit: i32) -> Result<Value, String> {
    let value = invoke(env, args.clone(), exit)?;
    super::schema::Registry::embedded()?.validate(&value, &args, true)?;
    let start = args.iter().position(|s| s == "search").unwrap_or(0);
    let (globals, command) = args.split_at(start);
    if super::mcp::call_with_globals(env, command, exit >= 2, globals)? != value {
        return Err("VAERS CLI/MCP mismatch".into());
    }
    budget::check(&value)?;
    Ok(value)
}

// Research compatibility keeps uncitable native summary rows in metadata.
pub(super) fn research_aggregate() -> Check {
    let stub = Stub::for_source("wonder.cdc.gov", "application/xml", |_| {
        Ok(fixture::table("MMR", "D8.V10"))
    })?;
    let env = environment(&stub)?;
    let args = vec![
        "search",
        "adverse-event",
        "MMR",
        "seriousness",
        "--source",
        "vaers",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect::<Vec<_>>();
    let cli = invoke(&env, args, 0)?;
    let result = super::research::call(
        &env,
        "search",
        json!({"query":"adverse-event MMR seriousness --source vaers"}),
        &[],
    )?;
    if result.to_string().len() > crate::runner::output_limit("vaers-full-summary")?
        || result.get("results").is_some()
        || result.pointer("/metadata/envelope") != Some(&cli)
        || result.pointer("/metadata/omitted_rows") != Some(&json!(3))
    {
        return Err(format!("uncitable VAERS rows disappeared: {result}"));
    }
    if stub.finish()?.len() != 1 {
        return Err("research wrapper repeated VAERS source request".into());
    }
    Ok(())
}
