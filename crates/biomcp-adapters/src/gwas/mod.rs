//! Native GWAS Catalog v2 association pages, without clinical interpretation.
pub mod get;
mod raw;
mod receive;
mod search;
use crate::quotas;
use biodata::VariantId;
use biomcp_fetch::{Arg, BadReply, Endpoint, Freshness, KeyUse, Leg, RequestError, SourceSpec};
use biomcp_types::Source;
pub use raw::{Association, EffectAllele, Trait};
pub use search::search;
use std::time::Duration;
/// Current catalog API; retired v1 is never used.
pub static SPEC: SourceSpec = SourceSpec {
    destinations: biomcp_fetch::Destinations::Primary,
    user_agent: None,
    terms: biomcp_fetch::Terms {
        name: "GWAS Catalog terms and dataset rights",
        url: "https://www.ebi.ac.uk/about/terms-of-use/",
        usage: biomcp_types::Use::Restricted,
    },
    source: Source::Gwas,
    base: "https://www.ebi.ac.uk/gwas/rest/api/v2",
    key: KeyUse::None,
    quota: &quotas::GWAS,
    deadline: Duration::from_secs(15),
};
const ASSOCIATIONS: Endpoint = SPEC.get("/associations").fresh(Freshness::Days(1));
/// Source-native page and its exact catalog denominator.
#[derive(Debug)]
pub struct Page {
    /// Native page rows before identity refusals.
    pub received: u32,
    /// Rejected source fields and row identities.
    pub problems: Vec<biomcp_types::Problem>,
    /// Selected source rows.
    pub rows: Vec<Association>,
    /// Source-reported total associations.
    pub total: u64,
}
/// Read one bounded page by exact rsID, preserving current v2 request identity.
pub fn associations(
    leg: &Leg<'_>,
    rs: &VariantId,
    page: u32,
    size: u32,
    skip: u32,
) -> Result<Page, RequestError> {
    let id = rs.to_string();
    let expected = format!(
        "{}/associations?rs_id={id}&page={page}&size={size}",
        SPEC.base
    );
    let (response, _) = leg.json_received_checked::<raw::Response>(
        &ASSOCIATIONS,
        &[
            ("rs_id", Arg::Text(&id)),
            ("page", Arg::Num(page.into())),
            ("size", Arg::Num(size.into())),
        ],
        |r| {
            if r.matches(&id, page, size, &expected) {
                Ok(())
            } else {
                Err(BadReply)
            }
        },
    )?;
    Ok(page(response, skip as usize))
}
fn page(response: raw::Response, skip: usize) -> Page {
    let received_rows = response
        .embedded
        .map(|e| e.associations)
        .unwrap_or_default();
    let received = received_rows.len() as u32;
    let mut problems = crate::mapped::Problems {
        source: Source::Gwas,
        record: None,
        list: Vec::new(),
    };
    let mut rows = Vec::new();
    for row in received_rows.into_iter().skip(skip) {
        if let Some(mut row) = problems.field("association_id", Some(row)) {
            if row.association_id == 0 {
                problems.required::<()>("association_id", None);
                continue;
            }
            problems.list.append(&mut row.problems);
            rows.push(row);
        }
    }
    Page {
        rows,
        received,
        problems: problems.list,
        total: response.page.total_elements,
    }
}
