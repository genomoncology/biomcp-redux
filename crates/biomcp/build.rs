#![expect(clippy::disallowed_methods, clippy::print_stdout, reason = "Build scripts read Cargo environment and emit Cargo directives")]
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
        let id = value.get("$id").and_then(serde_json::Value::as_str)
            .ok_or("schema artifact requires a string $id")?.to_owned();
        if schemas.insert(id, text).is_some() {
            return Err("duplicate schema artifact $id".into());
        }
    }
    let entries = schemas.iter().map(|(id, text)| format!("({id:?}, {text:?}),\n")).collect::<String>();
    let output = PathBuf::from(env::var("OUT_DIR")?).join("schemas.rs");
    fs::write(output, format!("&[\n{entries}]\n"))?;
    Ok(())
}
