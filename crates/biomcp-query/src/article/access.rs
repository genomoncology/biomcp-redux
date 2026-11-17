//! Open-access flags and licenses Europe PMC states for a search page's rows (1046).
use super::page::Cursor;
use crate::{
    Ctx, Outcome, fan_out, status,
    store::{Store, StoredItem},
};
use biodata::Pmid;
use biomcp_adapters::{Mapped, europepmc, pubmed};
use biomcp_fetch::{Deadline, LegReport, RequestError};
use biomcp_types::{ArticleHeader, Source, SourceOutcome};
use std::time::Duration;

/// The access lookup may hold a page this long. It runs beside the PubMed title read, so a
/// slow Europe PMC costs a page at most this much; Europe PMC's core search took 2.2 seconds
/// for two papers and 11 to 19 seconds on other tries on 2026-11-17.
const ACCESS_BUDGET: Duration = Duration::from_secs(3);

type Summaries = (Result<Mapped<Vec<ArticleHeader>>, RequestError>, LegReport);
type Access = (Result<Mapped<Vec<europepmc::Access>>, RequestError>, LegReport);

/// What the page's concurrent reads returned, and the longest of their times.
#[derive(Default)]
pub(super) struct Reads {
    pub(super) elapsed: Duration,
    pub(super) summaries: Option<Summaries>,
    pub(super) access: Option<Access>,
}

/// PMIDs of search rows with no open-access flag, unless the search or policy skips the lookup.
pub(super) fn wanted(cx: &Ctx<'_>, query: Cursor<'_>, rows: &[StoredItem]) -> Vec<Pmid> {
    let Cursor::Search(search) = query else {
        return Vec::new();
    };
    if search.no_access_lookup() || cx.fetch.disabled(Source::EuropePmc) {
        return Vec::new();
    }
    rows.iter()
        .filter(|row| row.item.open_access.is_none())
        .filter_map(|row| row.item.pmid)
        .collect()
}

/// Read missing PubMed titles and Europe PMC access at the same time.
pub(super) fn read<'a>(
    cx: &Ctx<'a>,
    deadline: Deadline<'a>,
    missing: &[Pmid],
    lookup: &[Pmid],
) -> Reads {
    let mut jobs: Vec<Box<dyn FnOnce() -> Reads + Send + '_>> = Vec::new();
    if !missing.is_empty() {
        jobs.push(Box::new(move || {
            let leg = cx.fetch.leg(&pubmed::SPEC, deadline).fresh(cx.fresh);
            let started = leg.remaining();
            let result = pubmed::summaries(&leg, missing, cx.ncbi_email.as_ref());
            let elapsed = started.saturating_sub(leg.remaining());
            Reads {
                elapsed,
                summaries: Some((result, leg.finish())),
                access: None,
            }
        }));
    }
    if !lookup.is_empty() {
        jobs.push(Box::new(move || {
            let leg = cx
                .fetch
                .leg(&europepmc::SPEC, deadline.child(ACCESS_BUDGET))
                .fresh(cx.fresh);
            let started = leg.remaining();
            let result = europepmc::access(&leg, lookup);
            let elapsed = started.saturating_sub(leg.remaining());
            Reads {
                elapsed,
                summaries: None,
                access: Some((result, leg.finish())),
            }
        }));
    }
    fan_out::run(jobs)
        .into_iter()
        .fold(Reads::default(), |mut all, one| {
            all.elapsed = all.elapsed.max(one.elapsed);
            all.summaries = all.summaries.or(one.summaries);
            all.access = all.access.or(one.access);
            all
        })
}

/// Fill rows from the lookup and keep the values with the stored page. A failed lookup
/// settles no row; it returns Europe PMC's outcome so the page can name the failure.
pub(super) fn apply<'a>(
    store: &mut Store<'a>,
    id: &str,
    rows: &mut [StoredItem],
    result: Result<Mapped<Vec<europepmc::Access>>, RequestError>,
    report: LegReport,
    out: &mut Outcome,
) -> Result<Option<SourceOutcome>, RequestError> {
    let found = match result {
        Ok(found) => found,
        Err(error) => return Ok(Some(status::source(Source::EuropePmc, report, Some(error)))),
    };
    status::append(&mut out.problems, found.problems);
    let mut changed = Vec::new();
    for row in rows.iter_mut() {
        if row.item.open_access.is_some() {
            continue;
        }
        let Some(access) = found.value.iter().find(|a| Some(a.pmid) == row.item.pmid) else {
            continue;
        };
        row.item.open_access = access.open_access;
        row.item.license = access.license;
        changed.push(row.item.clone());
    }
    if !changed.is_empty() {
        // A store that refuses the write still returns the values read for this page.
        crate::publish::keep(
            store.rewrite_article_text(id, &changed).map_err(Into::into),
            out,
        )?;
    }
    Ok(None)
}

/// Name Europe PMC's failed or timed-out lookup beside a page whose rows stay complete, so
/// `open_access_unknown` rows read as "not checked" rather than "Europe PMC does not know".
pub(super) fn failed(source: SourceOutcome, out: &mut Outcome) {
    // A search that also asked Europe PMC already lists it; its row takes the failure.
    if let Some(searched) = out.sources.iter_mut().find(|s| s.source == Source::EuropePmc) {
        status::retain_source_state(searched, &source);
    } else {
        out.sources.push(source);
    }
}
