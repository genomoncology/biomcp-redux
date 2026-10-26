//! Shared command interpretation and typed outcomes for CLI and MCP.
mod adverse_event;
mod author;
mod cell_line;
mod diagnostic;
mod disease;
mod drug;
mod drug_label;
mod maintenance;
mod pathway;
mod pgx;
mod phenotype;
mod protein;
mod steps;
mod sync;
mod unsupported;
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
    let action = match disease::prepare(action) {
        Ok(action) => action,
        Err(error) => return disease::selector_error(error),
    };
    if let Some(outcome) = unsupported::check(&action) {
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
fn execute(
    fetch: &biomcp_fetch::Fetch,
    action: Action,
    globals: &crate::grammar::Globals,
    allow_save: bool,
) -> io::Result<Outcome> {
    match action {
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
                    path.clone(),
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
            response(fetch, request, result, fresh)
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
    mut result: biomcp_query::Outcome,
    fresh: bool,
) -> io::Result<Outcome> {
    drug_label::bound(fetch, &request, &mut result, fresh)?;
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
    match request {
        Request::AuthorPapers(_) | Request::SearchAuthor(_) | Request::GetAuthor(_) => {
            author::response(meta, request, body)
        }
        Request::SearchCellLine(_) | Request::GetCellLine(_) | Request::CellLineAssayCount(_) => {
            cell_line::encode(meta, request, body)
        }
        Request::GetGwasAssociation(q) => gwas_get(meta, q.into(), body),
        Request::GetGwasStudy(q) => gwas_get(meta, q.into(), body),
        Request::SearchGwas(q) => match body {
            QueryBody::GwasPage(body) => meta.encode(biomcp_types::GwasPageQuery::Search(q), body),
            QueryBody::PoolError { mismatch } => pool_error(mismatch, meta.next),
            _ => Err(io::Error::other("gwas response mismatch")),
        },
        Request::SearchProtein(_)
        | Request::GetProtein(_)
        | Request::ProteinFeatures(_)
        | Request::ProteinIsoforms(_)
        | Request::ProteinDomains(_)
        | Request::ProteinComplexes(_)
        | Request::ProteinComplexMembers(_)
        | Request::ProteinStructures(_)
        | Request::ProteinSequence(_) => protein::encode(meta, request, body),
        Request::SearchPathway(_)
        | Request::GetPathway(_)
        | Request::PathwayGenes(_)
        | Request::PathwayEvents(_) => pathway::encode(meta, request, body),
        Request::SearchPhenotype(_) | Request::GetPhenotype(_) => {
            phenotype::encode(meta, request, body)
        }
        Request::SearchStudy(q) => match body {
            QueryBody::StudyPage(body) => meta.encode(q, body),
            QueryBody::PoolError { mismatch } => pool_error(mismatch, meta.next),
            _ => Err(io::Error::other("study response mismatch")),
        },
        Request::GetStudy(q) => match body {
            QueryBody::StudyCard(body) => meta.encode(q, body),
            QueryBody::StudyProfiles(body) => meta.encode(q, body),
            QueryBody::StudySampleLists(body) => meta.encode(q, body),
            QueryBody::StudyFiles(body) => meta.encode(q, body),
            QueryBody::PoolError { mismatch } => pool_error(mismatch, meta.next),
            _ => Err(io::Error::other("study response mismatch")),
        },
        Request::SearchPgx(_) | Request::GetPgx(_) | Request::GetPgxAnnotation(_) => {
            pgx::encode(meta, request, body)
        }
        Request::SearchDiagnostic(_) | Request::GetDiagnostic(_) => {
            diagnostic::encode(meta, request, body)
        }
        Request::SearchAdverseEvent(_) | Request::GetAdverseEvent(_) => {
            adverse_event::encode(meta, request, body)
        }
        Request::SearchDisease(_) | Request::GetDisease(_) => disease::encode(meta, request, body),
        Request::SearchDrug(_) | Request::GetWhoMedicine(_) | Request::GetDrug(_) => {
            drug::encode(meta, request, body)
        }
        _ => response_common(meta, request, body),
    }
}
fn response_common(meta: ResponseMeta, request: Request, body: QueryBody) -> io::Result<Outcome> {
    match (request, body) {
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
            let body = (meta.status != Status::Failed || body.condition_resolution.is_some())
                .then_some(body);
            meta.encode(query, body)
        }

        (Request::TrialDocument(query), QueryBody::TrialDocument(body)) => {
            meta.encode(query, Some(body))
        }
        (Request::GetTrial(query), QueryBody::TrialCards(body)) => meta.encode(query, Some(body)),
        (Request::SearchTrial(query), QueryBody::TrialPage(body)) => {
            let body = (meta.status != Status::Failed || body.condition_resolution.is_some())
                .then_some(body);
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
    let mut error = if matches!(root, Some("sync" | "key" | "store"))
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
    if let Some(message) = crate::retired::gwas_usage(args) {
        error = ErrorEnvelope::gwas_arguments(message);
    }
    if crate::grammar::gwas::invalid_threshold(args) {
        error = ErrorEnvelope::gwas_threshold();
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

fn response_status(result: &biomcp_query::Outcome) -> crate::exit::Status {
    if result.local.is_some() {
        crate::exit::Status::Local
    } else {
        crate::exit::Status::from(result.status)
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
    }
}

fn gwas_get(meta: ResponseMeta, q: biomcp_types::GwasGet, body: QueryBody) -> io::Result<Outcome> {
    match body {
        QueryBody::GwasAssociation(body) => meta.encode(q, body),
        QueryBody::GwasStudy(body) => meta.encode(q, body),
        QueryBody::GwasLoci(body) => meta.encode(q, body),
        QueryBody::GwasAncestries(body) => meta.encode(q, body),
        QueryBody::GwasPage(body) => meta.encode(biomcp_types::GwasPageQuery::Get(q), body),
        QueryBody::PoolError { mismatch } => pool_error(mismatch, meta.next),
        _ => Err(io::Error::other("gwas get response mismatch")),
    }
}
