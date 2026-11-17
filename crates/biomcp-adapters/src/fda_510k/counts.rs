//! 510(k) records counted by FDA product code through openFDA's `count=product_code` facet.
use biomcp_fetch::{Arg, BadReply, Freshness, Input, Leg, Read, RequestError};
use biomcp_types::{DiagnosticProductCodeCount, DiagnosticSearch};
use serde::Deserialize;

/// The most terms openFDA returns for one count; it refuses a larger `limit` and any `skip`
/// with a count, and returns 100 terms when no `limit` is sent (measured 2026-11-16).
pub const CODE_LIMIT: u32 = 1000;

#[derive(Debug, Deserialize)]
struct Term {
    term: Box<str>,
    count: u64,
}
impl crate::openfda::Item<crate::openfda::CountMeta> for Term {}
type Response = crate::openfda::Envelope<Term, crate::openfda::CountMeta>;

/// Product-code rows, most records first, and the records openFDA counts under an empty code.
#[derive(Debug)]
pub struct ProductCodes {
    /// Rows with a product code, in openFDA's order.
    pub items: Vec<DiagnosticProductCodeCount>,
    /// openFDA returned its most terms, so more codes may exist.
    pub limit_reached: bool,
    /// Records counted under an empty product code (2024 has one, measured 2026-11-16).
    pub without_product_code: u64,
}

/// Count the search's matching records by product code, most records first, in one request
/// with the search's own expression.
pub fn product_codes(leg: &Leg<'_>, q: &DiagnosticSearch) -> Result<ProductCodes, RequestError> {
    let selector = super::expression(q);
    let mut args = vec![
        ("count", Arg::Text("product_code")),
        ("limit", Arg::Num(CODE_LIMIT.into())),
    ];
    if !selector.is_empty() {
        args.push(("search", Arg::Text(&selector)));
    }
    let endpoint = super::SPEC
        .get("")
        .fresh(Freshness::Days(1))
        .bytes(2 * 1024 * 1024)
        .json_absence();
    let reply = leg.call(
        &endpoint,
        Input::Params(&args),
        Read::<Response>::json().check(&|reply| match reply {
            Response::Page { results, .. } if results.len() > CODE_LIMIT as usize => Err(BadReply),
            _ => Ok(()),
        }),
    )?;
    let rows = match reply.value {
        Response::Page { results, .. } => results,
        Response::Absent { .. } => Vec::new(),
    };
    let mut out = ProductCodes {
        limit_reached: rows.len() >= CODE_LIMIT as usize,
        items: Vec::new(),
        without_product_code: 0,
    };
    for row in rows {
        match biodata::Text::new(&row.term) {
            Ok(product_code) => out.items.push(DiagnosticProductCodeCount {
                product_code,
                count: row.count,
            }),
            Err(_) => out.without_product_code += row.count,
        }
    }
    Ok(out)
}
