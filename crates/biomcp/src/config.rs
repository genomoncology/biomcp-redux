//! The process argument and settings boundary.
use crate::grammar::Globals;
use biomcp_fetch::{Fetch, FetchConfig, Mode, ReplayCache, Secret};
use serde::Deserialize;
use std::{ffi::OsString, fs, path::PathBuf, time::Duration};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ConfigFile {
    proxy: Option<String>,
}
#[expect(
    clippy::disallowed_methods,
    reason = "Arguments enter only at the command boundary"
)]
pub(crate) fn arguments() -> Vec<OsString> {
    std::env::args_os().skip(1).collect()
}
#[expect(
    clippy::disallowed_methods,
    reason = "Only the settings boundary reads the process environment"
)]
fn variable(name: &str) -> Option<PathBuf> {
    std::env::var_os(name)
        .filter(|value| !value.is_empty())
        .map(PathBuf::from)
}

pub(crate) const BUDGET: Duration = Duration::from_secs(20);
pub(crate) fn fetch(globals: &Globals) -> Result<Fetch, ()> {
    let home = variable("HOME");
    let path = crate::paths::config(
        variable("XDG_CONFIG_HOME"),
        home.clone(),
        variable("APPDATA"),
    );
    let file = path
        .map(|path| match fs::read(path) {
            Ok(bytes) => serde_json::from_slice::<ConfigFile>(&bytes)
                .map(Some)
                .map_err(|_| ()),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(_) => Err(()),
        })
        .transpose()?
        .flatten();
    let cache = crate::paths::cache(variable("XDG_CACHE_HOME"), home, variable("LOCALAPPDATA"));
    let mode = if let Some(dir) = &globals.replay {
        Mode::Replay { dir: dir.clone() }
    } else if let Some(dir) = &globals.record {
        Mode::Record { dir: dir.clone() }
    } else if globals.offline {
        Mode::Offline
    } else {
        Mode::Live {
            fresh: globals.fresh,
        }
    };
    let replay_cache = globals
        .case_cache
        .as_ref()
        .map(|path| {
            if globals.replay.is_some() && !path.exists() {
                ReplayCache::create_case(path)
            } else {
                ReplayCache::open_case(path)
            }
        })
        .transpose()
        .map_err(|_| ())?;
    if !matches!(mode, Mode::Replay { .. })
        && replay_cache.is_none()
        && let Some(parent) = cache.as_ref().and_then(|path| path.parent())
    {
        fs::create_dir_all(parent).map_err(|_| ())?;
    }
    Fetch::open(FetchConfig {
        cache,
        mode,
        replay_cache,
        proxy: file.and_then(|file| file.proxy).map(Secret::new),
        ..FetchConfig::default()
    })
    .map_err(|_| ())
}
