//! Archive lookups establish ClinVar record identity independently of allele resolution.
use crate::{Ctx, Outcome, status};
use biomcp_adapters::clinvar;
use biomcp_fetch::Deadline;
use biomcp_types::{
    AssertionSource, Reason, Section, Source, SourceNotice, Text, VariantAssertionsGet,
    VariantCard, VariantGet, VariantHeader, VariantId, VariantResolution, VariantSections, VcvId,
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
    let error = result.as_ref().err().copied();
    *deadline = deadline.after_stage(started.saturating_sub(leg.remaining()));
    let mut source = status::source(Source::ClinVar, leg.finish(), error);
    let mut resolution = VariantResolution::Absent;
    let mut problems = Vec::new();
    let mut candidates = Vec::new();
    let mut dropped = false;
    if let Ok(Some(mapped)) = result {
        dropped = mapped.refusals == biomcp_adapters::Refusal::Refused;
        problems = mapped.problems;
        if let Some(candidate) = candidate(query, mapped.value) {
            candidates.push(candidate);
            resolution = VariantResolution::Resolved;
        } else if versioned.is_none() {
            source.notices.push(guidance(
                "ClinVar's current archive did not return a row matching the requested accession.",
            ));
        }
    }
    let mut rejection = None;
    if let Some(vcv) = versioned.filter(|_| candidates.is_empty())
        && (error.is_none()
            || matches!(
                error.and_then(status::reason),
                Some(Reason::BadReply | Reason::NotFound)
            ))
    {
        resolution = VariantResolution::Rejected;
        rejection = Text::new("clinvar_version_unavailable").ok();
        source.notices.push(guidance(format!(
            "ClinVar did not serve {vcv}. Use VCV{:09} to request the current archive.",
            vcv.get()
        )));
    } else if error.is_some() {
        resolution = VariantResolution::Unavailable;
    }
    let mut sources = vec![source];
    let mut local = error.and_then(status::local);
    let mut incomplete = dropped;
    if !candidates.is_empty() {
        let (more, issues, section_error, section_incomplete) =
            super::sections::apply(cx, query, &mut candidates, deadline, None);
        sources.extend(more);
        problems.extend(issues);
        local = local.or(section_error);
        incomplete |= section_incomplete;
    }
    let next = super::next(
        cx.fetch,
        query,
        resolution,
        &candidates,
        None,
        &mut problems,
    );
    status::finish(
        [super::overall(resolution, incomplete, &sources)],
        status::Parts {
            body: super::reply(
                query,
                VariantCard {
                    resolution,
                    source: Source::ClinVar,
                    candidates: candidates.into(),
                    rejection,
                },
                &sources,
            ),
            page: None,
            sources,
            problems,
            next,
            local,
            dropped,
        },
    )
}

fn candidate(query: &VariantGet, archive: clinvar::Assertions) -> Option<VariantHeader> {
    let vcv: VcvId = archive.archive?;
    let id: VariantId = format!("clinvar:{}", vcv.get()).parse().ok()?;
    let mut candidate = super::direct::candidate(query, Source::ClinVar);
    candidate.record.id = Text::new(&id.to_string()).ok()?;
    candidate.identity.ids = Box::new([id.clone(), VariantId::Vcv(vcv)]);
    let page = VariantAssertionsGet::from_identity(
        query
            .input()
            .identity()
            .cloned()
            .unwrap_or_else(|| id.clone()),
        query.assertion_limit(AssertionSource::Clinvar),
        query.section_offset(),
        query.clinvar_kind(),
    )
    .ok()?;
    let mut data = super::assertions::page(&page, archive.rows, archive.included_only);
    if let Some(assertion) = query.input().clinvar_assertion() {
        data.next = data.next.and_then(|next| {
            let text = assertion.to_string();
            let (record, version) = text
                .split_once('.')
                .map_or((text.as_str(), None), |(id, v)| (id, Text::new(v).ok()));
            VariantAssertionsGet::from_identity(id, next.limit(), next.offset(), None)
                .ok()?
                .select(Text::new(record).ok()?, version)
                .ok()
                .map(|selected| biomcp_types::Paged::at_offset(&selected, next.offset()))
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
    Some(candidate)
}

fn guidance(message: impl Into<Box<str>>) -> SourceNotice {
    SourceNotice::RequestGuidance {
        data_date: None,
        message: message.into(),
        retry_after_seconds: None,
    }
}
