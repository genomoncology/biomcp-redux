//! Feature adverse-event-failures: synthetic failures preserve source identity and terms.
use super::{configuration::invoke, launch::Environment, network_stub::Stub};
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::{fs, path::Path};
type Check = Result<(), String>;
pub(super) fn trials() -> Vec<Trial> {
    ["faers", "maude"]
        .into_iter()
        .map(|source| {
            Trial::test(
                format!("adverse-event-failures/{source}/synthetic-429-500-timeout-broken"),
                move || check(source).map_err(Into::into),
            )
        })
        .collect()
}
fn words(source: &str) -> Vec<String> {
    [
        "search",
        "adverse-event",
        "Synthetic failure",
        "--source",
        source,
    ]
    .into_iter()
    .map(str::to_owned)
    .collect()
}
fn check(source: &str) -> Check {
    let stub = Stub::for_source("api.fda.gov", "application/json", |_| {
        Ok("synthetic broken body".into())
    })?;
    let env = Environment::new().map_err(|e| e.to_string())?;
    let ca = env.root().join("ca.pem");
    fs::write(&ca, &stub.certificate).map_err(|e| e.to_string())?;
    env.config(&json!({"ca_bundle":ca,"proxy":format!("http://{}",stub.address)}))
        .map_err(|e| e.to_string())?;
    let original = env.root().join("recording");
    let mut args = words(source);
    args.extend(["--record".into(), original.to_string_lossy().into_owned()]);
    invoke(&env, args, 4)?;
    if stub.finish()?.len() != 1 {
        return Err("synthetic fixture delivery count changed".into());
    }
    for fault in ["429", "500", "timeout", "broken"] {
        let directory = env.root().join(fault);
        mutate(&original, &directory, fault)?;
        let replay = Environment::new().map_err(|e| e.to_string())?;
        let globals = replay.globals(Some(&directory), false);
        let mut args = globals.clone();
        args.extend(words(source));
        let value = invoke(&replay, args, 4)?;
        assert_failed(source, fault, &value)?;
        if super::mcp::call_with_globals(&replay, &words(source), true, &globals)? != value {
            return Err("failure differs through MCP".into());
        }
        super::schema::Registry::embedded()?.validate(&value, &words(source), true)?;
    }
    Ok(())
}
fn mutate(original: &Path, destination: &Path, fault: &str) -> Check {
    for source in fs::read_dir(original).map_err(|e| e.to_string())? {
        let source = source.map_err(|e| e.to_string())?;
        if !source.path().is_dir() {
            continue;
        }
        let target = destination.join(source.file_name());
        fs::create_dir_all(&target).map_err(|e| e.to_string())?;
        for file in fs::read_dir(source.path()).map_err(|e| e.to_string())? {
            let file = file.map_err(|e| e.to_string())?;
            if file.path().extension().is_none_or(|v| v != "json") {
                continue;
            }
            let mut value: Value =
                serde_json::from_slice(&fs::read(file.path()).map_err(|e| e.to_string())?)
                    .map_err(|e| e.to_string())?;
            let exchange = value.pointer_mut("/exchanges/0").ok_or("exchange")?;
            *exchange.get_mut("status").ok_or("status")? = json!(match fault {
                "429" => 429,
                "500" => 500,
                _ => 200,
            });
            *exchange.get_mut("delay_ms").ok_or("delay")? =
                json!(if fault == "timeout" { 180000 } else { 0 });
            *exchange.get_mut("body").ok_or("body")? =
                json!({"kind":"text","value":"synthetic broken body"});
            let exchanges = json!(vec![exchange.clone(); 5]);
            *value.get_mut("exchanges").ok_or("exchanges")? = exchanges;
            fs::write(target.join(file.file_name()), value.to_string())
                .map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}
fn assert_failed(source: &str, fault: &str, value: &Value) -> Check {
    let outcome = value
        .get("sources")
        .and_then(Value::as_array)
        .and_then(|rows| {
            rows.iter()
                .find(|row| row.get("source") == Some(&json!(source)))
        })
        .ok_or("source outcome")?;
    let reason = match fault {
        "429" => "rate_limited",
        "500" => "unavailable",
        "timeout" => "deadline_exceeded",
        _ => "bad_reply",
    };
    if value.get("status") != Some(&json!("failed"))
        || value.get("result").is_some()
        || outcome.get("state") != Some(&json!("failed"))
        || outcome.get("reason") != Some(&json!(reason))
    {
        return Err(format!("{source} {fault} became absence: {value}"));
    }
    let full = serde_json::to_vec(value).map_err(|e| e.to_string())?.len();
    let comparable = super::source_terms::size(value, "")?;
    // Measurements are printed by the first targeted run; final caps use its maxima +15%.
    if full > 10000 || comparable > 10000 {
        return Err("failure exceeds size ceiling".into());
    }
    use std::io::Write;
    writeln!(
        std::io::stdout(),
        "synthetic {source} {fault}: full={full} comparable={comparable}"
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}
