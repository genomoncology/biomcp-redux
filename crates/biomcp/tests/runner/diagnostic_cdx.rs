//! Feature diagnostic-cdx: synthetic stored companion rows retain owned identities.
use super::{
    configuration::invoke,
    launch::{Environment, fixtures},
};
use serde_json::{Value, json};
use std::{
    fs,
    path::{Path, PathBuf},
};
type Check = Result<(), String>;
mod pivots;

fn call(env: &Environment, replay: &Path, tail: &[&str], exit: i32) -> Result<Value, String> {
    let mut args = env.globals(Some(replay), false);
    args.extend(tail.iter().map(|s| (*s).to_owned()));
    invoke(env, args, exit)
}
pub(super) fn seed(env: &Environment) -> Result<PathBuf, String> {
    let source = fixtures().join("replies/diagnostic/fda-cdx-synthetic/fda-cdx");
    let replay = env.root().join("replay");
    fs::create_dir_all(replay.join("fda-cdx")).map_err(|e| e.to_string())?;
    for entry in fs::read_dir(source).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        fs::copy(entry.path(), replay.join("fda-cdx").join(entry.file_name()))
            .map_err(|e| e.to_string())?;
    }
    Ok(replay)
}
fn identities() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let replay = seed(&env)?;
    let receipt = call(&env, &replay, &["sync", "fda-cdx"], 0)?;
    if receipt.pointer("/result/rows") != Some(&json!(9)) {
        return Err(format!("wrong physical row total: {receipt}"));
    }
    let value = call(
        &env,
        &replay,
        &[
            "search",
            "diagnostic",
            "--source",
            "fda-cdx",
            "--device-id",
            "P200010/S026",
        ],
        0,
    )?;
    if value.pointer("/page/total") != Some(&json!(1))
        || value.pointer("/result/items/0/drug_applications/1/id") != Some(&json!("BLA125084"))
    {
        return Err(format!("combination identities lost: {value}"));
    }
    let args = value
        .get("next")
        .and_then(Value::as_array)
        .ok_or("missing pivots")?;
    for id in ["NDA210496", "BLA125084"] {
        if !args
            .iter()
            .any(|v| v.get("args") == Some(&json!(["get", "drug", id, "--source", "drugsfda"])))
        {
            return Err(format!("missing drug pivot {id}: {value}"));
        }
    }
    let mismatch = call(
        &env,
        &replay,
        &[
            "search",
            "diagnostic",
            "--source",
            "fda-cdx",
            "--device-id",
            "P150013/S020",
        ],
        0,
    )?;
    let applications = mismatch.pointer("/result/items/0/drug_applications");
    if applications.is_some()
        || mismatch.pointer("/problems/0/code") != Some(&json!("identity.conflict"))
    {
        return Err(format!("mismatched cell supplied identity: {mismatch}"));
    }
    let biomarker = call(
        &env,
        &replay,
        &[
            "search",
            "diagnostic",
            "--source",
            "fda-cdx",
            "--biomarker",
            "KRAS",
        ],
        0,
    )?;
    if biomarker.pointer("/page/total") != Some(&json!(8))
        || !biomarker
            .get("problems")
            .and_then(Value::as_array)
            .is_some_and(|a| {
                a.iter()
                    .any(|v| v.get("field") == Some(&json!("table_1.biomarker")))
            })
    {
        return Err(format!("group biomarker unsupported missing: {biomarker}"));
    }
    Ok(())
}
fn replacement() -> Check {
    for header in ["Biomarker(s)", "PMA (Approval Date)"] {
        for fault in ["missing", "extra", "changed", "reordered"] {
            let env = Environment::new().map_err(|e| e.to_string())?;
            let replay = seed(&env)?;
            call(&env, &replay, &["sync", "fda-cdx"], 0)?;
            let args = [
                "search",
                "diagnostic",
                "--source",
                "fda-cdx",
                "--device-id",
                "P200010/S026",
            ];
            let before = call(&env, &replay, &args, 0)?;
            for entry in fs::read_dir(replay.join("fda-cdx")).map_err(|e| e.to_string())? {
                let path = entry.map_err(|e| e.to_string())?.path();
                let mut value: Value =
                    serde_json::from_slice(&fs::read(&path).map_err(|e| e.to_string())?)
                        .map_err(|e| e.to_string())?;
                let cell = value
                    .pointer_mut("/exchanges/0/body/value")
                    .ok_or("missing body")?;
                let body = cell.as_str().ok_or("non-text body")?;
                let original = format!("<th>{header}</th>");
                let replacement = match fault {
                    "missing" => String::new(),
                    "extra" => format!("{original}<th>Extra</th>"),
                    "reordered" => original.clone(),
                    _ => format!("<th>{header} changed</th>"),
                };
                *cell = json!(if fault == "reordered" {
                    let previous = if header == "Biomarker(s)" {
                        "Drug Trade Name (Generic) NDA / BLA"
                    } else {
                        "Indication(s) - Sample Type"
                    };
                    body.replace(
                        &format!("<th>{previous}</th>{original}"),
                        &format!("{original}<th>{previous}</th>"),
                    )
                } else {
                    body.replace(&original, &replacement)
                });
                fs::write(path, value.to_string()).map_err(|e| e.to_string())?;
            }
            let failed = call(&env, &replay, &["sync", "fda-cdx"], 4)?;
            if failed.pointer("/sources/0/reason") != Some(&json!("bad-reply")) {
                return Err(format!("{header}/{fault} accepted: {failed}"));
            }
            if call(&env, &replay, &args, 0)? != before {
                return Err(format!("{header}/{fault} replaced prior snapshot"));
            }
        }
    }
    Ok(())
}
fn stored() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let replay = seed(&env)?;
    let args = [
        "search",
        "diagnostic",
        "--source",
        "fda-cdx",
        "--device-id",
        "P200010/S026",
    ];
    let missing = call(&env, &replay, &args, 4)?;
    if missing.pointer("/sources/0/reason") != Some(&json!("syncing"))
        || missing.pointer("/next/0/args") != Some(&json!(["sync", "fda-cdx"]))
    {
        return Err(format!("read synchronized: {missing}"));
    }
    call(&env, &replay, &["sync", "fda-cdx"], 0)?;
    let before = call(&env, &replay, &args, 0)?;
    fs::remove_dir_all(replay.join("fda-cdx")).map_err(|e| e.to_string())?;
    let mut fresh = args.to_vec();
    fresh.push("--fresh");
    let after = call(&env, &replay, &fresh, 0)?;
    if after.get("result") != before.get("result") {
        return Err("fresh read synchronized".into());
    }
    env.config(&json!({"disable_sources":["fda-cdx"]}))
        .map_err(|e| e.to_string())?;
    let disabled = call(&env, &replay, &args, 4)?;
    if disabled.get("result").is_some()
        || disabled.pointer("/sources/0/reason") != Some(&json!("disabled"))
    {
        return Err(format!("disabled source leaked data: {disabled}"));
    }
    Ok(())
}
fn concurrent() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let replay = seed(&env)?;
    call(&env, &replay, &["sync", "fda-cdx"], 0)?;
    let search = [
        "search",
        "diagnostic",
        "--source",
        "fda-cdx",
        "--device-id",
        "P200010",
    ];
    let before = call(&env, &replay, &search, 0)?;
    for entry in fs::read_dir(replay.join("fda-cdx")).map_err(|e| e.to_string())? {
        let path = entry.map_err(|e| e.to_string())?.path();
        let original = fs::read_to_string(&path).map_err(|e| e.to_string())?;
        fs::write(path, original.replace("Synthetic", "Updated synthetic"))
            .map_err(|e| e.to_string())?;
    }
    let mut sync = env.globals(Some(&replay), false);
    sync.extend(["sync".into(), "fda-cdx".into()]);
    let mut args = env.globals(Some(&replay), false);
    args.extend(search.map(str::to_owned));
    let spawn = |args: &[String]| {
        let mut command = env.command(env!("CARGO_BIN_EXE_biomcp"), args);
        let child = command
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| e.to_string())?;
        Ok::<_, String>((child, command))
    };
    let replacement = spawn(&sync)?;
    let readers = [spawn(&args)?, spawn(&args)?];
    let finish = |(child, mut command): (std::process::Child, super::capture::Capture)| {
        command.wait_with_output(child).map_err(|e| e.to_string())
    };
    let output = finish(replacement)?;
    let reader_outputs = readers
        .into_iter()
        .map(finish)
        .collect::<Result<Vec<_>, _>>()?;
    if !output.status.success() {
        return Err(format!(
            "replacement failed {:?}: stdout={} stderr={}; readers={:?}",
            output.status.code(),
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr),
            reader_outputs
        ));
    }
    let receipt: Value = serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;
    super::diagnostic_retirements::bounded(&receipt)?;
    let after = call(&env, &replay, &search, 0)?;
    if before.get("result") == after.get("result") {
        return Err("replacement did not change snapshot".into());
    }
    for output in reader_outputs {
        if !output.status.success() {
            return Err(format!(
                "concurrent reader failed {:?}: stdout={} stderr={}",
                output.status.code(),
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            ));
        }
        let value: Value = serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;
        super::diagnostic_retirements::bounded(&value)?;
        if value.get("result") != before.get("result") && value.get("result") != after.get("result")
        {
            return Err(format!("reader mixed snapshot rows: {value}"));
        }
    }
    Ok(())
}

fn paging() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let replay = seed(&env)?;
    unique_applications(&replay)?;
    call(&env, &replay, &["sync", "fda-cdx"], 0)?;
    let base = call(
        &env,
        &replay,
        &[
            "search",
            "diagnostic",
            "--source",
            "fda-cdx",
            "--device-id",
            "P200010",
        ],
        0,
    )?;
    if base.pointer("/page/total") != Some(&json!(6))
        || base.pointer("/page/returned") != Some(&json!(5))
    {
        return Err(format!("base search lost supplement or group rows: {base}"));
    }
    if base.pointer("/next/11/kind") != Some(&json!("page"))
        || !base
            .get("next")
            .and_then(Value::as_array)
            .is_some_and(|a| pivots::exact(a))
    {
        return Err(format!(
            "association pivots or continuation truncated: {base}"
        ));
    }
    let next = base
        .get("next")
        .and_then(Value::as_array)
        .and_then(|a| a.iter().find(|v| v.get("kind") == Some(&json!("page"))))
        .and_then(|v| v.get("args"))
        .and_then(Value::as_array)
        .ok_or("missing native page action")?;
    let args = next
        .iter()
        .map(|v| v.as_str().ok_or("non-text page arg"))
        .collect::<Result<Vec<_>, _>>()?;
    let page = call(&env, &replay, &args, 0)?;
    if page.pointer("/page/offset") != Some(&json!(5))
        || page.pointer("/result/items/0/kind") != Some(&json!("group"))
        || page.pointer("/result/items/0/drug_applications").is_some()
    {
        return Err(format!("page or ambiguous group identity changed: {page}"));
    }
    Ok(())
}
fn markdown() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let replay = seed(&env)?;
    call(&env, &replay, &["sync", "fda-cdx"], 0)?;
    let mut args = env.globals(Some(&replay), false);
    args.extend(
        [
            "search",
            "diagnostic",
            "--source",
            "fda-cdx",
            "--device-id",
            "P200010/S026",
            "--format",
            "md",
        ]
        .map(str::to_owned),
    );
    let output = env
        .command(env!("CARGO_BIN_EXE_biomcp"), &args)
        .output()
        .map_err(|e| e.to_string())?;
    if !output.status.success() || !output.stderr.is_empty() {
        return Err("companion Markdown command failed".into());
    }
    // Full synthetic Markdown stdout measured by j13400, plus fifteen percent.
    if output.stdout.len() > 2260 {
        return Err("diagnostic Markdown exceeds its measured ceiling".into());
    }
    let text = String::from_utf8(output.stdout).map_err(|e| e.to_string())?;
    for expected in [
        "Synthetic tissue context",
        "P200010/S026",
        "NDA210496",
        "BLA125084",
        "FDA website policies",
        "get drug",
    ] {
        if !text.contains(expected) {
            return Err(format!("Markdown omitted {expected}: {text}"));
        }
    }
    Ok(())
}

fn unique_applications(replay: &Path) -> Check {
    for entry in fs::read_dir(replay.join("fda-cdx")).map_err(|e| e.to_string())? {
        let path = entry.map_err(|e| e.to_string())?.path();
        let mut value: Value = serde_json::from_slice(&fs::read(&path).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
        let cell = value
            .pointer_mut("/exchanges/0/body/value")
            .ok_or("missing body")?;
        let mut rows = Vec::new();
        for row in cell.as_str().ok_or("non-text body")?.split("<tr>") {
            let mut updated = row.to_owned();
            for n in 27..=30 {
                if row.contains(&format!("P200010S0{n}")) {
                    updated = updated.replace("210496", &format!("{}", 210496 + n - 26));
                }
            }
            rows.push(updated);
        }
        *cell = json!(rows.join("<tr>"));
        fs::write(path, value.to_string()).map_err(|e| e.to_string())?;
    }
    Ok(())
}
