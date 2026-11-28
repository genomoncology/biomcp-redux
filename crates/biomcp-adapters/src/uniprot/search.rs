//! Native reviewed-human pages, keeping only a validated opaque cursor.
use super::{SPEC, entry};
use crate::Mapped;
use biodata::{GeneSymbol, ProteinHeader, Text, UniProtAcc};
use biomcp_fetch::{Arg, BadReply, Freshness, Leg, Reply, RequestError};
use biomcp_fetch::{Input, Read};
use biomcp_types::{Problem, ProblemCode, ProteinSearch, Source};
use serde::{Deserialize, Serialize};
const PATH: &str = "/uniprotkb/search";
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
    results: Vec<crate::raw::Field<entry::raw::Entry>>,
}
impl Reply for Rows {
    fn check(&self) -> Result<(), BadReply> {
        if self.results.len() > 5 {
            return Err(BadReply);
        }
        Ok(())
    }
}
/// Send the exact proved search shape and admit only reviewed human rows.
pub fn search(
    leg: &Leg<'_>,
    q: &ProteinSearch,
    cursor: Option<&Text>,
) -> Result<Mapped<SearchPage>, RequestError> {
    let selector = q.gene().map(gene_selector);
    let literal = selector
        .as_deref()
        .or_else(|| q.query().map(Text::as_str))
        .unwrap_or_default();
    search_page(leg, literal, cursor)
}

/// Find reviewed human candidates for an exact HGNC symbol missing UniProt links.
pub fn gene_candidates(
    leg: &Leg<'_>,
    symbol: &GeneSymbol,
) -> Result<Mapped<Vec<UniProtAcc>>, RequestError> {
    let mapped = search_page(leg, &gene_selector(symbol), None)?;
    if mapped.value.cursor.is_some() {
        return Err(biomcp_fetch::SourceError {
            source: Source::UniProt,
            reason: biomcp_types::Reason::TooLarge,
            status: Some(200),
            tls_cause: None,
            retry_after: None,
        }
        .into());
    }
    if mapped.refusals == crate::Refusal::Refused {
        return Err(biomcp_fetch::bad_reply(Source::UniProt, Some(200)));
    }
    Ok(Mapped {
        value: mapped
            .value
            .items
            .into_iter()
            .map(|row| row.protein.accession.as_str().parse())
            .collect::<Result<_, _>>()
            .map_err(|_| biomcp_fetch::bad_reply(Source::UniProt, Some(200)))?,
        refusals: mapped.refusals,
        problems: mapped.problems,
    })
}

fn gene_selector(symbol: &GeneSymbol) -> String {
    format!("gene_exact:{symbol}")
}

fn search_page(
    leg: &Leg<'_>,
    literal: &str,
    cursor: Option<&Text>,
) -> Result<Mapped<SearchPage>, RequestError> {
    let query = format!(
        "({literal}) AND (organism_id:{}) AND (reviewed:true)",
        super::HUMAN
    );
    let mut args = vec![
        ("query", Arg::Text(&query)),
        ("format", Arg::Text("json")),
        ("size", Arg::Num(5)),
        ("fields", Arg::Text(FIELDS)),
    ];
    if let Some(cursor) = cursor {
        args.push(("cursor", Arg::Text(cursor.as_str())));
    }
    let ep = SPEC
        .get(PATH)
        .identity_encoding()
        .stall_retry(super::STALL_LIMIT)
        .fresh(Freshness::Days(1));
    let (raw, meta) = leg
        .call(&ep, Input::Params(&args), Read::<Rows>::json())
        .map(|received| (received.value, received.meta))?;
    let page = meta
        .pagination
        .ok_or_else(|| biomcp_fetch::bad_reply(Source::UniProt, Some(200)))?;
    let returned = raw.results.len() as u64;
    if returned > page.total
        || returned == 0 && page.total > 0
        || cursor.is_none() && returned < page.total && page.next.is_none()
    {
        return Err(biomcp_fetch::bad_reply(Source::UniProt, Some(200)));
    }
    let cursor = page
        .next
        .as_deref()
        .map(|url| {
            SPEC.next_cursor(url, PATH, &args, "cursor")
                .map_err(|_| biomcp_fetch::bad_reply(Source::UniProt, Some(200)))
                .and_then(|value| {
                    Text::new(&value)
                        .map_err(|_| biomcp_fetch::bad_reply(Source::UniProt, Some(200)))
                })
        })
        .transpose()?;
    let mapped = headers(raw);
    Ok(Mapped {
        refusals: mapped.refusals,
        value: SearchPage {
            items: mapped.value,
            total: page.total,
            cursor,
        },
        problems: mapped.problems,
    })
}

fn headers(raw: Rows) -> Mapped<Vec<ProteinHeader>> {
    let mut items = Vec::new();
    let mut problems = Vec::new();
    let mut refusals = crate::Refusal::None;
    for raw in raw.results {
        let Some(raw) = raw.value() else {
            refusals = crate::Refusal::Refused;
            crate::mapped::push_problem(&mut problems, Source::UniProt, None, "identity");
            continue;
        };
        let id = raw.primary_accession.clone();
        let mapped = entry::map::entry(raw, &id);
        refusals.include(mapped.refusals);
        problems.extend(mapped.problems);
        if let Some(row) = mapped.value {
            if items
                .iter()
                .any(|h: &ProteinHeader| h.protein.accession == id)
            {
                refusals = crate::Refusal::Refused;
                problems.push(Problem {
                    source: Source::UniProt,
                    record: crate::mapped::record(&id),
                    field: "primaryAccession".into(),
                    code: ProblemCode::ValueUnknown,
                });
            } else {
                items.push(row.header);
            }
        }
    }
    Mapped {
        refusals,
        value: items,
        problems,
    }
}
