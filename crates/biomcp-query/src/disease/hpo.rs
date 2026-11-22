//! Page the intersection of disease annotations in one HPO release.
use crate::{Ctx, Outcome, status};
use biomcp_adapters::hpo::release;
use biomcp_types::{
    DiseaseSearch, HpoDiseasePage, Paged, Reason, ReplyBody, Request, Source, SourceNotice,
    StepKind,
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
    let resolved = if tag == "latest" {
        release::latest(&leg, query)
    } else {
        Ok(query.clone())
    };
    let pinned = resolved.as_ref().map_err(|error| *error).and_then(|query| {
        query
            .hpo()
            .map(|(tag, _)| (query, tag))
            .ok_or_else(|| biomcp_fetch::bad_reply(Source::Hpo, None))
    });
    let answer = pinned.and_then(|(query, tag)| {
        release::search(&leg, query, tag, terms).map(|result| (query, tag, result))
    });
    let mut source = status::source(Source::Hpo, leg.finish(), answer.as_ref().err().copied());
    let mut out = status::empty(HpoDiseasePage::into_reply(query.clone(), None));
    match answer {
        Ok((pinned, tag, result)) => {
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
            let evidence = release::EXCLUDED_EVIDENCE;
            source.notices.push(notice(format!(
                "This product uses Human Phenotype Ontology release {tag}. \
                 {evidence} annotations (inferred from electronic annotation) are excluded."
            )));
            apply(&mut out, query, pinned, tag, result);
        }
        Err(error) => {
            if source.reason == Some(Reason::NotFound)
                && let Ok((_, tag)) = pinned
            {
                source.notices.push(notice(format!(
                    "HPO release {tag} or a required release file was not found."
                )));
            }
            out.local = status::local(error);
        }
    }
    out.sources.push(source);
    status::search(&mut out);
    command.finish(deadline.remaining());
    out
}

fn apply(
    out: &mut Outcome,
    query: &DiseaseSearch,
    pinned: &DiseaseSearch,
    tag: &str,
    result: release::Matches,
) {
    let returned = result.items.len() as u32;
    let offset = query.offset().saturating_add(returned);
    if returned > 0 && u64::from(offset) < result.total {
        out.next.push(crate::next::Step::new(
            StepKind::Page,
            &[Source::Hpo],
            super::page::run(Request::SearchDisease(pinned.at_offset(offset))),
        ));
    }
    out.page = Some(status::page_info(query.offset(), returned, result.total));
    out.body = HpoDiseasePage {
        release: tag.into(),
        items: result.items,
    }
    .with_query(query.clone());
}

fn notice(message: String) -> SourceNotice {
    SourceNotice::RequestGuidance {
        data_date: None,
        message: message.into(),
        retry_after_seconds: None,
    }
}
