//! Source-local CIViC therapy candidates and independent native claim streams.
use crate::{Ctx, Outcome, status};
use biomcp_adapters::civic;
use biomcp_types::Paged;
use biomcp_types::ReplyBody;
use biomcp_types::{
    DrugGet, DrugPage, DrugSearch, DrugSection, DrugSource, PageInfo, Request, Source, StepKind,
};

pub(super) fn search(cx: &Ctx<'_>, query: &DrugSearch) -> Outcome {
    let mut out = super::empty(
        (DrugPage {
            items: Box::default(),
            total: None,
        })
        .with_query(query.clone()),
    );
    let Some(name) = query.query() else {
        return super::unsupported(query.source(), out);
    };
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let leg = cx.fetch.leg(&civic::SPEC, deadline).fresh(cx.fresh);
    let result = civic::drug_search(&leg, name, query.limit(), query.offset());
    let error = result.as_ref().err().copied();
    out.local = error.and_then(status::local);
    out.sources
        .push(status::source(Source::Civic, leg.finish(), error));
    if let Ok(mapped) = result {
        out.include_refusals(mapped.refusals);
        out.problems = mapped.problems;
        let (page, consumed) = mapped.value;
        {
            let found = !page.items.is_empty();
            crate::status::item(&mut out, found, false, false);
        };
        let returned = page.items.len() as u32;
        out.sources.iter_mut().for_each(|s| {
            s.count = page.total;
            s.retrieved_count = Some(returned);
        });
        out.page = page.total.map(|total| PageInfo {
            offset: query.offset(),
            returned,
            total,
            pool: None,
            truncated: None,
        });
        for item in &page.items {
            if let Some(get) = crate::next::checked(
                DrugGet::new(item.id.clone(), DrugSource::Civic)
                    .and_then(|q| q.for_section(DrugSection::Civic)),
                biomcp_types::Source::Civic,
                &mut out.problems,
            ) {
                out.next.push(crate::next::Step::run(
                    StepKind::Disambiguate,
                    &[biomcp_types::Source::Civic],
                    Request::GetDrug(get),
                    cx.fresh,
                ));
            }
        }
        if let Some(offset) = query
            .offset()
            .checked_add(consumed)
            .filter(|n| consumed > 0 && page.total.is_some_and(|t| u64::from(*n) < t))
        {
            out.next.push(crate::next::Step::run(
                StepKind::Page,
                &[biomcp_types::Source::Civic],
                Request::SearchDrug(query.at_offset(offset)),
                cx.fresh,
            ));
        }
        out.body = page.with_query(query.clone());
    }
    status::search(&mut out);
    command.finish(deadline.remaining());
    out
}

/// CIViC can list one name under several therapies, each with its own claims. A name read
/// opens the first; say so, and give every other one the same read as a next step.
pub(super) fn matched_therapies(
    query: &DrugGet,
    candidates: &[(super::resolve::Rank, biomcp_types::DrugHeader)],
    fresh: bool,
    out: &mut Outcome,
) {
    let biomcp_types::ReplyData::DrugCivic {
        body: Some(body), ..
    } = &mut out.body
    else {
        return;
    };
    let Some(resolution) = body.identity.name_resolution.as_mut() else {
        return;
    };
    if candidates.len() < 2 {
        return;
    }
    let unread: Vec<_> = candidates.iter().skip(1).map(|c| c.1.id.to_string()).collect();
    resolution.reason = Some(
        format!(
            "CIViC lists {} therapies under this name, each with its own evidence and \
             assertions; this reply reads therapy {} only and does not read therapy {}; get each \
             alternative to read the others",
            resolution.total,
            body.identity.id,
            unread.join(", ")
        )
        .into(),
    );
    let sections = query.sections().iter().map(|s| <&str>::from(*s).into());
    for step in resolution.alternatives.iter_mut() {
        let mut args = step.args.to_vec();
        args.splice(3..3, sections.clone());
        step.args = args.into();
    }
    // The other therapies come before per-record follow-ups, in source order.
    let mut reads = Vec::new();
    for (_, other) in candidates.iter().skip(1).take(5) {
        if let Some(get) = crate::next::checked(
            query.with_native_id(other.id.clone()),
            Source::Civic,
            &mut out.problems,
        ) {
            reads.push(crate::next::Step::run(
                StepKind::Disambiguate,
                &[Source::Civic],
                Request::GetDrug(get),
                fresh,
            ));
        }
    }
    out.next.splice(0..0, reads);
    // Models read a complete reply as the whole answer; the unread therapies make it partial.
    if out.status() == biomcp_types::Status::Complete {
        crate::status::item(out, true, false, true);
    }
}

pub(super) fn get(cx: &Ctx<'_>, query: &DrugGet) -> Outcome {
    let mut out = super::empty(biomcp_types::DrugCivic::into_reply(query.clone(), None));
    if query.sections().is_empty() || !query.supports_sections() {
        return super::unsupported(query.source(), out);
    }
    let Some(id) = query.civic_id() else {
        return super::unsupported(query.source(), out);
    };
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let leg = cx.fetch.leg(&civic::SPEC, deadline).fresh(cx.fresh);
    let result = civic::drug_civic(&leg, query, id);
    let error = result.as_ref().err().copied();
    out.local = error.and_then(status::local);
    out.sources
        .push(status::source(Source::Civic, leg.finish(), error));
    if let Ok(mapped) = result {
        out.include_refusals(mapped.refusals);
        out.problems = mapped.problems;
        let Some(mut value) = mapped.value else {
            crate::status::item(&mut out, true, false, false);
            command.finish(deadline.remaining());
            return out;
        };
        if let biomcp_types::Section::Data { data, .. } = &value.civic {
            for stream in [&data.evidence, &data.assertions].into_iter().flatten() {
                if let Some(next) = &stream.page.next {
                    out.next.push(crate::next::Step::run(
                        StepKind::Page,
                        &[biomcp_types::Source::Civic],
                        Request::GetDrug(next.clone()),
                        cx.fresh,
                    ));
                }
            }
        }
        super::next::civic(cx.fetch, &mut value, cx.fresh, &mut out);
        crate::status::item(&mut out, true, false, false);
        out.body = value.with_query(query.clone());
    }
    command.finish(deadline.remaining());
    out
}
