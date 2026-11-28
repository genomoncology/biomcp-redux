//! Native GWAS Catalog v2 association pages, without clinical interpretation.
use biomcp_fetch::Input;
pub mod get;
mod raw;
mod receive;
mod scope;
mod studies;
pub use studies::{Studies, studies};
mod search;
use crate::quotas;
use biodata::VariantId;
use biomcp_fetch::{Arg, BadReply, Endpoint, Freshness, KeyUse, Leg, RequestError, SourceSpec};
use biomcp_types::Source;
mod map;
pub use map::Association;
pub use search::search;
pub use search::traits::{TraitCandidate, TraitMatch, trait_exists, trait_matches};
use std::collections::BTreeMap;
use std::time::Duration;
/// Current catalog API; retired v1 is never used.
pub static SPEC: SourceSpec = SourceSpec {
    destinations: biomcp_fetch::Destinations::Primary,
    user_agent: None,
    terms: biomcp_fetch::Terms {
        name: "GWAS Catalog terms and dataset rights",
        url: "https://www.ebi.ac.uk/about/terms-of-use/",
        usage: biomcp_types::Use::Open,
    },
    source: Source::Gwas,
    base: "https://www.ebi.ac.uk/gwas/rest/api/v2",
    key: KeyUse::None,
    quota: &quotas::GWAS,
    deadline: Duration::from_secs(15),
    command_budget: None,
};
const ASSOCIATIONS: Endpoint = SPEC.get("/associations").fresh(Freshness::Days(1));
/// Source-native page and its exact catalog denominator.
#[derive(Debug)]
pub struct Page {
    /// Actual record refusals in the selected source slots.
    pub refusals: crate::Refusal,
    /// Native page rows before identity refusals.
    pub received: u32,
    /// Rejected source fields and row identities.
    pub problems: Vec<biomcp_types::Problem>,
    /// Selected source rows.
    pub rows: Vec<Association>,
    /// Source-reported total associations.
    pub total: u64,
}
/// Read one bounded page by exact rsID in ascending p-value order.
pub fn associations(
    leg: &Leg<'_>,
    rs: &VariantId,
    page: u32,
    size: u32,
    skip: u32,
) -> Result<Page, RequestError> {
    let id = rs.to_string();
    let mut values = association_params(page, size);
    values.insert("rs_id".into(), id.clone());
    search::window(
        leg,
        values,
        size,
        page == 0 && skip == 0,
        skip as usize,
        |row| row.alleles().any(|allele| allele.rs_id.as_str() == id),
    )
}

fn association_params(page: u32, size: u32) -> BTreeMap<String, String> {
    BTreeMap::from([
        ("direction".into(), "asc".into()),
        ("page".into(), page.to_string()),
        ("size".into(), size.to_string()),
        ("sort".into(), "p_value".into()),
    ])
}

fn native(
    leg: &Leg<'_>,
    values: &BTreeMap<String, String>,
    skip: usize,
    matches_row: &impl Fn(&scope::Selectors) -> bool,
) -> Result<Page, RequestError> {
    let params: Vec<_> = values
        .iter()
        .map(|(k, v)| (k.as_str(), Arg::Text(v)))
        .collect();
    let response = leg
        .call(
            &ASSOCIATIONS,
            Input::Params(&params),
            biomcp_fetch::Read::<raw::Response>::json()
                .check(&|reply| admission(reply, values, matches_row))
                .cache_if(&raw::Response::identities_admitted),
        )?
        .value;
    Ok(self::page(response, skip))
}

fn admission(
    r: &raw::Response,
    values: &BTreeMap<String, String>,
    matches_row: &impl Fn(&scope::Selectors) -> bool,
) -> Result<(), BadReply> {
    if Some(r.page.number.to_string()) != values.get("page").cloned()
        || Some(r.page.size.to_string()) != values.get("size").cloned()
        || !search::identity::resource_link(r.links.current.href.as_str(), "associations", values)
    {
        return Err(BadReply);
    }
    let rows = r
        .embedded
        .as_ref()
        .map(|e| e.associations.as_slice())
        .unwrap_or_default();
    if !rows.iter().all(|row| matches_row(&row.selectors)) {
        return Err(BadReply);
    }
    // Exact parts govern order even when native doubles underflow.
    if !values.contains_key("sort") {
        return Ok(());
    }
    let mut previous = None;
    for row in rows.iter().filter_map(|row| row.value.as_ref()) {
        if let Some(exact) = row.exact_p_value() {
            if previous.as_ref().is_some_and(|last| last > &exact) {
                return Err(BadReply);
            }
            previous = Some(exact);
        }
    }
    Ok(())
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
    let mut refusals = crate::Refusal::None;
    for row in received_rows.into_iter().skip(skip) {
        if let Some(mut row) = problems.field("association_id", Some(row.value)) {
            refusals.include(row.refusals);
            row.refusals = crate::Refusal::None;
            problems.list.append(&mut row.problems);
            rows.push(row.into());
        } else {
            refusals = crate::Refusal::Refused;
        }
    }
    Page {
        refusals,
        rows,
        received,
        problems: problems.list,
        total: response.page.total_elements,
    }
}

/// Complete source policy and command declaration.
pub(crate) mod decl;
