//! Filtered profile claims use CIViC's root collections, whose arguments support filters.
use super::{
    GENES, evidence_filter,
    variant::{claim_counts, failure, names_filtered},
    variant_raw as raw,
};
use crate::raw::Field;
use biodata::Text;
use biomcp_fetch::{BadReply, Input, Leg, Read, Reply, RequestError};
use biomcp_types::{CivicClaimKind, Reason, VariantGet};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct Request<'a> {
    query: String,
    variables: Variables<'a>,
}
#[derive(Serialize)]
struct Variables<'a> {
    id: u64,
    after: Option<&'a Text>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disease: Option<&'a Text>,
    #[serde(skip_serializing_if = "Option::is_none")]
    therapy: Option<&'a Text>,
}
#[derive(Deserialize)]
struct Response<T> {
    data: Data<T>,
    errors: Option<Vec<serde::de::IgnoredAny>>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Data<T> {
    #[serde(alias = "assertions")]
    evidence_items: raw::Connection<T>,
}
impl<T: serde::de::DeserializeOwned> Reply for Response<T> {
    fn check(&self) -> Result<(), BadReply> {
        if self
            .errors
            .as_ref()
            .is_some_and(|errors| !errors.is_empty())
        {
            return Err(BadReply);
        }
        self.data.evidence_items.check()
    }
}
/// Disease and drug text travel as variables, never inside the query text.
fn names(query: &VariantGet) -> (String, String) {
    let mut variables = String::new();
    let mut arguments = String::new();
    if query.disease().is_some() {
        variables.push_str(",$disease:String");
        arguments.push_str(",diseaseName:$disease");
    }
    if query.drug().is_some() {
        variables.push_str(",$therapy:String");
        arguments.push_str(",therapyName:$therapy");
    }
    (variables, arguments)
}
fn rows<T: serde::de::DeserializeOwned>(
    leg: &Leg<'_>,
    query: &VariantGet,
    id: u64,
    (collection, fields): (&str, &str),
    filters: &str,
    after: Option<&Text>,
    count: u32,
) -> Result<raw::Connection<T>, RequestError> {
    let (variables, arguments) = names(query);
    let body = Request {
        query: format!(
            "query($id:Int!,$after:String{variables}){{{collection}(molecularProfileId:$id,{filters}{arguments},first:{count},after:$after){{totalCount \
            pageInfo{{hasNextPage endCursor}} nodes{{{fields}}}}}}}"
        ),
        variables: Variables {
            id,
            after,
            disease: query.disease(),
            therapy: query.drug(),
        },
    };
    let reply: Response<T> = leg
        .call(
            &GENES,
            Input::json(&[], &body),
            Read::json().cache_if(&|reply: &Response<T>| {
                count == 0 || reply.data.evidence_items.progresses(after)
            }),
        )?
        .value;
    let rows = reply.data.evidence_items;
    if !rows.progresses(after) && count > 0 {
        return Err(failure(Reason::BadReply));
    }
    Ok(rows)
}
const EVIDENCE: (&str, &str) = (
    "evidenceItems",
    "id status evidenceType evidenceLevel evidenceDirection \
    significance disease{name doid} therapies{name} therapyInteractionType source{citationId sourceType}",
);
const ASSERTIONS: (&str, &str) = (
    "assertions",
    "id status assertionType assertionDirection significance \
    ampLevel disease{name doid} therapies{name} therapyInteractionType variantOrigin",
);
pub(super) fn apply(
    leg: &Leg<'_>,
    query: &VariantGet,
    profiles: &mut raw::Connection<raw::Profile>,
) -> Result<(), RequestError> {
    for profile in &mut profiles.nodes {
        let id = profile
            .id
            .as_ref()
            .and_then(Field::as_ref)
            .copied()
            .filter(|id| raw::id(*id))
            .ok_or_else(|| failure(Reason::BadReply))?;
        let count = claim_counts(query).0;
        profile.evidence_items = evidence_filter::page(
            query.evidence_filter(),
            if query.civic_kind() == Some(CivicClaimKind::Evidence) {
                query.civic_cursor()
            } else {
                None
            },
            count,
            |level, after, count| {
                let filters = evidence_filter::arguments(
                    query.evidence_filter(),
                    level,
                    "status:NON_REJECTED",
                );
                rows(leg, query, id, EVIDENCE, &filters, after, count)
            },
        )?;
        if names_filtered(query) {
            let after = if query.civic_kind() == Some(CivicClaimKind::Assertions) {
                query.civic_cursor()
            } else {
                None
            };
            profile.assertions = rows(
                leg,
                query,
                id,
                ASSERTIONS,
                "status:NON_REJECTED",
                after,
                claim_counts(query).1,
            )?;
        }
    }
    Ok(())
}
