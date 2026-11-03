//! First readers wait for an admitted publisher; no fixture sleeps release it.
use super::{configuration::invoke, launch::Environment, network_stub::Stub};
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::{fs, process::Stdio, sync::mpsc, time::Duration};

pub(super) fn trials() -> Vec<Trial> {
    vec![
        super::transcript::trial("gene-sections/gencc-first-publisher", || {
            first_reader(false).map_err(Into::into)
        }),
        super::transcript::trial("gene-sections/gencc-first-publisher-deadline", || {
            first_reader(true).map_err(Into::into)
        }),
    ]
}
fn first_reader(expires: bool) -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let replay = super::launch::fixtures().join("replies/gene/gencc");
    let cache = env.root().join("case-cache").to_string_lossy().into_owned();
    env.config(&json!({"disable_sources":["oncokb"]}))
        .map_err(|e| e.to_string())?;
    invoke(
        &env,
        vec![
            "get".into(),
            "gene".into(),
            "ODC1".into(),
            "--replay".into(),
            replay.to_string_lossy().into(),
            "--case-cache".into(),
            cache.clone(),
        ],
        0,
    )?;
    let body = native_body(&replay)?;
    let (admitted, admission) = mpsc::channel();
    let (publish, publication) = mpsc::channel();
    let peer = Stub::for_source("thegencc.org", "text/csv", move |_| {
        admitted.send(()).map_err(|e| e.to_string())?;
        publication
            .recv_timeout(Duration::from_secs(30))
            .map_err(|e| e.to_string())?;
        Ok(body.clone())
    })?;
    peer.retain(env.shared_root());
    let ca = env.root().join("ca.pem");
    fs::write(&ca, &peer.certificate).map_err(|e| e.to_string())?;
    let mut config = json!({"proxy":format!("http://{}",peer.address),"ca_bundle":ca,
        "disable_sources":["oncokb"]});
    if expires {
        config
            .as_object_mut()
            .ok_or("config map")?
            .insert("command_timeout_seconds".into(), json!(1));
    }
    env.config(&config).map_err(|e| e.to_string())?;
    let mut command = env.command(
        env!("CARGO_BIN_EXE_biomcp"),
        &[
            "sync".into(),
            "gencc".into(),
            "--case-cache".into(),
            cache.clone(),
        ],
    );
    let child = command
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| e.to_string())?;
    admission
        .recv_timeout(Duration::from_secs(15))
        .map_err(|e| e.to_string())?;
    let result = reader(&env, cache, publish, expires);
    let output = command.wait_with_output(child).map_err(|e| e.to_string())?;
    if !output.status.success() {
        return Err(format!(
            "publisher failed: {}",
            String::from_utf8_lossy(&output.stdout)
        ));
    }
    peer.finish()?;
    result
}

fn native_body(replay: &std::path::Path) -> Result<String, String> {
    let recording = fs::read_dir(replay.join("gencc"))
        .map_err(|e| e.to_string())?
        .next()
        .ok_or("GenCC recording missing")?
        .map_err(|e| e.to_string())?
        .path();
    let record: Value = serde_json::from_slice(&fs::read(recording).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    record
        .get("exchanges")
        .and_then(Value::as_array)
        .ok_or("GenCC exchanges")?
        .iter()
        .find(|exchange| exchange.get("status") == Some(&json!(200)))
        .and_then(|exchange| exchange.pointer("/body/value"))
        .and_then(Value::as_str)
        .map(str::to_owned)
        .ok_or_else(|| "GenCC CSV".into())
}

fn reader(
    env: &Environment,
    cache: String,
    publish: mpsc::Sender<()>,
    expires: bool,
) -> Result<(), String> {
    std::thread::scope(|scope| {
        let (done, completion) = mpsc::channel();
        let environment = env;
        scope.spawn(move || {
            let result = invoke(
                environment,
                vec![
                    "get".into(),
                    "gene".into(),
                    "ODC1".into(),
                    "gencc".into(),
                    "--case-cache".into(),
                    cache,
                ],
                0,
            );
            let _ = done.send(result);
        });
        // Await completion while publication is held. A premature refusal is
        // retained, then the actual response event releases the publisher.
        let early = completion.recv_timeout(if expires {
            Duration::from_secs(3)
        } else {
            Duration::from_millis(500)
        });
        publish.send(()).map_err(|e| e.to_string())?;
        let value = match early {
            Ok(result) => result?,
            Err(mpsc::RecvTimeoutError::Timeout) if !expires => completion
                .recv_timeout(Duration::from_secs(15))
                .map_err(|e| e.to_string())??,
            Err(error) => return Err(error.to_string()),
        };
        let valid = if expires {
            value
                .get("sources")
                .and_then(Value::as_array)
                .is_some_and(|sources| {
                    sources.iter().any(|source| {
                        source.get("source") == Some(&json!("gencc"))
                            && source.get("reason") == Some(&json!("syncing"))
                    })
                })
        } else {
            value.pointer("/result/items/0/sections/gencc/state") == Some(&json!("data"))
        };
        if !valid {
            return Err(format!(
                "first reader did not respect publication and deadline: {value}"
            ));
        }
        Ok(())
    })
}
