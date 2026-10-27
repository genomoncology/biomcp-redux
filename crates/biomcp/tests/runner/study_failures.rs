//! Feature study-failures: synthetic source faults preserve accepted metadata and fail explicitly.
use super::{call, rows, seed, study_files, study_get};
use crate::runner::launch::Environment;
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::{fs, path::Path};
type Check = Result<(), String>;
pub(super) fn trials() -> Vec<Trial> {
    let mut trials = Vec::new();
    for surface in ["catalog", "profiles", "sample-lists", "files"] {
        for fault in ["429", "500", "timeout", "broken-json", "error-object"] {
            trials.push(Trial::test(
                format!("study-failures/synthetic-{surface}-{fault}"),
                move || failure(surface, fault).map_err(Into::into),
            ));
        }
    }
    trials.push(Trial::test(
        "study-failures/synthetic-bundle-identity",
        || bundle_identity().map_err(Into::into),
    ));
    trials
}
fn failure(surface: &str, fault: &str) -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let replay = seed(&env, rows(), 200)?;
    call(
        &env,
        &replay,
        &["sync", "cbioportal-studies"],
        0,
        &format!("failures/failure/1/{surface}/{fault}"),
    )?;
    let before = call(
        &env,
        &replay,
        &["get", "study", "synthetic_0"],
        0,
        &format!("failures/failure/2/{surface}/{fault}"),
    )?;
    let status = fault.parse::<u16>().unwrap_or(200);
    let delay = if fault == "timeout" { 120001 } else { 0 };
    let body = if fault == "broken-json" {
        "[{"
    } else {
        r#"{"error":"synthetic provider error"}"#
    };
    let command = match surface {
        "catalog" => {
            catalog_recording(&env, &replay, body, status, delay)?;
            vec!["sync", "cbioportal-studies"]
        }
        "files" => {
            study_files::record_raw(&replay, body, status, delay)?;
            vec!["get", "study", "synthetic_0", surface]
        }
        _ => {
            let route = if surface == "profiles" {
                "molecular-profiles"
            } else {
                surface
            };
            study_get::recording_raw(&replay, route, body, status, delay)?;
            vec!["get", "study", "synthetic_0", surface]
        }
    };
    let value = call(
        &env,
        &replay,
        &command,
        4,
        &format!("failures/failure/3/{surface}/{fault}"),
    )?;
    failed(&value)?;
    let after = call(
        &env,
        &replay,
        &["get", "study", "synthetic_0"],
        0,
        &format!("failures/failure/4/{surface}/{fault}"),
    )?;
    bounded_card(&after)?;
    if before != after {
        return Err(format!("{surface}/{fault} changed stored card"));
    }
    Ok(())
}
fn failed(value: &Value) -> Check {
    if value.get("status") != Some(&json!("failed"))
        || value.get("result").is_some()
        || value.pointer("/sources/0/source") != Some(&json!("cbioportal"))
        || value.pointer("/sources/0/state") != Some(&json!("failed"))
        || value.pointer("/sources/0/use") != Some(&json!("restricted"))
        || value.pointer("/sources/0/terms").is_none()
    {
        return Err(format!(
            "synthetic fault became empty or lost source/terms: {value}"
        ));
    }
    bounded(value)
}
fn bounded(value: &Value) -> Check {
    // Largest synthetic failure: 502 bytes in j14854; catalog rejection: 401 in j14863.
    let bytes = serde_json::to_vec(value).map_err(|e| e.to_string())?.len();
    use std::io::Write;
    writeln!(std::io::stdout(), "Study synthetic failure bytes={bytes}")
        .map_err(|e| e.to_string())?;
    if bytes > (502_usize * 115).div_ceil(100) {
        return Err("synthetic failure exceeds its envelope ceiling".into());
    }
    Ok(())
}
fn catalog_recording(
    env: &Environment,
    replay: &Path,
    body: &str,
    status: u16,
    delay: u64,
) -> Check {
    seed(env, rows(), 200)?;
    let path = fs::read_dir(replay.join("cbioportal"))
        .map_err(|e| e.to_string())?
        .next()
        .ok_or("catalog recording")?
        .map_err(|e| e.to_string())?
        .path();
    let mut value: Value = serde_json::from_slice(&fs::read(&path).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    let exchange = json!({"status":status,"content_type":"application/json","delay_ms":delay,"retry_after_ms":null,"body":{"kind":"text","value":body}});
    let count = if matches!(status, 429 | 500) { 3 } else { 1 };
    *value.get_mut("exchanges").ok_or("exchanges")? = json!(vec![exchange; count]);
    fs::write(path, value.to_string()).map_err(|e| e.to_string())
}
fn bundle_identity() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let replay = seed(&env, rows(), 200)?;
    call(
        &env,
        &replay,
        &["sync", "cbioportal-studies"],
        0,
        "failures/bundle_identity/1",
    )?;
    let before = call(
        &env,
        &replay,
        &["get", "study", "synthetic_0"],
        0,
        "failures/bundle_identity/2",
    )?;
    let mut wrong = rows();
    *wrong
        .pointer_mut("/0/resourceCounts")
        .ok_or("resource counts")? = json!(
        [{ "resourceId":"synthetic_resource","displayName":"Synthetic resource","description":"Synthetic scope mismatch",
        "resourceType":"metadata","priority":"1","openByDefault":true,"cancerStudyIdentifier":"other_study","sampleCount":1,
        "patientCount":1 }]
    );
    catalog_recording(&env, &replay, &wrong.to_string(), 200, 0)?;
    let value = call(
        &env,
        &replay,
        &["sync", "cbioportal-studies"],
        4,
        "failures/bundle_identity/3",
    )?;
    failed(&value)?;
    if value.pointer("/sources/0/reason") != Some(&json!("bad_reply"))
        || value
            .get("problems")
            .and_then(Value::as_array)
            .is_none_or(Vec::is_empty)
    {
        return Err(format!(
            "cross-study bundle rejection lost its reason or problem: {value}"
        ));
    }
    let after = call(
        &env,
        &replay,
        &["get", "study", "synthetic_0"],
        0,
        "failures/bundle_identity/4",
    )?;
    bounded_card(&after)?;
    if before != after {
        return Err("cross-study replacement entered stored catalog".into());
    }
    Ok(())
}

fn bounded_card(value: &Value) -> Check {
    use std::io::Write;
    let bytes = serde_json::to_vec(value).map_err(|e| e.to_string())?.len();
    writeln!(
        std::io::stdout(),
        "Study synthetic retained card bytes={bytes}"
    )
    .map_err(|e| e.to_string())?;
    // Retained synthetic card: 1120 bytes in j14863, with fifteen percent headroom.
    if bytes > (1120_usize * 115).div_ceil(100) {
        return Err("retained synthetic card exceeds its envelope ceiling".into());
    }
    Ok(())
}
