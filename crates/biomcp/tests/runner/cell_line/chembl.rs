//! Feature cell-line-chembl: exact source metadata and separately requested assay totals.
use super::*;
mod faults;
pub(super) mod surface;
use surface::{compared, invoke_measured, measure_bytes, measure_mcp, parity};
const MEASURE: bool = false;
pub(super) fn trials() -> Vec<Trial> {
    let mut trials = vec![
        Trial::test(
            "cell-line-chembl/exact-native-records-and-assay-total",
            || native().map_err(Into::into),
        ),
        Trial::test("cell-line-chembl/assay-count-help-and-grammar", || {
            help().map_err(Into::into)
        }),
    ];
    trials.extend(faults::trials());
    trials
}
fn fixture(name: &str) -> Result<String, String> {
    fs::read_to_string(
        super::super::launch::fixtures()
            .join("fixtures/cell-line/chembl")
            .join(format!("{name}.json")),
    )
    .map_err(|e| e.to_string())
}
pub(super) fn reply(request: &str) -> Result<String, String> {
    let target = request.split_whitespace().nth(1).ok_or("target")?;
    if canonical(target)
        == canonical("/chembl/api/data/assay.json?cell_chembl_id=CHEMBL3706573&limit=1&offset=0")
    {
        return fixture("assays");
    }
    if target == "/chembl/api/data/status.json" {
        return fixture("status");
    }
    for id in ["CVCL_2119", "CVCL_0004", "CVCL_0064", "CVCL_9999"] {
        if canonical(target)
            == canonical(&format!(
                "/chembl/api/data/cell_line.json?cellosaurus_id={id}&limit=5&offset=0"
            ))
        {
            return fixture(id);
        }
    }
    Err(format!("Unproved request: {target}"))
}
fn stub() -> Result<Stub, String> {
    Stub::for_source("www.ebi.ac.uk", "application/json", reply)
}
fn words(assay: bool) -> Vec<&'static str> {
    if assay {
        vec!["cell-line", "assay-count", "CHEMBL3706573"]
    } else {
        vec!["get", "cell-line", "CVCL_2119", "chembl"]
    }
}
fn native() -> Result<(), String> {
    let stub = stub()?;
    let env = setup(&stub)?;
    for (id, chembl, name, count) in [
        ("CVCL_2119", "CHEMBL3706573", "MOLM-13", 1),
        ("CVCL_0004", "CHEMBL3308378", "K562", 1),
        ("CVCL_0064", "CHEMBL3308063", "MV4-11", 1),
        ("CVCL_9999", "", "", 0),
    ] {
        let args = ["get", "cell-line", id, "chembl"];
        let phase = format!("chembl/native/metadata/{id}");
        let value = parity(&phase, &env, &args, if count == 0 { 1 } else { 0 }, MEASURE)?;
        native_facts(&value, id)?;
        path(&value, "/result/items/total", json!(count))?;
        path(&value, "/result/accession", json!(id))?;
        path(&value, "/result/release/version", json!("ChEMBL_37"))?;
        if count > 0 {
            path(&value, "/result/items/items/0/id", json!(chembl))?;
            path(&value, "/result/items/items/0/name", json!(name))?;
            path(&value, "/result/items/items/0/taxon", json!(9606))?;
            path(
                &value,
                "/next/0/args",
                json!(["cell-line", "assay-count", chembl]),
            )?;
        }
        if id == "CVCL_0004" {
            path(&value, "/result/items/items/0/efo", json!("EFO_0002067"))?;
            path(&value, "/result/items/items/0/clo", json!("CLO_0007059"))?;
        }
        if id == "CVCL_2119"
            && (value.pointer("/result/items/items/0/efo").is_some()
                || value.pointer("/result/items/items/0/clo").is_some())
        {
            return Err("null ontology IDs became values".into());
        }
        markdown(&phase, &env, &args, &value)?;
    }
    let args = words(true);
    let value = parity("chembl/native/assays", &env, &args, 0, MEASURE)?;
    path(&value, "/result/assay_count", json!(1408))?;
    path(&value, "/result/id", json!("CHEMBL3706573"))?;
    path(&value, "/result/release/updated", json!("2026-05-01"))?;
    markdown("chembl/native/assays", &env, &args, &value)?;
    let requests = stub.finish()?;
    if requests.len() != 10
        || requests
            .iter()
            .filter(|(_, r)| r.contains("/assay.json?"))
            .count()
            != 1
    {
        return Err(format!("native request fanout changed: {}", requests.len()));
    }
    Ok(())
}
fn markdown(phase: &str, env: &Environment, words: &[&str], value: &Value) -> Result<(), String> {
    let recording = env.root().join("recordings").join(phase.replace(':', "_"));
    let mut args = words.to_vec();
    args.extend([
        "--replay",
        recording.to_str().ok_or("recording")?,
        "--format",
        "md",
    ]);
    let output = env
        .command(
            env!("CARGO_BIN_EXE_biomcp"),
            &args.iter().map(|s| s.to_string()).collect::<Vec<_>>(),
        )
        .output()
        .map_err(|e| e.to_string())?;
    if !output.status.success() && value.get("status") != Some(&json!("absent")) {
        return Err("Markdown failed".into());
    }
    let text = String::from_utf8(output.stdout).map_err(|e| e.to_string())?;
    let _ = writeln!(
        std::io::stderr(),
        "CELL_LINE_MEASURE {phase} [markdown] {}",
        text.len()
    );
    if !MEASURE && text.len() > ceiling(phase, "markdown")? {
        return Err("Markdown exceeds measured cap".into());
    }
    for fact in [
        value.pointer("/result/id"),
        value.pointer("/result/items/items/0/id"),
        value.pointer("/result/items/items/0/description"),
        value.pointer("/result/items/items/0/efo"),
        value.pointer("/result/items/items/0/clo"),
    ] {
        if let Some(fact) = fact.and_then(Value::as_str)
            && !text.contains(&super::super::source_terms::text::escape(fact))
        {
            return Err(format!("Markdown lost {fact}"));
        }
    }
    Ok(())
}

fn help() -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    discovery(&env)?;
    let args = ["help", "cell-line", "assay-count"];
    let (value, bytes) = invoke_measured(&env, &args, 0)?;
    let directory = env.root().join("help-mcp");
    fs::create_dir_all(&directory).map_err(|e| e.to_string())?;
    let mcp = super::super::mcp::call_retained(
        &env,
        &args.map(String::from),
        false,
        &[],
        Some(&directory),
    )?;
    if value != mcp {
        return Err("assay help CLI/MCP mismatch".into());
    }
    path(
        &value,
        "/result/commands/0/command",
        json!(["cell-line", "assay-count"]),
    )?;
    measure_bytes("chembl/help/assay-count", "cli", bytes, MEASURE)?;
    measure_mcp("chembl/help/assay-count", &directory, MEASURE)?;
    let output = env
        .command(
            env!("CARGO_BIN_EXE_biomcp"),
            &["help", "cell-line", "assay-count", "--format", "md"].map(String::from),
        )
        .output()
        .map_err(|e| e.to_string())?;
    if !output.status.success() {
        return Err("assay help Markdown failed".into());
    }
    let bytes = output.stdout.len();
    let _ = writeln!(
        std::io::stderr(),
        "CELL_LINE_MEASURE chembl/help/assay-count [markdown] {bytes}"
    );
    if !MEASURE && bytes > ceiling("chembl/help/assay-count", "markdown")? {
        return Err("assay help Markdown budget".into());
    }
    let text = String::from_utf8(output.stdout).map_err(|e| e.to_string())?;
    if !text.contains("assay-count") || !text.contains("id") {
        return Err("assay help lost command argument".into());
    }
    for args in [
        vec!["cell-line", "assay-count", "MOLM13"],
        vec!["cell-line", "assay-count", "CHEMBL3706573", "--limit", "1"],
        vec![
            "cell-line",
            "assay-count",
            "CHEMBL3706573",
            "--unknown",
            "x",
        ],
    ] {
        let rejected = invoke(&env, &args, 2)?;
        path(&rejected, "/status", json!("usage"))?;
    }
    Ok(())
}

pub(super) fn native_facts(value: &Value, id: &str) -> Result<(), String> {
    let native: Value = serde_json::from_str(&fixture(id)?).map_err(|e| e.to_string())?;
    let rows = native
        .get("cell_lines")
        .and_then(Value::as_array)
        .ok_or("native rows")?;
    let actual = value
        .pointer("/result/items/items")
        .and_then(Value::as_array)
        .ok_or("result rows")?;
    if rows.len() != actual.len() {
        return Err("metadata dropped complete source rows".into());
    }
    for (source, result) in rows.iter().zip(actual) {
        for (native, field) in [
            ("cell_chembl_id", "id"),
            ("cell_id", "cell_id"),
            ("cellosaurus_id", "cellosaurus_id"),
            ("cell_name", "name"),
            ("cell_description", "description"),
            ("cell_source_organism", "organism"),
            ("cell_source_tax_id", "taxon"),
            ("cell_source_tissue", "tissue"),
            ("clo_id", "clo"),
            ("efo_id", "efo"),
        ] {
            let expected = source.get(native).filter(|v| !v.is_null());
            if result.get(field) != expected {
                return Err(format!("{id}: lost native field {native}"));
            }
        }
    }
    Ok(())
}

fn discovery(env: &Environment) -> Result<(), String> {
    let replies = super::super::mcp::exchange(
        env,
        &[super::super::mcp::request(
            json!(1),
            "tools/list",
            json!({}),
        )],
    )?;
    let description = replies
        .first()
        .and_then(|r| r.pointer("/result/tools/0/description"))
        .and_then(Value::as_str)
        .ok_or("description")?;
    if !description.contains("cell-line assay-count") {
        return Err("MCP discovery lost assay-count".into());
    }
    let _ = writeln!(
        std::io::stderr(),
        "CHEMBL_DISCOVERY_DESCRIPTION_BYTES {}",
        description.len()
    );
    let output = env
        .command(env!("CARGO_BIN_EXE_biomcp"), &["help".to_string()])
        .output()
        .map_err(|e| e.to_string())?;
    if !output.status.success() {
        return Err("whole CLI help failed".into());
    }
    let _ = writeln!(
        std::io::stderr(),
        "CHEMBL_DISCOVERY_CLI_HELP_BYTES {}",
        output.stdout.len()
    );
    Ok(())
}
