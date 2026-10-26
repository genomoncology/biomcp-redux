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
