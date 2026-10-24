//! Checked caller IDs can select one source without making an allele-identity claim.
use crate::{Ctx, Outcome};
use biodata::{
    QueryBody, Source, Status, Text, Variant, VariantCard, VariantGet, VariantHeader,
    VariantNamespace, VariantResolution, VariantSection,
};
use biomcp_fetch::Deadline;
pub(super) fn source(cx: &Ctx<'_>, query: &VariantGet) -> Option<Source> {
    let [section] = query.sections() else {
        return None;
    };
    use VariantSection as S;
    let source = match section {
        S::Clinvar => Source::ClinVar,
        S::Erepo => Source::Erepo,
        S::Gwas => Source::Gwas,
        S::Civic => Source::Civic,
        S::Population => Source::Gnomad,
        S::Hotspots => Source::CancerHotspots,
        S::Oncokb => Source::Oncokb,
        S::Predict | S::Conservation | S::Cosmic => Source::MyVariant,
        _ => return None,
    };
    if cx.fetch.disabled(source) {
        return Some(source);
    }
    if matches!(section, S::Predict | S::Conservation) {
        return Some(source);
    }
    match (query.input().namespace().ok()?, section) {
        (VariantNamespace::Rs, S::Gwas)
        | (VariantNamespace::ClinVar, S::Clinvar)
        | (VariantNamespace::Ca, S::Erepo | S::Civic) => Some(source),
        _ => None,
    }
}
pub(super) fn get(
    cx: &Ctx<'_>,
    query: &VariantGet,
    source: Source,
    deadline: Deadline<'_>,
) -> Outcome {
    let input = query.input().as_str();
    let requested = if query.input().namespace().ok() == Some(VariantNamespace::ClinVar) {
        format!(
            "clinvar:{}",
            input.strip_prefix("clinvar:").unwrap_or(input)
        )
        .parse()
        .ok()
    } else {
        input.parse().ok()
    };
    let mut candidates = vec![VariantHeader {
        identity: Variant {
            ids: Box::default(),
            representations: Box::default(),
            genes: Box::default(),
        },
        requested,
        source,
        record: biodata::SourceRecord {
            id: Text::new(input).unwrap_or_else(|_| unreachable!()),
            url: None,
        },
        myvariant: None,
        sections: None,
    }];
    let (sources, problems, local) = super::sections::apply(cx, query, &mut candidates, deadline);
    let status = if sources.is_empty()
        || sources
            .iter()
            .all(|s| s.state == biodata::SourceState::Failed)
    {
        Status::Failed
    } else if sources.iter().any(|s| {
        matches!(
            s.state,
            biodata::SourceState::Failed | biodata::SourceState::Stale
        )
    }) || !problems.is_empty()
    {
        Status::Partial
    } else {
        Status::Complete
    };
    let next = super::next(query, VariantResolution::Unresolved, &candidates, None);
    Outcome {
        status,
        body: QueryBody::VariantCard(VariantCard {
            resolution: VariantResolution::Unresolved,
            source,
            candidates: candidates.into(),
            rejection: None,
        }),
        page: None,
        sources,
        problems,
        next,
        local,
    }
}
