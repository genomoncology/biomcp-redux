//! Open-access flags for a search page's rows: NCBI's open-access subset by default (1072), and
//! Europe PMC's flag and licence only when the caller asks (1046).
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

/// The NCBI lookup may hold a page this long. It runs beside the PubMed title read; NCBI
/// answered 44 of 44 page lookups within 0.31 seconds on 2026-11-19 (1072).
const OPEN_BUDGET: Duration = Duration::from_secs(3);
/// The asked-for Europe PMC lookup may hold a page this long. Its median was 7.2 seconds and
/// its 90th percentile 15 seconds over 44 pages on 2026-11-19 (1072); the caller chose to wait.
const LICENSE_BUDGET: Duration = Duration::from_secs(10);

type Summaries = (Result<Mapped<Vec<ArticleHeader>>, RequestError>, LegReport);
type Open = (Result<Vec<Pmid>, RequestError>, LegReport);
type Access = (
    Result<Mapped<Vec<europepmc::Access>>, RequestError>,
    LegReport,
);

/// What the page's concurrent reads returned, and the longest of their times.
#[derive(Default)]
pub(super) struct Reads {
    pub(super) elapsed: Duration,
    pub(super) summaries: Option<Summaries>,
    pub(super) open: Option<Open>,
    pub(super) access: Option<Access>,
}

/// PMIDs each lookup should read; empty when the search or policy skips it.
#[derive(Default)]
pub(super) struct Wanted {
    open: Vec<Pmid>,
    license: Vec<Pmid>,
}

/// Rows with a PMID and no open-access flag go to NCBI; with `license_lookup`, rows with a PMID
/// and no licence also go to Europe PMC.
pub(super) fn wanted(cx: &Ctx<'_>, query: Cursor<'_>, rows: &[StoredItem]) -> Wanted {
    let Cursor::Search(search) = query else {
        return Wanted::default();
    };
    let pmids = |keep: fn(&biomcp_types::ArticleItem) -> bool| {
        rows.iter()
            .filter(|row| keep(&row.item))
            .filter_map(|row| row.item.pmid)
            .collect()
    };
    let offline = cx.fetch.is_offline();
    Wanted {
        open: if search.no_access_lookup() || offline || cx.fetch.disabled(Source::PubMed) {
            Vec::new()
        } else {
            pmids(|item| item.open_access.is_none())
        },
        license: if !search.license_lookup() || offline || cx.fetch.disabled(Source::EuropePmc) {
            Vec::new()
        } else {
            pmids(|item| item.license.is_none())
        },
    }
}

/// Read missing PubMed titles, NCBI's open-access subset and any asked-for Europe PMC access
/// at the same time.
pub(super) fn read<'a>(
    cx: &Ctx<'a>,
    deadline: Deadline<'a>,
    missing: &[Pmid],
    wanted: &Wanted,
) -> Reads {
    let mut jobs: Vec<Box<dyn FnOnce() -> Reads + Send + '_>> = Vec::new();
    if !missing.is_empty() {
        jobs.push(Box::new(move || {
            let leg = cx.fetch.leg(&pubmed::SPEC, deadline).fresh(cx.fresh);
            let started = leg.remaining();
            let result = pubmed::summaries(&leg, missing, cx.ncbi_email.as_ref());
            Reads {
                elapsed: started.saturating_sub(leg.remaining()),
                summaries: Some((result, leg.finish())),
                ..Reads::default()
            }
        }));
    }
    if !wanted.open.is_empty() {
        let ids = &wanted.open;
        jobs.push(Box::new(move || {
            let leg = cx
                .fetch
                .leg(&pubmed::SPEC, deadline.child(OPEN_BUDGET))
                .fresh(cx.fresh);
            let started = leg.remaining();
            let result = pubmed::open_access(&leg, ids, cx.ncbi_email.as_ref());
            Reads {
                elapsed: started.saturating_sub(leg.remaining()),
                open: Some((result, leg.finish())),
                ..Reads::default()
            }
        }));
    }
    if !wanted.license.is_empty() {
        let ids = &wanted.license;
        jobs.push(Box::new(move || {
            let leg = cx
                .fetch
                .leg(&europepmc::SPEC, deadline.child(LICENSE_BUDGET))
                .fresh(cx.fresh);
            let started = leg.remaining();
            let result = europepmc::access(&leg, ids);
            Reads {
                elapsed: started.saturating_sub(leg.remaining()),
                access: Some((result, leg.finish())),
                ..Reads::default()
            }
        }));
    }
    fan_out::run(jobs)
        .into_iter()
        .fold(Reads::default(), |mut all, one| {
            all.elapsed = all.elapsed.max(one.elapsed);
            all.summaries = all.summaries.or(one.summaries);
            all.open = all.open.or(one.open);
            all.access = all.access.or(one.access);
            all
        })
}

/// Apply both lookups' answers; returns the failed lookups' outcomes and the PMIDs whose
/// licence Europe PMC answered.
pub(super) fn settle(
    store: &mut Store<'_>,
    id: &str,
    rows: &mut [StoredItem],
    (open, access): (Option<Open>, Option<Access>),
    out: &mut Outcome,
) -> Result<(Vec<SourceOutcome>, Vec<Pmid>), RequestError> {
    let mut failed = Vec::new();
    if let Some(open) = open {
        failed.extend(apply_open(store, id, rows, open, out)?);
    }
    let answered = match access {
        Some(access) => apply(store, id, rows, access, out)?.unwrap_or_else(|source| {
            failed.push(source);
            Vec::new()
        }),
        None => Vec::new(),
    };
    Ok((failed, answered))
}

/// Settle each asked row from NCBI's open-access subset and keep it with the stored page. A
/// failed lookup settles no row; it returns PubMed's outcome so the page can name the failure.
fn apply_open(
    store: &mut Store<'_>,
    id: &str,
    rows: &mut [StoredItem],
    (result, report): Open,
    out: &mut Outcome,
) -> Result<Option<SourceOutcome>, RequestError> {
    let open = match result {
        Ok(open) => open,
        Err(error) => {
            let mut source = status::source(Source::PubMed, report, Some(error));
            let message = if source.reason == Some(biomcp_types::Reason::BadReply) {
                "Rows were read; NCBI's open-access reply could not be used or did not apply its open-access filter, so rows say open_access_not_checked."
            } else {
                "Rows were read; the open-access lookup failed, so rows say open_access_not_checked."
            };
            source
                .notices
                .push(biomcp_types::SourceNotice::RequestGuidance {
                    data_date: None,
                    message: message.into(),
                    retry_after_seconds: None,
                });
            return Ok(Some(source));
        }
    };
    let mut changed = Vec::new();
    for row in rows.iter_mut() {
        if row.item.open_access.is_some() {
            continue;
        }
        let Some(pmid) = row.item.pmid else {
            continue;
        };
        row.item.open_access = Some(open.contains(&pmid));
        row.item.open_access_source = Some(Source::PubMed);
        changed.push(row.item.clone());
    }
    keep(store, id, &changed, out)?;
    Ok(None)
}

/// Fill rows from the asked-for Europe PMC lookup, whose flag replaces NCBI's on the rows it
/// holds, and keep the values with the stored page. Returns the PMIDs Europe PMC answered, whose
/// licence is therefore checked, or Europe PMC's outcome when the lookup failed.
fn apply<'a>(
    store: &mut Store<'a>,
    id: &str,
    rows: &mut [StoredItem],
    (result, report): Access,
    out: &mut Outcome,
) -> Result<Result<Vec<Pmid>, SourceOutcome>, RequestError> {
    let found = match result {
        Ok(found) => found,
        Err(error) => return Ok(Err(status::source(Source::EuropePmc, report, Some(error)))),
    };
    status::append(&mut out.problems, found.problems);
    let mut changed = Vec::new();
    for row in rows.iter_mut() {
        let Some(access) = found.value.iter().find(|a| Some(a.pmid) == row.item.pmid) else {
            continue;
        };
        if access.open_access.is_some() {
            row.item.open_access = access.open_access;
            row.item.open_access_source = Some(Source::EuropePmc);
        }
        row.item.license = row.item.license.or(access.license);
        changed.push(row.item.clone());
    }
    keep(store, id, &changed, out)?;
    Ok(Ok(found.value.iter().map(|a| a.pmid).collect()))
}

fn keep(
    store: &mut Store<'_>,
    id: &str,
    changed: &[biomcp_types::ArticleItem],
    out: &mut Outcome,
) -> Result<(), RequestError> {
    if !changed.is_empty() {
        // A store that refuses the write still returns the values read for this page.
        crate::publish::keep(
            store.rewrite_article_text(id, changed).map_err(Into::into),
            out,
        )?;
    }
    Ok(())
}

/// Mark search rows whose licence no Europe PMC answer in this reply covers.
pub(super) fn mark_licenses(query: Cursor<'_>, rows: &mut [StoredItem], answered: &[Pmid]) {
    if matches!(query, Cursor::Search(_)) {
        for row in rows {
            row.item.license_not_checked = row.item.license.is_none()
                && row.item.pmid.is_none_or(|pmid| !answered.contains(&pmid));
        }
    }
}

/// Name a failed or timed-out lookup beside a page whose rows stay, so their `*_not_checked`
/// flags read as "not checked" rather than "the source does not know".
pub(super) fn failed(source: SourceOutcome, out: &mut Outcome) {
    // A search that also asked this source already lists it; its row takes the failure.
    if let Some(searched) = out.sources.iter_mut().find(|s| s.source == source.source) {
        status::retain_source_state(searched, &source);
    } else {
        out.sources.push(source);
    }
}
