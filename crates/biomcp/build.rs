//! Embed the checked-in schema artifacts by their canonical identities.
#![expect(
    clippy::disallowed_methods,
    reason = "Build scripts read Cargo environment and emit Cargo directives"
)]
use std::{collections::BTreeMap, env, error::Error, fs, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    let directory = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?).join("../biodata/schemas");
    println!("cargo:rerun-if-changed={}", directory.display());
    let mut schemas = BTreeMap::new();
    for entry in fs::read_dir(&directory)? {
        let path = entry?.path();
        if path.extension().is_none_or(|extension| extension != "json") {
            continue;
        }
        println!("cargo:rerun-if-changed={}", path.display());
        let text = fs::read_to_string(&path)?;
        let value: serde_json::Value = serde_json::from_str(&text)?;
        let id = value
            .get("$id")
            .and_then(serde_json::Value::as_str)
            .ok_or("schema artifact requires a string $id")?
            .to_owned();
        if schemas.insert(id, text).is_some() {
            return Err("duplicate schema artifact $id".into());
        }
    }
    let entries = schemas
        .iter()
        .map(|(id, text)| format!("({id:?}, {text:?}),\n"))
        .collect::<String>();
    let output = PathBuf::from(env::var("OUT_DIR")?).join("schemas.rs");
    fs::write(output, format!("&[\n{entries}]\n"))?;
    let cases = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?).join("../../tests/cases");
    let mut examples = BTreeMap::<String, Vec<Vec<String>>>::new();
    let mut witnesses = Vec::new();
    for (file, index) in [
        ("gene/kras.yaml", 0),
        ("gene/braf.yaml", 0),
        ("gene/search.yaml", 0),
        ("gene/search.yaml", 1),
        ("article/search/pubmed-floor.yaml", 0),
        ("article/filters/since.yaml", 0),
        ("article/get/card.yaml", 0),
        ("article/preprint/reader.yaml", 0),
        ("article/fulltext/outline.yaml", 2),
        ("article/fulltext/variant.yaml", 0),
        ("article/supplements/workbook.yaml", 0),
        ("article/supplements/protein-change.yaml", 0),
        ("article/citations/citing.yaml", 0),
        ("article/edges/window.yaml", 0),
        ("article/citations/references.yaml", 0),
        ("article/edges/references.yaml", 0),
    ] {
        let path = cases.join(file);
        println!("cargo:rerun-if-changed={}", path.display());
        let case: serde_json::Value = serde_saphyr::from_str(&fs::read_to_string(path)?)?;
        let steps = case
            .get("steps")
            .and_then(serde_json::Value::as_array)
            .ok_or("case steps required")?;
        let run: Vec<String> = serde_json::from_value(
            steps
                .get(index)
                .and_then(|step| step.get("run"))
                .ok_or("missing example step")?
                .clone(),
        )?;
        let command = run
            .get(..2)
            .ok_or("example needs a verb and entity")?
            .join(" ");
        examples.entry(command).or_default().push(run);
        witnesses.push((file, index));
    }
    fs::write(
        PathBuf::from(env::var("OUT_DIR")?).join("examples.json"),
        serde_json::to_vec(&examples)?,
    )?;
    fs::write(
        PathBuf::from(env::var("OUT_DIR")?).join("example-cases.json"),
        serde_json::to_vec(&witnesses)?,
    )?;
    Ok(())
}
