mod eligibility;
mod publications;
mod results;
mod sites;
use super::Snapshot;
use crate::{Outcome, TrialArmProjection};
use biodata::{NctId, Text};
use biomcp_fetch::{CacheError, RequestError};
use biomcp_types::{
    Request, Section, Source, SourceRecord, StepKind, TrialCardItem, TrialGet, TrialRecords,
    TrialSection, TrialSections,
};

/// A character offset must fall inside the text; a record offset past the end is an empty page.
/// The text length an offset is past, when it is past the end of the eligibility text.
pub(super) fn past_text(query: &TrialGet, snapshot: &Snapshot) -> Option<usize> {
    let d = &snapshot.detail;
    match query.sections().first() {
        Some(TrialSection::Eligibility) => {
            let total = d
                .eligibility
                .as_ref()
                .and_then(|e| e.criteria.as_ref())
                .map_or(0, |s| s.as_str().chars().count());
            (query.offset() as usize > total).then_some(total)
        }
        _ => None,
    }
}
pub(super) fn render(
    query: &TrialGet,
    input: NctId,
    snapshot: Snapshot,
    pool: Option<Text>,
    out: &mut Outcome,
    arm_projection: Option<&TrialArmProjection<'_>>,
) -> Result<TrialCardItem, RequestError> {
    let requested = |section| {
        query.sections().contains(&section) || query.sections().contains(&TrialSection::All)
    };
    let id = snapshot.detail.card.nct_id;
    let mut sections = TrialSections::default();
    if requested(TrialSection::Eligibility) {
        sections.eligibility = Some(eligibility::render(query, &snapshot, pool.as_ref(), out));
    }
    results::render(query, &snapshot, pool.as_ref(), &mut sections, out)?;
    records_sections(
        query,
        &snapshot,
        pool.as_ref(),
        &mut sections,
        out,
        arm_projection,
    )?;
    sites::render(query, &snapshot, pool.as_ref(), &mut sections, out)?;
    publications::render(query, &snapshot, pool.as_ref(), &mut sections, out)?;
    for section in biomcp_types::TrialSource::Ctgov
        .sections()
        .iter()
        .copied()
        .filter(|s| !requested(*s))
    {
        out.next.push(crate::next::Step::run(
            StepKind::Section,
            &[query.source().source()],
            Request::GetTrial(TrialGet::single(id, Some(section), query.limit())),
            false,
        ));
    }
    out.problems
        .extend(snapshot.detail.problems.iter().cloned());
    for problem in &snapshot.processing_problems {
        if !out.problems.contains(problem) {
            out.problems.push(problem.clone());
        }
    }
    Ok(TrialCardItem {
        input,
        status: if super::partial(&snapshot) {
            crate::status::item_status(true, false, true)
        } else {
            crate::status::item_status(true, false, false)
        },
        card: Some(biomcp_types::TrialPostedCard {
            protocol: snapshot.detail.card,
            results_first_posted: snapshot.detail.results_first_posted,
            enrollment: snapshot.detail.enrollment,
            why_stopped: snapshot.detail.why_stopped,
        }),
        fetched: snapshot.fetched,
        sections: (!query.sections().is_empty()).then_some(sections),
        more: biomcp_types::TrialSource::Ctgov
            .sections()
            .iter()
            .copied()
            .filter(|s| !requested(*s))
            .collect(),
        failure: None,
    })
}
pub(in crate::trial) fn records<T: Clone>(
    query: &TrialGet,
    rows: &[T],
    budget: usize,
    size: impl Fn(&T) -> Result<Vec<u8>, serde_json::Error>,
) -> Result<TrialRecords<T>, RequestError> {
    let mut items = Vec::new();
    let mut bytes = 0;
    for item in rows
        .iter()
        .skip(query.offset() as usize)
        .take(query.limit() as usize)
    {
        let size = size(item).map_err(|_| CacheError::Encoding)?.len() + 1;
        if !items.is_empty() && bytes + size > budget {
            break;
        }
        bytes += size;
        items.push(item.clone());
    }
    Ok(TrialRecords::new(
        query.offset(),
        rows.len() as u32,
        items.into(),
    ))
}
pub(super) fn section<T>(page: TrialRecords<T>, id: NctId) -> Section<TrialRecords<T>> {
    if page.total == 0 {
        Section::Empty {
            source: Source::ClinicalTrialsGov,
            record: record(id),
        }
    } else {
        Section::Data {
            source: Source::ClinicalTrialsGov,
            data: page,
        }
    }
}
fn record(id: NctId) -> Option<SourceRecord> {
    Text::new(&id.to_string()).ok().map(|id| SourceRecord {
        namespace: biodata::IdentifierNamespace::Unknown,
        id,
        url: None,
    })
}
pub(in crate::trial) fn advance(
    query: &TrialGet,
    id: NctId,
    section: TrialSection,
    pool: Option<&Text>,
    end: u32,
    total: u32,
    out: &mut Outcome,
) {
    if end < total
        && let Some(pool) = pool
    {
        out.next.push(crate::next::Step::run(
            StepKind::Page,
            &[query.source().source()],
            Request::GetTrial(query.page(id, section, pool.clone(), end)),
            false,
        ));
    }
}

fn records_sections(
    query: &TrialGet,
    snapshot: &Snapshot,
    pool: Option<&Text>,
    sections: &mut TrialSections,
    out: &mut Outcome,
    arm_projection: Option<&TrialArmProjection<'_>>,
) -> Result<(), RequestError> {
    let id = snapshot.detail.card.nct_id;
    let requested = |section| {
        query.sections().contains(&section) || query.sections().contains(&TrialSection::All)
    };
    if requested(TrialSection::Outcomes) {
        let items = &snapshot.detail.outcomes;
        let mut page = records(query, items, 8000, serde_json::to_vec)?;
        continue_records(&mut page, query, id, TrialSection::Outcomes, pool);
        advance(
            query,
            id,
            TrialSection::Outcomes,
            pool,
            page.offset + page.items.len() as u32,
            page.total,
            out,
        );
        sections.outcomes = Some(section(page, id));
    }
    if requested(TrialSection::Arms) {
        sections.arms = Some(super::arms::render(
            query,
            snapshot,
            pool,
            out,
            arm_projection,
        )?);
    }
    Ok(())
}

pub(in crate::trial) fn continuation(
    query: &TrialGet,
    id: NctId,
    section: TrialSection,
    pool: Option<&Text>,
    end: u32,
    total: u32,
) -> Option<TrialGet> {
    pool.filter(|_| end < total)
        .map(|p| query.page(id, section, p.clone(), end))
}
pub(in crate::trial) fn continue_records<T>(
    page: &mut TrialRecords<T>,
    query: &TrialGet,
    id: NctId,
    section: TrialSection,
    pool: Option<&Text>,
) {
    page.continuation = continuation(
        query,
        id,
        section,
        pool,
        page.offset + page.items.len() as u32,
        page.total,
    );
}
