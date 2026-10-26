//! Resolve explicit folders supplied by the settings boundary.
use std::path::PathBuf;
pub(crate) fn config(
    xdg: Option<PathBuf>,
    home: Option<PathBuf>,
    appdata: Option<PathBuf>,
) -> Option<PathBuf> {
    if cfg!(windows) {
        appdata.map(|root| root.join("biomcp/config.json"))
    } else {
        xdg.or_else(|| home.map(|root| root.join(".config")))
            .map(|root| root.join("biomcp/config.json"))
    }
}
pub(crate) fn cache(
    xdg: Option<PathBuf>,
    home: Option<PathBuf>,
    localappdata: Option<PathBuf>,
) -> Option<PathBuf> {
    let root = if cfg!(windows) {
        localappdata
    } else {
        xdg.or_else(|| home.map(|root| root.join(".cache")))
    }?;
    let folder = root.join("biomcp");
    // Reuse the last owned pre-migration layout without renaming live SQLite files.
    for generation in [biomcp_query::store::CACHE_NAMESPACE_VERSION, 4] {
        let path = folder.join(format!(
            "cache-{}-{generation}.sqlite",
            biomcp_fetch::CACHE_NAMESPACE_VERSION
        ));
        if path.exists() {
            return Some(path);
        }
    }
    Some(folder.join(format!(
        "cache-{}-{}.sqlite",
        biomcp_fetch::CACHE_NAMESPACE_VERSION,
        biomcp_query::store::CACHE_NAMESPACE_VERSION
    )))
}
