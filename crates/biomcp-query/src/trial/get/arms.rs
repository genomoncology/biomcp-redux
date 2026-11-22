//! Page ordered ClinicalTrials.gov arm and intervention rows within the reply budget.
use super::{
    Snapshot,
    page::{advance, continue_records, records, section},
};
use crate::{Outcome, TrialArmProjection};
use biodata::Text;
use biomcp_fetch::RequestError;
use biomcp_types::{Section, Source, TrialArmRecord, TrialGet, TrialRecords, TrialSection};

pub(super) fn render(
    query: &TrialGet,
    snapshot: &Snapshot,
    pool: Option<&Text>,
    out: &mut Outcome,
    projection: Option<&TrialArmProjection<'_>>,
) -> Result<Section<TrialRecords<TrialArmRecord>>, RequestError> {
    let id = snapshot.detail.card.nct_id;
    let rows: Vec<_> = snapshot
        .detail
        .arms
        .iter()
        .cloned()
        .map(TrialArmRecord::Arm)
        .chain(
            snapshot
                .detail
                .interventions
                .iter()
                .cloned()
                .map(TrialArmRecord::Intervention),
        )
        .collect();
    const ARM_BYTES: usize = 4500;
    let mut page = records(query, &rows, ARM_BYTES, |row| match projection {
        Some(project) => project(row),
        None => serde_json::to_vec(row),
    })?;
    let requested = page.total.saturating_sub(query.offset()).min(query.limit());
    if projection.is_some() && page.page.returned < requested {
        page.page.truncated = Some(true);
        if let Some(source) = out
            .sources
            .iter_mut()
            .find(|source| source.source == Source::ClinicalTrialsGov)
        {
            source
                .notices
                .push(biomcp_types::SourceNotice::RequestGuidance {
                    data_date: None,
                    message: format!("{id} arms reached the {ARM_BYTES}-byte page limit.").into(),
                    retry_after_seconds: None,
                });
        }
    }
    continue_records(&mut page, query, id, TrialSection::Arms, pool);
    advance(
        query,
        id,
        TrialSection::Arms,
        pool,
        page.offset + page.items.len() as u32,
        page.total,
        out,
    );
    Ok(section(page, id))
}
