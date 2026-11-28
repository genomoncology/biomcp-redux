//! Archive lookups establish ClinVar record identity independently of allele resolution.
use crate::{Ctx, Outcome, status};
use biodata::{ClinVarVariationId, Text, VariantId, VcvId};
use biomcp_adapters::clinvar;
use biomcp_fetch::{Deadline, RequestError};
use biomcp_types::{
    AssertionId, AssertionSource, Paged, Problem, QueryInvalid, Reason, Section, Source,
    SourceNotice, SourceRecord, VariantAssertionsGet, VariantCard, VariantGet, VariantHeader,
    VariantResolution, VariantSections,
};

pub(super) fn get<'a>(cx: &Ctx<'a>, query: &VariantGet, deadline: &mut Deadline<'a>) -> Outcome {
    let leg = cx.fetch.leg(&clinvar::SPEC, *deadline).fresh(cx.fresh);
    let started = leg.remaining();
    let versioned = query.input().versioned_vcv();
    let result = if let Some(vcv) = versioned {
        clinvar::assertions(&leg, &vcv.get().to_string(), Some(vcv), None).map(Some)
    } else {
        clinvar::by_assertion(&leg, query.input())
    };
    let mut problems = Vec::new();
    let mut dropped = false;
    let found_record = result.as_ref().is_ok_and(Option::is_some);
    let found = result.and_then(|mapped| {
        let Some(mapped) = mapped else {
            return Ok(None);
        };
        dropped = mapped.refusals == biomcp_adapters::Refusal::Refused;
        problems = mapped.problems;
        candidate(query, mapped.value, &mut problems)
    });
    let error = found.as_ref().err().copied();
    *deadline = deadline.after_stage(started.saturating_sub(leg.remaining()));
    let mut source = status::source(Source::ClinVar, leg.finish(), error);
    let (mut resolution, mut candidates) = match found {
        Ok(Some(candidate)) => (VariantResolution::Resolved, vec![candidate]),
        Ok(None) => (VariantResolution::Absent, Vec::new()),
        Err(_) => (VariantResolution::Unavailable, Vec::new()),
    };
    if resolution == VariantResolution::Absent && found_record && versioned.is_none() {
        source.notices.push(guidance(
            "ClinVar's current archive did not return a row matching the requested accession.",
        ));
    }
    if let Some(vcv) = versioned.filter(|_| candidates.is_empty() && version_refused(error)) {
        resolution = VariantResolution::Rejected;
        source.notices.push(guidance(format!(
            "ClinVar did not serve {vcv}. Use get variant VCV{:09} clinvar to read the current archive.",
            vcv.get()
        )));
    }
    let mut sources = vec![source];
    let (section_sources, section_problems, section_error, incomplete) =
        super::sections::apply(cx, query, &mut candidates, deadline, None);
    sources.extend(section_sources);
    problems.extend(section_problems);
    let next = super::next(
        cx.fetch,
        query,
        resolution,
        &candidates,
        None,
        &mut problems,
    );
    status::finish(
        [super::overall(resolution, dropped || incomplete, &sources)],
        status::Parts {
            body: super::reply(
                query,
                VariantCard {
                    resolution,
                    source: Source::ClinVar,
                    candidates: candidates.into(),
                    rejection: None,
                },
                &sources,
            ),
            page: None,
            sources,
            problems,
            next,
            local: error.and_then(status::local).or(section_error),
            dropped,
        },
    )
}

fn version_refused(error: Option<RequestError>) -> bool {
    error.is_none_or(|error| {
        matches!(
            status::reason(error),
            Some(Reason::BadReply | Reason::NotFound)
        )
    })
}

fn candidate(
    query: &VariantGet,
    archive: clinvar::Assertions,
    problems: &mut Vec<Problem>,
) -> Result<Option<VariantHeader>, RequestError> {
    let invalid_archive = || biomcp_fetch::bad_reply(Source::ClinVar, None);
    let Some(vcv) = archive.archive.map_err(|_| invalid_archive())? else {
        return Ok(None);
    };
    let id = VariantId::ClinVarVariation(
        ClinVarVariationId::new(vcv.get()).map_err(|_| invalid_archive())?,
    );
    let mut candidate = super::direct::candidate(query, Source::ClinVar);
    candidate.record = SourceRecord::asked(None, &id).ok_or_else(invalid_archive)?;
    candidate.identity.ids = Box::new([id, VariantId::Vcv(vcv)]);
    let page = VariantAssertionsGet::from_identity(
        VariantId::Vcv(vcv),
        query.assertion_limit(AssertionSource::Clinvar),
        query.section_offset(),
        query.clinvar_kind(),
    )
    .map_err(|_| invalid_archive())?;
    let mut data = super::assertions::page(&page, archive.rows, archive.included_only);
    if let Some(assertion) = query.input().clinvar_assertion() {
        data.next = data.next.and_then(|next| {
            crate::next::checked(
                assertion_page(vcv, assertion, &next),
                Source::ClinVar,
                problems,
            )
        });
    }
    let section = if data.page.total == 0 && data.included_only.is_none() {
        Section::Empty {
            source: Source::ClinVar,
            record: Some(candidate.record.clone()),
        }
    } else {
        Section::Data {
            source: Source::ClinVar,
            data,
        }
    };
    candidate.sections = Some(Box::new(VariantSections {
        clinvar: Some(section),
        ..VariantSections::default()
    }));
    Ok(Some(candidate))
}

fn assertion_page(
    vcv: VcvId,
    assertion: &AssertionId,
    next: &VariantAssertionsGet,
) -> Result<VariantAssertionsGet, QueryInvalid> {
    let text = assertion.to_string();
    let (record, version) = text
        .split_once('.')
        .map_or((text.as_str(), None), |(id, v)| (id, Some(v)));
    let record = Text::new(record).map_err(|_| QueryInvalid::new(&["assertion"]))?;
    let version = version
        .map(Text::new)
        .transpose()
        .map_err(|_| QueryInvalid::new(&["version"]))?;
    VariantAssertionsGet::from_identity(VariantId::Vcv(vcv), next.limit(), next.offset(), None)?
        .select(record, version)
        .map(|selected| selected.at_offset(next.offset()))
}

fn guidance(message: impl Into<Box<str>>) -> SourceNotice {
    SourceNotice::RequestGuidance {
        data_date: None,
        message: message.into(),
        retry_after_seconds: None,
    }
}
