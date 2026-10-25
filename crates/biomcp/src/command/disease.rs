use super::{Outcome, ResponseMeta};
use biomcp_query::Action;
use biomcp_types::{DiseaseSection, DiseaseSource, QueryBody, Request};
use std::io;
pub(super) fn unsupported(action: &Action) -> bool {
    match action {
        Action::Run {
            request: Request::SearchDisease(q),
            ..
        } => q.source() != DiseaseSource::Ols,
        Action::Run {
            request: Request::GetDisease(q),
            ..
        } => match q.source() {
            DiseaseSource::Ols => !matches!(
                q.section(),
                None | Some(DiseaseSection::Synonyms | DiseaseSection::Xrefs)
            ),
            DiseaseSource::MyDisease => {
                let supported = biomcp_adapters::mydisease::supported_id(q.id().as_str());
                !supported
                    || !matches!(
                        q.section(),
                        None | Some(
                            DiseaseSection::Synonyms
                                | DiseaseSection::Xrefs
                                | DiseaseSection::Hierarchy
                        )
                    )
            }
            _ => true,
        },
        _ => false,
    }
}
pub(super) fn encode(meta: ResponseMeta, request: Request, body: QueryBody) -> io::Result<Outcome> {
    match (request, body) {
        (Request::SearchDisease(q), QueryBody::DiseasePage(body)) => {
            let body = (meta.status != biomcp_types::Status::Failed).then_some(body);
            meta.encode(q, body)
        }
        (Request::SearchDisease(q), QueryBody::DiseaseSites(body)) => meta.encode(q, Some(body)),
        (Request::GetDisease(q), QueryBody::DiseaseCard(body)) => meta.encode(q, body),
        (Request::GetDisease(q), QueryBody::DiseaseSynonyms(body)) => meta.encode(q, Some(body)),
        (Request::GetDisease(q), QueryBody::DiseaseXrefs(body)) => meta.encode(q, Some(body)),
        (Request::GetDisease(q), QueryBody::DiseaseHierarchy(body)) => meta.encode(q, Some(body)),
        (Request::GetDisease(q), QueryBody::DiseaseGenes(body)) => meta.encode(q, Some(body)),
        (Request::GetDisease(q), QueryBody::DiseasePhenotypes(body)) => meta.encode(q, Some(body)),
        (Request::GetDisease(q), QueryBody::DiseaseIncidence(body)) => meta.encode(q, Some(body)),
        (Request::GetDisease(q), QueryBody::DiseaseSurvival(body)) => meta.encode(q, Some(body)),
        _ => Err(io::Error::other("disease response mismatch")),
    }
}
