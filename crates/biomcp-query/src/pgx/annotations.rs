//! Independent ClinPGx collections share transport admission and local paging.
use super::{finish, next};
use crate::{Ctx, Outcome};
use biomcp_adapters::clinpgx;
use biomcp_types::*;
fn page<T>(q: &PgxGet, rows: Vec<T>) -> (Box<[T]>, PgxPageInfo) {
    let total = rows.len() as u64;
    let items: Box<[T]> = rows
        .into_iter()
        .skip(q.section_offset() as usize)
        .take(q.section_limit() as usize)
        .collect();
    let page = PgxPageInfo {
        offset: q.section_offset(),
        returned: items.len() as u32,
        total,
        total_kind: PgxTotalKind::Observed,
    };
    (items, page)
}
fn body(
    q: &PgxGet,
    clinical: Vec<biodata::PgxAnnotationHeader>,
    references: Vec<biodata::PgxAnnotationReference>,
) -> PgxAnnotations {
    let header = PgxHeader {
        source: PgxSource::Clinpgx,
        gene: q.gene().clone(),
    };
    match q.kind() {
        Some(PgxAnnotationKind::Guideline) => {
            let (items, page) = page(q, references);
            PgxAnnotations::Guideline {
                header,
                items,
                page,
            }
        }
        Some(PgxAnnotationKind::Label) => {
            let (items, page) = page(q, references);
            PgxAnnotations::Label {
                header,
                items,
                page,
            }
        }
        _ => {
            let (items, page) = page(q, clinical);
            PgxAnnotations::Clinical {
                header,
                items,
                page,
            }
        }
    }
}
pub(super) fn get(cx: &Ctx<'_>, q: &PgxGet) -> Outcome {
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let leg = cx.fetch.leg(&clinpgx::SPEC, deadline).fresh(cx.fresh);
    let reply = match q.kind() {
        Some(PgxAnnotationKind::Clinical) => {
            clinpgx::clinical(&leg, q).map(|rows| body(q, rows, vec![]))
        }
        _ => clinpgx::references(&leg, q).map(|rows| body(q, vec![], rows)),
    }
    .map(QueryBody::PgxAnnotations);
    let empty = QueryBody::PgxAnnotations(body(q, vec![], vec![]));
    let mut out = finish(leg, reply, empty, Source::ClinPgx);
    out.next = next::steps(&Request::GetPgx(q.clone()), &out);
    command.finish(deadline.remaining());
    out
}
pub(crate) fn annotation(cx: &Ctx<'_>, q: &PgxAnnotationGet) -> Outcome {
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let leg = cx.fetch.leg(&clinpgx::SPEC, deadline).fresh(cx.fresh);
    let reply = clinpgx::annotation(&leg, q).map(|annotation| {
        QueryBody::PgxAnnotation(Some(PgxAnnotation {
            header: PgxAnnotationIdentity {
                source: PgxSource::Clinpgx,
                id: q.id().clone(),
            },
            annotation,
        }))
    });
    let mut out = finish(leg, reply, QueryBody::PgxAnnotation(None), Source::ClinPgx);
    out.next = next::steps(&Request::GetPgxAnnotation(q.clone()), &out);
    command.finish(deadline.remaining());
    out
}
