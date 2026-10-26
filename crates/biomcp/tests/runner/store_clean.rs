//! Cleanup is exercised through the real isolated command and shared SQLite file.
use super::launch::Environment;
use libtest_mimic::Trial;
use serde_json::Value;

pub(super) fn trials() -> Vec<Trial> {
    vec![Trial::test("store-clean/expired-owned-state", || {
        clean().map_err(Into::into)
    })]
}
fn clean() -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let args = ["store", "clean", "--offline"].map(String::from);
    let output = env
        .command(env!("CARGO_BIN_EXE_biomcp"), &args)
        .output()
        .map_err(|e| e.to_string())?;
    let value: Value = serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;
    if !output.status.success() || value.pointer("/result/replies") != Some(&Value::from(0)) {
        return Err(format!("cleanup failed: {value}"));
    }
    Ok(())
}
