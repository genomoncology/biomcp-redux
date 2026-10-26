//! Named full-envelope phenotype phases; CLI includes its newline.
use serde_json::Value;
use std::{io::Write, path::Path};
pub(in crate::runner) fn check(value: &Value, phase: &str, surface: &str) -> Result<(), String> {
    let bytes =
        serde_json::to_vec(value).map_err(|e| e.to_string())?.len() + usize::from(surface == "cli");
    let comparable = super::super::source_terms::size(value, "")?;
    writeln!(
        std::io::stdout(),
        "PHENOTYPE_MEASURE {}",
        serde_json::json!({"phase":phase,"surface":surface,"bytes":bytes,"comparable":comparable})
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}
pub(super) fn mcp(
    phase: &str,
    directory: &Path,
    words: &[&str],
    value: &Value,
    exit: i32,
    _old: Option<usize>,
) -> Result<(), String> {
    check(value, phase, "mcp")?;
    super::native::mcp(directory, words, value, exit, Some(100000))
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
