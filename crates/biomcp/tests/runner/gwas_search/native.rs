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
    let budgets: std::collections::BTreeMap<String, [usize; 2]> = serde_json::from_str(
        &crate::runner::launch::fixture_text("gwas-search-budgets.json")?,
    )
    .map_err(|e| e.to_string())?;
    let outputs = directory
        .parent()
        .ok_or("fixture root")?
        .join("gwas-native-outputs");
    fs::create_dir(&outputs).map_err(|e| e.to_string())?;
    for path in paths {
        let value = super::super::run_case(&path, &registry)?;
        continuation(&value)?;
        let full = serde_json::to_vec(&value).map_err(|e| e.to_string())?.len();
        let comparable = super::super::source_terms::size(&value, "")?;
        let name = path
            .file_stem()
            .and_then(|name| name.to_str())
            .ok_or("case name")?;
        let [measured_full, measured_comparable] = budgets.get(name).ok_or("case budget")?;
        if full + 1 > (measured_full * 115).div_ceil(100)
            || comparable > (measured_comparable * 115).div_ceil(100)
        {
            return Err(format!(
                "{name}: full or comparable response exceeded its measured fifteen-percent ceiling"
            ));
        }
        fs::write(
            outputs.join(format!("{name}.json")),
            serde_json::to_vec(&value).map_err(|e| e.to_string())?,
        )
        .map_err(|e| e.to_string())?;
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

fn continuation(value: &Value) -> Result<(), String> {
    let Some(next) = value.pointer("/result/continuation/query") else {
        return Ok(());
    };
    let mut original = value.pointer("/query/query").ok_or("query echo")?.clone();
    let fields = original.as_object_mut().ok_or("query fields")?;
    let offset = fields.remove("offset");
    if offset.is_none() {
        return Err("missing original offset".into());
    }
    let mut continued = next.clone();
    continued
        .as_object_mut()
        .ok_or("continued fields")?
        .remove("offset");
    if original != continued || value.pointer("/next/0/kind") != Some(&json!("page")) {
        return Err("continuation changed checked selectors or source order".into());
    }
    Ok(())
}
