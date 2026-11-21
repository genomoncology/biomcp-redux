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
    let report = leg.finish();
    let page = HpoDiseasePage {
        release: tag.into(),
        items: Box::default(),
    };
    let mut out = status::empty(page.with_query(query.clone()));
    let (mut source, error) = match answer {
        Ok(result) => {
            let mut source = status::source(Source::Hpo, report, None);
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
            apply(&mut out, query, result);
            (source, None)
        }
        Err(error) => (
            status::source(Source::Hpo, report, Some(error)),
            status::local(error),
        ),
    };
    source.notices.push(notice(format!(
        "This product uses Human Phenotype Ontology release {tag}. Annotations requiring separate OMIM permission are excluded."
    )));
    out.sources.push(source);
    out.local = error;
    status::search(&mut out);
    command.finish(deadline.remaining());
    out
}

fn apply(out: &mut Outcome, query: &DiseaseSearch, result: release::Matches) {
    let returned = result.items.len() as u32;
    let offset = query.offset().saturating_add(returned);
    if returned > 0 && u64::from(offset) < result.total {
        out.next.push(crate::next::Step::new(
            StepKind::Page,
            &[Source::Hpo],
            super::page::run(Request::SearchDisease(query.at_offset(offset))),
        ));
    }
    out.page = Some(status::page_info(query.offset(), returned, result.total));
    if let biomcp_types::ReplyData::HpoDiseasePage {
        body: Some(body), ..
    } = &mut out.body
    {
        body.items = result.items;
    }
}

fn notice(message: String) -> SourceNotice {
    SourceNotice::RequestGuidance {
        data_date: None,
        message: message.into(),
        retry_after_seconds: None,
    }
}
