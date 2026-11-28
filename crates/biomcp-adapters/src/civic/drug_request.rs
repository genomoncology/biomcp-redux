//! Native drug claim requests retain query filters and source stream cursors.
use super::{GENES, MEMBER_FEATURE, drug_raw as raw, evidence_filter, variant::failure};
use biodata::Text;
use biomcp_fetch::{Input, Leg, Read, RequestError};
use biomcp_types::{CivicClaimKind, DrugGet, Reason};
use serde::Serialize;

const COMMON: &str =
    "id status significance disease{name doid} therapies{id name} therapyInteractionType";
#[derive(Serialize)]
struct Request<'a> {
    query: String,
    variables: Variables<'a>,
}
#[derive(Serialize)]
struct Variables<'a> {
    id: u64,
    after: Option<&'a Text>,
}

fn fetch_native(
    leg: &Leg<'_>,
    id: std::num::NonZeroU32,
    query: &DrugGet,
    kind: Option<CivicClaimKind>,
    after: Option<&Text>,
    count: u32,
    level: Option<biomcp_types::CivicEvidenceLevel>,
) -> Result<raw::Data, RequestError> {
    let members = format!(
        "molecularProfile{{id name rawName link isComplex variants{{id name link {MEMBER_FEATURE} ... on \
        GeneVariant{{alleleRegistryId hgvsDescriptions coordinates{{referenceBuild}}}}}}}}"
    );
    let profile = query
        .civic_profile()
        .map_or(String::new(), |id| format!(",molecularProfileId:{id}"));
    let connection = |kind| {
        let (name, fields) = match kind {
            CivicClaimKind::Evidence => (
                "evidenceItems",
                "evidenceType evidenceLevel evidenceDirection source{citationId sourceType}",
            ),
            CivicClaimKind::Assertions => (
                "assertions",
                "assertionType assertionDirection ampLevel variantOrigin",
            ),
        };
        let filter = if kind == CivicClaimKind::Evidence {
            evidence_filter::arguments(query.evidence_filter(), level)
        } else {
            evidence_filter::status_argument(query.evidence_filter().evidence_status)
        };
        format!(
            "{name}(therapyId:$id{profile},{filter},first:{count},after:$after){{totalCount \
                pageInfo{{hasNextPage endCursor}} nodes{{{COMMON} {fields} {members}}}}}"
        )
    };
    let fields = kind.map_or_else(
        || {
            format!(
                "{} {}",
                connection(CivicClaimKind::Evidence),
                connection(CivicClaimKind::Assertions)
            )
        },
        connection,
    );
    let body = Request {
        query: format!("query($id:Int!,$after:String){{therapy(id:$id){{id name link}} {fields}}}"),
        variables: Variables {
            id: u64::from(id.get()),
            after,
        },
    };
    let reply: raw::ReplyBody = leg
        .call(
            &GENES,
            Input::json(&[], &body),
            Read::json().cache_if(&|reply: &raw::ReplyBody| {
                matches(&reply.data, kind, after, count) && raw::cacheable(&reply.data, query)
            }),
        )?
        .value;
    if reply.data.therapy.is_none() {
        return Err(failure(Reason::NotFound));
    }
    if !matches(&reply.data, kind, after, count) {
        return Err(failure(Reason::BadReply));
    }
    Ok(reply.data)
}
pub(super) fn fetch(
    leg: &Leg<'_>,
    id: std::num::NonZeroU32,
    query: &DrugGet,
    kind: Option<CivicClaimKind>,
    after: Option<&Text>,
    count: u32,
) -> Result<raw::Data, RequestError> {
    if query.evidence_filter().is_empty() || kind == Some(CivicClaimKind::Assertions) {
        return fetch_native(leg, id, query, kind, after, count, None);
    }
    let mut therapy = None;
    let mut assertions = None;
    let evidence_items = evidence_filter::page(
        query.evidence_filter(),
        after,
        count,
        |level, cursor, count| {
            let data = fetch_native(leg, id, query, kind, cursor, count, level)?;
            therapy = data.therapy;
            assertions = data.assertions;
            data.evidence_items.ok_or_else(|| failure(Reason::BadReply))
        },
    )?;
    Ok(raw::Data {
        therapy,
        evidence_items: Some(evidence_items),
        assertions,
    })
}
fn matches(
    data: &raw::Data,
    kind: Option<CivicClaimKind>,
    after: Option<&Text>,
    count: u32,
) -> bool {
    (kind == Some(CivicClaimKind::Assertions)
        || data.evidence_items.as_ref().is_some_and(|rows| {
            rows.nodes.len() <= count as usize && (count == 0 || rows.progresses(after))
        }))
        && (kind == Some(CivicClaimKind::Evidence)
            || data.assertions.as_ref().is_some_and(|rows| {
                rows.nodes.len() <= count as usize && (count == 0 || rows.progresses(after))
            }))
}
