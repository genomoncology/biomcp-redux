//! Feature diagnostic-cdx: synthetic stored companion rows retain owned identities.
use super::{
    configuration::invoke,
    launch::{Environment, fixtures},
};
use serde_json::{Value, json};
use std::{
    fs,
    path::{Path, PathBuf},
};

pub(super) fn seed(env: &Environment) -> Result<PathBuf, String> {
    let source = fixtures().join("replies/diagnostic/fda-cdx-synthetic/fda-cdx");
    let replay = env.root().join("replay");
    fs::create_dir_all(replay.join("fda-cdx")).map_err(|e| e.to_string())?;
    for entry in fs::read_dir(source).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        fs::copy(entry.path(), replay.join("fda-cdx").join(entry.file_name()))
            .map_err(|e| e.to_string())?;
    }
    Ok(replay)
}
