//! Count buckets keep their own shape instead of pretending to be report pages.
use crate::{Ctx, Outcome, status};
use biomcp_adapters::faers;
use biomcp_types::{
    AdverseEventPage, AdverseEventReactionCounts, AdverseEventSearch, ReplyBody, ReportUnit, Source,
};

pub(super) fn search(cx: &Ctx<'_>, query: &AdverseEventSearch) -> Outcome {
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let leg = cx.fetch.leg(&faers::SPEC, deadline).fresh(cx.fresh);
    let reply = faers::reactions(&leg, query);
    let error = reply.as_ref().err().copied();
    let mut body = AdverseEventPage {
        reaction_counts: None,
        reaction_match: query
            .reaction()
            .map(|_| biomcp_types::AdverseEventReactionMatch::PhraseMatch),
        source_query: query.faers_expression().into_boxed_str(),
        source_updated: None,
        fetched: None,
        items: Box::default(),
        unit: ReportUnit::Reports,
        components: Box::default(),
    };
    if let Ok(page) = reply {
        if !page.items.is_empty() {
            body.reaction_counts = Some(AdverseEventReactionCounts {
                limit_reached: page.items.len() == query.limit() as usize,
                items: page.items.into(),
            });
        }
        body.source_updated = page.updated.map(Into::into);
        body.fetched = page
            .metadata
            .fetched
            .and_then(|value| u64::try_from(value).ok());
        body.components = page.components;
    }
    let mut out = status::finish(
        [status::item_status(false, true, false)],
        status::Parts {
            body: body.with_query(query.clone()),
            page: None,
            sources: vec![status::source(Source::Faers, leg.finish(), error)],
            problems: Vec::new(),
            next: Vec::new(),
            local: error.and_then(status::local),
            dropped: false,
        },
    );
    status::search(&mut out);
    command.finish(deadline.remaining());
    out
}
