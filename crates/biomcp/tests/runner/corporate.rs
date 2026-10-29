//! Corporate network settings cross the isolated command boundary.
use super::launch::Environment;
use crate::runner::transcript::trial as transcript_trial;
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::{fs, net::TcpListener};

pub(super) fn trials() -> Vec<Trial> {
    vec![
        transcript_trial(
            "network/configuration-causes-and-automatic-sync-limit",
            || configuration().map_err(Into::into),
        ),
        transcript_trial("network/proxy-environment-failure", || {
            proxy().map_err(Into::into)
        }),
        transcript_trial("network/bad-ca-is-local", || bad_ca().map_err(Into::into)),
        transcript_trial("network/additive-ca-tls-and-proxy-isolation", || {
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
        stub.retain(&env);
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
    offline_bypass()?;
    named_fields()?;
    named_environment()?;
    invalid_utf8_environment()?;
    duplicate_field_cause()?;
    named_server_and_key_causes()?;
    skipped_bypass()?;
    automatic_sync()
}

fn offline_bypass() -> Result<(), String> {
    for rule in ["::1", "[::1]:8080", "10.0.0.0/8", "fd00::/8"] {
        let env = Environment::new().map_err(|e| e.to_string())?;
        let output = env
            .command(
                env!("CARGO_BIN_EXE_biomcp"),
                &["health".into(), "--offline".into()],
            )
            .env("NO_PROXY", rule)
            .output()
            .map_err(|e| e.to_string())?;
        if !output.status.success() || !output.stderr.is_empty() {
            return Err(format!(
                "supported bypass prevented offline command: {rule}"
            ));
        }
    }
    Ok(())
}

fn named_fields() -> Result<(), String> {
    for (config, field) in [
        (json!({"formatt":"synthetic-private"}), "formatt"),
        (json!({"":"synthetic-private"}), "config.json"),
        (json!({"proxy":"http://[synthetic-private"}), "proxy"),
        (json!({"keys":{"oncokb_api_key":9}}), "keys.oncokb_api_key"),
        (
            json!({"auto_sync_max_bytes":"synthetic-private"}),
            "auto_sync_max_bytes",
        ),
        (json!({"http":{"allowed_hosts":9}}), "http.allowed_hosts"),
    ] {
        let env = Environment::new().map_err(|e| e.to_string())?;
        env.config(&config).map_err(|e| e.to_string())?;
        let value =
            super::configuration::invoke(&env, vec!["health".into(), "--offline".into()], 5)?;
        if value.pointer("/error/option") != Some(&json!(field))
            || value.to_string().contains("synthetic-private")
        {
            return Err(format!(
                "configuration failure lost safe field {field}: {value}"
            ));
        }
    }
    Ok(())
}

fn named_environment() -> Result<(), String> {
    for (variable, value) in [
        ("HTTP_PROXY", "http://[synthetic-private"),
        ("HTTPS_PROXY", "http://[synthetic-private"),
        ("NO_PROXY", "user:synthetic-private@example.org"),
        ("NO_PROXY", "example.org:abc"),
    ] {
        let env = Environment::new().map_err(|e| e.to_string())?;
        let output = env
            .command(
                env!("CARGO_BIN_EXE_biomcp"),
                &["health".into(), "--offline".into()],
            )
            .env(variable, value)
            .output()
            .map_err(|e| e.to_string())?;
        let value: Value = serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;
        if output.status.code() != Some(5)
            || !output.stderr.is_empty()
            || value.pointer("/error/option") != Some(&json!(variable))
            || value.to_string().contains("synthetic-private")
        {
            return Err(format!(
                "configuration failure lost safe variable {variable}: {value}"
            ));
        }
    }
    Ok(())
}

fn automatic_sync() -> Result<(), String> {
    for limit in [1, 1_000_000] {
        let env = Environment::new().map_err(|e| e.to_string())?;
        let stub = super::network_stub::Stub::new()?;
        let ca = env.root().join("ca.pem");
        stub.retain(&env);
        fs::write(&ca, &stub.certificate).map_err(|e| e.to_string())?;
        env.config(
            &json!({"proxy":format!("http://{}",stub.address), "ca_bundle":ca,
            "auto_sync_max_bytes":limit}),
        )
        .map_err(|e| e.to_string())?;
        let value = super::configuration::invoke(
            &env,
            vec!["search".into(), "gene".into(), "BRAF".into()],
            if limit == 1 { 4 } else { 0 },
        )?;
        if (limit == 1 && value.pointer("/sources/0/reason") != Some(&json!("syncing")))
            || (limit != 1 && value.pointer("/result/items/0/symbol") != Some(&json!("BRAF")))
        {
            return Err(format!(
                "automatic sync did not apply configured byte limit: {value}"
            ));
        }
        stub.finish()?;
    }
    Ok(())
}

fn skipped_bypass() -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let rules = "127.0.0.1,synthetic-private! , ,bad/entry";
    let health = |rule: &str| -> Result<Value, String> {
        let output = env
            .command(
                env!("CARGO_BIN_EXE_biomcp"),
                &["health".into(), "--offline".into()],
            )
            .env("NO_PROXY", rule)
            .output()
            .map_err(|e| e.to_string())?;
        if !output.status.success() || !output.stderr.is_empty() {
            return Err("skipped bypass stopped health".into());
        }
        serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())
    };
    let value = health(rules)?;
    if value.pointer("/result/configuration")
        != Some(&json!({"notices":[
        {"variable":"NO_PROXY", "entry":2, "code":"unparseable-entry"},
        {"variable":"NO_PROXY", "entry":4, "code":"unparseable-entry"}], "omitted_entries":0}))
        || value.to_string().contains("synthetic-private")
    {
        return Err(format!("missing bounded notice: {value}"));
    }
    let many = ["synthetic-private!"; 12].join(",");
    let value = health(&many)?;
    if value
        .pointer("/result/configuration/notices")
        .and_then(Value::as_array)
        .map(Vec::len)
        != Some(8)
        || value.pointer("/result/configuration/omitted_entries") != Some(&json!(4))
    {
        return Err("configuration notices were not bounded".into());
    }
    let args = [
        "get".into(),
        "gene".into(),
        "BRAF".into(),
        "--replay".into(),
        super::configuration::root().to_string_lossy().into(),
    ];
    let output = env
        .command(env!("CARGO_BIN_EXE_biomcp"), &args)
        .env("NO_PROXY", rules)
        .output()
        .map_err(|e| e.to_string())?;
    if !output.status.success()
        || !output.stderr.is_empty()
        || String::from_utf8_lossy(&output.stdout).contains("configuration")
    {
        return Err("ordinary command exposed configuration notices".into());
    }
    Ok(())
}

fn named_server_and_key_causes() -> Result<(), String> {
    for (config, args, field) in [
        (
            json!({"formatt":"synthetic-private"}),
            vec!["key", "list"],
            "formatt",
        ),
        (
            json!({"http":{"allowed_hosts":["synthetic-private/path"]}}),
            vec!["serve", "--http", "127.0.0.1:0"],
            "http.allowed_hosts",
        ),
    ] {
        let env = Environment::new().map_err(|e| e.to_string())?;
        env.config(&config).map_err(|e| e.to_string())?;
        let args = args.into_iter().map(str::to_owned).collect();
        let value = super::configuration::invoke(&env, args, 5)?;
        if value.pointer("/error/option") != Some(&json!(field))
            || value.to_string().contains("synthetic-private")
        {
            return Err("server or key command discarded its configuration cause".into());
        }
    }
    Ok(())
}

fn duplicate_field_cause() -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    env.config(&json!({})).map_err(|e| e.to_string())?;
    fs::write(
        env.root().join("config/biomcp/config.json"),
        br#"{"proxy":null,"proxy":"http://synthetic-private"}"#,
    )
    .map_err(|e| e.to_string())?;
    let value = super::configuration::invoke(&env, vec!["health".into(), "--offline".into()], 5)?;
    if value.pointer("/error/option") != Some(&json!("proxy"))
        || value.to_string().contains("synthetic-private")
    {
        return Err("duplicate field lost its safe cause".into());
    }
    Ok(())
}

#[cfg(unix)]
fn invalid_utf8_environment() -> Result<(), String> {
    use std::{ffi::OsString, os::unix::ffi::OsStringExt};
    for (name, args) in [
        ("HOME", vec!["health", "--offline"]),
        ("HTTPS_PROXY", vec!["health", "--offline"]),
        (
            "NCBI_API_KEY",
            vec![
                "search",
                "article",
                "EGFR",
                "--source",
                "pubmed",
                "--offline",
            ],
        ),
        ("NCBI_API_KEY", vec!["key", "list"]),
    ] {
        let env = Environment::new().map_err(|e| e.to_string())?;
        let invalid = OsString::from_vec(b"synthetic-private-\xff-key".to_vec());
        let output = env
            .command(
                env!("CARGO_BIN_EXE_biomcp"),
                &args.into_iter().map(Into::into).collect::<Vec<_>>(),
            )
            .env(name, invalid)
            .output()
            .map_err(|e| e.to_string())?;
        if output.status.success()
            || output.stdout.windows(17).any(|v| v == b"synthetic-private")
            || output.stderr.windows(17).any(|v| v == b"synthetic-private")
        {
            return Err(format!(
                "invalid UTF-8 environment was accepted or printed: {name}"
            ));
        }
    }
    Ok(())
}
#[cfg(not(unix))]
fn invalid_utf8_environment() -> Result<(), String> {
    Ok(())
}
