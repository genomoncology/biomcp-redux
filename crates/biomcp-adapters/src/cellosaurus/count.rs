//! Cellosaurus's own match count for a search, read from its txt header.
use super::{Arg, Freshness, Input, Leg, Read, RequestError, SPEC, Source};
use biomcp_fetch::BadReply;
use biomcp_types::CellLineSearchCommand;
/// Cellosaurus's own match count for the same query. Its JSON search reply states none, but
/// the txt format's header states `response.numFound`, so one `rows=0` request reads it
/// (0980, pm job j41467: 373 bytes in about 0.35 s, against 0.46 s for a 10-row JSON page).
pub fn count(leg: &Leg<'_>, q: &CellLineSearchCommand) -> Result<u64, RequestError> {
    let query = super::search_query(q);
    let endpoint = SPEC
        .get("/search/cell-line")
        .bytes(64 * 1024)
        .fresh(Freshness::Days(1));
    let args = [
        ("q", Arg::Text(&query)),
        ("format", Arg::Text("txt")),
        ("fields", Arg::Text("ac")),
        ("rows", Arg::Num(0)),
    ];
    let check = |bytes: &[u8]| {
        num_found(bytes, &query)
            .map(|_| ())
            .map_err(|_| biomcp_types::Reason::BadReply)
    };
    let bytes = leg
        .call(
            &endpoint,
            Input::Params(&args),
            Read::bytes().validate_body(&check),
        )?
        .value
        .0;
    num_found(&bytes, &query).map_err(|_| biomcp_fetch::bad_reply(Source::Cellosaurus, None))
}
/// The header's count, only when the header echoes this request's query.
fn num_found(bytes: &[u8], query: &str) -> Result<u64, BadReply> {
    let text = std::str::from_utf8(bytes).map_err(|_| BadReply)?;
    let mut echoed = None;
    let mut found = None;
    for line in text.lines() {
        let Some(field) = line.strip_prefix("##") else {
            continue;
        };
        let field = field.trim_start();
        if let Some(value) = field.strip_prefix("query.q: ") {
            echoed = Some(value);
        } else if let Some(value) = field.strip_prefix("response.numFound: ") {
            found = Some(value.trim().parse::<u64>().map_err(|_| BadReply)?);
        }
    }
    match (echoed, found) {
        (Some(echoed), Some(found)) if echoed == query => Ok(found),
        _ => Err(BadReply),
    }
}
