//! Exact therapy selection preserves independently filtered native claim streams.
use super::{GENES, drug_map, drug_raw as raw, variant::failure, variant_raw::Connection};
use crate::Mapped;
use biodata::Text;
use biomcp_fetch::{Leg, RequestError};
use biomcp_types::{
    CivicClaimKind, CivicClaimPage, DrugCivic, DrugCivicPage, DrugCivicStreams, DrugGet, Reason,
    Section, Source,
};
use serde::Serialize;

const MEMBERS: &str = "molecularProfile{id name rawName link isComplex variants{id name link feature{id name} ... on \
    GeneVariant{alleleRegistryId hgvsDescriptions coordinates{referenceBuild}}}}";
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

fn fetch(
    leg: &Leg<'_>,
    query: &DrugGet,
    kind: Option<CivicClaimKind>,
    after: Option<&Text>,
    count: u32,
) -> Result<raw::Data, RequestError> {
    let id = query
        .input()
        .to_string()
        .parse::<u64>()
        .map_err(|_| failure(Reason::BadReply))?;
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
        format!(
            "{name}(therapyId:$id{profile},status:ACCEPTED,first:{count},after:$after){{totalCount \
                pageInfo{{hasNextPage endCursor}} nodes{{{COMMON} {fields} {MEMBERS}}}}}"
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
        variables: Variables { id, after },
    };
    let reply: raw::ReplyBody =
        leg.post_json_checked_cache(&GENES, &body, |reply: &raw::ReplyBody| {
            matches(&reply.data, query, kind, after, count)
        })?;
    if reply.data.therapy.is_none() {
        return Err(failure(Reason::NotFound));
    }
    if !matches(&reply.data, query, kind, after, count) {
        return Err(failure(Reason::BadReply));
    }
    Ok(reply.data)
}
fn matches(
    data: &raw::Data,
    query: &DrugGet,
    kind: Option<CivicClaimKind>,
    after: Option<&Text>,
    count: u32,
) -> bool {
    data.therapy
        .as_ref()
        .is_some_and(|t| t.id.to_string() == query.input().to_string())
        && (kind == Some(CivicClaimKind::Assertions)
            || data
                .evidence_items
                .as_ref()
                .is_some_and(|rows| rows.nodes.len() <= count as usize && rows.progresses(after)))
        && (kind == Some(CivicClaimKind::Evidence)
            || data
                .assertions
                .as_ref()
                .is_some_and(|rows| rows.nodes.len() <= count as usize && rows.progresses(after)))
}
fn seek<T>(
    leg: &Leg<'_>,
    query: &DrugGet,
    kind: CivicClaimKind,
    mut rows: Connection<T>,
    extract: fn(raw::Data) -> Option<Connection<T>>,
) -> Result<Connection<T>, RequestError> {
    let offset = if query.civic_cursor().is_some() {
        0
    } else {
        query.section_offset().unwrap_or(0)
    };
    let mut skipped = 0_u32;
    while skipped < offset {
        if !rows.page_info.has_next_page {
            rows.nodes.clear();
            return Ok(rows);
        }
        skipped = skipped
            .checked_add(rows.nodes.len() as u32)
            .ok_or_else(|| failure(Reason::TooLarge))?;
        let count = if skipped < offset {
            (offset - skipped).min(50)
        } else {
            query.section_limit().unwrap_or(1)
        };
        let data = fetch(
            leg,
            query,
            Some(kind),
            rows.page_info.end_cursor.as_ref(),
            count,
        )?;
        rows = extract(data).ok_or_else(|| failure(Reason::BadReply))?;
    }
    Ok(rows)
}
fn page<T>(
    rows: &Connection<T>,
    query: &DrugGet,
    kind: CivicClaimKind,
) -> Result<CivicClaimPage<DrugGet>, RequestError> {
    let next = if rows.page_info.has_next_page {
        Some(
            query
                .with_civic_stream(kind, rows.page_info.end_cursor.clone())
                .map_err(|_| failure(Reason::BadReply))?,
        )
    } else {
        None
    };
    Ok(CivicClaimPage {
        returned: rows.nodes.len() as u32,
        total: rows.total_count,
        next,
    })
}
/// Retrieve only the exact native therapy and its requested accepted claim collections.
/// Claim profiles retain their own native context; collection totals never become profile counts.
pub fn drug_civic(leg: &Leg<'_>, query: &DrugGet) -> Result<Mapped<DrugCivic>, RequestError> {
    if query.source() != biomcp_types::DrugSource::Civic {
        return Err(failure(Reason::BadReply));
    }
    let offset = if query.civic_cursor().is_some() {
        0
    } else {
        query.section_offset().unwrap_or(0)
    };
    let count = if offset > 0 {
        offset.min(50)
    } else {
        query.section_limit().unwrap_or(1)
    };
    let data = fetch(leg, query, query.civic_kind(), query.civic_cursor(), count)?;
    let identity = data
        .therapy
        .ok_or_else(|| failure(Reason::NotFound))?
        .header()?;
    let mut cx = crate::mapped::Problems {
        source: Source::Civic,
        record: None,
        list: Vec::new(),
    };
    let evidence = data
        .evidence_items
        .map(|rows| {
            let rows = seek(leg, query, CivicClaimKind::Evidence, rows, |d| {
                d.evidence_items
            })?;
            let mut page = page(&rows, query, CivicClaimKind::Evidence)?;
            let records = rows
                .nodes
                .into_iter()
                .filter_map(|r| {
                    cx.record = None;
                    drug_map::evidence(r, query, &mut cx)
                })
                .collect::<Box<[_]>>();
            page.returned = records.len() as u32;
            Ok::<_, RequestError>(DrugCivicPage { records, page })
        })
        .transpose()?;
    let assertions = data
        .assertions
        .map(|rows| {
            let rows = seek(leg, query, CivicClaimKind::Assertions, rows, |d| {
                d.assertions
            })?;
            let mut page = page(&rows, query, CivicClaimKind::Assertions)?;
            let records = rows
                .nodes
                .into_iter()
                .filter_map(|r| {
                    cx.record = None;
                    drug_map::assertion(r, query, &mut cx)
                })
                .collect::<Box<[_]>>();
            page.returned = records.len() as u32;
            Ok::<_, RequestError>(DrugCivicPage { records, page })
        })
        .transpose()?;
    Ok(Mapped {
        value: DrugCivic {
            identity,
            civic: Section::Data {
                source: Source::Civic,
                data: DrugCivicStreams {
                    evidence,
                    assertions,
                },
            },
        },
        problems: cx.list,
    })
}
