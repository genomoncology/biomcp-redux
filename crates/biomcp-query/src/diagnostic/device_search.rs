//! Source-owned FDA device search and document access.
use super::{empty, next};
use crate::{Ctx, Outcome, status};
use biomcp_types::{
    DiagnosticAggregate, DiagnosticCountUnit, DiagnosticProductCodeCounts, DiagnosticSearch,
    PageInfo, Paged, ReplyBody, Request, Source, SourceNotice, StepKind,
};
pub(super) fn search(cx: &Ctx<'_>, q: &DiagnosticSearch) -> Outcome {
    let source = q.source().source();
    let spec = if source == Source::FdaPma {
        &biomcp_adapters::fda_pma::SPEC
    } else {
        &biomcp_adapters::fda_510k::SPEC
    };
    let expression = if source == Source::FdaPma {
        biomcp_adapters::fda_pma::expression(q)
    } else {
        biomcp_adapters::fda_510k::expression(q)
    };
    let source_query = || Some(expression.clone().into_boxed_str()).filter(|s| !s.is_empty());
    let mut out = empty(
        super::device_page(
            Box::default(),
            DiagnosticCountUnit::DeviceSubmissions,
            source_query(),
        )
        .with_query(q.clone()),
    );
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let leg = cx.fetch.leg(spec, deadline).fresh(cx.fresh);
    let result = if q.source() == biomcp_types::DiagnosticSearchSource::FdaPma {
        biomcp_adapters::fda_pma::search(&leg, q)
    } else {
        biomcp_adapters::fda_510k::search(&leg, q)
    };
    // The count reads the same expression as the page; its sum is checked against the page total.
    let result = result.and_then(|page| {
        let counts = match q.aggregate() {
            Some(DiagnosticAggregate::ProductCode) => Some(product_codes(&leg, q, page.total)?),
            None => None,
        };
        Ok((page, counts))
    });
    let error = match result {
        Ok((page, product_code_counts)) => {
            let returned = page.items.len() as u32;
            {
                let found = page.total != 0;
                crate::status::item(&mut out, found, false, false);
            };
            out.problems = page.problems;
            next::candidates(&mut out, q, &page.items, cx.fresh);
            if u64::from(q.offset()) + u64::from(returned) < page.total && returned > 0 {
                next::push(
                    &mut out,
                    StepKind::Page,
                    &[q.source().source()],
                    Request::SearchDiagnostic(
                        q.without_aggregate().at_offset(q.offset() + returned),
                    ),
                    cx.fresh,
                );
            }
            out.page = Some(PageInfo {
                offset: q.offset(),
                returned,
                total: page.total,
                pool: None,
                truncated: None,
            });
            let mut body = super::device_page(
                page.items,
                DiagnosticCountUnit::DeviceSubmissions,
                source_query(),
            );
            body.product_code_counts = product_code_counts;
            out.body = body.with_query(q.clone());
            None
        }
        Err(error) => Some(error),
    };
    out.local = error.and_then(status::local);
    out.sources
        .push(status::source(source, leg.finish(), error));
    applicant(&mut out, q, cx.fresh);
    status::search(&mut out);
    command.finish(deadline.remaining());
    out
}

/// An empty --name page matched only the trade or device name; the same words may be the
/// FDA applicant, so the reply says so and offers them as --manufacturer (1040).
fn applicant(out: &mut Outcome, q: &DiagnosticSearch, fresh: bool) -> Option<()> {
    let none_found = out.page.as_ref().is_some_and(|page| page.total == 0);
    if !none_found || q.name().is_none() || q.manufacturer().is_some() {
        return None;
    }
    let source = q.source().source();
    let get = crate::next::checked(q.name_as_manufacturer(), source, &mut out.problems)?;
    next::push(
        out,
        StepKind::Configure,
        &[source],
        Request::SearchDiagnostic(get),
        fresh,
    );
    let field = if source == Source::FdaPma {
        "PMA trade name"
    } else {
        "510(k) device name"
    };
    let report = out.sources.iter_mut().find(|r| r.source == source)?;
    report.notices.push(SourceNotice::RequestGuidance {
        data_date: None,
        message: format!(
            "--name matched the {field} only; the FDA applicant is searched with --manufacturer."
        )
        .into(),
        retry_after_seconds: None,
    });
    Some(())
}

/// Matching 510(k) records by product code, with the sum checked against the search total.
fn product_codes(
    leg: &biomcp_fetch::Leg<'_>,
    q: &DiagnosticSearch,
    total: u64,
) -> Result<DiagnosticProductCodeCounts, biomcp_fetch::RequestError> {
    let codes = biomcp_adapters::fda_510k::product_codes(leg, q)?;
    let sum = codes.items.iter().map(|row| row.count).sum::<u64>() + codes.without_product_code;
    Ok(DiagnosticProductCodeCounts {
        items: codes.items.into(),
        without_product_code: codes.without_product_code,
        sum,
        sum_matches_total: sum == total,
        limit_reached: codes.limit_reached,
    })
}
