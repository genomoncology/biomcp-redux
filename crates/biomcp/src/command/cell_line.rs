use super::{Outcome, ResponseMeta};
use biomcp_types::{CellLineSection, QueryBody, Request, Section};
use std::io;
pub(super) fn encode(meta: ResponseMeta, request: Request, body: QueryBody) -> io::Result<Outcome> {
    match (request, body) {
        (Request::SearchCellLine(q), QueryBody::CellLinePage(body)) => {
            meta.encode(q, Some(body))
        }
        (Request::CellLineAssayCount(q), QueryBody::CellLineAssayCount(body)) => {
            meta.encode(q, Some(body))
        }
        (Request::GetCellLine(q), QueryBody::CellLineChembl(body)) => {
            meta.encode(q, Some(body))
        }
        (Request::GetCellLine(q), QueryBody::CellLineCard(body)) if !body.candidates.is_empty() => {
            meta.encode(q, Some(body))
        }
        (Request::GetCellLine(q), QueryBody::CellLineCard(mut body)) => match q.section() {
            Some(CellLineSection::Names) => meta.encode(q, data(body.sections.names.take())),
            Some(CellLineSection::Identity) => meta.encode(q, data(body.sections.identity.take())),
            Some(CellLineSection::Xrefs) => meta.encode(q, data(body.sections.xrefs.take())),
            Some(CellLineSection::Variants) => meta.encode(q, data(body.sections.variants.take())),
            _ => meta.encode(q, Some(body)),
        },
        _ => Err(io::Error::other("cell line response mismatch")),
    }
}

fn data<T>(section: Option<Section<T>>) -> Option<T> {
    match section {
        Some(Section::Data { data, .. }) => Some(data),
        _ => None,
    }
}
