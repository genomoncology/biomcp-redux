//! Checked caller IDs can select one source without making an allele-identity claim.
use crate::{Ctx, Outcome};
use biodata::Variant;
use biomcp_fetch::Deadline;
use biomcp_types::{
    QueryBody, Source, Status, VariantCard, VariantGet, VariantHeader, VariantNamespace,
    VariantResolution, VariantSection,
};
pub(super) fn source(cx: &Ctx<'_>, query: &VariantGet) -> Option<Source> {
    let [section] = query.sections() else {
        return None;
    };
    use VariantSection as S;
    let source = super::sections::source(*section)?;
    if cx.fetch.disabled(source) {
        return Some(source);
    }
    if *section == S::Oncokb
        && (!cx.fetch.has_key(biomcp_fetch::SecretName::OncokbApiKey)
            || query
                .build()
                .is_some_and(|build| super::oncokb::input_genomic(query, build).is_some()))
    {
        return Some(source);
    }
    if *section == S::AlphaGenome
        && !cx
            .fetch
            .has_key(biomcp_fetch::SecretName::AlphaGenomeApiKey)
    {
        return Some(source);
    }
    match (query.input().namespace().ok()?, section) {
        (VariantNamespace::Ca, S::Civic | S::Erepo) | (VariantNamespace::ClinVar, S::Clinvar) => {
            Some(source)
        }
        (VariantNamespace::Rs, S::Gwas) => Some(source),
        _ => None,
    }
}
pub(super) fn get(
    cx: &Ctx<'_>,
    query: &VariantGet,
    source: Source,
    mut deadline: Deadline<'_>,
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
        record: biomcp_types::SourceRecord {
            namespace: biodata::IdentifierNamespace::Unknown,
            id: query.input().text().clone(),
            url: None,
        },
        transcripts: Box::default(),
        myvariant: None,
        sections: None,
        component_terms: Box::default(),
    }];
    let (sources, problems, local) =
        super::sections::apply(cx, query, &mut candidates, &mut deadline, None);
    let status = if sources.is_empty()
        || sources
            .iter()
            .all(|s| s.state == biomcp_types::SourceState::Failed)
    {
        Status::Failed
    } else if sources.iter().any(|s| {
        matches!(
            s.state,
            biomcp_types::SourceState::Failed | biomcp_types::SourceState::Stale
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
        sources: sources.clone(),
        problems,
        next: {
            let mut next = next;
            next.extend(super::oncokb::next(query, &sources));
            next.extend(super::alphagenome::next(&sources));
            next
        },
        local,
    }
}
