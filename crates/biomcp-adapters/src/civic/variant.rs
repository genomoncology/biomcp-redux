//! Molecular-profile lookups use source allele identities, never name-derived equivalence.
use super::{GENES, variant_map as map, variant_raw as raw};
use crate::Mapped;
use biodata::{Text, VariantId};
use biomcp_fetch::Input;

use biomcp_fetch::{Leg, RequestError};
use biomcp_types::{
    CivicClaimKind, CivicMembership, PageInfo, Reason, Source, VariantCivic, VariantGet,
    VariantHeader,
};
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
    #[serde(skip_serializing_if = "Option::is_none")]
    ids: Option<[u64; 1]>,
    evidence_after: Option<&'a Text>,
    assertion_after: Option<&'a Text>,
}
/// Read one bounded profile page or a selected native evidence/assertion cursor page.
///
/// When no CIViC profile carries the requested allele, CIViC variants that state the same
/// protein change on the same transcript are joined instead, marked by protein membership;
/// their continuations name CIViC's allele. An empty join carries a problem: field `hgvs_p`
/// when no protein change could be searched, `hgvs_p.civic_protein_search` when the search
/// found no CIViC variant.
pub fn variant(
    leg: &Leg<'_>,
    query: &VariantGet,
    candidate: &VariantHeader,
) -> Result<Mapped<VariantCivic>, RequestError> {
    let allele = candidate
        .identity
        .ids
        .iter()
        .chain(candidate.requested.iter())
        .find(|id| matches!(id, VariantId::Ca(_)))
        .map(ToString::to_string)
        .ok_or_else(|| failure(Reason::NoJoinId))?;
    let exact_query = query
        .for_candidate(allele.parse().map_err(|_| failure(Reason::BadReply))?)
        .map_err(|_| failure(Reason::BadReply))?;
    let exact = profiles(leg, &exact_query, &allele)?;
    if exact.value.page.total > 0 {
        return Ok(exact);
    }
    let (searched, others) = super::variant_protein::alleles(leg, candidate, &allele)?;
    for other in others {
        // Continuations name CIViC's own allele, whose exact join reaches the same profile.
        let other_query = query
            .for_candidate(other.parse().map_err(|_| failure(Reason::BadReply))?)
            .map_err(|_| failure(Reason::BadReply))?;
        let mut found = profiles(leg, &other_query, &other)?;
        if found.value.page.total > 0 {
            for profile in found.value.profiles.iter_mut() {
                if matches!(profile.membership, CivicMembership::Exact) {
                    profile.membership = CivicMembership::Protein;
                }
            }
            return Ok(found);
        }
    }
    // An empty page alone would read as no CIViC evidence; say how far the join could look.
    let mut exact = exact;
    exact.problems.push(biomcp_types::Problem {
        source: Source::Civic,
        record: None,
        field: if searched {
            "hgvs_p.civic_protein_search"
        } else {
            "hgvs_p"
        }
        .into(),
        code: biomcp_types::ProblemCode::ValueUnknown,
    });
    Ok(exact)
}
fn profiles(
    leg: &Leg<'_>,
    query: &VariantGet,
    allele: &str,
) -> Result<Mapped<VariantCivic>, RequestError> {
    let mut after = None;
    let mut offset = 0;
    let mut partial = false;
    loop {
        let body = request(query, allele, after.as_ref());
        let reply: raw::Profiles = leg
            .call(
                &GENES,
                Input::json(&[], &body),
                biomcp_fetch::Read::json().cache_if(&|page: &raw::Profiles| {
                    !page.has_errors()
                        && page.matches(allele, query.civic_profile())
                        && page.progresses(query, after.as_ref())
                }),
            )?
            .value;
        if !reply.progresses(query, after.as_ref()) {
            return Err(failure(Reason::BadReply));
        }
        partial |= reply.has_errors();
        let connection = reply.data.molecular_profiles;
        if query.civic_profile().is_some()
            || offset >= query.section_offset()
            || !connection.page_info.has_next_page
        {
            let mut connection = connection;
            if evidence_filtered(query) {
                super::variant_filter::apply(leg, query, &mut connection)?;
            }
            let mut mapped = map::page(connection, query, allele)?;
            if partial {
                mapped.problems.push(biomcp_types::Problem {
                    source: Source::Civic,
                    record: None,
                    field: "graphql.errors".into(),
                    code: biomcp_types::ProblemCode::ValueUnknown,
                });
            }
            return Ok(mapped);
        }
        if connection.nodes.is_empty() || connection.page_info.end_cursor == after {
            return Err(failure(Reason::BadReply));
        }
        offset += connection.nodes.len() as u32;
        after = connection.page_info.end_cursor;
    }
}
fn request<'a>(query: &'a VariantGet, allele: &'a str, after: Option<&'a Text>) -> Query<'a> {
    let (evidence, assertions) = claim_counts(query);
    let evidence = if evidence_filtered(query) {
        0
    } else {
        evidence
    };
    let assertions = if names_filtered(query) { 0 } else { assertions };
    let (ids_variable, ids_argument) = if query.civic_profile().is_some() {
        (",$ids:[Int!]", ",ids:$ids")
    } else {
        ("", "")
    };
    Query {
        query: format!(
            "query($allele:String!,$after:String{ids_variable},$evidence_after:String,\
                    $assertion_after:String){{molecularProfiles(alleleRegistryId:$allele{ids_argument},first:1,after:$after){{totalCount \
                pageInfo{{hasNextPage endCursor}} nodes{{id name rawName link isComplex variants{{id name link \
                feature{{id name}} ... on GeneVariant{{alleleRegistryId hgvsDescriptions}}}} \
                evidenceItems(first:{evidence},after:$evidence_after,includeRejected:false){{totalCount \
                pageInfo{{hasNextPage endCursor}} nodes{{id status evidenceType evidenceLevel evidenceDirection \
                significance disease{{name doid}} therapies{{name}} therapyInteractionType source{{citationId sourceType}}}}}} \
                assertions(first:{assertions},after:$assertion_after,includeRejected:false){{totalCount \
                pageInfo{{hasNextPage endCursor}} nodes{{id status assertionType assertionDirection significance \
                ampLevel disease{{name doid}} therapies{{name}} therapyInteractionType variantOrigin}}}}}}}}}}"
        ),
        variables: Variables {
            allele,
            after,
            ids: query.civic_profile().map(|id| [id]),
            evidence_after: if !evidence_filtered(query)
                && query.civic_kind() == Some(CivicClaimKind::Evidence)
            {
                query.civic_cursor()
            } else {
                None
            },
            assertion_after: if !names_filtered(query)
                && query.civic_kind() == Some(CivicClaimKind::Assertions)
            {
                query.civic_cursor()
            } else {
                None
            },
        },
    }
}
/// Evidence and assertion rows per page; an omitted section limit keeps the compact overview.
pub(super) fn claim_counts(query: &VariantGet) -> (u32, u32) {
    let limit = query.section_limit();
    match query.civic_kind() {
        Some(CivicClaimKind::Evidence) => (limit, 0),
        Some(CivicClaimKind::Assertions) => (0, limit),
        None => query
            .requested_section_limit()
            .map_or((2, 1), |limit| (limit, limit)),
    }
}
/// Disease or drug text restricts both claim collections through CIViC's root queries.
pub(super) fn names_filtered(query: &VariantGet) -> bool {
    query.disease().is_some() || query.drug().is_some()
}
/// Evidence selectors that CIViC applies only on its root evidence query.
pub(super) fn evidence_filtered(query: &VariantGet) -> bool {
    !query.evidence_filter().is_empty() || names_filtered(query)
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
        tls_cause: None,
        retry_after: None,
    }
    .into()
}
