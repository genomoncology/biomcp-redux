//! Owning synthetic protein case phases; CLI counts its trailing newline.
use serde_json::Value;
pub(super) fn check(value: &Value, case: &str, surface: &str) -> Result<(), String> {
    use std::io::Write;
    let bytes =
        serde_json::to_vec(value).map_err(|e| e.to_string())?.len() + usize::from(surface == "cli");
    let comparable = super::source_terms::size(value, "")? + usize::from(surface == "cli");
    writeln!(
        std::io::stdout(),
        "PROTEIN_ACCOUNTING {}",
        serde_json::json!({
            "case":case,"surface":surface,"bytes":bytes,"comparable":comparable
        })
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}
