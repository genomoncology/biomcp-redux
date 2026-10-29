//! Response-only fixture edits run through the ordinary command case machinery.
use serde_json::{Value, json};
use std::{fs, path::Path};
type Check = Result<(), Box<dyn std::error::Error>>;
pub(super) fn case(
    capture: &str,
    args: &[&str],
    change: impl FnMut(&str, &mut Value),
    expect: Value,
    exit: i32,
) -> Check {
    checked_case(capture, args, change, expect, exit).map(|_| ())
}

pub(super) fn checked_case(
    capture: &str,
    args: &[&str],
    change: impl FnMut(&str, &mut Value),
    expect: Value,
    exit: i32,
) -> Result<Value, Box<dyn std::error::Error>> {
    checked_steps(
        capture,
        change,
        json!([{"run": args, "expect": expect, "exit": exit}]),
    )
}

pub(super) fn checked_steps(
    capture: &str,
    mut change: impl FnMut(&str, &mut Value),
    steps: Value,
) -> Result<Value, Box<dyn std::error::Error>> {
    let directory = tempfile::tempdir()?;
    let recordings = directory.path().join("recordings");
    copy_capture(capture, &recordings, &mut change)?;
    let path = directory.path().join("case.yaml");
    fs::write(
        &path,
        serde_json::to_vec(&json!({
            "feature": "gene-commands",
            "story": "A checked command keeps valid source facts and reports malformed siblings",
            "replay": recordings,
            "steps": steps
        }))?,
    )?;
    Ok(super::run_case(
        &path,
        &super::schema::Registry::embedded()?,
    )?)
}

/// Copy transport-owned identities unchanged and mutate only response exchanges.
pub(super) fn copy_capture(
    capture: &str,
    recordings: &Path,
    mut change: impl FnMut(&str, &mut Value),
) -> Check {
    let original = super::launch::fixtures().join("replies").join(capture);
    for source in fs::read_dir(original)? {
        let source = source?;
        if !source.file_type()?.is_dir() {
            continue;
        }
        let name = source.file_name().to_string_lossy().into_owned();
        fs::create_dir_all(recordings.join(&name))?;
        for file in fs::read_dir(source.path())? {
            let file = file?;
            if !file.file_type()?.is_file() {
                continue;
            }
            if file.path().extension().is_some_and(|v| v == "bin") {
                fs::copy(file.path(), recordings.join(&name).join(file.file_name()))?;
                continue;
            }
            if file.path().extension().is_none_or(|v| v != "json") {
                continue;
            }
            let mut record = serde_json::from_slice(&fs::read(file.path())?)?;
            change(&name, &mut record);
            fs::write(
                recordings.join(&name).join(file.file_name()),
                serde_json::to_vec(&record)?,
            )?;
        }
    }
    Ok(())
}

/// Acquisition is synthetic cache setup, never a claim about a retained capture.
pub(super) const CACHE_ACQUIRED: i64 = 42;

pub(super) fn cache_acquisition(record: &mut Value, acquired: i64) {
    if let Some(exchanges) = record.get_mut("exchanges").and_then(Value::as_array_mut) {
        for exchange in exchanges {
            if exchange.get("status").and_then(Value::as_u64).is_some_and(|v| (200..300).contains(&v))
                && let Some(exchange) = exchange.as_object_mut()
            {
                exchange.entry("fetched").or_insert(json!(acquired));
            }
        }
    }
}

pub(super) fn cache_capture(capture: &str, target: &Path, acquired: i64) -> Result<(), String> {
    copy_capture(capture, target, |_, record| cache_acquisition(record, acquired))
        .map_err(|e| e.to_string())
}

pub(super) fn replay(
    env: &super::launch::Environment,
    capture: Option<&str>,
    acquired: Option<i64>,
) -> Result<Option<std::path::PathBuf>, String> {
    let Some(capture) = capture else { return Ok(None) };
    let Some(acquired) = acquired else {
        return Ok(Some(super::launch::fixtures().join("replies").join(capture)));
    };
    let target = env.root().join("cache-acquisition-recordings");
    cache_capture(capture, &target, acquired)?;
    Ok(Some(target))
}
