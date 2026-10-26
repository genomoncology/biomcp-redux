//! Native reviewed-human pages, keeping only a validated opaque cursor.
use super::{SPEC, entry};
use crate::Mapped;
use biodata::{ProteinHeader, Text};
use biomcp_fetch::{Arg, BadReply, Freshness, Leg, Reply, RequestError};
use biomcp_types::{Problem, ProblemCode, ProteinSearch, Source};
use serde::{Deserialize, Serialize};
const FIELDS: &str = "accession,id,protein_name,gene_names,organism_id,reviewed,length";
/// One admitted native window and its exact source count.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SearchPage {
    /// Native source order, without choosing among candidates.
    pub items: Vec<ProteinHeader>,
    /// Native total of the same search.
    pub total: u64,
    /// Opaque next cursor, never a user-facing URL.
    pub cursor: Option<Text>,
}
#[derive(Debug, Deserialize)]
struct Rows {
    results: Vec<entry::raw::Entry>,
}
impl Reply for Rows {
    fn check(&self) -> Result<(), BadReply> {
        if self.results.len() > 5 {
            return Err(BadReply);
        }
        self.results.iter().try_for_each(Reply::check)
    }
}
/// Send the exact proved search shape and admit only reviewed human rows.
pub fn search(
    leg: &Leg<'_>,
    q: &ProteinSearch,
    cursor: Option<&Text>,
) -> Result<Mapped<SearchPage>, RequestError> {
    let query = native_query(q);
    let mut args = vec![
        ("query", Arg::Text(&query)),
        ("format", Arg::Text("json")),
        ("size", Arg::Num(5)),
    ];
    if q.query().is_some() {
        args.push(("fields", Arg::Text(FIELDS)));
    }
    if let Some(cursor) = cursor {
        args.push(("cursor", Arg::Text(cursor.as_str())));
    }
    let ep = SPEC
        .get("/uniprotkb/search")
        .identity_encoding()
        .fresh(Freshness::Days(1));
    let (raw, meta) = leg.json_received_checked::<Rows>(&ep, &args, |_| Ok(()))?;
    let page = meta.pagination.ok_or_else(bad)?;
    let cursor = page
        .next
        .as_deref()
        .map(|url| next_cursor(url, &query, q.query().is_some()))
        .transpose()?;
    let mut items = Vec::new();
    let mut problems = Vec::new();
    for raw in raw.results {
        let id = raw.primary_accession.clone();
        let mapped = entry::map::entry(raw, &id);
        problems.extend(mapped.problems);
        if let Some(row) = mapped.value {
            if items
                .iter()
                .any(|h: &ProteinHeader| h.protein.accession == id)
            {
                problems.push(Problem {
                    source: Source::UniProt,
                    record: Text::new(id.as_str()).ok(),
                    field: "primaryAccession".into(),
                    code: ProblemCode::ValueUnknown,
                });
            } else {
                items.push(row.header);
            }
        }
    }
    Ok(Mapped {
        value: SearchPage {
            items,
            total: page.total,
            cursor,
        },
        problems,
    })
}
fn native_query(q: &ProteinSearch) -> String {
    let literal = q
        .gene()
        .map(|v| format!("gene_exact:{v}"))
        .unwrap_or_else(|| q.query().map_or("", Text::as_str).into());
    format!("({literal}) AND (organism_id:9606) AND (reviewed:true)")
}
fn bad() -> RequestError {
    biomcp_fetch::SourceError {
        source: Source::UniProt,
        reason: biomcp_types::Reason::BadReply,
        status: Some(200),
        retry_after: None,
    }
    .into()
}
fn next_cursor(text: &str, query: &str, fields: bool) -> Result<Text, RequestError> {
    let u = url::Url::parse(text).map_err(|_| bad())?;
    if u.scheme() != "https"
        || u.host_str() != Some("rest.uniprot.org")
        || u.port().is_some()
        || !u.username().is_empty()
        || u.password().is_some()
        || u.fragment().is_some()
        || u.path() != "/uniprotkb/search"
    {
        return Err(bad());
    }
    let mut pairs = std::collections::BTreeMap::new();
    for (key, value) in u.query_pairs() {
        if pairs.insert(key.into_owned(), value.into_owned()).is_some() {
            return Err(bad());
        }
    }
    let cursor = pairs
        .remove("cursor")
        .and_then(|v| Text::new(&v).ok())
        .ok_or_else(bad)?;
    if pairs.remove("query").as_deref() != Some(query)
        || pairs.remove("format").as_deref() != Some("json")
        || pairs.remove("size").as_deref() != Some("5")
        || pairs.remove("fields").as_deref() != fields.then_some(FIELDS)
        || !pairs.is_empty()
    {
        return Err(bad());
    }
    Ok(cursor)
}
