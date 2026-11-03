//! First readers wait for an admitted publisher; no fixture sleeps release it.
use super::{configuration::invoke, launch::Environment, network_stub::Stub};
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::{fs, process::Stdio, sync::mpsc, time::Duration};

pub(super) fn trials() -> Vec<Trial> {
    vec![super::transcript::trial(
        "gene-sections/gencc-first-publisher",
        || first_reader().map_err(Into::into),
    )]
}
fn first_reader() -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let replay = super::launch::fixtures().join("replies/gene/gencc");
    let cache = env.root().join("case-cache").to_string_lossy().into_owned();
    env.config(&json!({"disable_sources":["oncokb"]}))
        .map_err(|e| e.to_string())?;
    invoke(
        &env,
        vec![
            "sync".into(),
            "hgnc".into(),
            "--replay".into(),
            replay.to_string_lossy().into(),
            "--case-cache".into(),
            cache.clone(),
        ],
        0,
    )?;
    let recording = fs::read_dir(replay.join("gencc"))
        .map_err(|e| e.to_string())?
        .next()
        .ok_or("GenCC recording missing")?
        .map_err(|e| e.to_string())?
        .path();
    let record: Value = serde_json::from_slice(&fs::read(recording).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    let body = record
        .get("exchanges")
        .and_then(Value::as_array)
        .ok_or("GenCC exchanges")?
        .iter()
        .find(|exchange| exchange.get("status") == Some(&json!(200)))
        .and_then(|exchange| exchange.pointer("/body/value"))
        .and_then(Value::as_str)
        .ok_or("GenCC CSV")?
        .to_owned();
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
    env.config(
        &json!({"proxy":format!("http://{}",peer.address),"ca_bundle":ca,
        "disable_sources":["oncokb"]}),
    )
    .map_err(|e| e.to_string())?;
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
    let result = std::thread::scope(|scope| {
        let (done, completion) = mpsc::channel();
        let environment = &env;
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
        let early = completion.recv_timeout(Duration::from_millis(500));
        publish.send(()).map_err(|e| e.to_string())?;
        let value = match early {
            Ok(result) => result?,
            Err(mpsc::RecvTimeoutError::Timeout) => completion
                .recv_timeout(Duration::from_secs(15))
                .map_err(|e| e.to_string())??,
            Err(error) => return Err(error.to_string()),
        };
        if value.pointer("/result/items/0/sections/gencc/state") != Some(&json!("data")) {
            return Err(format!("first reader did not await publication: {value}"));
        }
        Ok(())
    });
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
