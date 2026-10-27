use super::{BUNDLE_HELP, QUERIES};
use clap::{Arg, ArgAction, Command};
pub(super) fn serve_command() -> Command {
    Command::new("serve")
        .arg(
            Arg::new("http")
                .long("http")
                .num_args(0..=1)
                .default_missing_value("127.0.0.1:8080"),
        )
        .arg(
            Arg::new("allow-host")
                .long("allow-host")
                .action(ArgAction::Append)
                .requires("http"),
        )
}

pub(super) fn help_command() -> Command {
    let mut help = Command::new("help")
        .subcommand(Command::new("version"))
        .subcommand(Command::new("mcp-config"))
        .subcommand(
            Command::new("network").arg(
                Arg::new("setting")
                    .required(true)
                    .value_parser(["ca_bundle", "proxy"]),
            ),
        )
        .subcommand(
            Command::new("key").arg(
                Arg::new("source")
                    .required(true)
                    .value_parser(crate::keys::names()),
            ),
        );
    help = help.subcommand(
        Command::new("store")
            .subcommand_required(true)
            .subcommand(Command::new("clean")),
    );
    let mut verbs = std::collections::BTreeMap::<&str, Command>::new();
    for (verb, entity, _) in QUERIES
        .iter()
        .copied()
        .chain(BUNDLE_HELP.iter().map(|&bundle| ("sync", bundle, "")))
    {
        let command = verbs
            .remove(verb)
            .unwrap_or_else(|| Command::new(verb).subcommand_required(verb != "author"));
        verbs.insert(verb, command.subcommand(Command::new(entity)));
    }
    for command in verbs.into_values() {
        help = help.subcommand(command);
    }
    help
}

pub(super) fn help_source(matches: &clap::ArgMatches) -> Result<biomcp_types::Source, ()> {
    matches
        .subcommand()
        .filter(|(name, _)| *name == "key")
        .and_then(|(_, m)| m.get_one::<String>("source"))
        .map_or(Ok(biomcp_types::Source::Oncokb), |s| {
            if s == "ncbi" {
                Ok(biomcp_types::Source::PubMed)
            } else if s == "openfda" {
                Ok(biomcp_types::Source::OpenfdaLabel)
            } else {
                s.parse()
            }
        })
        .map_err(|_| ())
}

pub(crate) fn housekeeping_commands() -> [Command; 2] {
    [
        Command::new("version").about("Print version; --verbose adds build, schema and path provenance. No source requests or key reads.").arg(
            Arg::new("verbose")
                .long("verbose")
                .help("Include commit, target, schema versions, config and cache paths.")
                .action(ArgAction::SetTrue),
        ),
        Command::new("mcp-config")
            .about("Print MCP configuration with this executable’s absolute path. JSON formats name their top-level object and retain released client aliases. The codex client prints its shell registration command. No source requests or key reads.")
            .arg(
                Arg::new("client")
                    .help("Choose a JSON client or the codex shell registration command; default mcp-servers.")
                    .long("client")
                    .value_parser(clap::value_parser!(crate::housekeeping::Client))
                    .default_value("mcp-servers"),
            )
            .arg(
                Arg::new("absolute-path")
                    .help("Retained compatibility flag; executable paths are always absolute.")
                    .long("absolute-path")
                    .action(ArgAction::SetTrue),
            ),
    ]
}
pub(super) fn housekeeping(
    verb: &str,
    matches: &clap::ArgMatches,
) -> Option<crate::housekeeping::Command> {
    use crate::housekeeping::{Client, Command};
    match verb {
        "version" => Some(Command::Version {
            verbose: matches.get_flag("verbose"),
        }),
        "mcp-config" => matches
            .get_one::<Client>("client")
            .map(|&client| Command::McpConfig { client }),
        _ => None,
    }
}

pub(super) fn serve(matches: &clap::ArgMatches, globals: super::Globals) -> super::Parsed {
    super::Parsed {
        key: None,
        housekeeping: None,
        http: matches.get_one::<String>("http").map(|address| {
            (
                address.clone(),
                matches
                    .get_many::<String>("allow-host")
                    .into_iter()
                    .flatten()
                    .cloned()
                    .collect(),
            )
        }),
        action: None,
        globals,
        help: None,
    }
}
