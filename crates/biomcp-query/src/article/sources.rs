use crate::{Ctx, Outcome, fan_out, status};
use biodata::{
    ArticleItem, ArticleKey, ArticleSearch, ArticleSource, ArticleType, Source, SourceOutcome,
    SourceState, Status, Text,
};
use biomcp_adapters::{europepmc, pubmed};
use biomcp_fetch::{Deadline, RequestError};
use std::time::Duration;
pub(super) struct ResultLeg {
    pub(super) source: ArticleSource,
    pub(super) outcome: SourceOutcome,
    pub(super) items: Vec<ArticleItem>,
    pub(super) elapsed: Duration,
    pub(super) error: Option<RequestError>,
}
pub(super) fn search<'a>(
    cx: &Ctx<'a>,
    query: &ArticleSearch,
    pubmed_term: &str,
    deadline: &mut Deadline<'a>,
    out: &mut Outcome,
) -> Result<Option<Vec<ArticleItem>>, RequestError> {
    let epmc_term = term(query);
    let start = *deadline;
    let mut results = fan_out::run(
        query
            .sources()
            .iter()
            .map(|selector| {
                let epmc_term = &epmc_term;
                Box::new(move || match selector {
                    ArticleSource::Pubmed => pubmed_search(cx, query, pubmed_term, start),
                    ArticleSource::Europepmc => europe_search(cx, query, epmc_term, start),
                    ArticleSource::SemanticScholar => {
                        super::semantic_scholar::search(cx, query, start)
                    }
                }) as Box<dyn FnOnce() -> ResultLeg + Send>
            })
            .collect(),
    );
    let elapsed = results.iter().map(|r| r.elapsed).max().unwrap_or_default();
    *deadline = deadline.after_stage(elapsed);
    let mut join_failures = Vec::new();
    for result in &mut results {
        if result.source == ArticleSource::SemanticScholar
            && result.items.iter().any(|item| {
                item.pmid.is_none() && item.header.as_ref().is_some_and(|h| h.doi.is_some())
            })
        {
            match super::semantic_scholar::resolve_dois(cx, &mut result.items, deadline) {
                Ok(source) => {
                    if !results_have_pubmed(query) {
                        out.sources.push(source);
                    }
                }
                Err(error) => {
                    if status::local(error).is_some() {
                        return Err(error);
                    }
                    if let RequestError::Source(error) = error {
                        join_failures.push(SourceOutcome {
                            source: Source::PubMed,
                            state: SourceState::Failed,
                            reason: Some(error.reason),
                            term: None,
                            count: None,
                            fetched: None,
                            notices: Vec::new(),
                        });
                    }
                }
            }
        }
        if result.outcome.reason == Some(biodata::Reason::RateLimited)
            && result.source == ArticleSource::SemanticScholar
        {
            out.next.push((
                biodata::StepKind::Key,
                crate::Action::KeyHelp(Source::SemanticScholar),
            ));
        }
    }
    for failure in join_failures {
        if let Some(existing) = results
            .iter_mut()
            .find(|r| r.outcome.source == failure.source)
        {
            existing.outcome.state = failure.state;
            existing.outcome.reason = failure.reason;
        } else {
            out.sources.push(failure);
        }
    }
    let failed_pubmed = results
        .iter()
        .any(|r| r.source == ArticleSource::Pubmed && r.error.is_some());
    let answered = results.iter().any(|r| r.error.is_none());
    let error = results
        .iter()
        .filter_map(|r| r.error)
        .find(|e| status::local(*e).is_some());
    for result in &results {
        out.sources.push(result.outcome.clone());
    }
    if let Some(error) = error {
        return Err(error);
    }
    if failed_pubmed || !answered {
        out.status = Status::Failed;
        return Ok(None);
    }
    Ok(Some(super::fold::fold(
        results.into_iter().map(|r| (r.source, r.items)).collect(),
        query.sources().len() > 1,
    )))
}
fn pubmed_search<'a>(
    cx: &Ctx<'a>,
    query: &ArticleSearch,
    term: &str,
    deadline: Deadline<'a>,
) -> ResultLeg {
    let leg = cx.fetch.leg(&pubmed::SPEC, deadline).fresh(cx.fresh);
    let start = leg.remaining();
    let result = pubmed::search_ids(&leg, term, query.sort(), cx.ncbi_email.as_ref());
    let elapsed = start.saturating_sub(leg.remaining());
    let error = result.as_ref().err().copied();
    let mut outcome = status::source(Source::PubMed, leg.finish(), error);
    outcome.term = Text::new(term).ok();
    let items = match result {
        Ok(ids) => {
            outcome.count = Some(ids.count);
            outcome.fetched = Some(ids.ids.len() as u32);
            outcome.notices = ids.notices;
            ids.ids
                .into_iter()
                .map(|id| ArticleItem {
                    key: ArticleKey::Pubmed(id),
                    pmid: Some(id),
                    header: None,
                    cited_by: None,
                    intents: None,
                    influential: None,
                    contexts: None,
                    found_by: Default::default(),
                })
                .collect()
        }
        Err(_) => Vec::new(),
    };
    ResultLeg {
        source: ArticleSource::Pubmed,
        outcome,
        items,
        elapsed,
        error,
    }
}
fn europe_search<'a>(
    cx: &Ctx<'a>,
    query: &ArticleSearch,
    term: &str,
    deadline: Deadline<'a>,
) -> ResultLeg {
    let leg = cx.fetch.leg(&europepmc::SPEC, deadline).fresh(cx.fresh);
    let start = leg.remaining();
    let result = europepmc::search(&leg, term, query.sort());
    let elapsed = start.saturating_sub(leg.remaining());
    let error = result.as_ref().err().copied();
    let mut outcome = status::source(Source::EuropePmc, leg.finish(), error);
    outcome.term = Text::new(term).ok();
    let items = match result {
        Ok(reply) => {
            outcome.count = Some(reply.count);
            outcome.fetched = Some(reply.items.len() as u32);
            reply.items
        }
        Err(_) => Vec::new(),
    };
    ResultLeg {
        source: ArticleSource::Europepmc,
        outcome,
        items,
        elapsed,
        error,
    }
}
pub(super) fn failed(sources: &[SourceOutcome]) -> bool {
    sources.iter().any(|s| s.state == SourceState::Failed)
}
fn term(query: &ArticleSearch) -> String {
    let mut parts = Vec::new();
    if let Some(gene) = query.gene() {
        parts.push(gene.to_string());
    }
    let words = query.words();
    if !words.is_empty() {
        parts.push(words);
    }
    if let Some(author) = query.author() {
        parts.push(format!("AUTH:{}", field(author.as_str())));
    }
    if let Some(journal) = query.journal() {
        parts.push(format!("JOURNAL:{}", field(journal.as_str())));
    }
    let (since, until) = query.dates();
    if since.is_some() || until.is_some() {
        parts.push(format!(
            "FIRST_PDATE:[{} TO {}]",
            since.map_or_else(|| "1800".into(), |v| v.to_string()),
            until.map_or_else(|| "3000".into(), |v| v.to_string())
        ));
    }
    if let Some(kind) = query.kind() {
        let name = match kind {
            ArticleType::Trial => "clinical trial",
            ArticleType::Randomized => "randomized controlled trial",
            ArticleType::Review => "review",
            ArticleType::SystematicReview => "systematic review",
            ArticleType::MetaAnalysis => "meta-analysis",
            ArticleType::Guideline => "guideline",
            ArticleType::CaseReport => "case reports",
        };
        parts.push(format!("PUB_TYPE:\"{name}\""));
    }
    parts.join(" AND ")
}

fn results_have_pubmed(query: &ArticleSearch) -> bool {
    query.sources().contains(&ArticleSource::Pubmed)
}
// Structured selectors are literal field values, not additional query syntax.
fn field(value: &str) -> String {
    if value
        .bytes()
        .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'-' | b'.' | b'_'))
    {
        value.to_owned()
    } else {
        format!("\"{}\"", value.replace('\\', "\\\\").replace('"', "\\\""))
    }
}
