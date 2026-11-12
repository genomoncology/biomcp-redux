//! Whole posted result rows share the existing immutable section paging.
use super::{Snapshot, advance, continue_records, records, section};
use crate::Outcome;
use biodata::Text;
use biomcp_fetch::RequestError;
use biomcp_types::{TrialGet, TrialResultRecord, TrialSection, TrialSections};

/// The posted result records a section shows: all of them for results, or only
/// the participant-flow or baseline records, so those answer without paging past outcomes.
pub(super) fn rows(section: TrialSection, results: &[TrialResultRecord]) -> Vec<TrialResultRecord> {
    use TrialResultRecord as R;
    results
        .iter()
        .filter(|row| match section {
            TrialSection::Flow => {
                matches!(
                    row,
                    R::ParticipantFlow(_) | R::FlowGroup(_) | R::FlowPeriod(_)
                )
            }
            TrialSection::Baseline => matches!(
                row,
                R::Baseline(_) | R::BaselineGroup(_) | R::BaselineMeasure(_)
            ),
            _ => true,
        })
        .cloned()
        .collect()
}

pub(super) fn render(
    query: &TrialGet,
    snapshot: &Snapshot,
    pool: Option<&Text>,
    sections: &mut TrialSections,
    out: &mut Outcome,
) -> Result<(), RequestError> {
    let all = query.sections().contains(&TrialSection::All);
    // Flow and baseline are already inside results, so `all` does not repeat them.
    for (part, requested) in [
        (TrialSection::Results, all),
        (TrialSection::Flow, false),
        (TrialSection::Baseline, false),
    ] {
        if !requested && !query.sections().contains(&part) {
            continue;
        }
        let id = snapshot.detail.card.nct_id;
        let part_rows = rows(part, &snapshot.detail.results);
        let mut page = records(query, &part_rows, 8000, serde_json::to_vec)?;
        continue_records(&mut page, query, id, part, pool);
        advance(
            query,
            id,
            part,
            pool,
            page.offset + page.items.len() as u32,
            page.total,
            out,
        );
        let page = Some(section(page, id));
        match part {
            TrialSection::Flow => sections.flow = page,
            TrialSection::Baseline => sections.baseline = page,
            _ => sections.results = page,
        }
    }
    Ok(())
}
