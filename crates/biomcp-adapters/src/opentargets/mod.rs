//! Canonical OPENTARGETS source declaration.
use biomcp_fetch::{KeyUse, SourceSpec, Terms};
use biomcp_types::Source;
use std::time::Duration;
/// Publisher policy shared by health, request admission and response outcomes.
pub static SPEC: SourceSpec = SourceSpec {
    source: Source::Opentargets,
    base: "https://api.platform.opentargets.org/api/v4",
    terms: Terms {
        name: crate::drug_sources::OPENTARGETS_TERMS.name,
        url: crate::drug_sources::OPENTARGETS_TERMS.url,
        usage: crate::drug_sources::OPENTARGETS_TERMS.usage,
    },
    quota: &crate::quotas::OPENTARGETS,
    key: KeyUse::None,
    deadline: Duration::from_secs(15),
};
mod raw;
pub use raw::{Disease, Drug, Hit, Indication, Mechanism, Rows, Search, Target};
use biomcp_fetch::{BadReply, Freshness, Leg, RequestError};
use serde::Serialize;
const GRAPHQL: biomcp_fetch::Endpoint = SPEC.post_json("/graphql").fresh(Freshness::Days(1));
const SEARCH: &str = "query DrugSearch($q:String!){search(queryString:$q,entityNames:[\"drug\"],page:{index:0,size:5}){total hits{id name entity}}}";
const DETAIL: &str = "query Drug($id:String!){drug(chemblId:$id){id name indications{rows{maxClinicalStage disease{id name}}} mechanismsOfAction{rows{mechanismOfAction actionType targets{id approvedSymbol approvedName}}}}}";
#[derive(Serialize)]
struct Query<'a, T> { query: &'a str, variables: T }
#[derive(Serialize)]
struct SearchVariables<'a> { q: &'a str }
#[derive(Serialize)]
struct DrugVariables<'a> { id: &'a str }
/// Fetch the proved native five-candidate collection; page locally without claiming unproved GraphQL arguments.
pub fn search(leg: &Leg<'_>, text: &str) -> Result<(Search, biomcp_fetch::ReplyMetadata), RequestError> {
    let body = Query { query: SEARCH, variables: SearchVariables { q: text } };
    leg.post_json_received_checked(&GRAPHQL, &[], &body, |r: &raw::Response<raw::SearchData>| {
        if r.data.search.hits.len() <= 5 && r.data.search.hits.iter().all(|h| h.entity == "drug" && h.id.starts_with("CHEMBL") && !h.name.is_empty()) { Ok(()) } else { Err(BadReply) }
    }).map(|(r, meta)| (r.data.search, meta))
}
/// Read one checked source ChEMBL identity; all section paging occurs over this returned collection.
pub fn drug(leg: &Leg<'_>, id: &biodata::DrugId) -> Result<(Option<Drug>, biomcp_fetch::ReplyMetadata), RequestError> {
    let id = id.to_string();
    let body = Query { query: DETAIL, variables: DrugVariables { id: &id } };
    leg.post_json_received_checked(&GRAPHQL, &[], &body, |r: &raw::Response<raw::DrugData>| {
        if r.data.drug.as_ref().is_none_or(|d| d.id == id && !d.name.is_empty()) { Ok(()) } else { Err(BadReply) }
    }).map(|(r, meta)| (r.data.drug, meta))
}
