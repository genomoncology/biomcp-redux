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
    if cfg!(windows) {
        localappdata.map(|root| root.join(format!("biomcp/cache-{}-{}.sqlite",biomcp_fetch::VERSION,biomcp_query::store::VERSION)))
    } else {
        xdg.or_else(|| home.map(|root| root.join(".cache")))
            .map(|root| root.join(format!("biomcp/cache-{}-{}.sqlite",biomcp_fetch::VERSION,biomcp_query::store::VERSION)))
    }
}
