//! Shared command interpretation and typed outcomes for CLI and MCP.
mod disease;
mod steps;
mod variant;

use biomcp_query::{Action, Bundle};
use biomcp_types::{Envelope, ErrorEnvelope, QueryBody, Request, Status};
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
        return encode(ErrorEnvelope::retired(message), crate::exit::Status::Usage);
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
    if let Some(command) = parsed.key {
        return if shared.is_some() {
            usage(args)
        } else {
            crate::keys::run(command, &parsed.globals)
        };
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
    if let Action::KeyHelp(source) = action {
        return key_help(source);
    }
    if let Some(outcome) = unsupported(&action) {
        return outcome;
    }
    let owned;
    let fetch = match shared.map(|(_, fetch)| fetch) {
        Some(Ok(fetch)) if !parsed.globals.offline => {
            let secrets = match crate::config::secrets(&parsed.globals) {
                Ok(secrets) => secrets,
                Err(()) => {
                    return encode(ErrorEnvelope::local(), crate::exit::Status::Local);
                }
            };
            owned = crate::config::restrict(fetch.with_secrets(secrets), &parsed.globals)?;
            &owned
        }
        Some(Err(())) if !parsed.globals.offline => {
            return encode(ErrorEnvelope::local(), crate::exit::Status::Local);
        }
        _ => {
            owned = match crate::config::fetch(&parsed.globals) {
                Ok(fetch) => fetch,
                Err(()) => {
                    return encode(ErrorEnvelope::local(), crate::exit::Status::Local);
                }
            };
            &owned
        }
    };
    execute(fetch, action, &parsed.globals, allow_save)
}
fn unsupported(action: &Action) -> Option<io::Result<Outcome>> {
    if disease::unsupported(action) {
        return Some(usage_envelope(ErrorEnvelope::arguments()));
    }
    if let Action::Run {
        request: Request::SearchTrial(q),
        ..
    } = action
    {
        if q.nci_disease_invalid() {
            return Some(usage_envelope(ErrorEnvelope::nci_disease()));
        }
        if let Some(option) = q.unsupported() {
            let source = if q.source() == biomcp_types::TrialSource::Nci {
                biomcp_types::Source::Nci
            } else {
                biomcp_types::Source::ClinicalTrialsGov
            };
            return Some(usage_envelope(ErrorEnvelope::unsupported_trial_filter(
                source, option,
            )));
        }
    }

    if let Action::Run {
        request: Request::SearchArticle(query),
        ..
    } = action
        && let Some((source, option)) = biomcp_query::article::unsupported(query)
    {
        return Some(encode(
            ErrorEnvelope::unsupported_filter(source, option),
            crate::exit::Status::Usage,
        ));
    }
    None
}
fn execute(
    fetch: &biomcp_fetch::Fetch,
    action: Action,
    globals: &crate::grammar::Globals,
    allow_save: bool,
) -> io::Result<Outcome> {
    match action {
        Action::QueryHelp(request) => {
            let path = crate::spell::command_words(&request);
            encode(
                Envelope::new(
                    Status::Complete,
                    path.clone(),
                    Some(crate::help::dispatch(&path)?),
                ),
                crate::exit::Status::Success,
            )
        }
        Action::Health => health(fetch),
        Action::KeyHelp(source) => key_help(source),
        Action::Sync { bundle } => sync(fetch, bundle),
        Action::Run { request, fresh } => {
            let result = run_request(fetch, &request, fresh, globals, allow_save)?;
            response(fetch, request, result)
        }
    }
}
struct ResponseMeta {
    status: Status,
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
        let mut envelope = Envelope::new(self.status, query, body);
        envelope.sources = self
            .sources
            .into_iter()
            .map(biomcp_adapters::registry::restore_outcome)
            .collect();
        envelope.problems = self.problems;
        envelope.next = self.next;
        envelope.page = self.page;
        encode(envelope, self.exit)
    }
}
fn response(
    fetch: &biomcp_fetch::Fetch,
    request: Request,
    result: biomcp_query::Outcome,
) -> io::Result<Outcome> {
    let steps = steps::next(fetch, &result)?;
    let meta = ResponseMeta {
        exit: response_status(&result),
        status: result.status,
        next: steps,
        page: result.page,
        sources: result.sources,
        problems: result.problems,
    };
    response_body(meta, request, result.body)
}
fn response_body(meta: ResponseMeta, request: Request, body: QueryBody) -> io::Result<Outcome> {
    if matches!(&request, Request::SearchDisease(_) | Request::GetDisease(_)) {
        return disease::encode(meta, request, body);
    }
    match (request, body) {
        (Request::SearchDrug(query), QueryBody::DrugPage(body)) => {
            let body = (meta.status != Status::Failed).then_some(body);
            meta.encode(query, body)
        }
        (Request::GetDrug(query), QueryBody::DrugCard(body)) => meta.encode(query, body),
        (Request::GetDrug(query), QueryBody::DrugXrefs(body)) => meta.encode(query, body),
        (Request::GetDrug(query), QueryBody::DrugTargets(body)) => meta.encode(query, body),
        (Request::GetDrug(query), QueryBody::DrugIndications(body)) => meta.encode(query, body),
        (Request::GetDrug(query), QueryBody::DrugCivic(body)) => meta.encode(query, body),
        (_, QueryBody::VariantQueryPolicyError) => {
            usage_envelope(ErrorEnvelope::variant_query_policy())
        }
        (_, QueryBody::UnsupportedTrialFilter { source, option }) => {
            usage_envelope(ErrorEnvelope::unsupported_trial_filter(source, option))
        }
        (_, QueryBody::NciDiseaseError) => usage_envelope(ErrorEnvelope::nci_disease()),
        (Request::GetTrial(query), QueryBody::NciTrialCards(body)) => {
            meta.encode(query, Some(body))
        }
        (Request::SearchTrial(query), QueryBody::NciTrialPage(body)) => {
            let body = (meta.status != Status::Failed).then_some(body);
            meta.encode(query, body)
        }

        (Request::TrialDocument(query), QueryBody::TrialDocument(body)) => {
            meta.encode(query, Some(body))
        }
        (Request::GetTrial(query), QueryBody::TrialCards(body)) => meta.encode(query, Some(body)),
        (Request::SearchTrial(query), QueryBody::TrialPage(body)) => {
            let body = (meta.status != Status::Failed).then_some(body);
            meta.encode(query, body)
        }
        (_, QueryBody::ArticleSaveRefused(url)) => usage_envelope(ErrorEnvelope::save_refused(url)),
        (_, QueryBody::PdfSaveRefused(link)) => {
            usage_envelope(ErrorEnvelope::save_refused(link.url))
        }
        (Request::ArticlePdf(query), QueryBody::ArticlePdf(body)) => meta.encode(query, Some(body)),
        (_, QueryBody::UnsupportedFilter { source, option }) => {
            usage_envelope(ErrorEnvelope::unsupported_filter(source, option))
        }
        (_, QueryBody::TextPositionError) => {
            encode(ErrorEnvelope::arguments(), crate::exit::Status::Usage)
        }
        (Request::ArticleEvidence(query), QueryBody::ArticleEvidence(body)) => {
            meta.encode(query, Some(body))
        }
        (Request::ArticleRecommend(query), QueryBody::ArticleRecommendations(body)) => {
            let body = (meta.status != Status::Failed).then_some(body);
            meta.encode(query, body)
        }
        (Request::ArticleSupplement(query), QueryBody::ArticleSupplement(body)) => {
            meta.encode(query, Some(body))
        }
        (Request::ArticleText(query), QueryBody::ArticleText(body)) => {
            meta.encode(query, Some(body))
        }
        (_, QueryBody::TooManyPmids { count }) => encode(
            ErrorEnvelope::too_many_pmids(count),
            crate::exit::Status::Usage,
        ),
        (_, QueryBody::PoolError { mismatch }) => pool_error(mismatch, meta.next),
        (
            Request::ArticleCitations(query) | Request::ArticleReferences(query),
            QueryBody::ArticleEdges(body),
        ) => {
            let body = (meta.status != Status::Failed).then_some(body);
            meta.encode(query, body)
        }
        (Request::SearchArticle(query), QueryBody::ArticlePage(body)) => {
            let body = (meta.status != Status::Failed).then_some(body);
            meta.encode(query, body)
        }
        (Request::GetArticle(query), QueryBody::ArticleCards(body)) => {
            meta.encode(query, Some(body))
        }
        (Request::GetGene(query), QueryBody::GeneCards(body)) => meta.encode(query, Some(body)),
        (Request::SearchGene(query), QueryBody::GenePage(body)) => meta.encode(query, Some(body)),
        (request, body) => variant::response(meta, request, body),
    }
}

pub(crate) fn tool(
    args: &[OsString],
    globals: &crate::grammar::Globals,
    fetch: &Result<std::sync::Arc<biomcp_fetch::Fetch>, ()>,
) -> io::Result<Outcome> {
    tool_using(args, globals, fetch, true)
}
pub(crate) fn tool_http(
    args: &[OsString],
    globals: &crate::grammar::Globals,
    fetch: &Result<std::sync::Arc<biomcp_fetch::Fetch>, ()>,
) -> io::Result<Outcome> {
    tool_using(args, globals, fetch, false)
}
fn tool_using(
    args: &[OsString],
    globals: &crate::grammar::Globals,
    fetch: &Result<std::sync::Arc<biomcp_fetch::Fetch>, ()>,
    allow_save: bool,
) -> io::Result<Outcome> {
    if args.iter().filter_map(|arg| arg.to_str()).any(|arg| {
        ["--record", "--replay", "--case-cache"]
            .iter()
            .any(|flag| arg == *flag || arg.starts_with(&format!("{flag}=")))
    }) {
        return encode(ErrorEnvelope::arguments(), crate::exit::Status::Usage);
    }
    if args.first().is_some_and(|arg| arg == "schema") {
        return dispatch(args);
    }
    dispatch_using(args, allow_save, Some((globals, fetch)))
}
fn usage(args: &[OsString]) -> io::Result<Outcome> {
    let mut words = args.iter().filter_map(|arg| arg.to_str());
    let root = loop {
        match words.next() {
            Some(
                "--format" | "--config" | "--replay" | "--record" | "--case-cache"
                | "--disable-sources",
            ) => {
                words.next();
            }
            Some("--fresh" | "--offline") => {}
            Some(word)
                if [
                    "--format=",
                    "--config=",
                    "--replay=",
                    "--record=",
                    "--case-cache=",
                    "--disable-sources=",
                ]
                .iter()
                .any(|prefix| word.starts_with(prefix)) => {}
            word => break word,
        }
    };
    let mut error = if matches!(root, Some("sync" | "key"))
        || root.is_some_and(|root| {
            crate::grammar::QUERIES
                .iter()
                .any(|(verb, _, _)| *verb == root)
        }) {
        ErrorEnvelope::arguments()
    } else {
        ErrorEnvelope::schema_usage()
    };
    if root == Some("get") && words.next() == Some("variant") {
        error = ErrorEnvelope::variant_arguments();
    }
    if root == Some("search") && args.get(1).and_then(|word| word.to_str()) == Some("trial") {
        let flags: Vec<_> = args
            .iter()
            .filter_map(|arg| arg.to_str())
            .map(|arg| arg.split('=').next().unwrap_or(arg))
            .collect();
        if flags.contains(&"--facility")
            && flags
                .iter()
                .any(|flag| ["--lat", "--lon", "--distance"].contains(flag))
        {
            error = ErrorEnvelope::trial_facility_radius();
        }
    }
    if root == Some("key") {
        error.next = biomcp_fetch::SecretName::ALL
            .iter()
            .map(|name| biomcp_types::Step {
                kind: biomcp_types::StepKind::Configure,
                args: vec!["help".into(), "key".into(), name.source().into()].into(),
            })
            .collect();
    }
    encode(error, crate::exit::Status::Usage)
}
fn sync(fetch: &biomcp_fetch::Fetch, bundle: Bundle) -> io::Result<Outcome> {
    let source = match bundle {
        Bundle::Hgnc => biomcp_types::Source::Hgnc,
        Bundle::ClinGen => biomcp_types::Source::ClinGen,
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
                    Ok(biomcp_types::SyncResult {
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
    let mut envelope = Envelope::<biomcp_types::SyncResult>::new(
        status,
        biomcp_types::SyncQuery { source },
        result.ok(),
    );
    envelope.sources.push(biomcp_adapters::registry::outcome(
        source,
        if reason == Some(biomcp_types::Reason::Disabled) {
            biomcp_types::SourceState::Skipped
        } else if status == Status::Complete {
            biomcp_types::SourceState::Ok
        } else {
            biomcp_types::SourceState::Failed
        },
        reason,
    ));
    steps::network_steps(&mut envelope.next, &envelope.sources, None);
    encode(
        envelope,
        if local {
            crate::exit::Status::Local
        } else {
            status.into()
        },
    )
}
fn key_help(source: biomcp_types::Source) -> io::Result<Outcome> {
    let name = match source {
        biomcp_types::Source::SemanticScholar => biomcp_fetch::SecretName::SemanticScholarApiKey,
        biomcp_types::Source::PubMed => biomcp_fetch::SecretName::NcbiApiKey,
        biomcp_types::Source::Nci => biomcp_fetch::SecretName::NciApiKey,
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
        Envelope::new(Status::Complete, (), Some(data)),
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
            .map_err(|()| io::Error::other("local configuration failed"))?,
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
    encode(
        ErrorEnvelope::pool(mismatch, restart),
        if mismatch {
            crate::exit::Status::Usage
        } else {
            crate::exit::Status::Expired
        },
    )
}

fn usage_envelope(error: ErrorEnvelope) -> io::Result<Outcome> {
    encode(error, crate::exit::Status::Usage)
}

fn health(fetch: &biomcp_fetch::Fetch) -> io::Result<Outcome> {
    match biomcp_query::health::run(fetch, crate::config::BUDGET) {
        Ok(result) => encode(
            Envelope::new(Status::Complete, (), Some(result)),
            crate::exit::Status::Success,
        ),
        Err(_) => encode(ErrorEnvelope::local(), crate::exit::Status::Local),
    }
}

fn response_status(result: &biomcp_query::Outcome) -> crate::exit::Status {
    if result.local.is_some() {
        crate::exit::Status::Local
    } else {
        crate::exit::Status::from(result.status)
    }
}
