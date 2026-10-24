//! Intact native records page by source position within an immutable NCI snapshot.
use super::get::Snapshot;
use crate::{Action, Outcome};
use biodata::{
    NciEligibilityPage, NciTrialItem, NciTrialSections, Request, Section, Source, Status, StepKind,
    Text, TrialGet, TrialSection, TrialSource,
};
use biomcp_fetch::RequestError;
const SECTIONS: [TrialSection; 4] = [
    TrialSection::Eligibility,
    TrialSection::Outcomes,
    TrialSection::Arms,
    TrialSection::Diseases,
];
const SECTION_BYTES: usize = 6000;
pub(super) fn valid(q: &TrialGet, s: &Snapshot) -> bool {
    let n = match q.sections().first() {
        Some(TrialSection::Eligibility) => s
            .detail
            .eligibility
            .as_ref()
            .map_or(0, |e| e.unstructured.len()),
        Some(TrialSection::Outcomes) => s.detail.outcomes.len(),
        Some(TrialSection::Arms) => s.detail.arms.len(),
        Some(TrialSection::Diseases) => s.detail.diseases.len(),
        _ => 0,
    };
    q.offset() as usize <= n
}
pub(super) fn render(
    q: &TrialGet,
    s: Snapshot,
    pool: Option<Text>,
    out: &mut Outcome,
) -> Result<NciTrialItem, RequestError> {
    let requested =
        |section| q.sections().contains(&section) || q.sections().contains(&TrialSection::All);
    let mut sections = NciTrialSections {
        eligibility: None,
        outcomes: None,
        arms: None,
        diseases: None,
    };
    let id = s.detail.card.nct_id;
    if requested(TrialSection::Eligibility) {
        sections.eligibility = Some(if let Some(e) = &s.detail.eligibility {
            let records = records(
                q,
                &e.unstructured,
                TrialSection::Eligibility,
                id,
                pool.as_ref(),
                out,
            )?;
            Section::Data {
                source: Source::Nci,
                data: NciEligibilityPage {
                    structured: e.structured.clone(),
                    unstructured: records,
                },
            }
        } else {
            Section::Empty {
                source: Source::Nci,
                record: None,
            }
        });
    }
    if requested(TrialSection::Outcomes) {
        sections.outcomes = Some(section(records(
            q,
            &s.detail.outcomes,
            TrialSection::Outcomes,
            id,
            pool.as_ref(),
            out,
        )?));
    }
    if requested(TrialSection::Arms) {
        sections.arms = Some(section(records(
            q,
            &s.detail.arms,
            TrialSection::Arms,
            id,
            pool.as_ref(),
            out,
        )?));
    }
    if requested(TrialSection::Diseases) {
        sections.diseases = Some(section(records(
            q,
            &s.detail.diseases,
            TrialSection::Diseases,
            id,
            pool.as_ref(),
            out,
        )?));
    }
    let more: Box<[_]> = SECTIONS.into_iter().filter(|s| !requested(*s)).collect();
    for section in &more {
        out.next.push((
            StepKind::Section,
            Action::Run {
                request: Request::GetTrial(
                    TrialGet::single(id, Some(*section), q.limit()).with_source(TrialSource::Nci),
                ),
                fresh: false,
            },
        ));
    }
    let unsupported: Box<[_]> = q
        .sections()
        .iter()
        .filter(|s| **s != TrialSection::All && !SECTIONS.contains(s))
        .copied()
        .collect();
    let state = if !unsupported.is_empty()
        || s.sources.iter().any(|s| {
            matches!(
                s.state,
                biodata::SourceState::Stale | biodata::SourceState::Failed
            )
        }) {
        Status::Partial
    } else {
        Status::Complete
    };
    Ok(NciTrialItem {
        input: id,
        status: state,
        card: Some(s.detail.card),
        fetched: Some(s.fetched),
        sections: (!q.sections().is_empty()).then_some(sections),
        more,
        unsupported_sections: unsupported,
    })
}
fn records<T: Clone + serde::Serialize>(
    q: &TrialGet,
    rows: &[T],
    section: TrialSection,
    id: biodata::NctId,
    pool: Option<&Text>,
    out: &mut Outcome,
) -> Result<biodata::TrialRecords<T>, RequestError> {
    let page = crate::trial::get::page::records(q, rows, SECTION_BYTES, serde_json::to_vec)?;
    crate::trial::get::page::advance(
        q,
        id,
        section,
        pool,
        page.offset + page.items.len() as u32,
        page.total,
        out,
    );
    Ok(page)
}
fn section<T>(page: biodata::TrialRecords<T>) -> Section<biodata::TrialRecords<T>> {
    if page.total == 0 {
        Section::Empty {
            source: Source::Nci,
            record: None,
        }
    } else {
        Section::Data {
            source: Source::Nci,
            data: page,
        }
    }
}
