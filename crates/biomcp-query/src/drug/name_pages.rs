//! Read the native name selectors and gather candidates for drug identity resolution.
use super::{Rank, carry, lookup_deadline, within};
use crate::{Ctx, Outcome, status};
use biomcp_types::{DrugGet, DrugHeader, DrugNameMatch, DrugSearch, ReplyBody, SourceState};
use std::cmp::Reverse;

#[path = "name_pages/fallback.rs"]
mod fallback;
pub(super) use fallback::{discover, ema};

/// The rank of a source without label packagers or numbered indications.
fn plain(name: u8, tier: u8, key: String) -> Rank {
    (name, tier, 0, Reverse(0), key)
}
/// Most MyChem rows a name get reads while looking for a drug record with an exact name.
const MYCHEM_NAME_CAP: u32 = 250;

/// Rows read from the source and whether they settled the lookup.
#[derive(Clone, Copy)]
struct Read {
    rows: u32,
    total: u64,
    settled: bool,
}
struct NativePage {
    candidates: Vec<(Rank, DrugHeader)>,
    /// How far a reader that may stop early has read, and whether it found its answer.
    read: Option<Read>,
    next_offset: Option<u32>,
    problems: Vec<biomcp_types::Problem>,
    refusals: biomcp_adapters::Refusal,
}
fn native_names(
    cx: &Ctx<'_>,
    query: &DrugSearch,
    spec: &'static biomcp_fetch::SourceSpec,
    read: impl Fn(&biomcp_fetch::Leg<'_>, u32) -> Result<NativePage, biomcp_fetch::RequestError>,
) -> (Outcome, Vec<(Rank, DrugHeader)>) {
    let (out, candidates, _) = native_pages(cx, query, spec, read);
    (out, candidates)
}
/// Read pages until the reader stops, keeping how far a reader that may stop early got.
fn native_pages(
    cx: &Ctx<'_>,
    query: &DrugSearch,
    spec: &'static biomcp_fetch::SourceSpec,
    read: impl Fn(&biomcp_fetch::Leg<'_>, u32) -> Result<NativePage, biomcp_fetch::RequestError>,
) -> (Outcome, Vec<(Rank, DrugHeader)>, Option<Read>) {
    let mut out = status::empty(biomcp_types::DrugPage::into_reply(query.clone(), None));
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let leg = cx.fetch.leg(spec, deadline).fresh(cx.fresh);
    let mut candidates = Vec::new();
    let mut offset = 0;
    let mut error = None;
    let mut examined = None;
    loop {
        match read(&leg, offset) {
            Ok(page) => {
                examined = page.read.map(|read| Read {
                    settled: read.settled || examined.is_some_and(|seen: Read| seen.settled),
                    ..read
                });
                out.include_refusals(page.refusals);
                out.problems.extend(page.problems);
                candidates.extend(page.candidates);
                let Some(next) = page.next_offset else {
                    break;
                };
                if next <= offset {
                    error = Some(biomcp_fetch::InputError::Serialization.into());
                    break;
                }
                offset = next;
            }
            Err(failure) => {
                error = Some(failure);
                break;
            }
        }
    }
    out.local = error.and_then(status::local);
    let remaining = leg.command_remaining();
    out.sources
        .push(status::source(query.source().source(), leg.finish(), error));
    status::search(&mut out);
    command.finish(remaining);
    (out, candidates, examined)
}
/// Drugs@FDA and openFDA label name lookups read every page on purpose, unlike MyChem's
/// 250-row cap: the ranking (NDA/BLA before ANDA, then earliest approval; labels by holder,
/// then indications) needs the whole set, so a lookup answers whole or fails, never partly.
pub(super) fn fda(
    cx: &Ctx<'_>,
    get: &DrugGet,
    query: &DrugSearch,
) -> (Outcome, Vec<(Rank, DrugHeader)>) {
    native_names(
        cx,
        query,
        &biomcp_adapters::drugsfda::SPEC,
        |leg, offset| {
            let page = biomcp_adapters::drugsfda::search_names(leg, get.input(), 50, offset)?;
            let next = offset + page.records.len() as u32;
            Ok(NativePage {
                read: None,
                candidates: page
                    .records
                    .iter()
                    .filter_map(|row| {
                        row.name_rank(get.input().as_str())
                            .map(|(name, app, date)| {
                                let date = date.unwrap_or_else(|| "9999".into());
                                (plain(name, app, date), row.header())
                            })
                    })
                    .collect(),
                next_offset: (next > offset && u64::from(next) < page.total).then_some(next),
                problems: page.problems,
                refusals: page.refusals,
            })
        },
    )
}
pub(super) fn label(
    cx: &Ctx<'_>,
    get: &DrugGet,
    query: &DrugSearch,
) -> (Outcome, Vec<(Rank, DrugHeader)>) {
    let (out, mut candidates) = native_names(
        cx,
        query,
        &biomcp_adapters::openfda_label::SPEC,
        |leg, _| {
            let mut out = NativePage {
                read: None,
                candidates: Vec::new(),
                next_offset: None,
                problems: Vec::new(),
                refusals: biomcp_adapters::Refusal::None,
            };
            for page in biomcp_adapters::openfda_label::search_names(leg, get.input())? {
                out.refusals.include(page.window.refusals());
                out.problems.extend(page.problems);
                for row in &page.window.rows {
                    if let Some(matched) = row.exact_name_match(get.input()) {
                        let rank = super::super::label_choice::rank(matched, row);
                        out.candidates.push((rank, row.header()?));
                    }
                }
            }
            Ok(out)
        },
    );
    super::super::label_choice::generic_name(&mut candidates);
    (out, candidates)
}

/// Read MyChem's free-text pages until one holds a drug record of the exact name. Its ranking can
/// place that record past looser matches and product-only records, so a page without one is no
/// answer, but the read stops at a page cap. Also says whether it stopped before finding one.
pub(super) fn mychem(
    cx: &Ctx<'_>,
    get: &DrugGet,
    query: &DrugSearch,
) -> (Outcome, Vec<(Rank, DrugHeader)>, bool) {
    let rows = super::super::search_names::NAME_ROWS;
    let (mut out, mut candidates, examined) =
        native_pages(cx, query, &biomcp_adapters::mychem::SPEC, |leg, offset| {
            let (page, received, drug) =
                biomcp_adapters::mychem::search_names(leg, get.input().as_str(), rows, offset)?;
            let next = offset + received;
            // The middle rank puts a record with drug rows of its own before a product-only one.
            let candidates: Vec<_> = page
                .value
                .items
                .iter()
                .zip(drug)
                .filter_map(|(header, drug)| {
                    header.matched_name.map(|matched| {
                        let rank = u8::from(matched == DrugNameMatch::Generic);
                        (plain(rank, u8::from(!drug), String::new()), header.clone())
                    })
                })
                .collect();
            let settled = candidates.iter().any(|((_, product, ..), _)| *product == 0);
            let more = received > 0
                && page
                    .value
                    .total
                    .is_some_and(|total| u64::from(next) < total);
            Ok(NativePage {
                read: page.value.total.map(|total| Read {
                    rows: next,
                    total,
                    settled,
                }),
                next_offset: (!settled && more && next < MYCHEM_NAME_CAP).then_some(next),
                candidates,
                problems: page.problems,
                refusals: page.refusals,
            })
        });
    // A product-only record answers only when no record with drug rows matched.
    if candidates.iter().any(|((_, product, ..), _)| *product == 0) {
        candidates.retain(|((_, product, ..), _)| *product == 0);
    }
    let Some(Read { rows, total, .. }) =
        examined.filter(|read| !read.settled && u64::from(read.rows) < read.total)
    else {
        return (out, candidates, false);
    };
    out.dropped |= candidates.is_empty();
    if let Some(source) = out
        .sources
        .iter_mut()
        .find(|s| s.source == biomcp_types::Source::Mychem)
    {
        source
            .notices
            .push(biomcp_types::SourceNotice::RequestGuidance {
                data_date: None,
                message: format!(
                    "Read the first {rows} of {total} matches without a drug record of that \
                     exact name; later matches were not examined."
                )
                .into(),
                retry_after_seconds: None,
            });
    }
    (out, candidates, true)
}
pub(super) fn chembl(
    cx: &Ctx<'_>,
    get: &DrugGet,
    query: &DrugSearch,
) -> (Outcome, Vec<(Rank, DrugHeader)>) {
    let deadline = lookup_deadline(cx);
    let mut all = Vec::new();
    let mut outcome = status::empty(biomcp_types::DrugPage::into_reply(query.clone(), None));
    for selector in [
        biomcp_types::DrugNameSelector::Preferred,
        biomcp_types::DrugNameSelector::Synonym,
    ] {
        let (out, candidates) = native_names(
            &within(cx, deadline.remaining()),
            query,
            &biomcp_adapters::chembl::SPEC,
            |leg, offset| {
                let (page, _) = biomcp_adapters::chembl::search(
                    leg,
                    get.input().as_str(),
                    selector,
                    50,
                    offset,
                )?;
                let next_offset = page.window.next_offset();
                let refusals = page.window.refusals();
                let mut problems = page.problems;
                let mut candidates = Vec::new();
                for row in &page.window.rows {
                    if let Some(matched) = row.name_match(get.input().as_str()) {
                        let header = biomcp_adapters::chembl::molecule_header(row)
                            .map_err(|_| biomcp_fetch::InputError::Serialization)?;
                        problems.extend(header.problems);
                        let rank = u8::from(matched == DrugNameMatch::Generic);
                        candidates.push((plain(rank, 0, String::new()), header.value));
                    }
                }
                Ok(NativePage {
                    read: None,
                    candidates,
                    next_offset,
                    problems,
                    refusals,
                })
            },
        );
        let failed = out
            .sources
            .iter()
            .any(|s| !matches!(s.state, SourceState::Ok | SourceState::Stale));
        let dropped = out.dropped;
        outcome.dropped |= dropped;
        outcome.local = outcome.local.or(out.local);
        carry(&out, &mut outcome);
        if dropped || failed {
            return (outcome, Vec::new());
        }
        all.extend(candidates);
    }
    (outcome, all)
}
