//! Local build provenance and stdio client configuration.
use std::io;

#[derive(Debug)]
pub(crate) enum Command {
    Version { verbose: bool },
    McpConfig { client: Client },
}
#[derive(Clone, Copy, Debug, clap::ValueEnum)]
pub(crate) enum Client {
    #[value(name = "mcp-servers", alias = "json")]
    McpServers,
    Servers,
}
impl Command {
    pub(crate) fn render(&self, globals: &crate::grammar::Globals) -> io::Result<Vec<u8>> {
        let text = match self {
            Self::Version { verbose } => version(*verbose, globals),
            Self::McpConfig { client } => {
                let key = match client {
                    Client::McpServers => "mcpServers",
                    Client::Servers => "servers",
                };
                let command = crate::config::executable()?;
                let value = serde_json::json!({key: {"biomcp": {
                    "command": command, "args": ["serve"]
                }}});
                format!("{}\n", serde_json::to_string_pretty(&value)?)
            }
        };
        Ok(text.into_bytes())
    }
}
fn version(verbose: bool, globals: &crate::grammar::Globals) -> String {
    let mut text = format!("biomcp {}\n", env!("CARGO_PKG_VERSION"));
    if verbose {
        let versions = crate::schema::versions().join(", ");
        text.push_str(&format!(
            "Commit: {}\nTarget: {}\nSchemas: {versions}\nConfig: {}\nCache: {}\n",
            env!("BIOMCP_BUILD_COMMIT"),
            env!("BIOMCP_BUILD_TARGET"),
            display_path(crate::config::config_path(globals.config.as_ref())),
            display_path(crate::config::cache_path()),
        ));
    }
    text
}
fn display_path(path: Option<std::path::PathBuf>) -> String {
    path.map_or_else(|| "unavailable".into(), |path| path.display().to_string())
}

pub(crate) fn help() -> Vec<biomcp_types::HelpCommand> {
    crate::grammar::housekeeping_commands()
        .into_iter()
        .map(|command| biomcp_types::HelpCommand {
            command: vec![command.get_name().into()],
            description: command
                .get_about()
                .map(ToString::to_string)
                .unwrap_or_default(),
            arguments: command
                .get_arguments()
                .map(|arg| biomcp_types::HelpArgument {
                    name: format!("--{}", arg.get_long().unwrap_or_default()),
                    description: arg.get_help().map(ToString::to_string).unwrap_or_default(),
                    value_type: if arg.get_action().takes_values() {
                        "string"
                    } else {
                        "boolean"
                    }
                    .into(),
                    required: false,
                    position: None,
                    minimum: None,
                    maximum: None,
                    choices: arg
                        .get_value_parser()
                        .possible_values()
                        .into_iter()
                        .flatten()
                        .map(|value| value.get_name().to_owned())
                        .collect(),
                })
                .collect(),
            sections: Vec::new(),
            section_rule: None,
            examples: Vec::new(),
        })
        .collect()
}
