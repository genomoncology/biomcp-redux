//! Feature key-registration: optional openFDA keys cross only the declared TLS wire.
use super::{launch::Environment, network_stub::Stub};
use crate::runner::transcript::trial as transcript_trial;
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::{fs, path::Path};
const KEY: &str = "synthetic-openfda-config-key";
const ENV_KEY: &str = "synthetic-openfda-env-key";
type Check = Result<(), String>;
pub(super) fn trials() -> Vec<Trial> {
    [
        (
            "openfda-label",
            "drug",
            "00000000-0000-0000-0000-000000000001",
        ),
        ("drugsfda", "drug", "NDA021335"),
        ("faers", "adverse-event", "10000001"),
        ("maude", "adverse-event", "10000001"),
        ("fda-pma", "diagnostic", "P150013"),
        ("fda-510k", "diagnostic", "K173492"),
    ]
    .into_iter()
    .flat_map(|(source, entity, id)| {
        [0, 1, 2].into_iter().map(move |mode| {
            transcript_trial(
                format!("key-registration/keys/openfda/{source}/{mode}"),
                move || check(source, entity, id, mode).map_err(Into::into),
            )
        })
    })
    .collect()
}
fn check(source: &str, entity: &str, id: &str, mode: u8) -> Check {
    let spec = biomcp_adapters::registry::SOURCES
        .iter()
        .find(|spec| spec.source.to_string() == source)
        .ok_or("openFDA source")?;
    for keyed in [false, true] {
        if spec.quota.interval(keyed) != std::time::Duration::from_millis(250) {
            return Err("openFDA must honor its documented 240/minute limit".into());
        }
    }
    let env = Environment::new().map_err(|e| e.to_string())?;
    let stub = Stub::for_source_status(
        "api.fda.gov",
        "application/json",
        |_| Ok(json!({"error":{"code":"NOT_FOUND","message":"No matches found!"}}).to_string()),
        |_| 404,
    )?;
    let ca = env.root().join("ca.pem");
    stub.retain(&env);
    fs::write(&ca, &stub.certificate).map_err(|e| e.to_string())?;
    let mut config = json!({"proxy":format!("http://{}", stub.address),"ca_bundle":ca});
    if mode == 2 {
        config
            .as_object_mut()
            .ok_or("config")?
            .insert("keys".into(), json!({"openfda_api_key":KEY}));
    }
    env.config(&config).map_err(|e| e.to_string())?;
    let directory = env.root().join("recordings");
    let args = ["get", entity, id, "--source", source];
    invoke(&env, &args, "--record", &directory, mode)?;
    let requests = stub.finish()?;
    let expected = if source == "openfda-label" { 2 } else { 1 };
    if requests.len() != expected {
        return Err(format!(
            "expected {expected} openFDA requests, got {}",
            requests.len()
        ));
    }
    for (index, (_, request)) in requests.iter().enumerate() {
        request_key(request, mode)?;
        if source == "openfda-label" {
            let target = request
                .lines()
                .next()
                .and_then(|line| line.split_whitespace().nth(1))
                .ok_or("openFDA request target")?;
            let url = url::Url::parse(&format!("https://api.fda.gov{target}"))
                .map_err(|error| error.to_string())?;
            let field = if index == 0 {
                "openfda.spl_set_id"
            } else {
                "set_id"
            };
            if !url
                .query_pairs()
                .any(|(key, value)| key == "search" && value == format!("{field}:\"{id}\""))
            {
                return Err("SPL identity fallback changed its exact selector".into());
            }
        }
    }
    for file in fs::read_dir(directory.join(source)).map_err(|e| e.to_string())? {
        let bytes = fs::read(file.map_err(|e| e.to_string())?.path()).map_err(|e| e.to_string())?;
        secret_free(&bytes)?;
    }
    invoke(&env, &args, "--replay", &directory, mode)?;
    Ok(())
}
fn request_key(request: &str, mode: u8) -> Check {
    let line = request.lines().next().ok_or("request line")?;
    let expected = match mode {
        0 => None,
        1 => Some(ENV_KEY),
        _ => Some(KEY),
    };
    if let Some(key) = expected {
        if !line.contains(&format!("?api_key={key}&")) || request.contains("Authorization:") {
            return Err("optional openFDA key missing from documented query location".into());
        }
    } else if request.contains("api_key") || request.contains(ENV_KEY) || request.contains(KEY) {
        return Err("keyless openFDA request carried a credential".into());
    }
    Ok(())
}

fn invoke(env: &Environment, args: &[&str], flag: &str, directory: &Path, mode: u8) -> Check {
    let args = args
        .iter()
        .map(|s| (*s).to_owned())
        .chain([flag.into(), directory.to_string_lossy().into_owned()])
        .collect::<Vec<_>>();
    let mut command = env.command(env!("CARGO_BIN_EXE_biomcp"), &args);
    if mode > 0 {
        command.env("OPENFDA_API_KEY", ENV_KEY);
    }
    let output = command.output().map_err(|e| e.to_string())?;
    secret_free(&output.stdout)?;
    secret_free(&output.stderr)?;
    let value: Value = serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;
    if output.status.code() != Some(1) || value.get("status") != Some(&json!("absent")) {
        return Err(format!("openFDA absence failed: {value}"));
    }
    Ok(())
}
fn secret_free(bytes: &[u8]) -> Check {
    for key in [KEY, ENV_KEY] {
        if bytes.windows(key.len()).any(|s| s == key.as_bytes()) {
            return Err("openFDA key escaped containment".into());
        }
    }
    Ok(())
}
