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
/// Saved work lives in the data folder, never the cache folder, so no cache cleanup,
/// predecessor scan or drop-and-rebuild can reach it.
pub(crate) fn saved(
    xdg: Option<PathBuf>,
    home: Option<PathBuf>,
    appdata: Option<PathBuf>,
) -> Option<PathBuf> {
    if cfg!(windows) {
        appdata.map(|root| root.join("biomcp/saved.sqlite"))
    } else {
        xdg.or_else(|| home.map(|root| root.join(".local/share")))
            .map(|root| root.join("biomcp/saved.sqlite"))
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
    for generation in biomcp_query::store::CACHE_NAMESPACES {
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

/// Resolve supported sibling files without opening them or changing their contents.
pub(crate) fn cache_predecessors(active: Option<&std::path::Path>) -> Vec<PathBuf> {
    let Some(active) = active else {
        return Vec::new();
    };
    active
        .parent()
        .into_iter()
        .flat_map(|folder| {
            biomcp_query::store::CACHE_NAMESPACES
                .iter()
                .map(move |generation| {
                    folder.join(format!(
                        "cache-{}-{generation}.sqlite",
                        biomcp_fetch::CACHE_NAMESPACE_VERSION
                    ))
                })
        })
        .filter(|path| path != active)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{cache, cache_predecessors, saved};
    use std::path::PathBuf;

    #[test]
    fn saved_file_is_outside_every_cache_path() {
        let home = Some(PathBuf::from("home"));
        let roaming = Some(PathBuf::from("roaming"));
        let local = Some(PathBuf::from("local"));
        let saved = saved(None, home.clone(), roaming).unwrap();
        let cache = cache(None, home, local).unwrap();
        assert_ne!(saved.parent(), cache.parent(), "saved work is outside the cache folder");
        assert!(
            !cache_predecessors(Some(&cache)).contains(&saved),
            "no cache generation names the saved file"
        );
        assert_eq!(saved.file_name().and_then(|name| name.to_str()), Some("saved.sqlite"), "saved file name");
    }
}
