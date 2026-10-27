//! Corporate network settings cross the isolated command boundary.
use super::launch::Environment;
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::{fs, net::TcpListener};

pub(super) fn trials() -> Vec<Trial> {
    vec![
        Trial::test("network/configuration-causes-and-automatic-sync-limit", || {
            configuration().map_err(Into::into)
        }),
        Trial::test("network/proxy-environment-failure", || {
            proxy().map_err(Into::into)
        }),
        Trial::test("network/bad-ca-is-local", || bad_ca().map_err(Into::into)),
        Trial::test("network/additive-ca-tls-and-proxy-isolation", || {
            tls().map_err(Into::into)
        }),
    ]
}
fn proxy() -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let socket = TcpListener::bind("127.0.0.1:0").map_err(|e| e.to_string())?;
    let address = socket.local_addr().map_err(|e| e.to_string())?;
    drop(socket);
    let args = ["sync".into(), "hgnc".into()];
    let proxy = format!("http://user:synthetic-private@{address}");
    let output = env
        .command(env!("CARGO_BIN_EXE_biomcp"), &args)
        .env("HTTPS_PROXY", &proxy)
        .output()
        .map_err(|e| e.to_string())?;
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
    env.config(&json!({"proxy":proxy}))
        .map_err(|e| e.to_string())?;
    let output = env
        .command(env!("CARGO_BIN_EXE_biomcp"), &args)
        .env("HTTPS_PROXY", "malformed-env-proxy")
        .env("HTTP_PROXY", "malformed-env-proxy")
        .env("NO_PROXY", "*")
        .output()
        .map_err(|e| e.to_string())?;
    let value: Value = serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;
    if value.pointer("/sources/0/reason") != Some(&json!("proxy")) {
        return Err("file proxy did not override environment".into());
    }
    let help = env
        .command(
            env!("CARGO_BIN_EXE_biomcp"),
            &["help".into(), "network".into(), "proxy".into()],
        )
        .output()
        .map_err(|e| e.to_string())?;
    if !help.status.success() || !String::from_utf8_lossy(&help.stdout).contains("HTTPS_PROXY") {
        return Err("proxy fix action is unusable".into());
    }
    Ok(())
}
fn bad_ca() -> Result<(), String> {
    for bytes in [b"".as_slice(), b"not a certificate".as_slice()] {
        let env = Environment::new().map_err(|e| e.to_string())?;
        let path = env.root().join("synthetic-private.pem");
        fs::write(&path, bytes).map_err(|e| e.to_string())?;
        env.config(&json!({"ca_bundle":path}))
            .map_err(|e| e.to_string())?;
        let output = env
            .command(
                env!("CARGO_BIN_EXE_biomcp"),
                &[
                    "get".into(),
                    "gene".into(),
                    "BRAF".into(),
                    "--offline".into(),
                ],
            )
            .output()
            .map_err(|e| e.to_string())?;
        let value: Value = serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;
        if value.pointer("/error/code") != Some(&json!("local.failure"))
            || value.to_string().contains("synthetic-private")
        {
            return Err(format!(
                "bad bundle was not a private local failure: {value}"
            ));
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
            fs::write(&cert, &stub.certificate).map_err(|e| e.to_string())?;
            env.config(&json!({"ca_bundle":cert}))
                .map_err(|e| e.to_string())?;
        }
        let output = env
            .command(
                env!("CARGO_BIN_EXE_biomcp"),
                &[
                    "sync".into(),
                    "hgnc".into(),
                    "--record".into(),
                    record.to_string_lossy().into(),
                ],
            )
            .env(
                "HTTPS_PROXY",
                format!("http://user:synthetic-private@{}", stub.address),
            )
            .output()
            .map_err(|e| e.to_string())?;
        let value: Value = serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;
        let requests = stub.finish()?;
        if trusted {
            if !output.status.success()
                || requests.len() != 2
                || value.pointer("/result/rows") != Some(&json!(176))
            {
                return Err(format!(
                    "additive CA failed: {value}; requests {}",
                    requests.len()
                ));
            }
            for (connect, target) in requests {
                if !connect.contains("Proxy-Authorization: Basic ")
                    || target.to_ascii_lowercase().contains("proxy-authorization")
                {
                    return Err("proxy authorization crossed the tunnel boundary".into());
                }
            }
            for file in fs::read_dir(record.join("hgnc")).map_err(|e| e.to_string())? {
                let text = fs::read_to_string(file.map_err(|e| e.to_string())?.path())
                    .map_err(|e| e.to_string())?;
                if text.contains("synthetic-private")
                    || text.contains("private-ca.pem")
                    || text.to_ascii_lowercase().contains("proxy-authorization")
                {
                    return Err("private network settings entered recordings".into());
                }
            }
        } else {
            if value.pointer("/sources/0/reason") != Some(&json!("tls")) || !requests.is_empty() {
                return Err(format!("certificate failure lost its class: {value}"));
            }
            if value.pointer("/next/0/args/2") != Some(&json!("ca_bundle")) {
                return Err("TLS fix lost ca_bundle".into());
            }
        }
        if !output.stderr.is_empty() || value.to_string().contains("synthetic-private") {
            return Err("network diagnostics leaked".into());
        }
    }
    Ok(())
}

fn configuration() -> Result<(), String> {
    for rule in ["::1", "[::1]:8080", "10.0.0.0/8", "fd00::/8"] {
        let env = Environment::new().map_err(|e| e.to_string())?;
        let output = env.command(env!("CARGO_BIN_EXE_biomcp"),
            &["health".into(), "--offline".into()]).env("NO_PROXY", rule)
            .output().map_err(|e| e.to_string())?;
        if !output.status.success() || !output.stderr.is_empty() {
            return Err(format!("supported bypass prevented offline command: {rule}"));
        }
    }
    for (config, field) in [
        (json!({"formatt":"synthetic-private"}), "formatt"),
        (json!({"proxy":"http://[synthetic-private"}), "proxy"),
        (json!({"keys":{"oncokb_api_key":9}}), "keys.oncokb_api_key"),
        (json!({"auto_sync_max_bytes":"synthetic-private"}), "auto_sync_max_bytes"),
        (json!({"http":{"allowed_hosts":9}}), "http.allowed_hosts"),
    ] {
        let env = Environment::new().map_err(|e| e.to_string())?;
        env.config(&config).map_err(|e| e.to_string())?;
        let value = super::configuration::invoke(&env,
            vec!["health".into(), "--offline".into()], 5)?;
        if value.pointer("/error/option") != Some(&json!(field))
            || value.to_string().contains("synthetic-private") {
            return Err(format!("configuration failure lost safe field {field}: {value}"));
        }
    }
    for (variable, value) in [("HTTP_PROXY", "http://[synthetic-private"),
        ("HTTPS_PROXY", "http://[synthetic-private"),
        ("NO_PROXY", "user:synthetic-private@example.org"),
        ("NO_PROXY", "example.org:abc")] {
        let env = Environment::new().map_err(|e| e.to_string())?;
        let output = env.command(env!("CARGO_BIN_EXE_biomcp"),
            &["health".into(), "--offline".into()]).env(variable, value)
            .output().map_err(|e| e.to_string())?;
        let value: Value = serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;
        if output.status.code() != Some(5) || !output.stderr.is_empty()
            || value.pointer("/error/option") != Some(&json!(variable))
            || value.to_string().contains("synthetic-private") {
            return Err(format!("configuration failure lost safe variable {variable}: {value}"));
        }
    }
    for limit in [1, 1_000_000] {
        let env = Environment::new().map_err(|e| e.to_string())?;
        let stub = super::network_stub::Stub::new()?;
        let ca = env.root().join("ca.pem");
        fs::write(&ca, &stub.certificate).map_err(|e| e.to_string())?;
        env.config(&json!({"proxy":format!("http://{}",stub.address), "ca_bundle":ca,
            "auto_sync_max_bytes":limit})).map_err(|e| e.to_string())?;
        let value = super::configuration::invoke(&env,
            vec!["get".into(), "gene".into(), "BRAF".into()], if limit == 1 {4} else {0})?;
        if (limit == 1 && value.pointer("/sources/0/reason") != Some(&json!("syncing")))
            || (limit != 1 && value.pointer("/result/items/0/answer/gene/symbol") != Some(&json!("BRAF"))) {
            return Err(format!("automatic sync did not apply configured byte limit: {value}"));
        }
        stub.finish()?;
    }
    Ok(())
}
