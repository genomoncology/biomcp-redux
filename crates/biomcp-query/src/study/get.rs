//! Stored cards and independently acquired bounded metadata sections.
mod files;
use super::{empty, finish};
use crate::{Ctx, Outcome};
use biomcp_adapters::cbioportal::{SPEC, studies};
use biomcp_fetch::{Leg, RequestError};
use biomcp_types::{
    PageInfo, Problem, ProblemCode, ReplyData, Request, Section, Source, Status, StepKind,
    StudyCard, StudyGet, StudyProfiles, StudySampleLists, StudySection,
};
/// Exact case-sensitive catalog card or one bounded independent metadata collection.
pub(crate) fn get(cx: &Ctx<'_>, q: &StudyGet) -> Outcome {
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let mut out = empty(match q.section() {
        Some(StudySection::Profiles) => ReplyData::StudyProfiles {
            query: q.clone(),
            body: None,
        },
        Some(StudySection::SampleLists) => ReplyData::StudySampleLists {
            query: q.clone(),
            body: None,
        },
        Some(StudySection::Files) => ReplyData::StudyFiles {
            query: q.clone(),
            body: None,
        },
        _ => ReplyData::StudyCard {
            query: q.clone(),
            body: None,
        },
    });
    if matches!(q.section(), None | Some(StudySection::All)) {
        let leg = cx.fetch.leg(&SPEC, deadline);
        let result = card(&leg, q, &mut out);
        finish(&mut out, leg, result.err());
        if let ReplyData::StudyCard { body: Some(_), .. } = out.body {
            for section in [
                StudySection::Profiles,
                StudySection::SampleLists,
                StudySection::Files,
            ] {
                next(&mut out, cx, StepKind::Section, q.with_section(section));
            }
        }
    }
    if q.section() == Some(StudySection::All)
        && !matches!(out.body, ReplyData::StudyCard { body: Some(_), .. })
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
                let was_partial = out.status() == Status::Partial;
                finish(out, leg, error);
                if q.section() == Some(StudySection::All) && (error.is_some() || was_partial) {
                    crate::status::item(out, true, false, true);
                }
            }
        }
    }
}
fn card(leg: &Leg<'_>, q: &StudyGet, out: &mut Outcome) -> Result<(), RequestError> {
    let (row, info) = studies::card(leg, q.id())?;
    out.include_refusals(row.refusals);
    out.problems.extend(row.problems);
    let row = row.value;
    let snapshot = studies::snapshot(&info)?;
    {
        let found = row.is_some();
        crate::status::item(out, found, false, false);
    };
    let source_url = format!("https://www.cbioportal.org/study/summary?id={}", q.id())
        .parse()
        .map_err(|_| bad())?;
    out.body = ReplyData::StudyCard {
        query: q.clone(),
        body: row.map(|study| StudyCard {
            source_url,
            study,
            snapshot,
            profiles: None,
            sample_lists: None,
            files: None,
        }),
    };
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
            let records = studies::profiles(leg, q.id(), q.section_offset(), q.section_limit())?;
            out.include_refusals(records.window.refusals());
            out.problems.extend(records.problems);
            let rejected = records.rejected;
            let next_offset = records.window.next_offset();
            let page = PageInfo {
                offset: records.window.offset(),
                returned: records.window.rows.len() as u32,
                total: records.window.total().unwrap_or(0),
                pool: None,
                truncated: None,
            };
            let items = records.window.rows.into();
            let data = StudyProfiles {
                study_id: q.id().clone(),
                items,
                page: page.clone(),
                acquired: records.acquired,
            };
            if let ReplyData::StudyCard {
                body: Some(card), ..
            } = &mut out.body
            {
                card.profiles = Some(Section::Data {
                    source: Source::Cbioportal,
                    data,
                });
            } else {
                out.body = ReplyData::StudyProfiles {
                    query: q.clone(),
                    body: Some(data),
                };
                out.page = Some(page.clone());
            }
            continuation(out, cx, q, rejected, next_offset);
        }
        Some(StudySection::SampleLists) => {
            let records =
                studies::sample_lists(leg, q.id(), q.section_offset(), q.section_limit())?;
            out.include_refusals(records.window.refusals());
            out.problems.extend(records.problems);
            let rejected = records.rejected;
            let next_offset = records.window.next_offset();
            let page = PageInfo {
                offset: records.window.offset(),
                returned: records.window.rows.len() as u32,
                total: records.window.total().unwrap_or(0),
                pool: None,
                truncated: None,
            };
            let items = records.window.rows.into();
            let data = StudySampleLists {
                study_id: q.id().clone(),
                items,
                page: page.clone(),
                acquired: records.acquired,
            };
            if let ReplyData::StudyCard {
                body: Some(card), ..
            } = &mut out.body
            {
                card.sample_lists = Some(Section::Data {
                    source: Source::Cbioportal,
                    data,
                });
            } else {
                out.body = ReplyData::StudySampleLists {
                    query: q.clone(),
                    body: Some(data),
                };
                out.page = Some(page.clone());
            }
            continuation(out, cx, q, rejected, next_offset);
        }
        _ => {}
    }
    Ok(())
}
fn continuation(
    out: &mut Outcome,
    cx: &Ctx<'_>,
    q: &StudyGet,
    rejected: usize,
    next_offset: Option<u32>,
) {
    if rejected != 0 {
        crate::status::item(out, true, false, true);
    } else if out.status() != Status::Partial {
        crate::status::item(out, true, false, false);
    }
    if let Some(offset) = next_offset
        && let Some(query) = crate::next::checked(
            q.at_section_offset(offset),
            biomcp_types::Source::Cbioportal,
            &mut out.problems,
        )
    {
        next(out, cx, StepKind::Page, query);
    }
}
fn next(out: &mut Outcome, cx: &Ctx<'_>, kind: StepKind, query: StudyGet) {
    out.next.push(crate::next::Step::run(
        kind,
        &[biomcp_types::Source::Cbioportal],
        Request::GetStudy(query),
        cx.fresh,
    ));
}
fn section_error(out: &mut Outcome, section: StudySection, error: Option<RequestError>) {
    let Some(error) = error else { return };
    let reason = crate::status::reason(error).unwrap_or(biomcp_types::Reason::BadReply);
    if let ReplyData::StudyCard {
        body: Some(card), ..
    } = &mut out.body
    {
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
