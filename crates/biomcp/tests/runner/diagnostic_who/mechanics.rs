use super::{Check, Environment, call, seed};
use serde_json::{Value, json};
use std::{fs, sync::mpsc, time::Duration};
fn body(replay: &std::path::Path) -> Result<String, String> {
    let path = fs::read_dir(replay.join("who-ivd"))
        .map_err(|e| e.to_string())?
        .next()
        .ok_or("missing recording")?
        .map_err(|e| e.to_string())?
        .path();
    let value: Value = serde_json::from_slice(&fs::read(path).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    value
        .pointer("/exchanges/0/body/value")
        .and_then(Value::as_str)
        .map(str::to_owned)
        .ok_or("missing CSV".into())
}
pub(super) fn concurrent() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let replay = seed(&env)?;
    let args = ["get", "diagnostic", "0466-046-00", "--source", "who-ivd"];
    let original = body(&replay)?;
    let replacement = original.replace("CODE-8", "REPLACED-8");
    let (admitted, receiving) = mpsc::channel();
    let (release, released) = mpsc::channel();
    let mut calls = 0;
    let stub = super::super::network_stub::Stub::for_sources(
        &["extranet.who.int"],
        |_| "text/csv",
        move |_| {
            calls += 1;
            if calls == 1 {
                return Ok(original.clone());
            }
            admitted.send(()).map_err(|e| e.to_string())?;
            released
                .recv_timeout(Duration::from_secs(20))
                .map_err(|e| e.to_string())?;
            Ok(replacement.clone())
        },
    )?;
    let cert = env.root().join("authority.pem");
    stub.retain(&env);
    fs::write(&cert, &stub.certificate).map_err(|e| e.to_string())?;
    env.config(&json!({"proxy":format!("http://{}",stub.address),"ca_bundle":cert}))
        .map_err(|e| e.to_string())?;
    let plain = |args: &[&str]| super::invoke(&env, args.iter().map(|s| (*s).into()).collect(), 0);
    plain(&["sync", "who-ivd"])?;
    let before = plain(&args)?;
    std::thread::scope(|scope| -> Check {
        let sync = scope.spawn(|| plain(&["sync", "who-ivd"]));
        receiving
            .recv_timeout(Duration::from_secs(20))
            .map_err(|e| e.to_string())?;
        let readers = scope.spawn(|| plain(&args));
        let other = plain(&args);
        let first = readers.join().map_err(|_| "reader panicked")?;
        release.send(()).map_err(|e| e.to_string())?;
        sync.join().map_err(|_| "sync panicked")??;
        if first? != before || other? != before {
            return Err("reader observed partial replacement".into());
        }
        Ok(())
    })?;
    let after = plain(&args)?;
    if after.pointer("/result/native_rows/8/product_code") != Some(&json!("REPLACED-8"))
        || stub.finish()?.len() != 2
    {
        return Err("replacement failed or a stored read made a request".into());
    }
    Ok(())
}
pub(super) fn markdown() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let replay = seed(&env)?;
    call(&env, &replay, &["sync", "who-ivd"], 0)?;
    let mut args = env.globals(Some(&replay), false);
    args.extend(
        [
            "get",
            "diagnostic",
            "0466-046-00",
            "--source",
            "who-ivd",
            "--format",
            "md",
        ]
        .map(str::to_owned),
    );
    let output = env
        .command(env!("CARGO_BIN_EXE_biomcp"), &args)
        .output()
        .map_err(|e| e.to_string())?;
    // Full synthetic Markdown stdout measured by j13400, plus fifteen percent.
    if output.stdout.len() > 1972 {
        return Err("diagnostic Markdown exceeds its measured ceiling".into());
    }
    let text = String::from_utf8(output.stdout).map_err(|e| e.to_string())?;
    if !output.status.success() || !output.stderr.is_empty() {
        return Err("WHO Markdown failed".into());
    }
    for expected in [
        "0466-046-00",
        "Synthetic",
        "CODE-8",
        "US FDA text without application",
        "2024",
        "WHO terms of use",
    ] {
        if !text.contains(expected) {
            return Err(format!("WHO Markdown omitted {expected}"));
        }
    }
    Ok(())
}
