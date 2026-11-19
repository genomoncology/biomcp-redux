//! cBioPortal's own amplification and deep-deletion counts for one gene, as a named section.
use super::finish;
use crate::{Ctx, Outcome};
use biomcp_adapters::cbioportal::{SPEC, studies};
use biomcp_fetch::{Deadline, Leg, RequestError};
use biomcp_types::{ReplyBody, ReplyData, StudyGet, StudySection};

pub(super) fn section(cx: &Ctx<'_>, q: &StudyGet, deadline: Deadline<'_>, out: &mut Outcome) {
    if q.section() != Some(StudySection::Cna) {
        return;
    }
    let leg = cx.fetch.leg(&SPEC, deadline).fresh(cx.fresh);
    let result = counts(&leg, q, out);
    finish(out, leg, result.err());
}
fn counts(leg: &Leg<'_>, q: &StudyGet, out: &mut Outcome) -> Result<(), RequestError> {
    let data = match studies::copy_number(leg, q) {
        Ok(data) => data,
        Err(error) => {
            if let Some(usage) = super::rows::profile_refused(
                leg,
                q,
                error,
                studies::copy_number_profiles,
                biomcp_types::ErrorMessage::ArgumentValue,
            )? {
                out.body = ReplyData::Usage(usage);
                return Ok(());
            }
            return Err(error);
        }
    };
    out.body = data.with_query(q.clone());
    crate::status::item(out, true, false, false);
    Ok(())
}
