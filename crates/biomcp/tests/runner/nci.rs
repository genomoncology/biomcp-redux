//! NCI security boundary through real isolated CLI commands and a synthetic TLS peer.
#![expect(
    clippy::indexing_slicing,
    reason = "Synthetic recording mutations fail directly on fixture shape drift"
)]
use super::launch::Environment;
use libtest_mimic::Trial;
use serde_json::Value;
mod biomarker;

mod organization;

const KEY: &str = "synthetic-nci-stored-key";
pub(super) fn trials() -> Vec<Trial> {
    organization::trials()
        .into_iter()
        .chain(biomarker::trials())
        .collect()
}

pub(super) fn invoke(env: &Environment, args: &[String], exit: i32) -> Result<Value, String> {
    let output = env
        .command(env!("CARGO_BIN_EXE_biomcp"), args)
        .env("NCI_API_KEY", "synthetic-nci-env-key")
        .output()
        .map_err(|e| e.to_string())?;
    let all = [output.stdout.as_slice(), output.stderr.as_slice()].concat();
    if all.windows(KEY.len()).any(|s| s == KEY.as_bytes())
        || all.windows(21).any(|s| s == b"synthetic-nci-env-key")
    {
        return Err("credential leaked into command output".into());
    }
    if output.status.code() != Some(exit) || !output.stderr.is_empty() {
        return Err(format!(
            "wrong NCI command exit {:?}, expected {exit}",
            output.status.code()
        ));
    }
    let value: Value = serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;
    super::schema::Registry::embedded()?.validate(&value, args, exit == 0)?;
    Ok(value)
}
