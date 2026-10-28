//! Shared Semantic Scholar search metadata and PubMed DOI joins.
use crate::{Ctx, status};
use biodata::{Doi, Text};

use biomcp_adapters::{pubmed, semantic_scholar};
use biomcp_fetch::{Deadline, RequestError};
use biomcp_types::{ArticleItem, ArticleKey, ArticleSearch, ArticleSort, ArticleSource, Source};
pub(super) fn search<'a>(
    cx: &Ctx<'a>,
    query: &ArticleSearch,
    deadline: Deadline<'a>,
) -> super::sources::ResultLeg {
    let term = query
        .gene()
        .map_or_else(|| query.words(), |gene| format!("{gene} {}", query.words()))
        .trim()
        .to_owned();
    let (since, until) = query.dates();
    let year = (since.is_some() || until.is_some()).then(|| {
        format!(
            "{}-{}",
            since.map_or_else(String::new, |v| v.year().to_string()),
            until.map_or_else(String::new, |v| v.year().to_string())
        )
    });
    let leg = cx
        .fetch
        .leg(&semantic_scholar::SPEC, deadline)
        .fresh(cx.fresh);
    let start = leg.remaining();
    let result = semantic_scholar::search(&leg, &term, year.as_deref(), query.open_access());
    let elapsed = start.saturating_sub(leg.remaining());
    let error = result.as_ref().err().copied();
    let mut outcome = status::source(Source::SemanticScholar, leg.finish(), error);
    outcome.term = Text::new(&term).ok();
    let mut dois = Vec::new();
    let items = match result {
        Ok(reply) => {
            outcome.count = Some(reply.total);
            outcome.fetched = Some(reply.data.len() as u32);
            dois = reply
                .data
                .iter()
                .map(|paper| paper.external_ids.as_ref().and_then(|ids| ids.doi.clone()))
                .collect();
            reply
                .data
                .into_iter()
                .map(semantic_scholar::Paper::item)
                .collect()
        }
        Err(_) => Vec::new(),
    };
    super::sources::ResultLeg {
        problems: Vec::new(),
        source: ArticleSource::SemanticScholar,
        outcome,
        items,
        dois,
        elapsed,
        error,
    }
}
/// A DOI join preserves its source outcome even when transport or local work fails.
#[derive(Debug)]
pub struct DoiResolution {
    /// Actual PubMed state, including a failed or stale leg.
    pub source: biomcp_types::SourceOutcome,
    /// Original error, so local failures remain distinct.
    pub error: Option<RequestError>,
    /// Malformed summary fields retained alongside valid mappings.
    pub problems: Vec<biomcp_types::Problem>,
}
/// Resolve DOI-only rows using PubMed's actual DOI field, retaining unmatched rows.
/// Return a failed join explicitly; the caller retains the original source items.
pub fn resolve_dois<'a>(
    cx: &Ctx<'a>,
    items: &mut [ArticleItem],
    deadline: &mut Deadline<'a>,
) -> DoiResolution {
    resolve_dois_with(cx, items, deadline, &[])
}
// Raw source identities survive conversion even when no title/header exists.
pub(super) fn resolve_dois_with<'a>(
    cx: &Ctx<'a>,
    items: &mut [ArticleItem],
    deadline: &mut Deadline<'a>,
    raw_dois: &[Option<Doi>],
) -> DoiResolution {
    let dois: Vec<_> = items
        .iter()
        .enumerate()
        .filter(|(_, item)| item.pmid.is_none())
        .filter_map(|(index, item)| {
            raw_dois
                .get(index)
                .and_then(Option::as_ref)
                .or_else(|| doi(item))
                .cloned()
        })
        .collect();
    let leg = cx.fetch.leg(&pubmed::SPEC, *deadline).fresh(cx.fresh);
    let started = leg.remaining();
    let mut problems = Vec::new();
    let result: Result<(), RequestError> = (|| {
        for batch in dois.chunks(100) {
            let term = batch
                .iter()
                .map(|doi| {
                    format!(
                        "\"{}\"[doi]",
                        doi.to_string()
                            .to_ascii_lowercase()
                            .replace('\\', "\\\\")
                            .replace('"', "\\\"")
                    )
                })
                .collect::<Vec<_>>()
                .join(" OR ");
            let ids = pubmed::search_ids(&leg, &term, ArticleSort::Rank, cx.ncbi_email.as_ref())?;
            problems.extend(ids.problems);
            for chunk in ids.ids.chunks(50) {
                let mapped = pubmed::summaries(&leg, chunk, cx.ncbi_email.as_ref())?;
                problems.extend(mapped.problems);
                for (index, item) in items
                    .iter_mut()
                    .enumerate()
                    .filter(|(_, item)| item.pmid.is_none())
                {
                    let doi = raw_dois
                        .get(index)
                        .and_then(Option::as_ref)
                        .or_else(|| doi(item));
                    if let Some(header) = mapped
                        .value
                        .iter()
                        .find(|h| h.doi.as_ref().zip(doi).is_some_and(|(a, b)| a == b))
                        && let Some(id) = header.pmid
                    {
                        item.pmid = Some(id);
                        item.key = Some(ArticleKey::Pubmed(id));
                        item.header = Some(header.clone());
                    }
                }
            }
        }
        Ok(())
    })();
    *deadline = super::consume(*deadline, started, leg.remaining());
    let error = result.as_ref().err().copied();
    let outcome = status::source(Source::PubMed, leg.finish(), error);
    DoiResolution {
        source: outcome,
        error,
        problems,
    }
}

// Identity remains available even when the provider omitted its title/header.
pub(super) fn doi(item: &ArticleItem) -> Option<&Doi> {
    item.header.as_ref().and_then(|h| h.doi.as_ref()).or({
        if let Some(ArticleKey::Doi(doi)) = &item.key {
            Some(doi)
        } else {
            None
        }
    })
}
