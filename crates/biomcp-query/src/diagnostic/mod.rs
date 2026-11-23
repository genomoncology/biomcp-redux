//! Explicit diagnostic source ownership; reads consume only complete local snapshots.
use biomcp_types::Paged;
use biomcp_types::ReplyBody;
mod clearance;
mod companion;
mod device_search;
mod get;
mod next;
mod pma;
mod recall;
mod summary;
#[cfg(test)]
mod summary_tests;
mod who;
pub(super) use crate::status::empty;
use crate::{Action, Bundle, Ctx, Outcome, status};
use biomcp_adapters::gtr;
use biomcp_fetch::{Leg, RequestError};
use biomcp_types::{
    DiagnosticPage, DiagnosticSearch, DiagnosticSearchSource, PageInfo, Reason, Request, Source,
    SourceState, Status, StepKind,
};
pub(crate) use get::get;
/// An openFDA device page; a 510(k) product-code count is set by its caller.
fn device_page(
    items: Box<[biomcp_types::DiagnosticCandidate]>,
    unit: biomcp_types::DiagnosticCountUnit,
    source_query: Option<Box<str>>,
) -> DiagnosticPage {
    DiagnosticPage {
        items,
        count_unit: Some(unit),
        distinct_id_total: None,
        source_query,
        product_code_counts: None,
    }
}
fn finish(out: &mut Outcome, leg: Leg<'_>, error: Option<RequestError>) {
    finish_bundle(out, leg, error, Source::Gtr, Bundle::GTR);
}
fn finish_bundle(
    out: &mut Outcome,
    leg: Leg<'_>,
    error: Option<RequestError>,
    source: Source,
    bundle: Bundle,
) {
    out.local = error.and_then(status::local);
    out.sources
        .push(status::source(source, leg.finish(), error));
    if error.is_some() {
        crate::status::item(out, false, true, false);
    } else if (out.dropped || out.sources.iter().any(|s| s.state == SourceState::Stale))
        && out.status() == Status::Complete
    {
        crate::status::item(out, true, false, true);
    }
    if out.sources.iter().any(|s| {
        matches!(s.state, SourceState::Syncing | SourceState::Stale)
            || (s.source, s.reason) == (Source::Gtr, Some(Reason::Unavailable))
    }) && !out
        .next
        .iter()
        .any(|step| step.1 == Action::Sync { bundle })
    {
        out.next.push(crate::next::Step::new(
            StepKind::Sync,
            &[source],
            Action::Sync { bundle },
        ));
    }
}
/// The openFDA firm and product code values every device source searches, each named by the
/// option typed: a device recall takes --firm for --manufacturer (0976).
fn firm(q: &DiagnosticSearch) -> [(&'static str, Option<&biodata::Text>, bool); 2] {
    [
        (q.manufacturer_option(), q.manufacturer(), false),
        ("--product-code", q.product_code(), false),
    ]
}
/// Search an openFDA device source and name each typed value sent as other words (0841).
fn openfda_search(cx: &Ctx<'_>, q: &DiagnosticSearch) -> Option<Outcome> {
    let (mut out, source) = match q.source() {
        DiagnosticSearchSource::FdaPma => (device_search::search(cx, q), Source::FdaPma),
        DiagnosticSearchSource::Fda510k => (device_search::search(cx, q), Source::Fda510k),
        // The recall name notice already names the phrase searched (0901).
        DiagnosticSearchSource::FdaDeviceRecall => {
            let mut out = recall::search(cx, q);
            crate::openfda_text::disclose(&mut out, Source::FdaDeviceRecall, firm(q));
            return Some(out);
        }
        _ => return None,
    };
    let values = [
        ("--name", q.name(), false),
        ("--intended-use", q.intended_use(), false),
        ("--classification-name", q.classification_name(), false),
    ];
    crate::openfda_text::disclose(&mut out, source, values.into_iter().chain(firm(q)));
    Some(out)
}
/// Search only stored source-native GTR relations and retain matched selectors.
pub(crate) fn search(cx: &Ctx<'_>, q: &DiagnosticSearch) -> Outcome {
    if let Some(out) = openfda_search(cx, q) {
        return out;
    }
    if q.source() == DiagnosticSearchSource::FdaCdx {
        return companion::search(cx, q);
    }
    let (spec, source, bundle, count) = search_source(q.source());
    let who = q.source() == DiagnosticSearchSource::WhoIvd;
    let mut out = empty(
        (DiagnosticPage {
            items: Box::default(),
            count_unit: Some(count),
            distinct_id_total: None,
            source_query: None,
            product_code_counts: None,
        })
        .with_query(q.clone()),
    );
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let leg = cx.fetch.leg(spec, deadline);
    let result = if who {
        biomcp_adapters::who_ivd::search(&leg, q).map(|p| (p.items, p.total, p.info))
    } else {
        gtr::search(&leg, q).map(|p| {
            out.include_refusals(p.refusals);
            status::append(&mut out.problems, p.problems);
            (p.items, p.total, p.info)
        })
    };
    let error = result.as_ref().err().copied();
    if let Ok((items, total, info)) = result {
        crate::status::item(&mut out, true, false, false);
        let bound = info
            .content_revision
            .as_deref()
            .and_then(|v| v.parse().ok())
            .map_or_else(|| q.clone(), |v| q.with_bundle_revision(v));
        next::candidates(&mut out, &bound, &items, cx.fresh);
        let returned = items.len() as u32;
        if u64::from(bound.offset()) + u64::from(returned) < total {
            next::push(
                &mut out,
                StepKind::Page,
                &[bound.source().source()],
                Request::SearchDiagnostic(bound.at_offset(bound.offset() + returned)),
                cx.fresh,
            );
        }
        out.page = Some(PageInfo {
            offset: bound.offset(),
            returned,
            total,
            pool: None,
            truncated: None,
        });
        out.body = (DiagnosticPage {
            items,
            count_unit: Some(count),
            distinct_id_total: None,
            source_query: None,
            product_code_counts: None,
        })
        .with_query(q.clone());
    }
    finish_bundle(&mut out, leg, error, source, bundle);
    status::search(&mut out);
    if !who && error.is_none() {
        name_notice(&mut out, q);
    }
    command.finish(deadline.remaining());
    out
}
/// Say how GTR matched `--name` and `--condition`, so a zero is read against the matching rule.
fn name_notice(out: &mut Outcome, q: &DiagnosticSearch) {
    let Some(source) = out.sources.last_mut() else {
        return;
    };
    if q.name().is_some() {
        source
            .notices
            .push(recall::guidance(GTR_NAME_NOTICE.into()));
    }
    if q.condition().is_some() {
        source
            .notices
            .push(recall::guidance(GTR_CONDITION_NOTICE.into()));
    }
}
const GTR_NAME_NOTICE: &str = "--name matched GTR lab_test_name ignoring letter case and hyphens; \
    hyphens and spaces in your text are optional, so non-small cell also finds Nonsmall cell; \
    no synonym or abbreviation expansion.";
const GTR_CONDITION_NOTICE: &str = "--condition matched the condition names in GTR \
    condition_identifiers by the same rule as --name; no synonym or abbreviation expansion, \
    so NSCLC does not find non-small cell lung cancer.";
fn search_source(
    source: DiagnosticSearchSource,
) -> (
    &'static biomcp_fetch::SourceSpec,
    Source,
    Bundle,
    biomcp_types::DiagnosticCountUnit,
) {
    if source == DiagnosticSearchSource::WhoIvd {
        (
            &biomcp_adapters::who_ivd::SPEC,
            Source::WhoIvd,
            Bundle::WHO_IVD,
            biomcp_types::DiagnosticCountUnit::ProductIdentities,
        )
    } else {
        (
            &gtr::SPEC,
            Source::Gtr,
            Bundle::GTR,
            biomcp_types::DiagnosticCountUnit::TestIdentities,
        )
    }
}
fn unsupported(source: Source, mut out: Outcome) -> Outcome {
    out.sources.push(biomcp_adapters::registry::outcome(
        source,
        biomcp_types::SourceStatus::Skipped(Some(biomcp_types::Reason::Unsupported)),
    ));
    out
}
