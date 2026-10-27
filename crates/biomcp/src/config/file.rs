//! Configuration decoding retains field names without retaining rejected values.
use super::{SecretName, config_path};
use serde::de::DeserializeOwned;
use serde_json::{Map, Value};
use std::{collections::BTreeMap, fs, path::PathBuf};

#[derive(Clone, Debug)]
pub(crate) struct ConfigError(pub(crate) String);
impl ConfigError {
    pub(crate) fn field(name: &str) -> Self {
        // An unknown key is a name, but arbitrary JSON strings are not safe diagnostics.
        let safe = name.len() <= 96
            && name
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'.');
        Self(if safe {
            name.into()
        } else {
            "config.json".into()
        })
    }
}
pub(super) struct ConfigFile {
    pub(super) format: Option<crate::grammar::Format>,
    pub(super) proxy: Option<String>,
    pub(super) http: Option<Http>,
    pub(super) disable_sources: Option<Vec<String>>,
    pub(super) ca_bundle: Option<PathBuf>,
    pub(super) ncbi_email: Option<String>,
    pub(super) keys: Option<BTreeMap<String, String>>,
    pub(super) auto_sync_max_bytes: Option<u64>,
}
pub(super) struct Http {
    pub(super) allowed_hosts: Vec<String>,
}
fn take<T: DeserializeOwned>(
    object: &mut Map<String, Value>,
    name: &str,
) -> Result<Option<T>, ConfigError> {
    object
        .remove(name)
        .filter(|value| !value.is_null())
        .map(|value| serde_json::from_value(value).map_err(|_| ConfigError::field(name)))
        .transpose()
}
fn object(value: Value, name: &str) -> Result<Map<String, Value>, ConfigError> {
    match value {
        Value::Object(map) => Ok(map),
        _ => Err(ConfigError::field(name)),
    }
}
fn unknown(map: &Map<String, Value>, prefix: &str) -> Result<(), ConfigError> {
    if let Some(key) = map.keys().next() {
        return Err(ConfigError::field(&format!("{prefix}{key}")));
    }
    Ok(())
}
fn decode(bytes: &[u8]) -> Result<ConfigFile, ConfigError> {
    let value = serde_json::from_slice(bytes).map_err(|_| ConfigError::field("config.json"))?;
    let mut map = object(value, "config.json")?;
    let http = map
        .remove("http")
        .filter(|v| !v.is_null())
        .map(|value| {
            let mut map = object(value, "http")?;
            let allowed_hosts = take(&mut map, "allowed_hosts")
                .map_err(|_| ConfigError::field("http.allowed_hosts"))?
                .ok_or_else(|| ConfigError::field("http.allowed_hosts"))?;
            unknown(&map, "http.")?;
            Ok(Http { allowed_hosts })
        })
        .transpose()?;
    let keys = map
        .remove("keys")
        .filter(|v| !v.is_null())
        .map(|value| {
            let map = object(value, "keys")?;
            map.into_iter()
                .map(|(key, value)| {
                    let field = format!("keys.{key}");
                    if !SecretName::ALL.iter().any(|name| name.config() == key) {
                        return Err(ConfigError::field(&field));
                    }
                    let value =
                        serde_json::from_value(value).map_err(|_| ConfigError::field(&field))?;
                    Ok((key, value))
                })
                .collect::<Result<BTreeMap<String, String>, ConfigError>>()
        })
        .transpose()?;
    let file = ConfigFile {
        format: take(&mut map, "format")?,
        proxy: take(&mut map, "proxy")?,
        http,
        disable_sources: take(&mut map, "disable_sources")?,
        ca_bundle: take(&mut map, "ca_bundle")?,
        ncbi_email: take(&mut map, "ncbi_email")?,
        keys,
        auto_sync_max_bytes: take(&mut map, "auto_sync_max_bytes")?,
    };
    unknown(&map, "")?;
    Ok(file)
}
pub(super) fn read_file(
    override_path: Option<&PathBuf>,
) -> Result<Option<ConfigFile>, ConfigError> {
    config_path(override_path)
        .map(|path| match fs::read(path) {
            Ok(bytes) => decode(&bytes).map(Some),
            Err(error)
                if error.kind() == std::io::ErrorKind::NotFound && override_path.is_none() =>
            {
                Ok(None)
            }
            Err(_) => Err(ConfigError::field("config.json")),
        })
        .transpose()
        .map(Option::flatten)
}
