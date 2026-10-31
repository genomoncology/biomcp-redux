use super::{run, snapshot::Rows};
use crate::Outcome;
use biomcp_types::{
    PageInfo, PhenotypeGet, PhenotypeHierarchy, PhenotypeSection, PhenotypeSynonyms,
    PhenotypeXrefs, ReplyData, Request, StepKind,
};
pub(super) fn empty(q: &PhenotypeGet) -> ReplyData {
    match q.section() {
        Some(PhenotypeSection::Synonyms) => ReplyData::PhenotypeSynonyms {
            query: q.clone(),
            body: None,
        },
        Some(PhenotypeSection::Xrefs) => ReplyData::PhenotypeXrefs {
            query: q.clone(),
            body: None,
        },
        Some(PhenotypeSection::Hierarchy) => ReplyData::PhenotypeHierarchy {
            query: q.clone(),
            body: None,
        },
        _ => ReplyData::PhenotypeTermCard {
            query: q.clone(),
            body: None,
        },
    }
}
pub(super) fn apply(out: &mut Outcome, q: &PhenotypeGet, rows: Rows) {
    let components = rows.0.components.clone();
    let fetched = rows.4;
    match q.section() {
        None => {
            for section in &rows.0.sections {
                if let Some(request) = crate::next::checked(
                    q.with_section(*section),
                    super::snapshot::source(q),
                    &mut out.problems,
                ) {
                    out.next.push(crate::next::Step::new(
                        StepKind::Section,
                        &[super::snapshot::source(q)],
                        run(Request::GetPhenotype(request)),
                    ));
                }
            }
            out.body = ReplyData::PhenotypeTermCard {
                query: q.clone(),
                body: Some(rows.0),
            };
        }
        Some(PhenotypeSection::Synonyms) => {
            if let Some(rows) = rows.1 {
                let items = page(out, q, &rows);
                out.body = ReplyData::PhenotypeSynonyms {
                    query: q.clone(),
                    body: Some(PhenotypeSynonyms {
                        items,
                        components,
                        fetched,
                    }),
                };
            } else {
                unavailable(out);
            }
        }
        Some(PhenotypeSection::Xrefs) => {
            if let Some(rows) = rows.2 {
                let items = page(out, q, &rows);
                out.body = ReplyData::PhenotypeXrefs {
                    query: q.clone(),
                    body: Some(PhenotypeXrefs {
                        items,
                        components,
                        fetched,
                    }),
                };
            } else {
                unavailable(out);
            }
        }
        Some(PhenotypeSection::Hierarchy) => {
            if let Some(rows) = rows.3 {
                let items = page(out, q, &rows);
                out.body = ReplyData::PhenotypeHierarchy {
                    query: q.clone(),
                    body: Some(PhenotypeHierarchy {
                        items,
                        components,
                        fetched,
                    }),
                };
            } else {
                unavailable(out);
            }
        }
        _ => unavailable(out),
    }
}
fn unavailable(out: &mut Outcome) {
    crate::status::item(out, false, false, false);
}
fn page<T: Clone>(out: &mut Outcome, q: &PhenotypeGet, rows: &[T]) -> Box<[T]> {
    let offset = q.section_offset() as usize;
    let items: Box<[T]> = rows
        .iter()
        .skip(offset)
        .take(q.section_limit() as usize)
        .cloned()
        .collect();
    let returned = items.len() as u32;
    let next = q.section_offset().saturating_add(returned);
    out.page = Some(PageInfo {
        offset: q.section_offset(),
        returned,
        total: rows.len() as u64,
        pool: None,
        truncated: None,
    });
    if returned > 0 && (next as usize) < rows.len() {
        out.next.push(crate::next::Step::new(
            StepKind::Page,
            &[super::snapshot::source(q)],
            run(Request::GetPhenotype(q.at_section_offset(next))),
        ));
    }
    {
        let absent = rows.is_empty();
        crate::status::item(out, !absent, false, false);
    }
    items
}
