//! Corporate network settings cross the isolated command boundary.
use super::launch::Environment;
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::{fs, net::TcpListener};

pub(super) fn trials() -> Vec<Trial> {
    vec![
        Trial::test("network/proxy-environment-failure", || proxy().map_err(Into::into)),
        Trial::test("network/bad-ca-is-local", || bad_ca().map_err(Into::into)),
    ]
}
fn proxy() -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let socket = TcpListener::bind("127.0.0.1:0").map_err(|e| e.to_string())?;
    let address = socket.local_addr().map_err(|e| e.to_string())?;
    drop(socket);
    let output = env.command(env!("CARGO_BIN_EXE_biomcp"), &["sync".into(), "hgnc".into()])
        .env("HTTPS_PROXY", format!("http://user:synthetic-private@{address}"))
        .output().map_err(|e| e.to_string())?;
    let value: Value = serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;
    if value.to_string().contains("synthetic-private") || !output.stderr.is_empty() {
        return Err("proxy credential leaked".into());
    }
    if value.pointer("/sources/0/reason") != Some(&json!("proxy")) {
        return Err(format!("expected proxy failure: {value}"));
    }
    if !value.to_string().contains("proxy") {
        return Err("missing proxy fix guidance".into());
    }
    Ok(())
}
fn bad_ca() -> Result<(), String> {
    for bytes in [b"".as_slice(), b"not a certificate".as_slice()] {
        let env = Environment::new().map_err(|e| e.to_string())?;
        let path = env.root().join("synthetic-private.pem");
        fs::write(&path, bytes).map_err(|e| e.to_string())?;
        env.config(&json!({"ca_bundle":path})).map_err(|e| e.to_string())?;
        let output = env.command(env!("CARGO_BIN_EXE_biomcp"), &["get".into(), "gene".into(), "BRAF".into(), "--offline".into()])
            .output().map_err(|e| e.to_string())?;
        let value: Value = serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;
        if value.pointer("/error/code") != Some(&json!("local.failure")) || value.to_string().contains("synthetic-private") {
            return Err(format!("bad bundle was not a private local failure: {value}"));
        }
    }
    Ok(())
}
