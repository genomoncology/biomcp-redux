//! A real TLS peer checks trust addition and proxy credentials across the CONNECT tunnel.
use super::launch::Environment;
use crate::runner::transcript::trial as transcript_trial;
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::fs;
pub(super) fn trials() -> Vec<Trial> {
    vec![transcript_trial(
        "corporate-network/additive-ca-tls-and-proxy-isolation",
        || tls().map_err(Into::into),
    )]
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
