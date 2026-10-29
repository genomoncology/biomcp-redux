//! CPIC retrieval preserves source contexts and performs no clinical inference.
mod annotations;
mod next;
mod snapshot;
use crate::{Ctx, Outcome, status};
pub(crate) use annotations::annotation;
use biomcp_adapters::cpic;
use biomcp_fetch::{Leg, RequestError};
use biomcp_types::{
    PgxCard, PgxDiplotype, PgxGet, PgxGuidelines, PgxHeader, PgxPage, PgxPageInfo,
    PgxRecommendations, PgxSearch, PgxSection, PgxSource, PgxTotalKind, Problem, ProblemCode,
    QueryBody, Reason, Request, Source, SourceOutcome, SourceState, Status,
};
/// Discover one independent native CPIC pair page.
pub fn search(cx: &Ctx<'_>, query: &PgxSearch) -> Outcome {
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let leg = cx.fetch.leg(&cpic::SPEC, deadline).fresh(cx.fresh);
    let reply = cpic::search(&leg, query).map(|page| {
        let info = page_info(query.offset(), page.items.len(), page.total);
        QueryBody::PgxPage(PgxPage {
            header: header(query.gene()),
            items: page.items.into(),
            page: info,
        })
    });
    let empty = QueryBody::PgxPage(PgxPage {
        header: header(query.gene()),
        items: Box::default(),
        page: page_info(query.offset(), 0, 0),
    });
    let mut out = finish(leg, reply, empty, Source::Cpic);
    out.next = next::steps(&Request::SearchPgx(query.clone()), &out);
    command.finish(deadline.remaining());
    out
}
/// Read one explicit card or section with the same source policy on every path.
pub fn get(cx: &Ctx<'_>, query: &PgxGet) -> Outcome {
    if query.source() == PgxSource::Clinpgx {
        return annotations::get(cx, query);
    }
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let leg = cx.fetch.leg(&cpic::SPEC, deadline).fresh(cx.fresh);
    let reply = read(&leg, query);
    let mut out = finish(leg, reply, empty(query), Source::Cpic);
    out.next = next::steps(&Request::GetPgx(query.clone()), &out);
    command.finish(deadline.remaining());
    out
}
fn read(leg: &Leg<'_>, q: &PgxGet) -> Result<QueryBody, RequestError> {
    let header = header(q.gene());
    Ok(match q.section() {
        None => QueryBody::PgxCard(
            cpic::card(leg, q)?
                .items
                .into_iter()
                .next()
                .map(|identity| PgxCard { header, identity }),
        ),
        Some(PgxSection::Recommendations) if q.drug_id().is_some() => {
            let p = cpic::drug_recommendations(leg, q)?;
            QueryBody::PgxRecommendations(PgxRecommendations::Table {
                header,
                page: page_info(q.section_offset(), p.items.len(), p.total),
                items: p.items.into(),
            })
        }
        Some(PgxSection::Recommendations) => {
            let p = cpic::recommendations(leg, q)?;
            QueryBody::PgxRecommendations(PgxRecommendations::View {
                header,
                page: page_info(q.section_offset(), p.items.len(), p.total),
                items: p.items.into(),
            })
        }
        Some(PgxSection::Guidelines) => {
            let p = cpic::guidelines(leg, q)?;
            QueryBody::PgxGuidelines(PgxGuidelines {
                header,
                page: page_info(q.section_offset(), p.items.len(), p.total),
                items: p.items.into(),
            })
        }
        Some(PgxSection::Diplotype) => QueryBody::PgxDiplotype(PgxDiplotype {
            header,
            diplotypes: cpic::diplotype(leg, q)?.items.into(),
        }),
        Some(PgxSection::Annotations) => return Err(biomcp_fetch::InputError::Serialization.into()),
    })
}
fn empty(q: &PgxGet) -> QueryBody {
    let header = header(q.gene());
    let page = page_info(q.section_offset(), 0, 0);
    match q.section() {
        None | Some(PgxSection::Annotations) => QueryBody::PgxCard(None),
        Some(PgxSection::Recommendations) if q.drug_id().is_some() => {
            QueryBody::PgxRecommendations(PgxRecommendations::Table {
                header,
                items: Box::default(),
                page,
            })
        }
        Some(PgxSection::Recommendations) => {
            QueryBody::PgxRecommendations(PgxRecommendations::View {
                header,
                items: Box::default(),
                page,
            })
        }
        Some(PgxSection::Guidelines) => QueryBody::PgxGuidelines(PgxGuidelines {
            header,
            items: Box::default(),
            page,
        }),
        Some(PgxSection::Diplotype) => QueryBody::PgxDiplotype(PgxDiplotype {
            header,
            diplotypes: Box::default(),
        }),
    }
}
fn finish(
    leg: Leg<'_>,
    reply: Result<QueryBody, RequestError>,
    empty: QueryBody,
    source: Source,
) -> Outcome {
    let error = reply.as_ref().err().copied();
    let source = status::source(source, leg.finish(), error);
    let state = if reply.is_err() {
        Status::Failed
    } else if source.state == SourceState::Stale {
        Status::Partial
    } else {
        Status::Complete
    };
    Outcome {
        status: state,
        body: reply.unwrap_or(empty),
        page: None,
        problems: record_problem(&source).into_iter().collect(),
        sources: vec![source],
        next: vec![],
        local: error.and_then(status::local),
    }
}
fn header(gene: &biodata::GeneSymbol) -> PgxHeader {
    PgxHeader {
        gene: gene.clone(),
        source: PgxSource::Cpic,
    }
}
fn page_info(offset: u32, returned: usize, total: u64) -> PgxPageInfo {
    PgxPageInfo {
        offset,
        returned: returned as u32,
        total,
        total_kind: PgxTotalKind::Source,
    }
}

fn record_problem(source: &SourceOutcome) -> Option<Problem> {
    (source.reason == Some(Reason::BadReply)).then(|| Problem {
        source: source.source,
        record: None,
        field: "record".into(),
        code: ProblemCode::ValueUnknown,
    })
}
