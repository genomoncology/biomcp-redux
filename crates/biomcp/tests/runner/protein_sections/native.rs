use super::{Path, Value, fs, json};
use std::io::Write;
pub(super) fn run(directory: &Path) -> Result<(), String> {
    let registry = super::super::schema::Registry::embedded()?;
    let mut paths = fs::read_dir(directory)
        .map_err(|e| e.to_string())?
        .map(|e| e.map(|e| e.path()).map_err(|e| e.to_string()))
        .collect::<Result<Vec<_>, _>>()?;
    paths.sort();
    let budgets: std::collections::BTreeMap<String, [usize; 2]> = serde_json::from_str(
        &crate::runner::launch::fixture_text("protein-section-budgets.json")?,
    )
    .map_err(|e| e.to_string())?;
    let output = directory
        .parent()
        .ok_or("native directory")?
        .join("outputs");
    fs::create_dir(&output).map_err(|e| e.to_string())?;
    for path in paths {
        let value = super::super::run_case(&path, &registry)?;
        let name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or("native case name")?;
        facts(name, &value)?;
        let full = serde_json::to_vec(&value).map_err(|e| e.to_string())?.len();
        let comparable = super::super::source_terms::size(&value, "")?;
        let [measured_full, measured_comparable] = budgets.get(name).ok_or("native case budget")?;
        if full + 1 > (measured_full * 115).div_ceil(100)
            || comparable > (measured_comparable * 115).div_ceil(100)
        {
            return Err(format!(
                "{name}: native full or comparable response exceeded its measured fifteen-percent ceiling"
            ));
        }
        fs::write(
            output.join(format!("{name}.json")),
            serde_json::to_vec(&value).map_err(|e| e.to_string())?,
        )
        .map_err(|e| e.to_string())?;
        let interface = "CLI/MCP parity";
        writeln!(
            std::io::stdout(),
            "PROTEIN {name} {interface} CLI={} JSON={full} comparable={comparable} ceiling={}",
            full + 1,
            (comparable * 115).div_ceil(100)
        )
        .map_err(|e| e.to_string())?;
    }
    walks(directory, &output, &budgets)
}

fn walks(
    directory: &Path,
    output: &Path,
    budgets: &std::collections::BTreeMap<String, [usize; 2]>,
) -> Result<(), String> {
    let registry = super::super::schema::Registry::embedded()?;
    let walks = directory.parent().ok_or("native directory")?.join("walks");
    for path in fs::read_dir(walks).map_err(|e| e.to_string())? {
        let path = path.map_err(|e| e.to_string())?.path();
        let value = super::super::run_case(&path, &registry)?;
        let name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or("walk name")?;
        let [whole, small] = budgets
            .get(&format!("walk-{name}"))
            .ok_or("unmeasured native walk")?;
        // Retained continuation proof ran on CLI only; the first page owns CLI/MCP parity.
        let bytes = serde_json::to_vec(&value).map_err(|e| e.to_string())?.len() + 1;
        if bytes > (whole * 115).div_ceil(100)
            || super::super::source_terms::size(&value, "")? > (small * 115).div_ceil(100)
        {
            return Err(format!(
                "{name}: continuation exceeded its own measured CLI ceiling"
            ));
        }
        fs::write(
            output.join(format!("walk-{name}.json")),
            serde_json::to_vec(&value).map_err(|e| e.to_string())?,
        )
        .map_err(|e| e.to_string())?;
    }
    Ok(())
}

fn facts(name: &str, value: &Value) -> Result<(), String> {
    if name == "sequence-Q9UIF7" {
        let sequence = value
            .pointer("/result/sequence/sequence")
            .and_then(Value::as_str)
            .ok_or("sequence")?;
        if sequence.as_bytes().get(367) != Some(&b'S')
            || sequence.as_bytes().get(392) != Some(&b'G')
        {
            return Err(
                "displayed source residues changed or historical labels were translated".into(),
            );
        }
    }
    if let Some(sequence) = value.pointer("/result/sequence") {
        let length = sequence
            .get("length")
            .and_then(Value::as_u64)
            .ok_or("sequence length")?;
        if sequence
            .get("sequence")
            .and_then(Value::as_str)
            .map(str::len)
            != Some(length as usize)
        {
            return Err("source sequence was truncated".into());
        }
    }
    if name == "get-P00533-function"
        && (value
            .pointer("/result/items/0/sections/function/data/items/0/molecule")
            .is_some()
            || value.pointer("/result/items/0/sections/function/data/items/1/molecule")
                != Some(&json!("Isoform 2")))
    {
        return Err("native function qualifier was lost or applied to another paragraph".into());
    }
    Ok(())
}
