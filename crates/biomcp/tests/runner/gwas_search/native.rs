//! Retained restricted source replies are opt-in external fixtures, never Git data.
use super::*;
use std::io::Write;
pub(super) fn run(directory: &Path) -> Result<(), String> {
    let registry = super::super::schema::Registry::embedded()?;
    let mut paths = fs::read_dir(directory)
        .map_err(|e| e.to_string())?
        .map(|entry| entry.map(|entry| entry.path()).map_err(|e| e.to_string()))
        .collect::<Result<Vec<_>, _>>()?;
    paths.sort();
    if paths.len() != 22 {
        return Err("expected the 22 declared native request cases".into());
    }
    for path in paths {
        let value = super::super::run_case(&path, &registry)?;
        let full = serde_json::to_vec(&value).map_err(|e| e.to_string())?.len();
        let comparable = super::super::source_terms::size(&value, "")?;
        writeln!(
            std::io::stdout(),
            "GWAS native {}: CLI={}, MCP={}, comparable={}, estimated_tokens={}",
            path.file_stem().ok_or("case name")?.to_string_lossy(),
            full + 1,
            full,
            comparable,
            comparable.div_ceil(4)
        )
        .map_err(|e| e.to_string())?;
    }
    Ok(())
}
