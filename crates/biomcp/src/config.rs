//! The process argument and settings boundary.
pub(crate) mod key_file;
use crate::grammar::Globals;
use biomcp_fetch::{Fetch, FetchConfig, Mode, ReplayCache, Secret, SecretName, Secrets};
use serde::Deserialize;
use std::{ffi::OsString, fs, path::PathBuf, time::Duration};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ConfigFile {
    format: Option<crate::grammar::Format>,
    proxy: Option<String>,
    http: Option<Http>,
    disable_sources: Option<Vec<String>>,
    ca_bundle: Option<PathBuf>,
    ncbi_email: Option<String>,
    keys: Option<std::collections::BTreeMap<String, String>>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Http {
    allowed_hosts: Vec<String>,
}
pub(crate) fn http_hosts(globals: &Globals) -> Result<Vec<String>, ()> {
    Ok(read_file(globals.config.as_ref())?
        .and_then(|file| file.http)
        .map(|http| http.allowed_hosts)
        .unwrap_or_default())
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

pub(crate) fn retired_study_directory() -> bool {
    variable("BIOMCP_STUDY_DIR").is_some()
}

pub(crate) const BUDGET: Duration = Duration::from_secs(20);
pub(crate) fn fetch(globals: &Globals) -> Result<Fetch, ()> {
    let home = variable("HOME");
    let file = read_file(globals.config.as_ref())?;
    fetch_using(globals, home, file)
}
fn read_file(override_path: Option<&PathBuf>) -> Result<Option<ConfigFile>, ()> {
    let path = config_path(override_path);
    let file = path
        .map(|path| match fs::read(path) {
            Ok(bytes) => serde_json::from_slice::<ConfigFile>(&bytes)
                .map(Some)
                .map_err(|_| ()),
            Err(error)
                if error.kind() == std::io::ErrorKind::NotFound && override_path.is_none() =>
            {
                Ok(None)
            }
            Err(_) => Err(()),
        })
        .transpose()?
        .flatten();
    if file
        .as_ref()
        .and_then(|f| f.keys.as_ref())
        .is_some_and(|keys| {
            keys.keys()
                .any(|key| !SecretName::ALL.iter().any(|n| n.config() == key))
        })
    {
        return Err(());
    }
    Ok(file)
}
pub(crate) fn ncbi_email(globals: &Globals) -> Result<Option<Secret>, ()> {
    Ok(read_file(globals.config.as_ref())?
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
    let secrets = secrets_from(file.as_ref());
    let network = network(file.as_ref())?;
    Fetch::open(FetchConfig {
        disabled_sources: disabled_sources(globals, file.as_ref())?,
        disabled_uses: biomcp_adapters::registry::disabled_uses(&disabled_names(
            globals,
            file.as_ref(),
        )),
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

fn disabled_sources(
    globals: &Globals,
    file: Option<&ConfigFile>,
) -> Result<std::collections::BTreeSet<biomcp_types::Source>, ()> {
    biomcp_adapters::registry::disabled(&disabled_names(globals, file)).map_err(|_| ())
}
fn disabled_names(globals: &Globals, file: Option<&ConfigFile>) -> Vec<String> {
    let mut names = globals.disable_sources.clone();
    names.extend(
        file.and_then(|f| f.disable_sources.clone())
            .unwrap_or_default(),
    );
    names
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

pub(crate) fn format(path: Option<&PathBuf>) -> Option<crate::grammar::Format> {
    read_file(path).ok().flatten().and_then(|file| file.format)
}

pub(crate) fn config_path(override_path: Option<&PathBuf>) -> Option<PathBuf> {
    override_path.cloned().or_else(|| {
        crate::paths::config(
            variable("XDG_CONFIG_HOME"),
            variable("HOME"),
            variable("APPDATA"),
        )
    })
}
pub(crate) fn secrets(globals: &Globals) -> Result<Secrets, ()> {
    Ok(secrets_from(read_file(globals.config.as_ref())?.as_ref()))
}
fn secrets_from(file: Option<&ConfigFile>) -> Secrets {
    let mut secrets = Secrets::default();
    let keys = file.and_then(|file| file.keys.clone());
    let deferred_keys = keys.clone();
    secrets.defer(SecretName::SemanticScholarApiKey, move || {
        configured_secret(deferred_keys.as_ref(), SecretName::SemanticScholarApiKey)
    });
    for &name in SecretName::ALL
        .iter()
        .filter(|&&name| name != SecretName::SemanticScholarApiKey)
    {
        if let Some(value) = configured_secret(keys.as_ref(), name) {
            secrets.insert(name, value);
        }
    }
    secrets
}
fn configured_secret(
    keys: Option<&std::collections::BTreeMap<String, String>>,
    name: SecretName,
) -> Option<Secret> {
    keys.and_then(|keys| keys.get(name.config()))
        .filter(|v| !v.is_empty())
        .cloned()
        .or_else(|| variable(name.environment()).map(|v| v.to_string_lossy().into_owned()))
        .map(Secret::new)
}

pub(crate) fn has_environment_key(name: SecretName) -> bool {
    variable(name.environment()).is_some()
}

/// Apply per-call policy as a union with the already configured transport policy.
pub(crate) fn restrict(fetch: Fetch, globals: &Globals) -> std::io::Result<Fetch> {
    let disabled = biomcp_adapters::registry::disabled(&globals.disable_sources)
        .map_err(|_| std::io::Error::other("unchecked source policy"))?;
    Ok(fetch
        .restrict_sources(disabled)
        .restrict_uses(biomcp_adapters::registry::disabled_uses(
            &globals.disable_sources,
        )))
}
