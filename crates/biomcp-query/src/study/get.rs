//! Stored cards and independently acquired bounded metadata sections.
mod files;
use super::{empty, finish};
use crate::{Action, Ctx, Outcome};
use biomcp_adapters::cbioportal::{SPEC, studies};
use biomcp_fetch::{Leg, RequestError};
use biomcp_types::{
    PageInfo, Problem, ProblemCode, QueryBody, Request, Section, Source, Status, StepKind,
    StudyCard, StudyGet, StudyProfiles, StudySampleLists, StudySection,
};
/// Exact case-sensitive catalog card or one bounded independent metadata collection.
pub fn get(cx: &Ctx<'_>, q: &StudyGet) -> Outcome {
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let mut out = empty(match q.section() {
        Some(StudySection::Profiles) => QueryBody::StudyProfiles(None),
        Some(StudySection::SampleLists) => QueryBody::StudySampleLists(None),
        Some(StudySection::Files) => QueryBody::StudyFiles(None),
        _ => QueryBody::StudyCard(None),
    });
    if matches!(q.section(), None | Some(StudySection::All)) {
        let leg = cx.fetch.leg(&SPEC, deadline);
        let result = card(&leg, q, &mut out);
        finish(&mut out, leg, result.err());
        if let QueryBody::StudyCard(Some(_)) = out.body {
            for section in [
                StudySection::Profiles,
                StudySection::SampleLists,
                StudySection::Files,
            ] {
                next(&mut out, cx, StepKind::Section, q.with_section(section));
            }
        }
    }
    if q.section() == Some(StudySection::All) && !matches!(out.body, QueryBody::StudyCard(Some(_)))
    {
        command.finish(deadline.remaining());
        return out;
    }
    sections(cx, q, deadline, &mut out);
    files::section(cx, q, deadline, &mut out);
    command.finish(deadline.remaining());
    out
}
fn sections(cx: &Ctx<'_>, q: &StudyGet, deadline: biomcp_fetch::Deadline<'_>, out: &mut Outcome) {
    if matches!(
        q.section(),
        Some(StudySection::Profiles | StudySection::SampleLists | StudySection::All)
    ) {
        for section in [StudySection::Profiles, StudySection::SampleLists] {
            if q.section() == Some(section) || q.section() == Some(StudySection::All) {
                let leg = cx.fetch.leg(&SPEC, deadline).fresh(cx.fresh);
                let query = if q.section() == Some(StudySection::All) {
                    q.with_section(section)
                } else {
                    q.clone()
                };
                let result = section_page(&leg, &query, cx, out);
                let error = result.err();
                if matches!(error, Some(RequestError::Source(e)) if e.reason == biomcp_types::Reason::BadReply)
                {
                    out.problems.push(Problem {
                        source: Source::Cbioportal,
                        record: biodata::Text::new(q.id().as_str()).ok(),
                        field: match section {
                            StudySection::Profiles => "profiles",
                            _ => "sample-lists",
                        }
                        .into(),
                        code: ProblemCode::CollectionIncomplete,
                    });
                }
                if q.section() == Some(StudySection::All) {
                    section_error(out, section, error);
                }
                let was_partial = out.status == Status::Partial;
                finish(out, leg, error);
                if q.section() == Some(StudySection::All) && (error.is_some() || was_partial) {
                    out.status = Status::Partial;
                }
            }
        }
    }
}
fn card(leg: &Leg<'_>, q: &StudyGet, out: &mut Outcome) -> Result<(), RequestError> {
    let (row, info) = studies::card(leg, q.id())?;
    let snapshot = studies::snapshot(&info)?;
    out.status = if row.is_some() {
        Status::Complete
    } else {
        Status::Absent
    };
    let source_url = format!("https://www.cbioportal.org/study/summary?id={}", q.id())
        .parse()
        .map_err(|_| bad())?;
    out.body = QueryBody::StudyCard(row.map(|study| StudyCard {
        source_url,
        study,
        snapshot,
        profiles: None,
        sample_lists: None,
        files: None,
    }));
    Ok(())
}
fn section_page(
    leg: &Leg<'_>,
    q: &StudyGet,
    cx: &Ctx<'_>,
    out: &mut Outcome,
) -> Result<(), RequestError> {
    match q.section() {
        Some(StudySection::Profiles) => {
            let records = studies::profiles(leg, q.id())?;
            let rejected = records.rejected;
            let (items, page) = page(records.rows, q);
            let data = StudyProfiles {
                study_id: q.id().clone(),
                items,
                page: page.clone(),
                acquired: records.acquired,
            };
            if let QueryBody::StudyCard(Some(card)) = &mut out.body {
                card.profiles = Some(Section::Data {
                    source: Source::Cbioportal,
                    data,
                });
            } else {
                out.body = QueryBody::StudyProfiles(Some(data));
                out.page = Some(page.clone());
            }
            continuation(out, cx, q, &page, rejected);
        }
        Some(StudySection::SampleLists) => {
            let records = studies::sample_lists(leg, q.id())?;
            let rejected = records.rejected;
            let (items, page) = page(records.rows, q);
            let data = StudySampleLists {
                study_id: q.id().clone(),
                items,
                page: page.clone(),
                acquired: records.acquired,
            };
            if let QueryBody::StudyCard(Some(card)) = &mut out.body {
                card.sample_lists = Some(Section::Data {
                    source: Source::Cbioportal,
                    data,
                });
            } else {
                out.body = QueryBody::StudySampleLists(Some(data));
                out.page = Some(page.clone());
            }
            continuation(out, cx, q, &page, rejected);
        }
        _ => {}
    }
    Ok(())
}
fn page<T>(rows: Vec<T>, q: &StudyGet) -> (Box<[T]>, PageInfo) {
    let total = rows.len() as u64;
    let items: Box<[_]> = rows
        .into_iter()
        .skip(q.section_offset() as usize)
        .take(q.section_limit() as usize)
        .collect();
    let page = PageInfo {
        offset: q.section_offset(),
        returned: items.len() as u32,
        total,
        pool: None,
        truncated: None,
    };
    (items, page)
}
fn continuation(out: &mut Outcome, cx: &Ctx<'_>, q: &StudyGet, page: &PageInfo, rejected: usize) {
    if rejected != 0 {
        out.status = Status::Partial;
        out.problems.push(Problem {
            source: Source::Cbioportal,
            record: biodata::Text::new(q.id().as_str()).ok(),
            field: "studyId".into(),
            code: ProblemCode::IdentityConflict,
        });
    } else if out.status != Status::Partial {
        out.status = Status::Complete;
    }
    if u64::from(page.offset) + u64::from(page.returned) < page.total
        && let Ok(query) = q.at_section_offset(page.offset + page.returned)
    {
        next(out, cx, StepKind::Page, query);
    }
}
fn next(out: &mut Outcome, cx: &Ctx<'_>, kind: StepKind, query: StudyGet) {
    out.next.push((
        kind,
        Action::Run {
            request: Request::GetStudy(query),
            fresh: cx.fresh,
        },
    ));
}
fn section_error(out: &mut Outcome, section: StudySection, error: Option<RequestError>) {
    let Some(error) = error else { return };
    let reason = crate::status::reason(error).unwrap_or(biomcp_types::Reason::BadReply);
    if let QueryBody::StudyCard(Some(card)) = &mut out.body {
        match section {
            StudySection::Profiles => {
                card.profiles = Some(Section::unavailable(Source::Cbioportal, reason))
            }
            StudySection::SampleLists => {
                card.sample_lists = Some(Section::unavailable(Source::Cbioportal, reason))
            }
            _ => {}
        }
    }
}

fn bad() -> RequestError {
    biomcp_fetch::SourceError {
        source: Source::Cbioportal,
        reason: biomcp_types::Reason::BadReply,
        status: None,
        retry_after: None,
    }
    .into()
}
