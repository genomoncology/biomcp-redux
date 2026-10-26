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
            .unwrap_or_else(|| Command::new(verb).subcommand_required(true));
        verbs.insert(verb, command.subcommand(Command::new(entity)));
    }
    for command in verbs.into_values() {
        help = help.subcommand(command);
    }
    help
}
