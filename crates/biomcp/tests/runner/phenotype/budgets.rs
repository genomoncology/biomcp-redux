//! Named full-envelope phenotype phases; CLI includes its newline.
use serde_json::Value;
use std::path::Path;
pub(super) fn check(value: &Value, phase: &str, surface: &str) -> Result<(), String> {
    let observed =
        serde_json::to_vec(value).map_err(|e| e.to_string())?.len() + usize::from(surface == "cli");
    bytes(observed, phase, surface)
}

pub(super) fn bytes(observed: usize, phase: &str, surface: &str) -> Result<(), String> {
    let bytes = observed;
    let max_bytes = match ceiling(phase, surface) {
        Ok(value) => value,
        Err(error) if error == "missing phenotype phase" => {
            use std::io::Write;
            writeln!(
                std::io::stdout(),
                "PHENOTYPE_MISSING {}",
                serde_json::json!({"phase":phase,"surface":surface,"bytes":observed})
            )
            .map_err(|e| e.to_string())?;
            return Ok(());
        }
        Err(error) => return Err(error),
    };
    let key = format!("{phase} [{surface}]");
    if bytes > max_bytes {
        return Err(format!(
            "phenotype {key}: {bytes} exceeds max_bytes={max_bytes}"
        ));
    }
    Ok(())
}
pub(super) fn mcp(
    phase: &str,
    directory: &Path,
    words: &[&str],
    value: &Value,
    exit: i32,
) -> Result<(), String> {
    check(value, phase, "mcp")?;
    // Attribution only: measure unknown identities, then replace this branch with fixed data.
    let ceiling = match ceiling(phase, "mcp") {
        Ok(value) => value,
        Err(error) if error == "missing phenotype phase" => {
            let bytes = serde_json::to_vec(value).map_err(|e| e.to_string())?.len();
            (bytes * 115).div_ceil(100)
        }
        Err(error) => return Err(error),
    };
    super::native::mcp(directory, words, value, exit, Some(ceiling))
}

fn ceiling(phase: &str, surface: &str) -> Result<usize, String> {
    let measurements: std::collections::BTreeMap<String, usize> = serde_json::from_str(
        include_str!("../../../../../tests/phenotype-command-budgets.json"),
    )
    .map_err(|e| e.to_string())?;
    measurements
        .get(&format!("{phase} [{surface}]"))
        .map(|n| (n * 115).div_ceil(100))
        .ok_or("missing phenotype phase".into())
}
