use super::{overall, run, snapshot::Rows};
use crate::Outcome;
use biomcp_types::{
    PageInfo, PhenotypeGet, PhenotypeHierarchy, PhenotypeSection, PhenotypeSynonyms,
    PhenotypeXrefs, QueryBody, Request, Status, StepKind,
};
pub(super) fn empty(q: &PhenotypeGet) -> QueryBody {
    match q.section() {
        Some(PhenotypeSection::Synonyms) => QueryBody::PhenotypeSynonyms(None),
        Some(PhenotypeSection::Xrefs) => QueryBody::PhenotypeXrefs(None),
        Some(PhenotypeSection::Hierarchy) => QueryBody::PhenotypeHierarchy(None),
        _ => QueryBody::PhenotypeCard(None),
    }
}
pub(super) fn apply(out: &mut Outcome, q: &PhenotypeGet, rows: Rows) {
    let components = rows.0.components.clone();
    let fetched = rows.4;
    match q.section() {
        None => {
            for section in &rows.0.sections {
                if let Ok(request) = q.with_section(*section) {
                    out.next
                        .push((StepKind::Section, run(Request::GetPhenotype(request))));
                }
            }
            out.body = QueryBody::PhenotypeCard(Some(rows.0));
        }
        Some(PhenotypeSection::Synonyms) => {
            if let Some(rows) = rows.1 {
                let items = page(out, q, &rows);
                out.body = QueryBody::PhenotypeSynonyms(Some(PhenotypeSynonyms {
                    items,
                    components,
                    fetched,
                }));
            } else {
                unavailable(out);
            }
        }
        Some(PhenotypeSection::Xrefs) => {
            if let Some(rows) = rows.2 {
                let items = page(out, q, &rows);
                out.body = QueryBody::PhenotypeXrefs(Some(PhenotypeXrefs {
                    items,
                    components,
                    fetched,
                }));
            } else {
                unavailable(out);
            }
        }
        Some(PhenotypeSection::Hierarchy) => {
            if let Some(rows) = rows.3 {
                let items = page(out, q, &rows);
                out.body = QueryBody::PhenotypeHierarchy(Some(PhenotypeHierarchy {
                    items,
                    components,
                    fetched,
                }));
            } else {
                unavailable(out);
            }
        }
        _ => unavailable(out),
    }
}
fn unavailable(out: &mut Outcome) {
    out.status = Status::Absent;
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
        out.next.push((
            StepKind::Page,
            run(Request::GetPhenotype(q.at_section_offset(next))),
        ));
    }
    out.status = overall(out, rows.is_empty());
    items
}
