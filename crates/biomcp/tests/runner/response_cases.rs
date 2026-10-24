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
    mut change: impl FnMut(&str, &mut Value),
    expect: Value,
    exit: i32,
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
            "steps": [{"run": args, "expect": expect, "exit": exit}]
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
    let original = super::launch::fixtures()
        .join("replies")
        .join(capture);
    for source in fs::read_dir(original)? {
        let source = source?;
        if !source.file_type()?.is_dir() {
            continue;
        }
        let name = source.file_name().to_string_lossy().into_owned();
        fs::create_dir_all(recordings.join(&name))?;
        for file in fs::read_dir(source.path())? {
            let file = file?;
            if !file.file_type()?.is_file() || file.path().extension().is_none_or(|v| v != "json") {
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
