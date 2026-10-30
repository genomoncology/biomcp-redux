//! File references share the command deadline and retain independent section outcomes.
use super::{bad, finish, next, page};
use crate::{Ctx, Outcome};
use biomcp_adapters::cbioportal::{SPEC, studies};
use biomcp_fetch::{Deadline, RequestError};
use biomcp_types::{
    QueryBody, Section, Source, Status, StepKind, StudyFiles, StudyGet, StudySection,
};
pub(super) fn section(cx: &Ctx<'_>, q: &StudyGet, deadline: Deadline<'_>, out: &mut Outcome) {
    let on_card = matches!(out.body, QueryBody::StudyCard(Some(_)));
    if !matches!(q.section(), Some(StudySection::Files | StudySection::All)) {
        return;
    }
    let leg = cx.fetch.leg(&SPEC, deadline).fresh(cx.fresh);
    let result = studies::files(&leg, q.id()).and_then(|directory| apply(cx, q, out, directory));
    let error = result.err();
    if let Some(error) = error
        && let QueryBody::StudyCard(Some(card)) = &mut out.body
    {
        let reason = crate::status::reason(error).unwrap_or(biomcp_types::Reason::BadReply);
        card.files = Some(Section::unavailable(Source::Cbioportal, reason));
    }
    let partial = out.status() == Status::Partial;
    finish(out, leg, error);
    if on_card && (error.is_some() || partial) {
        crate::status::item(out, true, false, true);
    }
}
fn apply(
    cx: &Ctx<'_>,
    q: &StudyGet,
    out: &mut Outcome,
    directory: studies::Directory,
) -> Result<(), RequestError> {
    let Some(rows) = directory.rows else {
        if let QueryBody::StudyCard(Some(card)) = &mut out.body {
            card.files = Some(Section::Empty {
                source: Source::Cbioportal,
                record: None,
            });
        } else {
            crate::status::item(out, false, false, false);
        }
        return Ok(());
    };
    let query = if q.section() == Some(StudySection::All) {
        q.with_section(StudySection::Files)
    } else {
        q.clone()
    };
    let (items, bounds, _) = page(rows.into_iter().map(Some), &query);
    let data = StudyFiles {
        study_id: q.id().clone(),
        items,
        page: bounds.clone(),
        acquired: directory.acquired,
        license_notice: directory.license_notice,
    };
    if let QueryBody::StudyCard(Some(card)) = &mut out.body {
        card.files = Some(Section::Data {
            source: Source::Cbioportal,
            data,
        });
    } else {
        if q.section() != Some(StudySection::Files) {
            return Err(bad());
        }
        out.body = QueryBody::StudyFiles(Some(data));
        out.page = Some(bounds.clone());
        crate::status::item(out, true, false, false);
    }
    if u64::from(bounds.offset) + u64::from(bounds.returned) < bounds.total {
        let query = query
            .at_section_offset(bounds.offset + bounds.returned)
            .map_err(|_| bad())?;
        next(out, cx, StepKind::Page, query);
    }
    Ok(())
}
