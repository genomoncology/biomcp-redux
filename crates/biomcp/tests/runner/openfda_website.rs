//! Feature key-registration: the public FDA companion website receives no API key.
use super::{
    launch::{Environment, fixtures},
    network_stub::Stub,
};
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::fs;
pub(super) fn trial() -> Trial {
    Trial::test("keys/openfda/companion-website-has-no-key", || {
        check().map_err(Into::into)
    })
}
fn check() -> Result<(), String> {
    let source = fixtures().join("replies/diagnostic/fda-cdx-synthetic/fda-cdx");
    let entry = fs::read_dir(source)
        .map_err(|e| e.to_string())?
        .next()
        .ok_or("companion fixture")?
        .map_err(|e| e.to_string())?;
    let record: Value = serde_json::from_slice(&fs::read(entry.path()).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    let body = record
        .pointer("/exchanges/0/body/value")
        .and_then(Value::as_str)
        .ok_or("companion HTML")?
        .to_owned();
    let stub = Stub::for_sources(&["www.fda.gov"], |_| "text/html", move |_| Ok(body.clone()))?;
    let env = Environment::new().map_err(|e| e.to_string())?;
    let ca = env.root().join("ca.pem");
    fs::write(&ca, &stub.certificate).map_err(|e| e.to_string())?;
    env.config(&json!({"proxy":format!("http://{}", stub.address),"ca_bundle":ca,"keys":{"openfda_api_key":"synthetic-website-key"}})).map_err(|e|e.to_string())?;
    let output = env
        .command(
            env!("CARGO_BIN_EXE_biomcp"),
            &["sync".into(), "fda-cdx".into()],
        )
        .env("OPENFDA_API_KEY", "synthetic-website-environment")
        .output()
        .map_err(|e| e.to_string())?;
    if !output.status.success() {
        return Err("companion sync failed".into());
    }
    let requests = stub.finish()?;
    let [(_, request)] = requests.as_slice() else {
        return Err("companion request count".into());
    };
    for marker in ["api_key", "synthetic-website", "Authorization:"] {
        if request.contains(marker) {
            return Err("openFDA credential reached FDA website".into());
        }
    }
    Ok(())
}

pub(super) fn help_trial() -> Trial {
    Trial::test("keys/openfda/configuration-help", || {
        let env = Environment::new().map_err(|e| e.to_string())?;
        let output = env
            .command(
                env!("CARGO_BIN_EXE_biomcp"),
                &["help".into(), "key".into(), "openfda".into()],
            )
            .output()
            .map_err(|e| e.to_string())?;
        let value: Value = serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;
        if !output.status.success()
            || value.pointer("/result/config") != Some(&json!("keys.openfda_api_key"))
            || value.pointer("/result/environment") != Some(&json!("OPENFDA_API_KEY"))
            || value.pointer("/result/guidance") != Some(&json!("biomcp key set openfda"))
        {
            return Err("openFDA key help omitted registration or names".into());
        }
        Ok(())
    })
}
