//! Shared command interpretation and typed outcomes for CLI and MCP.
use biodata::{Envelope, QueryBody, Request, Status};
use biomcp_query::{Action, Bundle};
use serde_json::Value;
use std::{ffi::OsString, io};

pub(crate) struct Outcome {
    pub(crate) value: Value,
    pub(crate) status: crate::exit::Status,
    pub(crate) cli_bytes: Vec<u8>,
}
pub(crate) fn dispatch(args: &[OsString]) -> io::Result<Outcome> {
    dispatch_using(args, None)
}
fn dispatch_using(
    args: &[OsString],
    shared: Option<(
        &crate::grammar::Globals,
        &Result<std::sync::Arc<biomcp_fetch::Fetch>, ()>,
    )>,
) -> io::Result<Outcome> {
    if let [command, name] = args
        && command == "schema"
        && let Some(text) = name.to_str().and_then(crate::schema::find)
    {
        return Ok(Outcome {
            value: serde_json::from_str(text)?,
            status: crate::exit::Status::Success,
            cli_bytes: text.as_bytes().to_vec(),
        });
    }
    if let Some(message) = crate::retired::message(args) {
        return encode(
            biodata::ErrorEnvelope::retired(message),
            crate::exit::Status::Usage,
        );
    }
    let mut parsed = match crate::grammar::parse(args) {
        Ok(parsed) => parsed,
        Err(()) => return usage(args),
    };
    if let Some((globals, _)) = shared
        && parsed.globals.inherit(globals).is_err()
    {
        return usage(args);
    }
    let Some(action) = parsed.action else {
        return usage(args);
    };
    if let Some(path) = parsed.help {
        return encode(
            Envelope::new(
                Status::Complete,
                path.clone(),
                Some(crate::help::dispatch(&path)?),
            ),
            crate::exit::Status::Success,
        );
    }
    if action == Action::KeyHelp {
        return key_help();
    }
    let owned;
    let fetch = match shared.map(|(_, fetch)| fetch) {
        Some(Ok(fetch)) if !parsed.globals.offline => fetch.as_ref(),
        Some(Err(())) if !parsed.globals.offline => {
            return encode(biodata::ErrorEnvelope::local(), crate::exit::Status::Local);
        }
        _ => {
            owned = match crate::config::fetch(&parsed.globals) {
                Ok(fetch) => fetch,
                Err(()) => {
                    return encode(biodata::ErrorEnvelope::local(), crate::exit::Status::Local);
                }
            };
            &owned
        }
    };
    execute(fetch, action)
}
fn execute(fetch: &biomcp_fetch::Fetch, action: Action) -> io::Result<Outcome> {
    match action {
        Action::KeyHelp => key_help(),
        Action::Sync { bundle } => sync(fetch, bundle),
        Action::Run { request, fresh } => {
            let result = biomcp_query::run(
                &biomcp_query::Ctx {
                    ncbi_email: crate::config::ncbi_email().map_err(|()|io::Error::other("local configuration failed"))?,
                    fetch,
                    budget: crate::config::BUDGET,
                    fresh,
                },
                &request,
            );
            let status = if result.local.is_some() {
                crate::exit::Status::Local
            } else {
                crate::exit::Status::from(result.status)
            };
            let steps = result
                .next
                .iter()
                .map(|(kind, action)| crate::spell::step(*kind, action))
                .collect::<Result<Vec<_>, _>>()?;
            match (request, result.body) {
                (Request::SearchArticle(_), QueryBody::PoolError{mismatch}) => {
                    let restart=steps.into_iter().next().ok_or_else(||io::Error::other("missing restart"))?;
                    encode(biodata::ErrorEnvelope::pool(mismatch,restart),if mismatch {crate::exit::Status::Usage}else{crate::exit::Status::Expired})
                }
                (Request::SearchArticle(query), QueryBody::ArticlePage(body)) => {
                    let mut envelope=Envelope::new(result.status,query,(result.status!=Status::Failed).then_some(body));
                    envelope.sources=result.sources;envelope.problems=result.problems;envelope.next=steps;envelope.page=result.page;
                    encode(envelope,status)
                }
                (Request::GetGene(query), QueryBody::GeneCards(body)) => {
                    let mut envelope = Envelope::new(result.status, query, Some(body));
                    envelope.sources = result.sources;
                    envelope.problems = result.problems;
                    envelope.next = steps;
                    encode(envelope, status)
                }
                (Request::SearchGene(query), QueryBody::GenePage(body)) => {
                    let mut envelope = Envelope::new(result.status, query, Some(body));
                    envelope.sources = result.sources;
                    envelope.problems = result.problems;
                    envelope.next = steps;
                    envelope.page = result.page;
                    encode(envelope, status)
                }
                _ => Err(io::Error::other("query response mismatch")),
            }
        }
    }
}
pub(crate) fn tool(
    args: &[OsString],
    globals: &crate::grammar::Globals,
    fetch: &Result<std::sync::Arc<biomcp_fetch::Fetch>, ()>,
) -> io::Result<Outcome> {
    if args.iter().filter_map(|arg| arg.to_str()).any(|arg| {
        ["--record", "--replay", "--case-cache"]
            .iter()
            .any(|flag| arg == *flag || arg.starts_with(&format!("{flag}=")))
    }) {
        return encode(
            biodata::ErrorEnvelope::arguments(),
            crate::exit::Status::Usage,
        );
    }
    if args.first().is_some_and(|arg| arg == "schema") {
        return dispatch(args);
    }
    dispatch_using(args, Some((globals, fetch)))
}
fn usage(args: &[OsString]) -> io::Result<Outcome> {
    let error = if args
        .iter()
        .any(|arg| arg == "get" || arg == "search" || arg == "sync")
    {
        biodata::ErrorEnvelope::arguments()
    } else {
        biodata::ErrorEnvelope::schema_usage()
    };
    encode(error, crate::exit::Status::Usage)
}
fn sync(fetch: &biomcp_fetch::Fetch, bundle: Bundle) -> io::Result<Outcome> {
    let source = match bundle {
        Bundle::Hgnc => biodata::Source::Hgnc,
        Bundle::ClinGen => biodata::Source::ClinGen,
    };
    let result = match bundle {
        Bundle::ClinGen => biomcp_adapters::clingen::sync(fetch),
        Bundle::Hgnc => {
            biomcp_adapters::hgnc::sync(fetch, biomcp_adapters::hgnc::HgncBundle::Complete)
                .and_then(|complete| {
                    let withdrawn = biomcp_adapters::hgnc::sync(
                    fetch,
                        biomcp_adapters::hgnc::HgncBundle::Withdrawn,
                    )?;
                    Ok(biodata::SyncResult {
                        rows: complete.rows + withdrawn.rows,
                        bytes: complete.bytes + withdrawn.bytes,
                    })
                })
        }
    };
    let status = if result.is_ok() {
        Status::Complete
    } else {
        Status::Failed
    };
    let local = result
        .as_ref()
        .err()
        .is_some_and(|error| !matches!(error, biomcp_fetch::RequestError::Source(_)));
    let reason = result.as_ref().err().and_then(|error| match error {
        biomcp_fetch::RequestError::Source(error) => Some(error.reason),
        _ => None,
    });
    let mut envelope =
        Envelope::<biodata::SyncResult>::new(status, biodata::SyncQuery { source }, result.ok());
    envelope.sources.push(biodata::SourceOutcome {
        term: None, count: None, fetched: None, notices: Vec::new(),
        source,
        state: if status == Status::Complete {
            biodata::SourceState::Ok
        } else {
            biodata::SourceState::Failed
        },
        reason,
    });
    encode(
        envelope,
        if local {
            crate::exit::Status::Local
        } else {
            status.into()
        },
    )
}
fn key_help() -> io::Result<Outcome> {
    let data = biodata::KeyHelp {
        source: biodata::Source::Oncokb,
        config: biodata::Text::new("keys.oncokb_api_key").map_err(io::Error::other)?,
        environment: biodata::Text::new("ONCOKB_API_KEY").map_err(io::Error::other)?,
    };
    encode(
        Envelope::new(Status::Complete, (), Some(data)),
        crate::exit::Status::Success,
    )
}
fn encode(value: impl serde::Serialize, status: crate::exit::Status) -> io::Result<Outcome> {
    let mut bytes = serde_json::to_vec(&value)?;
    bytes.push(b'\n');
    Ok(Outcome {
        value: serde_json::from_slice(&bytes)?,
        status,
        cli_bytes: bytes,
    })
}
