//! Page the intersection of disease annotations in one HPO release.
use crate::{Ctx, Outcome, status};
use biomcp_adapters::hpo::release;
use biomcp_types::{
    DiseaseSearch, HpoDiseasePage, Paged, ReplyBody, Request, Source, SourceNotice, StepKind,
};

pub(super) fn search(
    cx: &Ctx<'_>,
    query: &DiseaseSearch,
    tag: &str,
    terms: &[biodata::HpId],
) -> Outcome {
    let command = cx.fetch.source_command_deadline(&release::SPEC, cx.budget);
    let deadline = command.deadline();
    let leg = cx.fetch.leg(&release::SPEC, deadline).fresh(cx.fresh);
    let answer = release::search(&leg, query, tag, terms);
    let error = answer.as_ref().err().copied();
    let mut source = status::source(Source::Hpo, leg.finish(), error);
    let mut page = HpoDiseasePage {
        release: tag.into(),
        items: Box::default(),
    };
    let mut next = Vec::new();
    let mut bounds = None;
    if let Ok(result) = answer {
        let returned = result.items.len() as u32;
        let offset = query.offset().saturating_add(returned);
        if returned > 0 && u64::from(offset) < result.total {
            next.push(crate::next::Step::new(
                StepKind::Page,
                &[Source::Hpo],
                super::page::run(Request::SearchDisease(query.at_offset(offset))),
            ));
        }
        if !result.unknown.is_empty() {
            let terms = result
                .unknown
                .iter()
                .map(biodata::HpId::as_str)
                .collect::<Vec<_>>();
            source.notices.push(notice(format!(
                "No current term in this release for {}.",
                terms.join(", ")
            )));
        }
        bounds = Some(status::page_info(query.offset(), returned, result.total));
        page.items = result.items;
    }
    source.notices.push(notice(format!(
        "This product uses Human Phenotype Ontology release {tag}. Annotations requiring separate OMIM permission are excluded."
    )));
    let mut out = status::empty(page.with_query(query.clone()));
    out.page = bounds;
    out.next = next;
    out.sources.push(source);
    out.local = error.and_then(status::local);
    status::search(&mut out);
    command.finish(deadline.remaining());
    out
}

fn notice(message: String) -> SourceNotice {
    SourceNotice::RequestGuidance {
        data_date: None,
        message: message.into(),
        retry_after_seconds: None,
    }
}
