use super::{call, seed, Check, Environment};
use serde_json::{Value, json};
use std::{fs, sync::mpsc, time::Duration};
fn body(replay: &std::path::Path) -> Result<String, String> {
    let path = fs::read_dir(replay.join("who-ivd")).map_err(|e| e.to_string())?
        .next().ok_or("missing recording")?.map_err(|e| e.to_string())?.path();
    let value: Value = serde_json::from_slice(&fs::read(path).map_err(|e| e.to_string())?).map_err(|e| e.to_string())?;
    value.pointer("/exchanges/0/body/value").and_then(Value::as_str).map(str::to_owned).ok_or("missing CSV".into())
}
pub(super) fn concurrent() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let replay = seed(&env)?;
    call(&env, &replay, &["sync", "who-ivd"], 0)?;
    let args = ["get", "diagnostic", "0466-046-00", "--source", "who-ivd"];
    let before = call(&env, &replay, &args, 0)?;
    let replacement = body(&replay)?.replace("CODE-8", "REPLACED-8");
    let (admitted, receiving) = mpsc::channel();
    let (release, released) = mpsc::channel();
    let stub = super::super::network_stub::Stub::for_sources(&["extranet.who.int"], |_| "text/csv", move |_| {
        admitted.send(()).map_err(|e| e.to_string())?;
        released.recv_timeout(Duration::from_secs(20)).map_err(|e| e.to_string())?;
        Ok(replacement.clone())
    })?;
    let cert = env.root().join("authority.pem");
    fs::write(&cert, &stub.certificate).map_err(|e| e.to_string())?;
    env.config(&json!({"proxy":format!("http://{}",stub.address),"ca_bundle":cert})).map_err(|e| e.to_string())?;
    std::thread::scope(|scope| -> Check {
        let sync = scope.spawn(|| {
            let mut command = vec!["--case-cache".into(), env.root().join("case-cache").to_string_lossy().into_owned()];
            command.extend(["sync".into(), "who-ivd".into()]);
            super::invoke(&env, command, 0)
        });
        receiving.recv_timeout(Duration::from_secs(20)).map_err(|e| e.to_string())?;
        let readers = scope.spawn(|| call(&env, &replay, &args, 0));
        let other = call(&env, &replay, &args, 0);
        let first = readers.join().map_err(|_| "reader panicked")?;
        release.send(()).map_err(|e| e.to_string())?;
        sync.join().map_err(|_| "sync panicked")??;
        if first? != before || other? != before { return Err("reader observed partial replacement".into()); }
        Ok(())
    })?;
    let after = call(&env, &replay, &args, 0)?;
    if after.pointer("/result/native_rows/8/product_code") != Some(&json!("REPLACED-8"))
        || stub.finish()?.len() != 1 {
        return Err("replacement failed or a stored read made a request".into());
    }
    Ok(())
}
pub(super) fn markdown() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let replay = seed(&env)?;
    call(&env, &replay, &["sync", "who-ivd"], 0)?;
    let mut args = env.globals(Some(&replay), false);
    args.extend(["get", "diagnostic", "0466-046-00", "--source", "who-ivd", "--format", "md"].map(str::to_owned));
    let output = env.command(env!("CARGO_BIN_EXE_biomcp"), &args).output().map_err(|e| e.to_string())?;
    let text = String::from_utf8(output.stdout).map_err(|e| e.to_string())?;
    if !output.status.success() || !output.stderr.is_empty() { return Err("WHO Markdown failed".into()); }
    for expected in ["0466-046-00", "Synthetic", "CODE-8", "US FDA text without application", "2024", "WHO terms of use"] {
        if !text.contains(expected) { return Err(format!("WHO Markdown omitted {expected}")); }
    }
    Ok(())
}
