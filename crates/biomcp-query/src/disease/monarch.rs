use super::page::{empty, page_info, run};
use crate::{Ctx, Outcome, status};
use biomcp_adapters::monarch;
use biomcp_fetch::RequestError;
use biomcp_types::ReplyBody;
use biomcp_types::{
    DiseaseGenes, DiseaseGet, DiseasePhenotypes, DiseaseSection, GeneGet, GeneInput, Reason,
    ReplyData, Request, Source, SourceNotice, StepKind,
};
/// Read one native association page for a MONDO ID, or for the MONDO record that MyDisease finds
/// cross-referencing another ID, without other cross-source joins.
pub(crate) fn get(cx: &Ctx<'_>, query: &DiseaseGet, id: &biodata::Curie) -> Outcome {
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let mut mapped = None;
    let mut out = empty(body(query, Vec::new(), Box::default(), None));
    let id = if id.as_str().starts_with("MONDO:") {
        id.clone()
    } else {
        match super::mydisease::mondo(cx, query, id, deadline, &mut out) {
            Some(mondo) => {
                mapped = Some(out);
                mondo
            }
            None => {
                command.finish(deadline.remaining());
                return out;
            }
        }
    };
    let id = &id;
    let leg = cx.fetch.leg(&monarch::SPEC, deadline).fresh(cx.fresh);
    let reply = monarch::associations(&leg, query, id);
    // An empty page reads the same for an unknown disease; only the entity read tells them apart.
    let held = match &reply {
        Ok(page) if page.total == 0 => Some(monarch::deprecated(&leg, id)),
        _ => None,
    };
    let unknown =
        |e: &RequestError| matches!(e, RequestError::Source(s) if s.reason == Reason::NotFound);
    let error = reply
        .as_ref()
        .err()
        .copied()
        .or(held.and_then(Result::err).filter(|e| !unknown(e)));
    let mut out = empty(body(query, Vec::new(), Box::default(), None));
    // The MyDisease read that named the MONDO record stays first, with its mapping notice.
    if let Some(resolved) = mapped {
        out.sources = resolved.sources;
        out.problems = resolved.problems;
    }
    let mut source = status::source(Source::Monarch, leg.finish(), error);
    let obsolete = matches!(held, Some(Ok(true)));
    if obsolete {
        source.notices.push(SourceNotice::RequestGuidance {
            data_date: None,
            message: format!("Monarch marks {id} obsolete.").into(),
            retry_after_seconds: None,
        });
    }
    out.sources.push(source);
    out.local = error.and_then(status::local);
    if obsolete {
        status::item(&mut out, true, false, true);
        status::missing(
            &mut out,
            biodata::text!("/result/items"),
            Source::Monarch,
            None,
        );
        command.finish(deadline.remaining());
        return out;
    }
    match reply {
        Ok(page) => {
            apply_page(&mut out, query, page);
            match held {
                // Monarch holds the disease: no associations is a complete answer.
                None | Some(Ok(_)) => status::item(&mut out, true, false, false),
                Some(Err(e)) => status::item(&mut out, false, !unknown(&e), false),
            }
        }
        Err(_) => status::item(&mut out, false, true, false),
    }
    super::page::record_problem(&mut out, query, Source::Monarch);
    command.finish(deadline.remaining());
    out
}

fn apply_page(out: &mut Outcome, query: &DiseaseGet, page: monarch::Page) {
    out.include_refusals(page.refusals);
    out.problems.extend(page.problems);
    let returned = page.items.len() as u32;
    out.page = Some(page_info(query.section_offset(), returned, page.total));
    if query.section() == Some(DiseaseSection::Genes) {
        gene_steps(out, &page.items);
    }
    if page.received > 0
        && u64::from(query.section_offset()) + u64::from(page.received) < page.total
    {
        out.next.push(crate::next::Step::new(
            StepKind::Page,
            &[Source::Monarch],
            run(Request::GetDisease(query.at_section_offset(
                query.section_offset().saturating_add(page.received),
            ))),
        ));
    }
    out.body = body(
        query,
        page.items,
        page.components,
        page.metadata
            .fetched
            .and_then(|value| (value >= 0).then_some(value.cast_unsigned())),
    );
}

fn body(
    query: &DiseaseGet,
    items: Vec<biomcp_types::DiseaseAssociationItem>,
    components: Box<[biomcp_types::ComponentLabel]>,
    fetched: Option<u64>,
) -> ReplyData {
    if query.section() == Some(DiseaseSection::Genes) {
        (DiseaseGenes {
            items: items.into(),
            components,
            fetched,
        })
        .with_query(query.clone())
    } else {
        (DiseasePhenotypes {
            items: items.into(),
            components,
            fetched,
        })
        .with_query(query.clone())
    }
}
fn gene_steps(out: &mut Outcome, items: &[biomcp_types::DiseaseAssociationItem]) {
    for row in items {
        let id = &row.association.subject.id;
        let id = id.to_string();
        if let Some(query) = gene_query(&id, &mut out.problems) {
            let step = crate::next::Step::new(
                StepKind::Disambiguate,
                &[biomcp_types::Source::Hgnc],
                run(Request::GetGene(query)),
            );
            if !out.next.contains(&step) {
                out.next.push(step);
            }
        }
    }
}

/// Accept only an identifier the existing gene command understands, without symbol repair.
pub(crate) fn gene_query(id: &str, problems: &mut Vec<biomcp_types::Problem>) -> Option<GeneGet> {
    let input = if let Some(value) = id.strip_prefix("NCBIGene:") {
        crate::next::checked(
            value.parse::<biodata::NcbiGeneId>(),
            Source::Monarch,
            problems,
        )
        .map(GeneInput::EntrezId)?
    } else if id.starts_with("HGNC:")
        || id.starts_with("ENSG")
        || id.bytes().all(|byte| byte.is_ascii_digit())
    {
        crate::next::checked(GeneInput::new(id), Source::Monarch, problems)?
    } else {
        return None;
    };
    crate::next::checked(GeneGet::new(vec![input].into()), Source::Monarch, problems)
}
