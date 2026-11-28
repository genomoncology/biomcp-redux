//! Study results, preserving native sample descriptions.
use crate::{Ctx, Outcome, status};
use biomcp_adapters::{Mapped, gwas};
use biomcp_fetch::{Absent, Leg, LegReport, RequestError};
use biomcp_types::{
    GwasSearch, GwasStudy, GwasStudyPage, GwasStudyRow, PageInfo, Paged, ReplyBody, Source,
    StepKind,
};
pub(super) fn search(cx: &Ctx<'_>, q: &GwasSearch) -> Outcome {
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let leg = cx.fetch.leg(&gwas::SPEC, deadline).fresh(cx.fresh);
    let result = gwas::studies(&leg, q);
    let out = finish(cx, q, result, leg.finish());
    command.finish(deadline.remaining());
    out
}

pub(super) fn find_unpublished(
    leg: &Leg<'_>,
    q: &GwasSearch,
) -> Result<Option<Mapped<GwasStudy>>, RequestError> {
    // An association filter cannot be evaluated against an unpublished study card.
    if let Some(id) = q.study()
        && q.variant().is_none()
        && q.trait_label().is_none()
        && q.trait_id().is_none()
        && q.gene().is_none()
        && q.pubmed().is_none()
    {
        return gwas::get::unpublished_study(leg, id).absent();
    }
    Ok(None)
}

pub(super) fn unpublished(
    cx: &Ctx<'_>,
    q: &GwasSearch,
    study: Mapped<GwasStudy>,
    report: LegReport,
) -> Outcome {
    let items = (q.offset() == 0)
        .then_some(GwasStudyRow {
            study: study.value,
            association_count: Some(0),
        })
        .into_iter()
        .collect();
    let rows = gwas::Studies {
        total: 1,
        received: 1,
        items,
        refusals: study.refusals,
        problems: study.problems,
    };
    finish(cx, q, Ok(rows), report)
}

fn finish(
    cx: &Ctx<'_>,
    q: &GwasSearch,
    result: Result<gwas::Studies, RequestError>,
    report: LegReport,
) -> Outcome {
    let mut out = status::empty(GwasStudyPage::into_reply(q.clone(), None));
    let error = result.as_ref().err().copied();
    let mut source = status::source(Source::Gwas, report, error);
    match result {
        Ok(rows) => {
            out.include_refusals(rows.refusals);
            out.dropped |= rows.items.iter().any(|row| row.association_count.is_none());
            out.problems.extend(rows.problems);
            source.retrieved_count = Some(rows.items.len() as u32);
            source.count = Some(rows.total);
            let start = q.offset() - q.offset() % q.limit();
            let continuation = (u64::from(start) + u64::from(rows.received) < rows.total)
                .then(|| q.at_offset(start.saturating_add(rows.received)));
            let page = PageInfo {
                offset: q.offset(),
                returned: rows.items.len() as u32,
                total: rows.total,
                pool: None,
                truncated: None,
            };
            out.page = Some(page.clone());
            if let Some(query) = &continuation {
                out.next.push(crate::next::Step::run(
                    StepKind::Page,
                    &[Source::Gwas],
                    biomcp_types::Request::SearchGwas(query.clone()),
                    cx.fresh,
                ));
            }
            for row in rows.items.iter().take(3) {
                let get = biomcp_types::GwasGet::Study {
                    id: row.study.id.clone(),
                    section: None,
                    page: Default::default(),
                };
                out.next.push(crate::next::Step::run(
                    StepKind::Pivot,
                    &[Source::Gwas],
                    get.into(),
                    false,
                ));
            }
            out.body = GwasStudyPage {
                items: rows.items.into(),
                page,
                continuation,
            }
            .with_query(q.clone());
            status::item(
                &mut out,
                true,
                false,
                source.state == biomcp_types::SourceState::Stale,
            );
        }
        Err(error) => out.local = status::local(error),
    }
    out.sources.push(source);
    status::search(&mut out);
    out
}
