//! Source-local association search, without clinical interpretation.
use biomcp_types::Paged;
use biomcp_types::ReplyBody;
mod get;
pub(crate) mod map;
pub(crate) use get::get;
mod next;
mod studies;
mod threshold;
mod traits;
use crate::{Ctx, Outcome, status};
use biomcp_adapters::gwas;
use biomcp_types::{
    GwasPage, GwasPageQuery, GwasSearch, PageInfo, Problem, ProblemCode, ReplyData, Source,
    SourceState, Status,
};
/// One bounded native page; no per-hit metadata calls.
pub(crate) fn search(cx: &Ctx<'_>, q: &GwasSearch) -> Outcome {
    if q.studies() {
        return studies::search(cx, q);
    }
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    if q.p_value().is_some() {
        let mut out = threshold::search(cx, q, deadline);
        traits::resolve(cx, q, deadline, &mut out);
        command.finish(deadline.remaining());
        return out;
    }
    let mut out = empty(q);
    let leg = cx.fetch.leg(&gwas::SPEC, deadline).fresh(cx.fresh);
    let result = gwas::search(&leg, q);
    let fallback = match &result {
        Ok(page) if page.total == 0 => studies::find_unpublished(&leg, q),
        _ => Ok(None),
    };
    if let Ok(Some(study)) = fallback {
        let out = studies::unpublished(cx, q, study, leg.finish());
        command.finish(deadline.remaining());
        return out;
    }
    let fallback_error = fallback.err();
    let error = result.as_ref().err().copied();
    let counts = result
        .as_ref()
        .ok()
        .map(|page| (page.rows.len() as u32, page.total));
    if let Ok(page) = result {
        association_page(q, &mut out, page);
    }
    let mut source = status::source(Source::Gwas, leg.finish(), error.or(fallback_error));
    if let Some((accepted, total)) = counts {
        source.retrieved_count = Some(accepted);
        source.count = Some(total);
    }
    if source.reason == Some(biomcp_types::Reason::BadReply) {
        out.problems.push(Problem {
            source: Source::Gwas,
            record: None,
            field: if fallback_error.is_some() {
                "unpublished_study".into()
            } else {
                "association_page".into()
            },
            code: ProblemCode::ValueUnknown,
        });
    }
    if out.status() == Status::Complete && source.state == SourceState::Stale {
        crate::status::item(&mut out, true, false, true);
    }
    let ReplyData::GwasPage { body, .. } = &out.body else {
        return out;
    };
    out.next = next::search(q, body.as_ref(), source.state, cx.fresh, &mut out.problems);
    out.sources.push(source);
    status::search(&mut out);
    if fallback_error.is_some() {
        status::item(&mut out, true, false, true);
    }
    out.local = error.or(fallback_error).and_then(status::local);
    traits::resolve(cx, q, deadline, &mut out);
    command.finish(deadline.remaining());
    out
}

fn association_page(q: &GwasSearch, out: &mut Outcome, rows: gwas::Page) {
    out.include_refusals(rows.refusals);
    out.problems.extend(rows.problems);
    let items: Box<[_]> = rows
        .rows
        .into_iter()
        .map(|row| map::association(row, &mut out.problems))
        .collect();
    let start = q.offset() - q.offset() % q.limit();
    let continuation = (u64::from(start) + u64::from(rows.received) < rows.total)
        .then(|| GwasPageQuery::Search(q.at_offset(start.saturating_add(rows.received))));
    let page = PageInfo {
        offset: q.offset(),
        returned: items.len() as u32,
        total: rows.total,
        pool: None,
        truncated: None,
    };
    out.page = Some(page.clone());
    out.body = (GwasPage {
        items,
        page,
        continuation,
        qualifying_total: None,
    })
    .with_query(GwasPageQuery::Search(q.clone()));
    status::item(out, true, false, false);
}

fn empty(q: &GwasSearch) -> Outcome {
    crate::status::empty(biomcp_types::GwasPage::into_reply(
        biomcp_types::GwasPageQuery::Search(q.clone()),
        None,
    ))
}
