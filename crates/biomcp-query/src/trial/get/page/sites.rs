use super::{Snapshot, advance, continue_records, records, section};
use crate::Outcome;
use biodata::{Text, TrialLocation};
use biomcp_fetch::RequestError;
use biomcp_types::{
    Section, Source, TrialContactRecord, TrialGet, TrialLocationsPage, TrialSection, TrialSections,
    TrialSite,
};
/// Bytes for one locations page: room for the largest `--limit` (200) of typical sites,
/// so the page holds what the caller asked for.
const LOCATIONS_BUDGET: usize = 120_000;
pub(super) fn render(
    query: &TrialGet,
    snapshot: &Snapshot,
    pool: Option<&Text>,
    sections: &mut TrialSections,
    out: &mut Outcome,
) -> Result<(), RequestError> {
    let requested = |section| {
        query.sections().contains(&section) || query.sections().contains(&TrialSection::All)
    };
    let d = &snapshot.detail;
    let id = d.card.nct_id;
    if requested(TrialSection::Locations) {
        let (rows, missing_coordinates) = sites(&d.locations, query);
        let mut page = records(query, &rows, LOCATIONS_BUDGET, serde_json::to_vec)?;
        continue_records(&mut page, query, id, TrialSection::Locations, pool);
        advance(
            query,
            id,
            TrialSection::Locations,
            pool,
            page.offset + page.items.len() as u32,
            page.total,
            out,
        );
        let filtered = query.filters_sites();
        let locations = TrialLocationsPage {
            missing_coordinates,
            records: page,
        };
        sections.locations = Some(if filtered || locations.records.total > 0 {
            Section::Data {
                source: Source::ClinicalTrialsGov,
                data: locations,
            }
        } else {
            Section::Empty {
                source: Source::ClinicalTrialsGov,
                record: super::record(id),
            }
        });
    }
    if requested(TrialSection::Contacts) {
        let rows: Vec<_> = d
            .contacts
            .iter()
            .cloned()
            .map(TrialContactRecord::Central)
            .chain(
                d.officials
                    .iter()
                    .cloned()
                    .map(TrialContactRecord::Official),
            )
            .collect();
        let mut page = records(query, &rows, 4500, serde_json::to_vec)?;
        continue_records(&mut page, query, id, TrialSection::Contacts, pool);
        advance(
            query,
            id,
            TrialSection::Contacts,
            pool,
            page.offset + page.items.len() as u32,
            page.total,
            out,
        );
        sections.contacts = Some(section(page, id));
    }
    Ok(())
}
/// Match the same site against all filters. Count unknown distances before applying the radius.
fn sites(locations: &[TrialLocation], query: &TrialGet) -> (Vec<TrialSite>, u32) {
    let state = query.site_state();
    let status = query.site_status();
    let point = query.point();
    let radius = query.site_radius_miles();
    let mut rows = Vec::new();
    let mut missing_coordinates = 0;
    for site in locations {
        if state.is_some_and(|wanted| {
            !site
                .state
                .as_deref()
                .is_some_and(|label| label.eq_ignore_ascii_case(wanted))
        }) || status.is_some_and(|wanted| site.status != Some(wanted))
        {
            continue;
        }
        if site.coordinates.is_none() {
            missing_coordinates += 1;
        }
        let distance = point
            .zip(site.coordinates)
            .map(|(origin, c)| miles(origin, (c.latitude().get(), c.longitude().get())));
        if radius.is_none_or(|max| distance.is_some_and(|miles| miles <= max)) {
            rows.push(TrialSite {
                site: site.clone(),
                distance_mi: distance,
            });
        }
    }
    if point.is_some() {
        rows.sort_by(|a, b| match (a.distance_mi, b.distance_mi) {
            (Some(a), Some(b)) => a.total_cmp(&b),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => std::cmp::Ordering::Equal,
        });
        for row in &mut rows {
            row.distance_mi = row.distance_mi.map(|miles| (miles * 10.0).round() / 10.0);
        }
    }
    (rows, missing_coordinates)
}
/// Great-circle (haversine) miles between two points in degrees.
fn miles(from: (f64, f64), to: (f64, f64)) -> f64 {
    const EARTH_RADIUS_MI: f64 = 3958.8;
    let (lat1, lat2) = (from.0.to_radians(), to.0.to_radians());
    let half_lat = (lat2 - lat1) / 2.0;
    let half_lon = (to.1 - from.1).to_radians() / 2.0;
    let a = half_lat.sin().powi(2) + lat1.cos() * lat2.cos() * half_lon.sin().powi(2);
    2.0 * EARTH_RADIUS_MI * a.sqrt().min(1.0).asin()
}
