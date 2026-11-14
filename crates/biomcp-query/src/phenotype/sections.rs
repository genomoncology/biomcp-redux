use super::{record::Rows, run};
use crate::Outcome;
use biomcp_types::ReplyBody;
use biomcp_types::{
    PageInfo, PhenotypeGet, PhenotypeHierarchy, PhenotypeSection, PhenotypeSource,
    PhenotypeSynonyms, PhenotypeXrefs, ReplyData, Request, StepKind,
};
pub(super) fn empty(q: &PhenotypeGet) -> ReplyData {
    match q.section() {
        Some(PhenotypeSection::Synonyms) => {
            biomcp_types::PhenotypeSynonyms::into_reply(q.clone(), None)
        }
        Some(PhenotypeSection::Xrefs) => biomcp_types::PhenotypeXrefs::into_reply(q.clone(), None),
        Some(PhenotypeSection::Hierarchy) => {
            biomcp_types::PhenotypeHierarchy::into_reply(q.clone(), None)
        }
        _ => biomcp_types::PhenotypeTermCard::into_reply(q.clone(), None),
    }
}
/// Called only once the source returned the term; an unknown term never reaches here.
pub(super) fn apply(out: &mut Outcome, echo: &PhenotypeGet, q: &PhenotypeGet, rows: Rows) {
    let components = rows.0.components.clone();
    let fetched = rows.4;
    match q.section() {
        None => {
            for section in &rows.0.sections {
                if let Some(request) = crate::next::checked(
                    q.with_section(*section),
                    super::record::source(q),
                    &mut out.problems,
                ) {
                    out.next.push(crate::next::Step::new(
                        StepKind::Section,
                        &[super::record::source(q)],
                        run(Request::GetPhenotype(request)),
                    ));
                }
            }
            out.body = (rows.0).with_query(echo.clone());
        }
        Some(PhenotypeSection::Synonyms) => {
            let missing = rows.1.is_none();
            let items = page(out, q, &rows.1.unwrap_or_default());
            unsent(out, q, missing);
            out.body = (PhenotypeSynonyms {
                items,
                components,
                fetched,
            })
            .with_query(echo.clone());
        }
        Some(PhenotypeSection::Xrefs) => {
            let missing = rows.2.is_none();
            let items = page(out, q, &rows.2.unwrap_or_default());
            unsent(out, q, missing);
            out.body = (PhenotypeXrefs {
                items,
                components,
                fetched,
            })
            .with_query(echo.clone());
        }
        Some(PhenotypeSection::Hierarchy) => {
            let items = page(out, q, &rows.3.unwrap_or_default());
            out.body = (PhenotypeHierarchy {
                items,
                components,
                fetched,
            })
            .with_query(echo.clone());
        }
        _ => unavailable(out),
    }
}
/// One hop a call cannot answer "is this term under that one"; OLS lists every ancestor at once.
pub(super) fn ancestors(out: &mut Outcome, q: &PhenotypeGet) {
    if let Some(request) = crate::next::checked(
        q.with_section(PhenotypeSection::Ancestors),
        super::record::source(q),
        &mut out.problems,
    ) {
        out.next.push(crate::next::Step::new(
            StepKind::Section,
            &[biomcp_types::Source::Ols],
            run(Request::GetPhenotype(request)),
        ));
    }
}
/// Monarch sends null for an empty list, but HPO sends `[]`, so a missing HPO list is unknown.
fn unsent(out: &mut Outcome, q: &PhenotypeGet, missing: bool) {
    if missing && q.source() == PhenotypeSource::Hpo {
        crate::status::item(out, true, false, true);
    }
}
fn unavailable(out: &mut Outcome) {
    crate::status::item(out, false, false, false);
}
pub(super) fn page<T: Clone>(out: &mut Outcome, q: &PhenotypeGet, rows: &[T]) -> Box<[T]> {
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
            &[super::record::source(q)],
            run(Request::GetPhenotype(q.at_section_offset(next))),
        ));
    }
    // The source holds the term, so an empty section is a complete answer, not an absent term.
    crate::status::item(out, true, false, false);
    items
}
