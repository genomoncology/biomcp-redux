//! 510(k) records counted by FDA product code through openFDA's `count=product_code` facet.
use biomcp_fetch::{Arg, BadReply, Freshness, Input, Leg, Read, RequestError};
use biomcp_types::{DiagnosticProductCodeCount, DiagnosticSearch};
use serde::Deserialize;

/// The most terms openFDA returns for one count; it refuses a larger `limit` and any `skip`
/// with a count, and returns 100 terms when no `limit` is sent (measured 2026-11-16).
pub const CODE_LIMIT: u32 = 1000;

#[derive(Debug, Deserialize)]
struct Term {
    term: biodata::Text,
    count: u64,
}
impl crate::openfda::Item<crate::openfda::CountMeta> for Term {}
type Response = crate::openfda::Envelope<Term, crate::openfda::CountMeta>;

/// Count the search's matching records by product code, most records first, in one request
/// with the search's own expression.
pub fn product_codes(
    leg: &Leg<'_>,
    q: &DiagnosticSearch,
) -> Result<Vec<DiagnosticProductCodeCount>, RequestError> {
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
    Ok(match reply.value {
        Response::Page { results, .. } => results
            .into_iter()
            .map(|row| DiagnosticProductCodeCount {
                product_code: row.term,
                count: row.count,
            })
            .collect(),
        Response::Absent { .. } => Vec::new(),
    })
}
