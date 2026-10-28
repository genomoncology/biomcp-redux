//! The process argument and settings boundary.
pub(crate) mod key_file;
use crate::grammar::Globals;
use biomcp_fetch::{Fetch, FetchConfig, Mode, ReplayCache, Secret, SecretName, Secrets};
use std::{ffi::OsString, fs, path::PathBuf, sync::Arc, time::Duration};
#[cfg(test)]
mod deferred_keys_tests;

mod file;
pub(crate) use file::ConfigError;
use file::{ConfigFile, read_file};
pub(crate) fn http_hosts(globals: &Globals) -> Result<Vec<String>, ConfigError> {
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
#[derive(Clone, Debug)]
pub(crate) enum StartupError {
    Cache(biomcp_fetch::CacheError),
    Configuration(ConfigError),
    Local,
}
impl From<ConfigError> for StartupError {
    fn from(error: ConfigError) -> Self {
        Self::Configuration(error)
    }
}
impl From<()> for StartupError {
    fn from((): ()) -> Self {
        Self::Local
    }
}
impl From<biomcp_fetch::CacheError> for StartupError {
    fn from(error: biomcp_fetch::CacheError) -> Self {
        Self::Cache(error)
    }
}
pub(crate) fn fetch(globals: &Globals) -> Result<Fetch, StartupError> {
    let home = variable("HOME");
    let file = read_file(globals.config.as_ref())?;
    fetch_using(globals, home, file)
}
pub(crate) fn ncbi_email(globals: &Globals) -> Result<Option<Secret>, ConfigError> {
    Ok(read_file(globals.config.as_ref())?
        .and_then(|file| file.ncbi_email)
        .filter(|s| !s.is_empty())
        .map(Secret::new))
}
fn fetch_using(
    globals: &Globals,
    home: Option<PathBuf>,
    file: Option<ConfigFile>,
) -> Result<Fetch, StartupError> {
    let cache = cache_path_using(home);
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
        .map_err(StartupError::from)?;
    if !matches!(mode, Mode::Replay { .. })
        && replay_cache.is_none()
        && let Some(parent) = cache.as_ref().and_then(|path| path.parent())
    {
        create_cache_directory(parent).map_err(|_| ())?;
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
        cache_predecessors: crate::paths::cache_predecessors(cache.as_deref()),
        cache,
        mode,
        replay_cache,
        auto_sync_max_bytes: file
            .as_ref()
            .and_then(|file| file.auto_sync_max_bytes)
            .unwrap_or_else(|| FetchConfig::default().auto_sync_max_bytes),
        proxy: file.and_then(|file| file.proxy).map(Secret::new),
        ..FetchConfig::default()
    })
    .map_err(|error| match error {
        biomcp_fetch::FetchError::Cache(error) => error.into(),
        biomcp_fetch::FetchError::Proxy => ConfigError::field("proxy").into(),
        _ => ().into(),
    })
}

fn create_cache_directory(path: &std::path::Path) -> std::io::Result<()> {
    let mut directory = fs::DirBuilder::new();
    directory.recursive(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::DirBuilderExt;
        directory.mode(0o700);
    }
    directory.create(path)
}

fn disabled_sources(
    globals: &Globals,
    file: Option<&ConfigFile>,
) -> Result<std::collections::BTreeSet<biomcp_types::Source>, ConfigError> {
    biomcp_adapters::registry::disabled(&disabled_names(globals, file))
        .map_err(|_| ConfigError::field("disable_sources"))
}
fn disabled_names(globals: &Globals, file: Option<&ConfigFile>) -> Vec<String> {
    let mut names = globals.disable_sources.clone();
    names.extend(
        file.and_then(|f| f.disable_sources.clone())
            .unwrap_or_default(),
    );
    names
}

fn network(file: Option<&ConfigFile>) -> Result<biomcp_fetch::Network, ConfigError> {
    use std::io::Read;
    const MAX_PEM_BYTES: u64 = 4 * 1024 * 1024;
    let roots = file
        .and_then(|file| file.ca_bundle.as_ref())
        .map(|path| {
            let mut bytes = Vec::new();
            fs::File::open(path)
                .map_err(|_| ConfigError::field("ca_bundle"))?
                .take(MAX_PEM_BYTES + 1)
                .read_to_end(&mut bytes)
                .map_err(|_| ConfigError::field("ca_bundle"))?;
            if bytes.len() as u64 > MAX_PEM_BYTES {
                return Err(ConfigError::field("ca_bundle"));
            }
            biomcp_fetch::Certificates::from_pem(&bytes)
                .map_err(|_| ConfigError::field("ca_bundle"))
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
    .map_err(|error| {
        ConfigError::field(match error {
            biomcp_fetch::FetchError::HttpProxy => "HTTP_PROXY",
            biomcp_fetch::FetchError::HttpsProxy => "HTTPS_PROXY",
            _ => "NO_PROXY",
        })
    })
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
pub(crate) fn secrets(globals: &Globals) -> Result<Secrets, ConfigError> {
    Ok(secrets_from(read_file(globals.config.as_ref())?.as_ref()))
}
fn secrets_from(file: Option<&ConfigFile>) -> Secrets {
    secrets_using(file.and_then(|file| file.keys.clone()), variable)
}
fn secrets_using(
    keys: Option<std::collections::BTreeMap<String, String>>,
    environment: impl Fn(&str) -> Option<PathBuf> + Send + Sync + 'static,
) -> Secrets {
    let mut secrets = Secrets::default();
    let keys = Arc::new(keys);
    let environment = Arc::new(environment);
    for &name in SecretName::ALL {
        let keys = keys.clone();
        let environment = environment.clone();
        secrets.defer(name, move || {
            configured_secret(keys.as_ref().as_ref(), name, environment.as_ref())
        });
    }
    secrets
}
fn configured_secret(
    keys: Option<&std::collections::BTreeMap<String, String>>,
    name: SecretName,
    environment: &impl Fn(&str) -> Option<PathBuf>,
) -> Option<Secret> {
    keys.and_then(|keys| keys.get(name.config()))
        .filter(|v| !v.is_empty())
        .cloned()
        .or_else(|| environment(name.environment()).map(|v| v.to_string_lossy().into_owned()))
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

pub(crate) fn cache_path() -> Option<PathBuf> {
    cache_path_using(variable("HOME"))
}
fn cache_path_using(home: Option<PathBuf>) -> Option<PathBuf> {
    crate::paths::cache(variable("XDG_CACHE_HOME"), home, variable("LOCALAPPDATA"))
}
#[expect(
    clippy::disallowed_methods,
    reason = "The settings boundary resolves the running executable"
)]
pub(crate) fn executable() -> std::io::Result<PathBuf> {
    let executable = std::env::current_exe()?;
    let Some(path) = variable("PATH") else {
        return Ok(executable);
    };
    let identity = executable.canonicalize()?;
    for directory in std::env::split_paths(&path) {
        let candidate = directory.join(format!("biomcp{}", std::env::consts::EXE_SUFFIX));
        if candidate.canonicalize().is_ok_and(|path| path == identity) {
            return std::path::absolute(candidate);
        }
    }
    Ok(executable)
}
