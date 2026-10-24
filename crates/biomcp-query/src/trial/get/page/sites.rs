use super::{Snapshot, advance, records, section};
use crate::Outcome;
use biodata::{Text, TrialContactRecord, TrialGet, TrialSection, TrialSections};
use biomcp_fetch::RequestError;
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
        let page = records(query, &d.locations, 4500, serde_json::to_vec)?;
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
        let page = records(query, &rows, 4500, serde_json::to_vec)?;
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
