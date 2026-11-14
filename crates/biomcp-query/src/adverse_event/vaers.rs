//! One native table per request, with provider-free retained continuation.
use crate::{Ctx, Outcome, source_snapshot, status};
use biodata::Text;
use biomcp_adapters::vaers;
use biomcp_fetch::{Deadline, RequestError};
use biomcp_types::Paged;
use biomcp_types::ReplyBody;
use biomcp_types::{
    AdverseEventSearch, PageInfo, ReplyData, Request, Source, SourceOutcome, StepKind,
    VaersSection, VaersSort, VaersSummary,
};
type Saved = (VaersSummary, Vec<SourceOutcome>);

pub(super) fn search(cx: &Ctx<'_>, q: &AdverseEventSearch) -> Outcome {
    let mut out = super::unsupported();
    out.body = biomcp_types::VaersSummary::into_reply(q.clone(), None);
    let command = cx.fetch.command_deadline(vaers::SPEC.budget_for(cx.budget));
    let deadline = command.deadline();
    if let Err(error) = read(cx, q, deadline, &mut out) {
        out.local = status::local(error);
    }
    command.finish(deadline.remaining());
    out
}
fn key(q: &AdverseEventSearch) -> Result<Vec<u8>, RequestError> {
    source_snapshot::key("vaers", &(q.vaccine(), q.vaers_section().code()))
}
fn expired(q: &AdverseEventSearch, out: &mut Outcome) {
    out.body = ReplyData::PoolError { mismatch: false };
    out.next.push(crate::next::Step::run(
        StepKind::Restart,
        &[biomcp_types::Source::Vaers],
        Request::SearchAdverseEvent(q.restart()),
        false,
    ));
}
fn read(
    cx: &Ctx<'_>,
    q: &AdverseEventSearch,
    deadline: Deadline<'_>,
    out: &mut Outcome,
) -> Result<(), RequestError> {
    let leg = cx.fetch.leg(&vaers::SPEC, deadline).fresh(cx.fresh);
    if cx.fetch.disabled(Source::Vaers) {
        let error = biomcp_fetch::SourceError::disabled(Source::Vaers).into();
        out.sources
            .push(status::source(Source::Vaers, leg.finish(), Some(error)));
        return Err(error);
    }
    if q.offset() > 0 && q.snapshot().is_none() {
        expired(q, out);
        return Ok(());
    }
    let (saved, snapshot) = if let Some(snapshot) = q.snapshot() {
        let saved: Option<Saved> =
            source_snapshot::read(cx, &key(q)?, snapshot.as_str(), deadline)?;
        let Some(saved) = saved else {
            expired(q, out);
            return Ok(());
        };
        (saved, Some(snapshot.clone()))
    } else {
        let reply = vaers::read(&leg, q.vaccine().unwrap_or(""), q.vaers_section());
        let error = reply.as_ref().err().copied();
        out.sources
            .push(status::source(Source::Vaers, leg.finish(), error));
        let saved = (reply?, out.sources.clone());
        let snapshot = crate::publish::keep(
            source_snapshot::write(cx, &key(q)?, &saved, &out.sources, deadline),
            out,
        )?;
        (saved, snapshot)
    };
    out.sources = saved.1;
    {
        let unavailable = out
            .sources
            .iter()
            .any(|s| s.state == biomcp_types::SourceState::Stale);
        crate::status::item(out, true, false, unavailable);
    };
    render(q, saved.0, snapshot, out);
    Ok(())
}
fn render(
    q: &AdverseEventSearch,
    mut table: VaersSummary,
    snapshot: Option<Text>,
    out: &mut Outcome,
) {
    let total = table.rows.len() as u64;
    if q.vaers_sort() == Some(VaersSort::Reports) {
        by_reports(&mut table, out);
    }
    table.rows = table
        .rows
        .iter()
        .skip(q.offset() as usize)
        .take(q.limit() as usize)
        .cloned()
        .collect();
    let returned = table.rows.len() as u32;
    out.page = Some(PageInfo {
        offset: q.offset(),
        returned,
        total,
        pool: None,
        truncated: None,
    });
    let offset = q.offset().saturating_add(returned);
    // Only a held table can be sorted without a new source request.
    if let Some(snapshot) = snapshot
        .as_ref()
        .filter(|_| q.vaers_sort().is_none() && u64::from(returned) < total)
    {
        offer_sort(q, snapshot, out);
    }
    if let Some(snapshot) = snapshot.filter(|_| returned > 0 && u64::from(offset) < total) {
        out.next.push(crate::next::Step::run(
            StepKind::Page,
            &[biomcp_types::Source::Vaers],
            Request::SearchAdverseEvent(q.with_snapshot(snapshot).at_offset(offset)),
            false,
        ));
    }
    for section in [
        VaersSection::Symptoms,
        VaersSection::Seriousness,
        VaersSection::Age,
    ] {
        if section != q.vaers_section() {
            out.next.push(crate::next::Step::run(
                StepKind::Section,
                &[biomcp_types::Source::Vaers],
                Request::SearchAdverseEvent(q.with_vaers_section(section)),
                false,
            ));
        }
    }
    out.body = table.with_query(q.clone());
}

/// Offer the same retained table largest first, without a new source request.
fn offer_sort(q: &AdverseEventSearch, snapshot: &Text, out: &mut Outcome) {
    let held = q.with_snapshot(snapshot.clone());
    out.next.push(crate::next::Step::run(
        StepKind::Sort,
        &[biomcp_types::Source::Vaers],
        Request::SearchAdverseEvent(held.with_vaers_sort(VaersSort::Reports)),
        false,
    ));
}
/// Stable largest-first order by the native Events Reported value; rows without a count last.
/// One notice says the whole table was ordered locally from the source's own counts.
fn by_reports(table: &mut VaersSummary, out: &mut Outcome) {
    let mut rows = table.rows.to_vec();
    // A count is digits with thousands commas; compare by digit length, then digits, so any
    // size orders without parsing. Other text, such as Suppressed, has no count.
    rows.sort_by_key(|row| {
        std::cmp::Reverse(row.reports.get("v").and_then(|v| {
            let digits = v.replace(',', "");
            let digits = digits.trim_start_matches('0');
            (!v.is_empty() && v.bytes().all(|b| b.is_ascii_digit() || b == b','))
                .then(|| (digits.len(), digits.to_owned()))
        }))
    });
    let total = rows.len();
    table.rows = rows.into();
    if let Some(report) = out.sources.first_mut() {
        report
            .notices
            .push(biomcp_types::SourceNotice::RequestGuidance {
                data_date: None,
                message: format!(
                    "--sort reports: BioMCP ordered all {total} rows of this CDC WONDER table, \
                     not just this page, by the source's Events Reported count, largest first; \
                     equal counts and rows without a count keep the source's order. Counts \
                     and the report total are the source's, and each percent is the source's \
                     percent of the {} reports.",
                    table.report_total
                )
                .into(),
                retry_after_seconds: None,
            });
    }
}
