//! Local build provenance and stdio client configuration.
use std::io;

#[derive(Debug)]
pub(crate) enum Command {
    Skill(crate::skill::Skill),
    Version { verbose: bool },
    McpConfig { client: Client },
}
#[derive(Clone, Copy, Debug, clap::ValueEnum)]
pub(crate) enum Client {
    #[value(name = "mcp-servers", aliases = ["json", "claude-desktop", "claude-code", "cursor", "cline"])]
    McpServers,
    #[value(alias = "vscode")]
    Servers,
    Codex,
}
impl Command {
    pub(crate) fn render(&self, globals: &crate::grammar::Globals) -> io::Result<Vec<u8>> {
        let text = match self {
            Self::Skill(command) => command.render()?,
            Self::Version { verbose } => version(*verbose, globals),
            Self::McpConfig { client } => mcp_config(*client)?,
        };
        Ok(text.into_bytes())
    }
}
fn mcp_config(client: Client) -> io::Result<String> {
    let command = crate::config::executable()?;
    let key = match client {
        Client::McpServers => "mcpServers",
        Client::Servers => "servers",
        Client::Codex => {
            let path = command
                .to_str()
                .ok_or_else(|| io::Error::other("executable path is not Unicode"))?;
            return shlex::try_join(["codex", "mcp", "add", "biomcp", "--", path, "serve"])
                .map(|text| format!("{text}\n"))
                .map_err(io::Error::other);
        }
    };
    let value = serde_json::json!({key: {"biomcp": {"command": command, "args": ["serve"]}}});
    Ok(format!("{}\n", serde_json::to_string_pretty(&value)?))
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
        .flat_map(|command| {
            let name = command.get_name().to_owned();
            let mut entries = vec![(vec![name.clone()], command.clone())];
            entries.extend(
                command
                    .get_subcommands()
                    .map(|child| (vec![name.clone(), child.get_name().into()], child.clone())),
            );
            entries
        })
        .map(|(path, command)| biomcp_types::HelpCommand {
            command: path,
            description: command
                .get_about()
                .map(ToString::to_string)
                .unwrap_or_default(),
            arguments: command
                .get_arguments()
                .map(|arg| biomcp_types::HelpArgument {
                    name: arg
                        .get_long()
                        .map_or_else(|| arg.get_id().to_string(), |name| format!("--{name}")),
                    description: arg.get_help().map(ToString::to_string).unwrap_or_default(),
                    value_type: if arg.get_action().takes_values() {
                        "string"
                    } else {
                        "boolean"
                    }
                    .into(),
                    required: arg.is_required_set(),
                    position: arg.get_index().map(|index| (index - 1) as u64),
                    minimum: None,
                    maximum: None,
                    choices: arg
                        .get_value_parser()
                        .possible_values()
                        .into_iter()
                        .flatten()
                        .flat_map(|value| {
                            value
                                .get_name_and_aliases()
                                .map(str::to_owned)
                                .collect::<Vec<_>>()
                        })
                        .collect(),
                })
                .collect(),
            sections: Vec::new(),
            section_rule: None,
            examples: Vec::new(),
        })
        .collect()
}
