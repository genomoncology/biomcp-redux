use super::{Outcome, ResponseMeta};
use biomcp_types::{QueryBody, Request};
use std::io;
pub(super) fn encode(meta: ResponseMeta, request: Request, body: QueryBody) -> io::Result<Outcome> {
    match (request, body) {
        (Request::SearchPhenotype(q), QueryBody::PhenotypePage(body)) => meta.encode(q, Some(body)),
        (Request::GetPhenotype(q), QueryBody::PhenotypeCard(body)) => meta.encode(q, body),
        (Request::GetPhenotype(q), QueryBody::PhenotypeSynonyms(body)) => meta.encode(q, body),
        (Request::GetPhenotype(q), QueryBody::PhenotypeXrefs(body)) => meta.encode(q, body),
        (Request::GetPhenotype(q), QueryBody::PhenotypeHierarchy(body)) => meta.encode(q, body),
        (Request::GetPhenotype(q), QueryBody::PhenotypeDiseases(body)) => {
            meta.encode(q, Some(body))
        }
        (Request::GetPhenotype(q), QueryBody::PhenotypeGenes(body)) => meta.encode(q, Some(body)),
        (_, QueryBody::Usage(biomcp_types::UsageError::PoolError { mismatch })) => super::pool_error(mismatch, meta.next),
        _ => Err(io::Error::other("phenotype response mismatch")),
    }
}
