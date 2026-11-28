//! The named, gene-filtered structural variant section.
use super::{continuation, finish};
use crate::{Ctx, Outcome};
use biomcp_adapters::cbioportal::{SPEC, studies};
use biomcp_fetch::{Deadline, Leg, RequestError};
use biomcp_types::{ReplyBody, ReplyData, StudyGet, StudySection, StudyStructuralVariants};

pub(super) fn section(cx: &Ctx<'_>, q: &StudyGet, deadline: Deadline<'_>, out: &mut Outcome) {
    if q.section() != Some(StudySection::StructuralVariants) {
        return;
    }
    let leg = cx.fetch.leg(&SPEC, deadline).fresh(cx.fresh);
    let result = page(&leg, q, cx, out);
    finish(out, leg, result.err());
}

fn page(leg: &Leg<'_>, q: &StudyGet, cx: &Ctx<'_>, out: &mut Outcome) -> Result<(), RequestError> {
    let (rows, profile, gene) = match studies::structural_variants(leg, q) {
        Ok(rows) => rows,
        Err(error) => {
            if let Some(usage) = super::rows::profile_refused(
                leg,
                q,
                error,
                studies::structural_variant_profiles,
                biomcp_types::ErrorMessage::ArgumentValue,
            )? {
                out.body = ReplyData::Usage(usage);
                return Ok(());
            }
            return Err(error);
        }
    };
    let page = super::rows::page_info(&rows.window);
    let offset = rows.window.next_offset();
    out.include_refusals(rows.window.refusals());
    out.problems.extend(rows.problems);
    let next = q.with_molecular_profile(profile.clone());
    out.body = StudyStructuralVariants {
        study_id: q.id().clone(),
        molecular_profile_id: profile,
        entrez_gene_id: gene,
        total: page.total,
        page: page.clone(),
        items: rows.window.rows.into(),
        acquired: rows.acquired,
    }
    .with_query(q.clone());
    out.page = Some(page);
    continuation(out, cx, &next, rows.rejected != 0, offset);
    Ok(())
}
