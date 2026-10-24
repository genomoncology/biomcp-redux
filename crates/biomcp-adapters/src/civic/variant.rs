//! Molecular-profile lookups use source allele identities, never name-derived equivalence.
use super::{GENES, variant_map as map, variant_raw as raw};
use crate::Mapped;
use biodata::{
    CivicClaimKind, PageInfo, Reason, Source, Text, VariantCivic, VariantGet, VariantHeader,
    VariantId,
};
use biomcp_fetch::{Leg, RequestError};
use serde::Serialize;

#[derive(Serialize)]
struct Query<'a> {
    query: String,
    variables: Variables<'a>,
}
#[derive(Serialize)]
struct Variables<'a> {
    allele: &'a str,
    after: Option<&'a Text>,
    ids: Option<[u64; 1]>,
    evidence_after: Option<&'a Text>,
    assertion_after: Option<&'a Text>,
}
/// Read one bounded profile page or a selected native evidence/assertion cursor page.
pub fn variant(
    leg: &Leg<'_>,
    query: &VariantGet,
    candidate: &VariantHeader,
) -> Result<Mapped<VariantCivic>, RequestError> {
    let allele = candidate
        .identity
        .ids
        .iter()
        .find(|id| matches!(id, VariantId::Ca(_)))
        .map(ToString::to_string)
        .ok_or_else(|| failure(Reason::NoJoinId))?;
    let mut after = None;
    let mut offset = 0;
    loop {
        let body = request(query, &allele, after.as_ref());
        let reply: raw::Profiles =
            leg.post_json_checked_cache(&GENES, &body, |page: &raw::Profiles| {
                page.matches(&allele, query.civic_profile())
            })?;
        let connection = reply.data.molecular_profiles;
        if query.civic_profile().is_some()
            || offset >= query.section_offset()
            || !connection.page_info.has_next_page
        {
            return map::page(connection, query, &allele);
        }
        if connection.nodes.is_empty() || connection.page_info.end_cursor == after {
            return Err(failure(Reason::BadReply));
        }
        offset += connection.nodes.len() as u32;
        after = connection.page_info.end_cursor;
    }
}
fn request<'a>(query: &VariantGet, allele: &'a str, after: Option<&'a Text>) -> Query<'a> {
    let limit = query.section_limit();
    let (evidence, assertions) = match query.civic_kind() {
        Some(CivicClaimKind::Evidence) => (limit, 0),
        Some(CivicClaimKind::Assertions) => (0, limit),
        None => (limit.min(2), 1),
    };
    Query {
        query: format!(
            "query($allele:String!,$after:String,$ids:[Int!],$evidence_after:String,$assertion_after:String){{molecularProfiles(alleleRegistryId:$allele,ids:$ids,first:1,after:$after){{totalCount pageInfo{{hasNextPage endCursor}} nodes{{id name link isComplex variants{{id name link feature{{name}} ... on GeneVariant{{alleleRegistryId hgvsDescriptions}}}} evidenceItems(first:{evidence},after:$evidence_after,includeRejected:false){{totalCount pageInfo{{hasNextPage endCursor}} nodes{{id status evidenceType evidenceLevel evidenceDirection significance disease{{name doid}} therapies{{name}} source{{citationId sourceType}}}}}} assertions(first:{assertions},after:$assertion_after,includeRejected:false){{totalCount pageInfo{{hasNextPage endCursor}} nodes{{id status assertionType assertionDirection significance ampLevel disease{{name doid}} therapies{{name}} variantOrigin}}}}}}}}}}"
        ),
        variables: Variables {
            allele,
            after,
            ids: query.civic_profile().map(|id| [id]),
            evidence_after: if query.civic_kind() == Some(CivicClaimKind::Evidence) {
                query.civic_cursor()
            } else {
                None
            },
            assertion_after: if query.civic_kind() == Some(CivicClaimKind::Assertions) {
                query.civic_cursor()
            } else {
                None
            },
        },
    }
}
pub(super) fn bounds(offset: u32, returned: usize, total: u64) -> PageInfo {
    PageInfo {
        offset,
        returned: returned as u32,
        total,
        pool: None,
        truncated: None,
    }
}
pub(super) fn failure(reason: Reason) -> RequestError {
    biomcp_fetch::SourceError {
        source: Source::Civic,
        reason,
        status: None,
        retry_after: None,
    }
    .into()
}
