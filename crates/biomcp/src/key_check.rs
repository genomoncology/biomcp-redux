//! Dispatch a single source key check for the CLI and internal HTTP route.
use biomcp_fetch::{Fetch, SecretName};
use biomcp_query::sources;
use biomcp_types::{Envelope, KeyCheck, KeyCheckStatus, Reason, Status};

const KEYLESS: biomcp_types::Source = biomcp_types::Source::ClinicalTrialsGov;
pub(crate) const HELP: &str =
    "Check one configured key with its publisher. Show whether it works and why a check failed.";

#[derive(Clone, Copy, Debug)]
pub(crate) enum Source {
    ClinicalTrials,
    Keyed(SecretName),
}

impl Source {
    pub(crate) fn parse(word: &str) -> Option<Self> {
        if word == <&str>::from(KEYLESS) {
            return Some(Self::ClinicalTrials);
        }
        supported()
            .find(|&secret| sources::key_name(secret) == word)
            .map(Self::Keyed)
    }

    pub(crate) fn run(self, fetch: Option<&Fetch>, settings: &crate::config::Settings) -> KeyCheck {
        let (source, status) = match self {
            Self::ClinicalTrials => (KEYLESS.into(), KeyCheckStatus::NoKeyRequired),
            Self::Keyed(secret) => {
                let status = fetch.map_or_else(
                    || KeyCheckStatus::CouldNotCheck {
                        reason: Reason::Unavailable.to_string(),
                    },
                    |fetch| sources::key_check::run(fetch, secret, crate::config::budget(settings)),
                );
                (sources::key_name(secret), status)
            }
        };
        KeyCheck::new(source, status)
    }
}

fn supported() -> impl Iterator<Item = SecretName> {
    sources::keys()
        .into_iter()
        .filter(|&secret| sources::key_check::supported(secret))
}

pub(crate) fn grammar() -> clap::Command {
    let names: Vec<_> = supported()
        .map(sources::key_name)
        .chain([KEYLESS.into()])
        .collect();
    clap::Command::new("check")
        .about(HELP)
        .arg(
            clap::Arg::new("source")
                .required(true)
                .help("Source to check.")
                .value_parser(names),
        )
        .after_help("Example: biomcp key check ncbi")
}

pub(crate) fn cli(
    source: Source,
    globals: &crate::grammar::Globals,
    settings: &crate::config::Settings,
    encoding: crate::command::Encoding,
) -> std::io::Result<crate::command::Outcome> {
    let fetch = if matches!(source, Source::Keyed(_)) {
        match crate::config::fetch(globals, settings) {
            Ok(fetch) => Some(crate::config::restrict(fetch, globals)?),
            Err(error) => return crate::command::local::startup(error, encoding),
        }
    } else {
        None
    };
    encoding.encode(
        Envelope::new(
            Status::Complete,
            biomcp_types::EmptyQuery {},
            Some(source.run(fetch.as_ref(), settings)),
        ),
        crate::exit::Status::Success,
    )
}
