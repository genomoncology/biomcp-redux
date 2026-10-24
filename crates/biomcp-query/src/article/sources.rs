use crate::{Ctx, Outcome, fan_out, status};
use biodata::{
    ArticleItem, ArticleKey, ArticleSearch, ArticleSource, ArticleType, Source, SourceOutcome,
    SourceState, Status, Text,
};
use biomcp_adapters::{europepmc, litvar, pubmed};
use biomcp_fetch::{Deadline, RequestError};
use std::time::Duration;
struct ResultLeg {
    source: ArticleSource,
    outcome: SourceOutcome,
    items: Vec<ArticleItem>,
    elapsed: Duration,
    error: Option<RequestError>,
    candidates: Vec<biodata::LitVarCandidate>,
}
pub(super) fn search<'a>(
    cx: &Ctx<'a>,
    query: &ArticleSearch,
    pubmed_term: &str,
    gene: Option<&str>,
    deadline: &mut Deadline<'a>,
    out: &mut Outcome,
) -> Result<Option<Vec<ArticleItem>>, RequestError> {
    let epmc_term = term(query);
    let start = *deadline;
    let results = fan_out::run(
        query
            .sources()
            .iter()
            .map(|selector| {
                let epmc_term = &epmc_term;
                Box::new(move || match selector {
                    ArticleSource::Pubmed => pubmed_search(cx, query, pubmed_term, start),
                    ArticleSource::Europepmc => europe_search(cx, query, epmc_term, start),
                    ArticleSource::Litvar => litvar_search(cx, query, gene, start),
                }) as Box<dyn FnOnce() -> ResultLeg + Send>
            })
            .collect(),
    );
    let elapsed = results.iter().map(|r| r.elapsed).max().unwrap_or_default();
    *deadline = deadline.after_stage(elapsed);
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
    if query.sources().len() == 1
        && results.iter().any(|r| {
            !r.candidates.is_empty()
                || (r.source == ArticleSource::Litvar && r.outcome.term.is_none())
        })
    {
        out.status = Status::Absent;
        let candidates: Vec<_> = results.into_iter().flat_map(|r| r.candidates).collect();
        for candidate in &candidates {
            out.next.push((
                biodata::StepKind::Disambiguate,
                crate::Action::Run {
                    request: biodata::Request::SearchArticle(
                        query.with_variant(candidate.rsid.clone()),
                    ),
                    fresh: cx.fresh,
                },
            ));
        }
        out.body = biodata::QueryBody::ArticlePage(biodata::ArticlePage {
            items: Box::default(),
            candidates,
        });
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
        candidates: Vec::new(),
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
        candidates: Vec::new(),
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
        parts.push(format!("AUTH:{author}"));
    }
    if let Some(journal) = query.journal() {
        parts.push(format!("JOURNAL:{journal}"));
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

fn litvar_search<'a>(
    cx: &Ctx<'a>,
    query: &ArticleSearch,
    gene: Option<&str>,
    deadline: Deadline<'a>,
) -> ResultLeg {
    let leg = cx.fetch.leg(&litvar::SPEC, deadline).fresh(cx.fresh);
    let start = leg.remaining();
    let result = litvar::search(&leg, gene, query.variant().map_or("", Text::as_str));
    let elapsed = start.saturating_sub(leg.remaining());
    let error = result.as_ref().err().copied();
    let mut outcome = status::source(Source::LitVar, leg.finish(), error);
    let mut candidates = Vec::new();
    let items = match result {
        Ok(litvar::Search::Selected { rsid, count, ids }) => {
            outcome.term = Some(rsid);
            outcome.count = Some(count);
            outcome.fetched = Some(ids.len() as u32);
            ids.into_iter()
                .map(|id| ArticleItem {
                    key: ArticleKey::Pubmed(id),
                    pmid: Some(id),
                    header: None,
                    cited_by: None,
                    found_by: Default::default(),
                })
                .collect()
        }
        Ok(litvar::Search::Candidates(nearby)) => {
            candidates = nearby;
            Vec::new()
        }
        Err(_) => Vec::new(),
    };
    ResultLeg {
        source: ArticleSource::Litvar,
        outcome,
        items,
        elapsed,
        error,
        candidates,
    }
}
