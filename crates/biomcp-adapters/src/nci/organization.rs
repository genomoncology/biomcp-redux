//! One unsized NCI organization request under an explicit response byte bound.
use biodata::Text;
use biomcp_fetch::{Arg, Endpoint, Freshness, Leg, RequestError};
use biomcp_types::{Organization, OrganizationSearch};
/// Maximum complete organization observation before a typed narrowing refusal.
pub const MAX_BYTES: u64 = 1024 * 1024;
const SEARCH: Endpoint = super::SPEC.get("/organizations").fresh(Freshness::Days(1)).bytes(MAX_BYTES);
/// Exact documented native selectors; size and offset never reach this endpoint.
pub fn parameters(q: &OrganizationSearch) -> Vec<(&'static str, &Text)> {
    [("name",q.name()),("org_city",q.city()),("org_state_or_province",q.state()),
     ("org_country",q.country()),("org_postal_code",q.postal_code()),("org_family",q.family()),
     ("org_coordinates_lat",q.lat()),("org_coordinates_lon",q.lon()),("org_coordinates_dist",q.distance())]
        .into_iter().filter_map(|(key,value)| value.map(|v| (key,v))).collect()
}
/// Retrieve a complete checked observation. Bucket errors remain source errors.
pub fn search(leg: &Leg<'_>, q: &OrganizationSearch) -> Result<crate::Mapped<Vec<Organization>>, RequestError> {
    let args: Vec<_> = parameters(q).into_iter().map(|(k,v)| (k, Arg::Text(v.as_str()))).collect();
    leg.json::<super::organization_raw::Page>(&SEARCH,&args).map(super::organization_map::page)
}
