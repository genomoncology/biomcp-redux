//! One page of a drug's WHO Model List of Essential Medicines listings, read live from the WHO eEML.
use super::empty;
use crate::{Ctx, Outcome, status};
use biomcp_adapters::who_eml;
use biomcp_types::{
    DrugEssentialMedicines, DrugGet, PageInfo, ReplyBody, Request, Section, Source, StepKind,
};

/// Listings per page when the call names no section limit; each is one eEML page read.
const DEFAULT_LIMIT: u32 = 5;

pub(super) fn get(cx: &Ctx<'_>, query: &DrugGet) -> Outcome {
    let mut out = empty(DrugEssentialMedicines::into_reply(query.clone(), None));
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let leg = cx.fetch.leg(&who_eml::SPEC, deadline).fresh(cx.fresh);
    let result = who_eml::essential_medicines(
        &leg,
        query.input().as_str(),
        query.section_limit().unwrap_or(DEFAULT_LIMIT),
        query.section_offset().unwrap_or(0),
    );
    let error = result.as_ref().err().copied();
    out.local = error.and_then(status::local);
    out.sources
        .push(status::source(Source::WhoEml, leg.finish(), error));
    if let Ok(mapped) = result {
        out.include_refusals(mapped.refusals);
        out.problems.extend(mapped.problems);
        let found = mapped.value.is_some();
        if let Some(body) = &mapped.value {
            page(query, cx.fresh, body, &mut out);
        }
        out.body = DrugEssentialMedicines::into_reply(query.clone(), mapped.value);
        status::item(&mut out, found, false, false);
    }
    command.finish(deadline.remaining());
    out
}

/// Page metadata and the next page's call.
fn page(query: &DrugGet, fresh: bool, body: &DrugEssentialMedicines, out: &mut Outcome) {
    let Section::Data { data, .. } = &body.listings else {
        return;
    };
    let returned = data.records.len() as u32;
    let total = data.total.unwrap_or(0);
    out.page = Some(PageInfo {
        offset: data.offset,
        returned,
        total,
        pool: None,
        truncated: None,
    });
    if let Some(offset) = data
        .offset
        .checked_add(returned)
        .filter(|n| returned > 0 && u64::from(*n) < total)
        && let Some(get) =
            crate::next::checked(query.at_section_offset(offset), Source::WhoEml, &mut out.problems)
    {
        out.next.push(crate::next::Step::run(
            StepKind::Page,
            &[Source::WhoEml],
            Request::GetDrug(get),
            fresh,
        ));
    }
}
