use super::{Snapshot, advance, continue_records, records, section};
use crate::Outcome;
use biodata::{Text, TrialLocation};
use biomcp_fetch::RequestError;
use biomcp_types::{TrialContactRecord, TrialGet, TrialSection, TrialSections, TrialSite};
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
        let rows = sites(&d.locations, query.point());
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
        sections.locations = Some(section(page, id));
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
/// Source order without a point; with one, each site's straight-line miles, nearest first,
/// and sites without coordinates last in source order.
fn sites(locations: &[TrialLocation], point: Option<(f64, f64)>) -> Vec<TrialSite> {
    let mut rows: Vec<TrialSite> = locations.iter().cloned().map(TrialSite::from).collect();
    let Some((lat, lon)) = point else {
        return rows;
    };
    for row in &mut rows {
        // Checked degrees keep the haversine finite.
        row.distance_mi = row.site.coordinates.map(|c| {
            let miles = miles((lat, lon), (c.latitude().get(), c.longitude().get()));
            (miles * 10.0).round() / 10.0
        });
    }
    rows.sort_by(|a, b| match (a.distance_mi, b.distance_mi) {
        (Some(a), Some(b)) => a.total_cmp(&b),
        (Some(_), None) => std::cmp::Ordering::Less,
        (None, Some(_)) => std::cmp::Ordering::Greater,
        (None, None) => std::cmp::Ordering::Equal,
    });
    rows
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
