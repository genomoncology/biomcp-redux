//! Native query windows follow https://docs.myvariant.info/en/latest/doc/variant_query_service.html.
//! The query layer reports its result-window bound without replacing native order with scroll order.
use super::{QUERY, map, raw};
use crate::Mapped;
use biodata::{Build, VariantHeader, VariantSearch};
use biomcp_fetch::{Arg, BadReply, Leg, Reply, RequestError};
use serde::Deserialize;
/// Native MyVariant page; no local sort or deduplication changes candidate identity.
#[derive(Debug)]
pub struct SearchPage {
    /// Total reported by MyVariant for this exact query.
    pub total: u64,
    /// Every candidate in native order, with source field problems kept separate.
    pub rows: Mapped<Vec<VariantHeader>>,
}
#[derive(Deserialize)]
struct Page {
    total: u64,
    error: Option<serde::de::IgnoredAny>,
    success: Option<bool>,
    hits: Vec<raw::Hit>,
}
impl Reply for Page {
    fn check(&self) -> Result<(), BadReply> {
        if self.error.is_some()
            || self.success == Some(false)
            || self.hits.len() > 50
            || self.hits.len() as u64 > self.total
            || self.hits.iter().any(|h| h.check().is_err())
        {
            return Err(BadReply);
        }
        Ok(())
    }
}
/// Send one declared native query window and validate it before cache admission.
pub fn search(leg: &Leg<'_>, query: &VariantSearch) -> Result<SearchPage, RequestError> {
    let term = query.source_query();
    let (page, _) = leg.json_received_checked::<Page>(
        &QUERY,
        &[
            ("q", Arg::Text(&term)),
            ("size", Arg::Num(u64::from(query.limit()))),
            ("from", Arg::Num(u64::from(query.offset()))),
            ("fields", Arg::Text(super::FIELDS)),
            (
                "assembly",
                Arg::Text(if query.build() == Build::Grch38 {
                    "hg38"
                } else {
                    "hg19"
                }),
            ),
        ],
        |page| {
            if page.hits.len() <= query.limit() as usize
                && page.hits.len() as u64
                    == page
                        .total
                        .saturating_sub(u64::from(query.offset()))
                        .min(u64::from(query.limit()))
            {
                Ok(())
            } else {
                Err(BadReply)
            }
        },
    )?;
    let mut rows = Mapped {
        value: Vec::new(),
        problems: Vec::new(),
    };
    for hit in page.hits {
        let id = hit.id.clone();
        let mapped = map::candidates(vec![hit], id.as_str(), query.build());
        rows.value.extend(mapped.value);
        rows.problems.extend(mapped.problems);
    }
    Ok(SearchPage {
        total: page.total,
        rows,
    })
}
