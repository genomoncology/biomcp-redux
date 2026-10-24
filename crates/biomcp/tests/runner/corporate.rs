//! Corporate network settings cross the isolated command boundary.
use super::launch::Environment;
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::{fs, net::TcpListener};

pub(super) fn trials() -> Vec<Trial> {
    vec![
        Trial::test("network/proxy-environment-failure", || proxy().map_err(Into::into)),
        Trial::test("network/bad-ca-is-local", || bad_ca().map_err(Into::into)),
        Trial::test("network/additive-ca-tls-and-proxy-isolation", || tls().map_err(Into::into)),
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
    if value.pointer("/next/0/args/2") != Some(&json!("proxy")) {
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

fn tls() -> Result<(), String> {
    for trusted in [false, true] {
        let env = Environment::new().map_err(|e| e.to_string())?;
        let stub = super::network_stub::Stub::new()?;
        let record = env.root().join("capture");
        if trusted {
            let cert = env.root().join("private-ca.pem");
            fs::write(&cert, super::network_stub::CERT).map_err(|e| e.to_string())?;
            env.config(&json!({"ca_bundle":cert})).map_err(|e| e.to_string())?;
        }
        let output = env.command(env!("CARGO_BIN_EXE_biomcp"), &["sync".into(), "hgnc".into(), "--record".into(), record.to_string_lossy().into()])
            .env("HTTPS_PROXY", format!("http://user:synthetic-private@{}", stub.address))
            .output().map_err(|e| e.to_string())?;
        let value: Value = serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;
        let requests = stub.finish()?;
        if trusted {
            if !output.status.success() || requests.len() != 2 || value.pointer("/result/rows") != Some(&json!(176)) {
                return Err(format!("additive CA failed: {value}; requests {}", requests.len()));
            }
            for (connect, target) in requests {
                if !connect.contains("Proxy-Authorization: Basic ") || target.to_ascii_lowercase().contains("proxy-authorization") {
                    return Err("proxy authorization crossed the tunnel boundary".into());
                }
            }
            for file in fs::read_dir(record.join("hgnc")).map_err(|e| e.to_string())? {
                let text = fs::read_to_string(file.map_err(|e| e.to_string())?.path()).map_err(|e| e.to_string())?;
                if text.contains("synthetic-private") || text.contains("private-ca.pem") || text.to_ascii_lowercase().contains("proxy-authorization") {
                    return Err("private network settings entered recordings".into());
                }
            }
        } else {
            if value.pointer("/sources/0/reason") != Some(&json!("tls")) || !requests.is_empty() {
                return Err(format!("certificate failure lost its class: {value}"));
            }
            if value.pointer("/next/0/args/2") != Some(&json!("ca_bundle")) { return Err("TLS fix lost ca_bundle".into()); }
        }
        if !output.stderr.is_empty() || value.to_string().contains("synthetic-private") { return Err("network diagnostics leaked".into()); }
    }
    Ok(())
}
