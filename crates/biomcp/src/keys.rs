//! Human-only credential registration; no credential enters a command argument.
use biomcp_fetch::SecretName;
use clap::{Arg, ArgAction};
use serde_json::{Value, json};
use std::io::{self, BufRead, IsTerminal};

#[derive(Debug)]
pub(crate) enum Command {
    Set(SecretName, bool),
    List,
    Remove(SecretName),
}
pub(crate) fn names() -> Vec<&'static str> {
    SecretName::ALL.iter().map(|name| name.source()).collect()
}
pub(crate) fn grammar() -> clap::Command {
    let source = || {
        Arg::new("source")
            .help("Source credential name to register or remove.")
            .required(true)
            .value_parser(names())
    };
    clap::Command::new("key")
        .after_help("Example: biomcp key list")
        .subcommand_required(true)
        .subcommand(clap::Command::new("list").after_help("Example: biomcp key list"))
        .subcommand(
            clap::Command::new("set")
                .after_help("Example: biomcp key set oncokb")
                .arg(source())
                .arg(
                    Arg::new("replace")
                        .long("replace")
                        .help("Replace an existing stored credential.")
                        .action(ArgAction::SetTrue),
                ),
        )
        .subcommand(
            clap::Command::new("remove")
                .after_help("Example: biomcp key remove oncokb")
                .arg(source()),
        )
}
pub(crate) fn parse(matches: &clap::ArgMatches) -> Result<Command, crate::grammar::ParseInvalid> {
    let (verb, matches) = matches
        .subcommand()
        .ok_or_else(crate::grammar::ParseInvalid::default)?;
    if verb == "list" {
        return Ok(Command::List);
    }
    let source = matches
        .get_one::<String>("source")
        .ok_or_else(crate::grammar::ParseInvalid::default)?;
    let name = SecretName::ALL
        .iter()
        .copied()
        .find(|n| n.source() == source)
        .ok_or_else(crate::grammar::ParseInvalid::default)?;
    Ok(if verb == "set" {
        Command::Set(name, matches.get_flag("replace"))
    } else {
        Command::Remove(name)
    })
}
pub(crate) fn run(
    command: Command,
    globals: &crate::grammar::Globals,
) -> io::Result<crate::command::Outcome> {
    match run_inner(command, globals) {
        Ok(value) => crate::command::encode(
            biomcp_types::Envelope::new(
                biomcp_types::Status::Complete,
                biomcp_types::EmptyQuery {},
                Some(value),
            ),
            crate::exit::Status::Success,
        ),
        Err(Failure::Usage) => crate::command::encode(
            biomcp_types::ErrorEnvelope::from(biomcp_types::UsageError::arguments()),
            crate::exit::Status::Usage,
        ),
        Err(Failure::Configuration(field)) => crate::command::encode(
            biomcp_types::ErrorEnvelope::configuration(field),
            crate::exit::Status::Local,
        ),
        Err(Failure::Replace) => crate::command::encode(
            biomcp_types::ErrorEnvelope::from(biomcp_types::UsageError::argument(
                "--replace".into(),
            )),
            crate::exit::Status::Usage,
        ),
        Err(Failure::Local) => crate::command::encode(
            biomcp_types::ErrorEnvelope::local(),
            crate::exit::Status::Local,
        ),
    }
}
enum Failure {
    Configuration(String),
    Usage,
    Replace,
    Local,
}
impl From<io::Error> for Failure {
    fn from(_: io::Error) -> Self {
        Self::Local
    }
}
fn run_inner(
    command: Command,
    globals: &crate::grammar::Globals,
) -> Result<biomcp_types::RegisteredKeys, Failure> {
    let path = crate::config::config_path(globals.config.as_ref())
        .map_err(|error| Failure::Configuration(error.0))?
        .ok_or(Failure::Local)?;
    // Validate the same config contract used by ordinary commands before preserving it.
    if path.exists() {
        crate::config::secrets(globals).map_err(|error| Failure::Configuration(error.0))?;
    }
    if matches!(command, Command::List) {
        let value = crate::config::key_file::read(&path)?;
        return Ok(biomcp_types::RegisteredKeys {
            entries: SecretName::ALL
                .iter()
                .map(|&name| entry(name, state(&value, name)?))
                .collect::<Result<_, _>>()?,
        });
    }
    let _lock = crate::config::key_file::lock(&path)?;
    let mut value = crate::config::key_file::read(&path)?;
    let (name, status) = match command {
        Command::Set(name, replace) => {
            if stored(&value, name).is_some() && !replace {
                return Err(Failure::Replace);
            }
            let key = input()?;
            let object = value.as_object_mut().ok_or(Failure::Local)?;
            let keys = object.entry("keys").or_insert_with(|| json!({}));
            if keys.is_null() {
                *keys = json!({});
            }
            keys.as_object_mut()
                .ok_or(Failure::Local)?
                .insert(name.config().into(), Value::String(key));
            (name, biomcp_types::KeyState::Stored)
        }
        Command::Remove(name) => {
            if let Some(keys) = value.get_mut("keys").and_then(Value::as_object_mut) {
                keys.remove(name.config());
            }
            (name, biomcp_types::KeyState::None)
        }
        Command::List => return Err(Failure::Local),
    };
    crate::config::key_file::write(&path, &value)?;
    Ok(biomcp_types::RegisteredKeys {
        entries: vec![entry(name, status)?],
    })
}
fn stored(value: &Value, name: SecretName) -> Option<&str> {
    value
        .get("keys")?
        .get(name.config())?
        .as_str()
        .filter(|s| !s.is_empty())
}
fn state(value: &Value, name: SecretName) -> Result<biomcp_types::KeyState, Failure> {
    Ok(if stored(value, name).is_some() {
        biomcp_types::KeyState::Stored
    } else if crate::config::has_environment_key(name)
        .map_err(|error| Failure::Configuration(error.0))?
    {
        biomcp_types::KeyState::Environment
    } else {
        biomcp_types::KeyState::None
    })
}
fn entry(
    name: SecretName,
    state: biomcp_types::KeyState,
) -> Result<biomcp_types::RegisteredKey, Failure> {
    let text = |s: String| biodata::Text::new(&s).map_err(|_| Failure::Local);
    Ok(biomcp_types::RegisteredKey {
        source: text(name.source().into())?,
        config: text(format!("keys.{}", name.config()))?,
        environment: text(name.environment().into())?,
        header: text(name.header())?,
        tier: biomcp_query::sources::key_tier(name),
        state,
    })
}
fn input() -> Result<String, Failure> {
    let mut value = if io::stdin().is_terminal() {
        rpassword::prompt_password("API key: ")?
    } else {
        let mut bytes = Vec::new();
        use std::io::Read;
        io::stdin()
            .lock()
            .take(8194)
            .read_until(b'\n', &mut bytes)?;
        if bytes.len() > 8193 {
            return Err(Failure::Usage);
        }
        String::from_utf8(bytes).map_err(|_| Failure::Usage)?
    };
    value.truncate(value.trim_end_matches(['\n', '\r']).len());
    if value.is_empty() || value.len() > 8192 || value.chars().any(char::is_control) {
        return Err(Failure::Usage);
    }
    Ok(value)
}
