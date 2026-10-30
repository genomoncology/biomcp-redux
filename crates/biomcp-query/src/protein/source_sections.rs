//! Independent source sections reuse their public bounded page handlers.
use super::{
    sections::wants,
    source_relations::{self, Selection},
};
use crate::{Ctx, Outcome, status};
use biomcp_fetch::Deadline;
use biomcp_types::{
    ProteinCardItem, ProteinDomainsQuery, ProteinGet, ProteinRelationQuery, ProteinSection,
    QueryBody, Section, Source, Status,
};
pub(super) fn supplement(
    cx: &Ctx<'_>,
    q: &ProteinGet,
    items: &mut [ProteinCardItem],
    out: &mut Outcome,
    deadline: Deadline<'_>,
) {
    for item in items {
        item.more = item
            .more
            .iter()
            .copied()
            .filter(|s| match s {
                ProteinSection::Domains => !cx.fetch.disabled(Source::InterPro),
                ProteinSection::Complexes => !cx.fetch.disabled(Source::ComplexPortal),
                _ => true,
            })
            .collect();
        let Some(header) = &item.header else {
            continue;
        };
        let id = header.protein.accession.clone();
        let mut selections = Vec::new();
        if wants(q, ProteinSection::Domains)
            && let Ok(query) = ProteinDomainsQuery::new(id.clone(), q.section_limit())
        {
            selections.push(Selection::Domains(query));
        }
        if wants(q, ProteinSection::Complexes)
            && let Ok(query) = ProteinRelationQuery::new(id, q.section_limit())
        {
            selections.push(Selection::Complexes(query));
        }
        for selection in selections {
            apply(item, out, page(cx, &selection, deadline));
        }
    }
}
fn apply(item: &mut ProteinCardItem, out: &mut Outcome, result: Outcome) {
    let reason = reason(&result);
    if let Some(sections) = &mut item.sections {
        match result.body {
            QueryBody::ProteinDomains(data) => {
                sections.domains = Some(match data {
                    Some(data) if data.items.is_empty() => Section::Empty {
                        source: Source::InterPro,
                        record: None,
                    },
                    Some(data) => Section::Data {
                        source: Source::InterPro,
                        data,
                    },
                    None => Section::unavailable(Source::InterPro, reason),
                })
            }
            QueryBody::ProteinComplexes(data) => {
                sections.complexes = Some(match data {
                    Some(data) if data.items.is_empty() => Section::Empty {
                        source: Source::ComplexPortal,
                        record: None,
                    },
                    Some(data) => Section::Data {
                        source: Source::ComplexPortal,
                        data,
                    },
                    None => Section::unavailable(Source::ComplexPortal, reason),
                })
            }
            _ => {}
        }
    }
    merge(
        item,
        out,
        result.sources,
        result.next,
        result.problems,
        result.status,
    );
}

fn page(cx: &Ctx<'_>, selection: &Selection, deadline: Deadline<'_>) -> Outcome {
    let mut out = super::empty(selection.empty());
    if let Err(e) = source_relations::read(cx, selection, deadline, &mut out) {
        out.local = status::local(e);
    }
    out
}
fn reason(out: &Outcome) -> biomcp_types::Reason {
    out.sources
        .iter()
        .find_map(|s| s.reason)
        .unwrap_or(biomcp_types::Reason::Unavailable)
}
fn merge(
    item: &mut ProteinCardItem,
    out: &mut Outcome,
    sources: Vec<biomcp_types::SourceOutcome>,
    steps: Vec<(biomcp_types::StepKind, crate::Action)>,
    problems: Vec<biomcp_types::Problem>,
    state: Status,
) {
    out.sources.extend(sources);
    out.next.extend(steps);
    out.problems.extend(problems);
    if state != Status::Complete {
        item.status = crate::status::item_status(true, false, true);
    }
}
