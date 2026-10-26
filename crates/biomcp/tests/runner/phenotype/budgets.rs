//! Named full-envelope phenotype phases; CLI includes its newline.
use serde_json::Value;
use std::path::Path;
pub(in crate::runner) fn check(value: &Value, phase: &str, surface: &str) -> Result<(), String> {
    let bytes =
        serde_json::to_vec(value).map_err(|e| e.to_string())?.len() + usize::from(surface == "cli");
    if phase.starts_with("case/help ") {
        use std::io::Write;
        writeln!(
            std::io::stdout(),
            "PHENOTYPE_HELP_MEASURE {}",
            serde_json::json!({"phase":phase,"surface":surface,"bytes":bytes})
        )
        .map_err(|e| e.to_string())?;
        return Ok(());
    }
    let max_bytes = ceiling(phase, surface)?;
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
    super::native::mcp(directory, words, value, exit, Some(ceiling(phase, "mcp")?))
}

pub(in crate::runner) fn case(value: &Value, args: &[String], surface: &str) -> Result<(), String> {
    if args.get(1).is_some_and(|v| v == "phenotype")
        || (args.first().is_some_and(|v| v == "help") && args.iter().any(|v| v == "phenotype"))
    {
        let mut words = Vec::new();
        let mut args = args.iter();
        while let Some(word) = args.next() {
            match word.as_str() {
                "--snapshot" | "--record" | "--replay" => {
                    args.next();
                }
                _ => words.push(word.as_str()),
            }
        }
        check(value, &format!("case/{}", words.join(" ")), surface)?;
    }
    Ok(())
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
