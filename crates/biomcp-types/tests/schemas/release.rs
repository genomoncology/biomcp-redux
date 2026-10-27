//! Compare generated contracts with an independently recorded release baseline.
use super::{compatibility, registry};
use serde_json::Value;
use std::{collections::BTreeMap, fs, path::Path};

type Check<T> = Result<T, Box<dyn std::error::Error>>;

#[expect(
    clippy::disallowed_methods,
    reason = "The release contract test owns baseline file and environment I/O"
)]
pub(super) fn generated_schemas_preserve_release_contracts() -> Check<()> {
    let Some(baseline) = std::env::var_os("BIOMCP_SCHEMA_BASELINE") else {
        return Ok(());
    };
    let mut old = read_baseline(Path::new(&baseline))?;
    for contract in registry() {
        let mut new = (contract.generate)();
        new.as_object_mut()
            .ok_or("generated schema is not an object")?
            .insert("$id".into(), contract.id.into());
        compatibility::check(&new, &new)?;
        if let Some(previous) = old.remove(contract.name) {
            compatibility::check(&previous, &new)
                .map_err(|error| format!("{}: {error}", contract.name))?;
        }
    }
    if !old.is_empty() {
        return Err(format!("removed released schemas: {:?}", old.keys()).into());
    }
    Ok(())
}

#[expect(
    clippy::disallowed_methods,
    reason = "Read the independently supplied schema snapshot at the release contract boundary"
)]
fn read_baseline(root: &Path) -> Check<BTreeMap<String, Value>> {
    let mut schemas = BTreeMap::new();
    for entry in fs::read_dir(root)? {
        let path = entry?.path();
        if path.extension().is_none_or(|extension| extension != "json") {
            return Err(format!("unexpected baseline file: {}", path.display()).into());
        }
        let name = path
            .file_stem()
            .and_then(|name| name.to_str())
            .ok_or("invalid baseline filename")?
            .to_owned();
        let value = serde_json::from_slice(&fs::read(path)?)?;
        compatibility::check(&value, &value)?;
        schemas.insert(name, value);
    }
    if schemas.is_empty() {
        return Err("empty schema baseline".into());
    }
    Ok(schemas)
}
