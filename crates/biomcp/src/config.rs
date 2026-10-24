//! The process argument and settings boundary.
use crate::grammar::Globals;
use biomcp_fetch::{Fetch, FetchConfig, Mode, ReplayCache, Secret, SecretName, Secrets};
use serde::Deserialize;
use std::{ffi::OsString, fs, path::PathBuf, time::Duration};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ConfigFile {
    proxy: Option<String>,
    ca_bundle: Option<PathBuf>,
    ncbi_email: Option<String>,
    keys: Option<Keys>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Keys {
    oncokb_api_key: Option<String>,
    ncbi_api_key: Option<String>,
    semantic_scholar_api_key: Option<String>,
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
    let file = read_file()?;
    fetch_using(globals, home, file)
}
fn read_file() -> Result<Option<ConfigFile>, ()> {
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
    Ok(file)
}
pub(crate) fn ncbi_email() -> Result<Option<Secret>, ()> {
    Ok(read_file()?
        .and_then(|file| file.ncbi_email)
        .filter(|s| !s.is_empty())
        .map(Secret::new))
}
fn fetch_using(
    globals: &Globals,
    home: Option<PathBuf>,
    file: Option<ConfigFile>,
) -> Result<Fetch, ()> {
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
                ReplayCache::create_case_with_owners(path, &[biomcp_query::store::OWNER])
            } else {
                ReplayCache::open_case_with_owners(path, &[biomcp_query::store::OWNER])
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
    let mut secrets = Secrets::default();
    let key = variable("ONCOKB_API_KEY")
        .map(|value| value.to_string_lossy().into_owned())
        .or_else(|| {
            file.as_ref()
                .and_then(|file| file.keys.as_ref())
                .and_then(|keys| keys.oncokb_api_key.clone())
        });
    if let Some(key) = key {
        secrets.insert(SecretName::OncokbApiKey, Secret::new(key));
    }
    let ncbi = variable("NCBI_API_KEY")
        .map(|v| v.to_string_lossy().into_owned())
        .or_else(|| {
            file.as_ref()
                .and_then(|f| f.keys.as_ref())
                .and_then(|k| k.ncbi_api_key.clone())
        });
    if let Some(key) = ncbi {
        secrets.insert(SecretName::NcbiApiKey, Secret::new(key));
    }
    let scholar = variable("SEMANTIC_SCHOLAR_API_KEY")
        .map(|v| v.to_string_lossy().into_owned())
        .or_else(|| {
            file.as_ref()
                .and_then(|f| f.keys.as_ref())
                .and_then(|k| k.semantic_scholar_api_key.clone())
        });
    if let Some(key) = scholar {
        secrets.insert(SecretName::SemanticScholarApiKey, Secret::new(key));
    }
    let network = network(file.as_ref())?;
    Fetch::open(FetchConfig {
        observations: globals
            .observe
            .as_ref()
            .map(|path| biomcp_fetch::Observations::create(path).map(std::sync::Arc::new))
            .transpose()
            .map_err(|_| ())?,
        cache_owners: vec![biomcp_query::store::OWNER],
        secrets,
        network,
        cache,
        mode,
        replay_cache,
        proxy: file.and_then(|file| file.proxy).map(Secret::new),
        ..FetchConfig::default()
    })
    .map_err(|_| ())
}

fn network(file: Option<&ConfigFile>) -> Result<biomcp_fetch::Network, ()> {
    use std::io::Read;
    const MAX_PEM_BYTES: u64 = 4 * 1024 * 1024;
    let roots = file
        .and_then(|file| file.ca_bundle.as_ref())
        .map(|path| {
            let mut bytes = Vec::new();
            fs::File::open(path)
                .map_err(|_| ())?
                .take(MAX_PEM_BYTES + 1)
                .read_to_end(&mut bytes)
                .map_err(|_| ())?;
            if bytes.len() as u64 > MAX_PEM_BYTES {
                return Err(());
            }
            biomcp_fetch::Certificates::from_pem(&bytes).map_err(|_| ())
        })
        .transpose()?
        .unwrap_or_default();
    let explicit = file.is_some_and(|file| file.proxy.is_some());
    let proxy = |name| {
        if explicit {
            None
        } else {
            variable(name).map(|v| Secret::new(v.to_string_lossy().into_owned()))
        }
    };
    let bypass = if explicit { None } else { variable("NO_PROXY") };
    biomcp_fetch::Network::new(
        proxy("HTTP_PROXY"),
        proxy("HTTPS_PROXY"),
        &bypass
            .map(|v| v.to_string_lossy().into_owned())
            .unwrap_or_default(),
        roots,
    )
    .map_err(|_| ())
}
