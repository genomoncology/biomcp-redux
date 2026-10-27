//! Retained native catalog measurements without provider calls.
use super::{Check, fs, json};
pub(super) fn run() -> Check {
    use std::io::Write;
    let root = crate::runner::launch::fixtures().join("cases/wikipathways-native");
    let registry = crate::runner::schema::Registry::embedded()?;
    let budgets: std::collections::BTreeMap<String, [usize; 3]> =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../tests/wikipathways-budgets.json"
        )))
        .map_err(|e| e.to_string())?;
    let outputs = crate::runner::launch::fixtures().join("wikipathways-native-outputs");
    fs::create_dir(&outputs).map_err(|e| e.to_string())?;
    let mut files = fs::read_dir(root)
        .map_err(|e| e.to_string())?
        .map(|e| e.map(|e| e.path()).map_err(|e| e.to_string()))
        .collect::<Result<Vec<_>, _>>()?;
    files.sort();
    for path in files {
        let value = crate::runner::run_case(&path, &registry)?;
        let full = serde_json::to_vec(&value).map_err(|e| e.to_string())?.len();
        let comparable = crate::runner::source_terms::size(&value, "")?;
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or("filename")?;
        let measured = budgets.get(name).ok_or("case budget")?;
        let observed = [full + 1, comparable + 1, comparable.div_ceil(4)];
        if observed
            .iter()
            .zip(measured)
            .any(|(actual, baseline)| *actual > (baseline * 115).div_ceil(100))
        {
            return Err(format!(
                "{name}: response exceeded its measured fifteen-percent ceiling"
            ));
        }
        fs::write(
            outputs
                .join(path.file_stem().ok_or("case name")?)
                .with_extension("json"),
            serde_json::to_vec(&value).map_err(|e| e.to_string())?,
        )
        .map_err(|e| e.to_string())?;
        writeln!(
            std::io::stdout(),
            "WIKIPATHWAYS_MEASURE {}",
            json!(
                {"case":path.file_name().and_then(|n|n.to_str()).ok_or("filename")?,"full_cli_bytes":full+1,"full_mcp_bytes":full,
                "comparable_cli_bytes":comparable+1,"comparable_mcp_bytes":comparable,"tokens":comparable.div_ceil(4)}
            )
        )
        .map_err(|e| e.to_string())?;
    }
    Ok(())
}
