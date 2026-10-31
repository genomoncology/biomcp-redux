//! Shared command interpretation and typed outcomes for CLI and MCP.
mod context;
mod reply;
pub(crate) use context::Context;
mod hosted_keys;
pub(crate) mod local;
mod maintenance;
mod mcp_policy;
mod steps;
mod sync;
mod transport;

use biomcp_query::{Action, Bundle};
use biomcp_types::{Envelope, ErrorEnvelope, Request, Status, UsageArguments, UsageError};
use serde_json::Value;
use std::{ffi::OsString, io};

pub(crate) struct Outcome {
    pub(crate) value: Value,
    pub(crate) status: crate::exit::Status,
    pub(crate) cli_bytes: Vec<u8>,
}
pub(crate) fn dispatch(args: &[OsString]) -> io::Result<Outcome> {
    dispatch_using(args, true, None)
}
fn dispatch_using(
    args: &[OsString],
    allow_save: bool,
    context: Option<Context<'_>>,
) -> io::Result<Outcome> {
    if let [command] = args
        && command == "schema"
    {
        return encode(
            Envelope::new(
                Status::Complete,
                biomcp_types::EmptyQuery {},
                Some(crate::schema::catalog()),
            ),
            crate::exit::Status::Success,
        );
    }
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
        return usage_envelope(UsageError::Retired(message));
    }
    let mut parsed = match crate::grammar::parse(args) {
        Ok(parsed) => parsed,
        Err(cause) => return parse_refusal(args, cause),
    };
    if let Some(context) = context
        && parsed.globals.inherit(context.globals).is_err()
    {
        return usage(args);
    }
    if context.is_some()
        && let Some(error) = mcp_policy::refusal(&parsed, !allow_save)
    {
        return usage_envelope(error);
    }
    if let Some(command) = parsed.key {
        return crate::keys::run(command, &parsed.globals);
    }
    let Some(action) = parsed.action else {
        return usage(args);
    };
    if let Some(path) = parsed.help {
        return encode(
            Envelope::new(
                Status::Complete,
                biomcp_types::HelpQuery {
                    command: path.clone(),
                },
                Some(crate::help::dispatch(&path)?),
            ),
            crate::exit::Status::Success,
        );
    }
    if let Action::KeyHelp(source) = action {
        return key_help(source);
    }
    let action = match action.prepare() {
        Ok(action) => action,
        Err(error) => return usage_envelope(error),
    };
    let fetch = match transport::fetch(&parsed.globals, context)? {
        Ok(fetch) => fetch,
        Err(error) => return local::startup(error),
    };
    let outcome = execute(&fetch, action, &parsed.globals, allow_save)?;
    hosted_keys::outcome(outcome, parsed.globals.hosted)
}

pub(crate) fn execute(
    fetch: &biomcp_fetch::Fetch,
    action: Action,
    globals: &crate::grammar::Globals,
    allow_save: bool,
) -> io::Result<Outcome> {
    match action {
        Action::PubmedSearch(q) => execute(
            fetch,
            Action::Run {
                request: Request::SearchArticle(q),
                fresh: false,
            },
            globals,
            allow_save,
        ),
        Action::AuthorPapers(q) => execute(
            fetch,
            Action::Run {
                request: Request::AuthorPapers(q),
                fresh: false,
            },
            globals,
            allow_save,
        ),
        Action::QueryHelp(request) => {
            let path = crate::spell::command_words(&request);
            encode(
                Envelope::new(
                    Status::Complete,
                    biomcp_types::HelpQuery {
                        command: path.clone(),
                    },
                    Some(crate::help::dispatch(&path)?),
                ),
                crate::exit::Status::Success,
            )
        }
        Action::NetworkHelp(setting) => {
            let path = vec!["network".into(), setting.to_string()];
            encode(
                Envelope::new(
                    Status::Complete,
                    biomcp_types::HelpQuery {
                        command: path.clone(),
                    },
                    Some(crate::help::dispatch(&path)?),
                ),
                crate::exit::Status::Success,
            )
        }
        Action::StoreClean => maintenance::clean(fetch),
        Action::Health => maintenance::health(fetch),
        Action::KeyHelp(source) => key_help(source),
        Action::Sync { bundle } => sync::run(fetch, bundle),
        Action::Run { request, fresh } => {
            let result = run_request(fetch, &request, fresh, globals, allow_save)?;
            response(fetch, request, result, fresh, globals)
        }
    }
}
struct ResponseMeta {
    status: Status,
    body_visible: bool,
    local_error: Option<biomcp_types::CacheProblem>,
    sources: Vec<biomcp_types::SourceOutcome>,
    problems: Vec<biomcp_types::Problem>,
    next: Vec<biomcp_types::Step>,
    page: Option<biomcp_types::PageInfo>,
    exit: crate::exit::Status,
}
impl ResponseMeta {
    fn encode<B: biomcp_types::ResponseBody>(
        self,
        query: B::Query,
        body: Option<B>,
    ) -> io::Result<Outcome> {
        let mut envelope = Envelope::new(self.status, query, body.filter(|_| self.body_visible));
        envelope.sources = self
            .sources
            .into_iter()
            .map(biomcp_query::sources::restore_outcome)
            .collect();
        envelope.problems = self.problems;
        envelope.next = self.next;
        envelope.page = self.page;
        envelope.local_error = self.local_error;
        encode(envelope, self.exit)
    }
}
fn response(
    fetch: &biomcp_fetch::Fetch,
    request: Request,
    mut result: biomcp_query::Outcome,
    fresh: bool,
    globals: &crate::grammar::Globals,
) -> io::Result<Outcome> {
    result
        .finish(fetch, &request, fresh)
        .map_err(io::Error::other)?;
    let steps = steps::next(&result, globals)?;
    let meta = ResponseMeta {
        exit: response_status(&result),
        body_visible: result.body_visible(),
        local_error: local::kind(result.local),
        status: result.status(),
        next: steps,
        page: result.page,
        sources: result.sources,
        problems: result.problems,
    };
    result.body.visit(meta)
}
pub(crate) fn tool(
    args: &[OsString],
    globals: &crate::grammar::Globals,
    fetch: &Result<std::sync::Arc<biomcp_fetch::Fetch>, crate::config::StartupError>,
) -> io::Result<Outcome> {
    tool_using(args, Context::new(globals, fetch), true)
}
pub(crate) fn tool_http(args: &[OsString], context: Context<'_>) -> io::Result<Outcome> {
    tool_using(args, context, false)
}
fn tool_using(args: &[OsString], context: Context<'_>, allow_save: bool) -> io::Result<Outcome> {
    if args.iter().filter_map(|arg| arg.to_str()).any(|arg| {
        ["--record", "--replay", "--case-cache"]
            .iter()
            .any(|flag| arg == *flag || arg.starts_with(&format!("{flag}=")))
    }) {
        return usage_envelope(UsageError::arguments());
    }
    if args.first().is_some_and(|arg| arg == "schema") {
        return dispatch(args);
    }
    dispatch_using(args, allow_save, Some(context))
}
fn usage(args: &[OsString]) -> io::Result<Outcome> {
    parse_refusal(args, crate::grammar::ParseInvalid::default())
}
fn parse_refusal(args: &[OsString], cause: crate::grammar::ParseInvalid) -> io::Result<Outcome> {
    if let Some(path) = crate::grammar::unknown_command(args) {
        return usage_envelope(UsageError::UnknownCommand {
            next: biomcp_types::Step {
                kind: biomcp_types::StepKind::Configure,
                args: std::iter::once("help".into())
                    .chain(path.into_iter().map(String::into_boxed_str))
                    .collect::<Vec<_>>()
                    .into(),
            },
        });
    }
    let mut words = crate::grammar::command_words(args);
    let root = words.next();
    let mut arguments = match cause.usage {
        Some(UsageError::Arguments(arguments)) => arguments,
        Some(error) => return usage_envelope(error),
        None => {
            if cause.argument.is_none()
                && !matches!(
                    root,
                    Some("sync" | "key" | "store" | "version" | "mcp-config" | "skill")
                )
                && !root.is_some_and(|root| {
                    crate::grammar::QUERIES
                        .iter()
                        .any(|(verb, _, _)| *verb == root)
                })
            {
                return usage_envelope(UsageError::Schema);
            }
            UsageArguments::default()
        }
    };
    if root == Some("get") && words.next() == Some("variant") {
        arguments.message = Some(biomcp_types::ErrorMessage::VariantIdentifier);
    }
    if root == Some("mcp-config") {
        arguments.next = vec![biomcp_types::Step {
            kind: biomcp_types::StepKind::Configure,
            args: vec!["mcp-config".into(), "--client".into(), "mcp-servers".into()].into(),
        }];
    }
    if root == Some("key") {
        arguments.next = biomcp_fetch::SecretName::ALL
            .iter()
            .map(|name| biomcp_types::Step {
                kind: biomcp_types::StepKind::Configure,
                args: vec!["help".into(), "key".into(), name.source().into()].into(),
            })
            .collect();
    }
    if let Some(option) = cause.argument {
        arguments.argument = Some(option.into());
    }
    usage_envelope(UsageError::Arguments(arguments))
}

fn key_help(source: biomcp_types::Source) -> io::Result<Outcome> {
    let name = match source {
        biomcp_types::Source::SemanticScholar => biomcp_fetch::SecretName::SemanticScholarApiKey,
        biomcp_types::Source::PubMed => biomcp_fetch::SecretName::NcbiApiKey,
        biomcp_types::Source::AlphaGenome => biomcp_fetch::SecretName::AlphaGenomeApiKey,
        biomcp_types::Source::Nci => biomcp_fetch::SecretName::NciApiKey,
        biomcp_types::Source::OpenfdaLabel => biomcp_fetch::SecretName::OpenfdaApiKey,
        biomcp_types::Source::Orcid => biomcp_fetch::SecretName::OrcidAccessToken,
        _ => biomcp_fetch::SecretName::OncokbApiKey,
    };
    let data = biomcp_types::KeyHelp {
        guidance: Some(
            biodata::Text::new(&format!(
                "biomcp key set {}{}",
                name.source(),
                if source == biomcp_types::Source::SemanticScholar {
                    "; a free key raises the request limit"
                } else {
                    ""
                }
            ))
            .map_err(io::Error::other)?,
        ),
        source,
        config: biodata::Text::new(&format!("keys.{}", name.config())).map_err(io::Error::other)?,
        environment: biodata::Text::new(name.environment()).map_err(io::Error::other)?,
    };
    encode(
        Envelope::new(Status::Complete, biomcp_types::EmptyQuery {}, Some(data)),
        crate::exit::Status::Success,
    )
}
pub(crate) fn encode(
    value: impl serde::Serialize,
    status: crate::exit::Status,
) -> io::Result<Outcome> {
    let mut bytes = serde_json::to_vec(&value)?;
    bytes.push(b'\n');
    Ok(Outcome {
        value: serde_json::from_slice(&bytes)?,
        status,
        cli_bytes: bytes,
    })
}

fn run_request(
    fetch: &biomcp_fetch::Fetch,
    request: &Request,
    fresh: bool,
    globals: &crate::grammar::Globals,
    allow_save: bool,
) -> io::Result<biomcp_query::Outcome> {
    let cx = biomcp_query::Ctx {
        ncbi_email: crate::config::ncbi_email(globals)
            .map_err(|_| io::Error::other("local configuration failed"))?,
        fetch,
        budget: crate::config::BUDGET,
        fresh,
    };
    Ok(match request {
        Request::TrialDocument(query) => crate::save::trial::document(&cx, query, allow_save),
        Request::ArticlePdf(query) => crate::save::run(&cx, query, allow_save),
        Request::ArticleText(query) if query.save().is_some() => {
            crate::save::article::text(&cx, query, allow_save)
        }
        Request::ArticleSupplement(query) if query.save().is_some() => {
            crate::save::article::supplement(&cx, query, allow_save)
        }
        _ => biomcp_query::run(&cx, request),
    })
}

fn pool_error(mismatch: bool, steps: Vec<biomcp_types::Step>) -> io::Result<Outcome> {
    let restart = steps
        .into_iter()
        .next()
        .ok_or_else(|| io::Error::other("missing restart"))?;
    if mismatch {
        usage_envelope(UsageError::PoolMismatch { restart })
    } else {
        encode(
            ErrorEnvelope::expired_pool(restart),
            crate::exit::Status::Expired,
        )
    }
}

fn usage_envelope(error: UsageError) -> io::Result<Outcome> {
    encode(ErrorEnvelope::from(error), crate::exit::Status::Usage)
}

fn response_status(result: &biomcp_query::Outcome) -> crate::exit::Status {
    if result.local.is_some() {
        crate::exit::Status::Local
    } else {
        crate::exit::Status::from(result.status())
    }
}

fn bundle_source(bundle: Bundle) -> biomcp_types::Source {
    match bundle {
        Bundle::FdaCdx => biomcp_types::Source::FdaCdx,
        Bundle::WikiPathways => biomcp_types::Source::WikiPathways,
        Bundle::CbioportalStudies => biomcp_types::Source::Cbioportal,
        Bundle::Gtr => biomcp_types::Source::Gtr,
        Bundle::WhoIvd => biomcp_types::Source::WhoIvd,
        Bundle::Ema => biomcp_types::Source::Ema,
        Bundle::WhoPq => biomcp_types::Source::WhoPq,
        Bundle::Ddinter => biomcp_types::Source::Ddinter,
        Bundle::Hgnc => biomcp_types::Source::Hgnc,
        Bundle::ClinGen => biomcp_types::Source::ClinGen,
        Bundle::Gencc => biomcp_types::Source::Gencc,
    }
}
