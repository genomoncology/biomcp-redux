//! Encode organization observations and typed narrowing refusals.
use super::{Outcome, ResponseMeta};
use biomcp_types::{ErrorEnvelope, OrganizationSearch, QueryBody};
use std::io;
pub(super) fn encode(
    meta: ResponseMeta,
    q: OrganizationSearch,
    body: QueryBody,
) -> io::Result<Outcome> {
    match body {
        QueryBody::OrganizationPage(mut body) => {
            for row in &mut body.items {
                row.next = Some(crate::spell::step(
                    biomcp_types::StepKind::Pivot,
                    &biomcp_query::Action::Run {
                        request: biomcp_types::Request::SearchTrial(
                            biomcp_types::TrialSearch::for_nci_facility(row.name.clone()),
                        ),
                        fresh: false,
                    },
                )?);
            }
            let body = Some(body);
            meta.encode(q, body)
        }
        QueryBody::OrganizationTooBroad => {
            super::usage_envelope(ErrorEnvelope::organization_too_broad())
        }
        QueryBody::PoolError { mismatch } => super::pool_error(mismatch, meta.next),
        _ => Err(io::Error::other("organization response mismatch")),
    }
}
