//! Independent NCI source: never a registry fallback or local disease matcher.
mod get;
mod page;
use crate::{Action, Ctx, Outcome, status};
use biodata::{
    NciTrialCards, NciTrialItem, NciTrialPage, QueryBody, Reason, Request, Source, Status,
    StepKind, Text, TrialSearch,
};
use biomcp_adapters::nci;
pub(super) use get::run as get;
const ATTRIBUTION: &str =
    "NCI Clinical Trials Search; source-native fields, no eligibility assessment";
/// An NCIt code is an explicit identifier, never a disease name guess.
fn ncit(value: &Text) -> bool {
    value
        .as_str()
        .strip_prefix('C')
        .is_some_and(|n| !n.is_empty() && n.bytes().all(|b| b.is_ascii_digit()))
}
/// Reject unsupported registry filters without contacting either provider.
fn valid(q: &TrialSearch) -> bool {
    q.limit() <= nci::PAGE_LIMIT
        && q.query().is_none()
        && q.gene().is_none()
        && q.variant().is_none()
        && q.criteria().is_none()
        && q.intervention().is_none()
        && q.status().is_none()
        && q.phase().is_none()
        && q.study_type().is_none()
        && q.sex().is_none()
        && q.age().is_none()
        && q.facility().is_none()
        && q.lat().is_none()
        && q.sponsor().is_none()
        && q.sponsor_type().is_none()
        && q.updated_since().is_none()
        && q.updated_until().is_none()
        && q.id().is_none()
        && !q.has_results()
        && q.next_page()
            .is_none_or(|p| p.as_str().parse::<u32>().is_ok())
}
pub(super) fn search(cx: &Ctx<'_>, q: &TrialSearch) -> Outcome {
    let mut out = empty(QueryBody::NciTrialPage(NciTrialPage {
        items: Box::default(),
        request: Box::default(),
        attribution: ATTRIBUTION.into(),
    }));
    if q.condition().is_some_and(|v| !ncit(v)) {
        out.body = QueryBody::NciDiseaseError;
        return out;
    }
    if !valid(q) {
        out.body = QueryBody::TextPositionError;
        return out;
    }
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let leg = cx.fetch.leg(&nci::SPEC, deadline).fresh(cx.fresh);
    let params = nci::parameters(q);
    let result = nci::search(&leg, &params);
    let error = result.as_ref().err().copied();
    let mut source = status::source(Source::Nci, leg.finish(), error);
    if let Ok(page) = result {
        source.count = Some(page.total);
        source.fetched = Some(page.data.len() as u32);
        out.status = if source.state == biodata::SourceState::Stale {
            Status::Partial
        } else {
            Status::Complete
        };
        let offset = q
            .next_page()
            .and_then(|v| v.as_str().parse::<u32>().ok())
            .unwrap_or_default();
        let returned = if q.count_only() {
            0
        } else {
            page.data.len() as u32
        };
        out.page = Some(biodata::PageInfo {
            offset,
            returned,
            total: page.total,
            pool: None,
            truncated: None,
        });
        if !q.count_only()
            && returned > 0
            && u64::from(offset.saturating_add(returned)) < page.total
        {
            let token = Text::new(&offset.saturating_add(returned).to_string());
            if let Ok(token) = token {
                out.next.push((
                    StepKind::Page,
                    Action::Run {
                        request: Request::SearchTrial(q.at_page(token)),
                        fresh: cx.fresh,
                    },
                ));
            }
        }
        if let QueryBody::NciTrialPage(body) = &mut out.body {
            if !q.count_only() {
                body.items = page.data.into_iter().map(|r| r.card(false)).collect();
            }
            body.request = params
                .into_iter()
                .map(|(name, value)| biodata::TrialParameter { name, value })
                .collect();
        }
    }
    finish(&mut out, source, error);
    command.finish(deadline.remaining());
    out
}

fn finish(
    out: &mut Outcome,
    source: biodata::SourceOutcome,
    error: Option<biomcp_fetch::RequestError>,
) {
    if source.reason == Some(Reason::NoKey) {
        out.next
            .insert(0, (StepKind::Key, Action::KeyHelp(Source::Nci)));
    }
    out.local = error.and_then(status::local);
    out.sources.push(source);
}
fn empty(body: QueryBody) -> Outcome {
    Outcome {
        status: Status::Failed,
        body,
        page: None,
        sources: Vec::new(),
        problems: Vec::new(),
        next: Vec::new(),
        local: None,
    }
}
