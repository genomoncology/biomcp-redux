//! Checked caller IDs can select one source without making an allele-identity claim.
use crate::{Ctx, Outcome};
use biodata::Variant;
use biomcp_fetch::Deadline;
use biomcp_types::{
    Source, VariantCard, VariantGet, VariantHeader, VariantNamespace, VariantResolution,
    VariantSection,
};
pub(super) fn source(fetch: &biomcp_fetch::Fetch, query: &VariantGet) -> Option<Source> {
    let [section] = query.sections() else {
        return None;
    };
    use VariantSection as S;
    let source = super::sections::source(*section)?;
    if fetch.disabled(source) {
        return Some(source);
    }
    if *section == S::Oncokb
        && (!fetch.has_key(biomcp_fetch::SecretName::OncokbApiKey)
            || query
                .build()
                .is_some_and(|build| super::oncokb::input_genomic(query, build).is_some()))
    {
        return Some(source);
    }
    if *section == S::AlphaGenome && !fetch.has_key(biomcp_fetch::SecretName::AlphaGenomeApiKey) {
        return Some(source);
    }
    match (query.input().namespace(), section) {
        (_, S::Population) if super::population_identity::use_native_coordinate(fetch, query) => {
            Some(source)
        }
        (VariantNamespace::Ca, S::Civic | S::Erepo) | (VariantNamespace::ClinVar, S::Clinvar) => {
            Some(source)
        }
        (VariantNamespace::CivicProfile, S::Civic) => Some(source),
        (VariantNamespace::Rs, S::Gwas) => Some(source),
        _ => None,
    }
}
/// Use the execution router before adding any identity-provider requirement.
pub(crate) fn requirements(fetch: &biomcp_fetch::Fetch, query: &VariantGet) -> Vec<Source> {
    if query.input().clinvar_archive_lookup() {
        return std::iter::once(Source::ClinVar)
            .chain(
                super::sections::selected(query)
                    .filter(|section| *section != VariantSection::Clinvar)
                    .map(|section| super::sections::source(section).unwrap_or(Source::MyVariant)),
            )
            .collect();
    }
    if let Some(source) = source(fetch, query) {
        return vec![source];
    }
    let mut sources = vec![super::source(query).source];
    // Prediction, conservation and COSMIC use the MyVariant component path.
    sources.extend(
        super::sections::selected(query)
            .map(|section| super::sections::source(section).unwrap_or(Source::MyVariant)),
    );
    sources
}

pub(super) fn get(
    cx: &Ctx<'_>,
    query: &VariantGet,
    source: Source,
    mut deadline: Deadline<'_>,
) -> Outcome {
    let mut candidates = vec![candidate(query, source)];
    let (sources, mut problems, local, incomplete) =
        super::sections::apply(cx, query, &mut candidates, &mut deadline, None);
    let status = status(&sources, incomplete);
    let next = super::next(
        cx.fetch,
        query,
        VariantResolution::Unresolved,
        &candidates,
        None,
        &mut problems,
    );
    crate::status::finish(
        [status],
        crate::status::Parts {
            dropped: false,
            body: super::reply(
                query,
                VariantCard {
                    resolution: VariantResolution::Unresolved,
                    source,
                    candidates: candidates.into(),
                    rejection: None,
                },
                &sources,
            ),
            page: None,
            sources: sources.clone(),
            problems,
            next: {
                let mut next = next;
                next.extend(super::oncokb::next(query, &sources));
                next.extend(super::alphagenome::next(&sources));
                next
            },
            local,
        },
    )
}

pub(super) fn candidate(query: &VariantGet, source: Source) -> VariantHeader {
    VariantHeader {
        identity: Variant {
            ids: Box::default(),
            representations: Box::default(),
            genes: Box::default(),
        },
        label: None,
        requested: query.input().identity().cloned(),
        source,
        record: biomcp_types::SourceRecord {
            namespace: biodata::IdentifierNamespace::Unknown,
            id: query.input().text().clone(),
            url: None,
        },
        transcripts: Box::default(),
        myvariant: None,
        sections: None,
        component_terms: Box::default(),
    }
}

fn status(sources: &[biomcp_types::SourceOutcome], incomplete: bool) -> biomcp_types::Status {
    let civic_join_missing = sources.iter().any(|source| {
        source.source == Source::Civic && source.reason == Some(biomcp_types::Reason::NoJoinId)
    });
    if !civic_join_missing
        && (sources.is_empty()
            || sources
                .iter()
                .all(|s| s.state == biomcp_types::SourceState::Failed))
    {
        crate::status::item_status(false, true, false)
    } else if sources.iter().any(|s| {
        matches!(
            s.state,
            biomcp_types::SourceState::Failed | biomcp_types::SourceState::Stale
        )
    }) || incomplete
    {
        crate::status::item_status(true, false, true)
    } else {
        crate::status::item_status(true, false, false)
    }
}
