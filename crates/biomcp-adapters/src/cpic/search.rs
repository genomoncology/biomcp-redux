//! Gene pair search, with a page past the end read as empty with its total (1061).
use super::read::{Page, pairs, pairs_for_gene};
use biodata::PgxPair;
use biomcp_fetch::{Leg, RequestError};
use biomcp_types::PgxSearch;

/// Search exact gene pairs, preserving native level namespaces and row order.
///
/// CPIC answers an offset past the last row with status 416, so that page is read as empty
/// and the total comes from a one-row read of the same search.
pub fn search(leg: &Leg<'_>, query: &PgxSearch) -> Result<Page<PgxPair>, RequestError> {
    match pairs(leg, query, query.offset(), query.limit()) {
        Err(RequestError::Source(error)) if error.status == Some(416) && query.offset() > 0 => {
            let first = pairs(leg, query, 0, 1)?;
            if u64::from(query.offset()) < first.total {
                if let Some(pair) = first.items.first()
                    && pair.genesymbol.as_str() != query.gene().as_str()
                {
                    let gene = format!("eq.{}", pair.genesymbol);
                    return pairs_for_gene(leg, query, query.offset(), query.limit(), &gene);
                }
                return Err(error.into());
            }
            Ok(Page {
                refusals: crate::Refusal::None,
                items: Vec::new(),
                total: first.total,
                consumed: 0,
                problems: Vec::new(),
            })
        }
        reply => reply,
    }
}
