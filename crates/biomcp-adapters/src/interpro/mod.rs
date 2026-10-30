//! Integrated InterPro entries only; member-database material retains upstream rights.
use biomcp_fetch::{KeyUse, SourceSpec, Terms};
use biomcp_types::{Source, Use};
use std::time::Duration;
/// One publisher declaration owns admission, terms and keyless request policy.
pub static SPEC: SourceSpec = SourceSpec {
    destinations: biomcp_fetch::Destinations::Primary,
    user_agent: None,
    source: Source::InterPro,
    base: "https://www.ebi.ac.uk/interpro/api",
    terms: Terms {
        name: "CC0",
        url: "https://interpro-documentation.readthedocs.io/en/latest/license.html",
        usage: Use::Open,
    },
    key: KeyUse::None,
    quota: &crate::quotas::INTERPRO,
    deadline: Duration::from_secs(8),
};
mod map;
mod raw;
use biodata::{ProteinDomainMatch, Text, UniProtIsoform};
use biomcp_fetch::{Arg, Freshness, Leg, RequestError};
use serde::{Deserialize, Serialize};
/// One checked native page with its complete native count and opaque continuation.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Page {
    /// Actual refused native records, independent of optional fields.
    #[serde(
        default = "crate::mapped::refusal_serde::none",
        with = "crate::mapped::refusal_serde"
    )]
    pub refusals: crate::Refusal,

    /// Whole native match rows.
    pub items: Vec<ProteinDomainMatch>,
    /// Source count before local type selection.
    pub total: u64,
    /// Source cursor, checked against the accession and request selectors.
    pub cursor: Option<Text>,
    /// Rejected optional fields for retained rows.
    pub problems: Vec<biomcp_types::Problem>,
}
/// Fetch a native accession page; type selection remains over the retained complete collection.
pub fn page(
    leg: &Leg<'_>,
    id: &UniProtIsoform,
    cursor: Option<&Text>,
    size: u32,
) -> Result<Page, RequestError> {
    if ![5, 25].contains(&size) {
        return Err(bad());
    }
    let path = format!("/entry/interpro/protein/uniprot/{id}/");
    let endpoint = SPEC
        .get("")
        .with_path(path.clone())?
        .identity_encoding()
        .fresh(Freshness::Days(1));
    let mut args = vec![("page_size", Arg::Num(size.into()))];
    if let Some(cursor) = cursor {
        args.push(("cursor", Arg::Text(cursor.as_str())));
        args.push(("page", Arg::Num(1)));
    }
    let raw = leg.json_checked_cache::<raw::Page>(&endpoint, &args, |p| p.admitted(id))?;
    if !raw.admitted(id) {
        return Err(bad());
    }
    let expected = [("page", Arg::Num(1)), ("page_size", Arg::Num(size.into()))];
    let cursor = raw
        .next
        .as_deref()
        .map(|next| {
            SPEC.next_cursor(next, &path, &expected, "cursor")
                .map_err(|_| bad())
                .and_then(|s| Text::new(&s).map_err(|_| bad()))
        })
        .transpose()?;
    if raw.results.is_empty() && cursor.is_some() {
        return Err(bad());
    }
    let mapped = map::rows(raw.results, id)?;
    Ok(Page {
        refusals: mapped.refusals,
        items: mapped.value,
        problems: mapped.problems,
        total: raw.count,
        cursor,
    })
}
fn bad() -> RequestError {
    biomcp_fetch::SourceError {
        source: Source::InterPro,
        reason: biomcp_types::Reason::BadReply,
        status: Some(200),
        retry_after: None,
    }
    .into()
}
