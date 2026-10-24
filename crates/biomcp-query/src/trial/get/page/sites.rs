use super::{Snapshot, advance, continue_records, records, section};
use crate::Outcome;
use biodata::Text;
use biomcp_fetch::RequestError;
use biomcp_types::{TrialContactRecord, TrialGet, TrialSection, TrialSections};
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
        let mut page = records(query, &d.locations, 4500, serde_json::to_vec)?;
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
