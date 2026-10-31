use crate::{Action, Bundle};

use biomcp_types::{
    GeneGet, GeneItem, GeneSearch, PageInfo, Request, SourceOutcome, SourceState, StepKind,
};
use biomcp_types::{GeneHeader, GeneResolution};
fn retry(sources: &[biomcp_types::Source], request: Request) -> crate::next::Step {
    crate::next::Step::new(
        StepKind::Fresh,
        sources,
        Action::Run {
            request,
            fresh: true,
        },
    )
}
fn sync() -> crate::next::Step {
    crate::next::Step::new(
        StepKind::Sync,
        &[biomcp_types::Source::Hgnc],
        Action::Sync {
            bundle: Bundle::Hgnc,
        },
    )
}
fn get_id(
    id: biodata::HgncId,
    problems: &mut Vec<biomcp_types::Problem>,
) -> Option<crate::next::Step> {
    let query = crate::next::checked(
        GeneGet::new(vec![biomcp_types::GeneInput::HgncId(id)].into()),
        biomcp_types::Source::Hgnc,
        problems,
    )?;
    Some(crate::next::Step::new(
        StepKind::Disambiguate,
        &[biomcp_types::Source::Hgnc],
        Action::Run {
            request: Request::GetGene(query),
            fresh: false,
        },
    ))
}
pub(crate) fn get(
    query: &GeneGet,
    items: &[GeneItem],
    sources: &[SourceOutcome],
    problems: &mut Vec<biomcp_types::Problem>,
) -> Vec<crate::next::Step> {
    let mut steps = Vec::new();
    if sources.iter().any(|source| {
        matches!(
            source.state,
            SourceState::Stale | SourceState::Failed | SourceState::Syncing
        )
    }) {
        steps.push(retry(&get_sources(query), Request::GetGene(query.clone())));
    }
    if sources.iter().any(|source| {
        source.source == biomcp_types::Source::Hgnc
            && matches!(
                source.state,
                SourceState::Stale | SourceState::Failed | SourceState::Syncing
            )
    }) {
        steps.push(sync());
    }
    for (source, bundle) in [
        (biomcp_types::Source::ClinGen, Bundle::ClinGen),
        (biomcp_types::Source::Gencc, Bundle::Gencc),
    ] {
        if sources
            .iter()
            .any(|outcome| outcome.source == source && outcome.state != SourceState::Ok)
        {
            steps.push(crate::next::Step::new(
                StepKind::Sync,
                &[source],
                Action::Sync { bundle },
            ));
        }
    }
    if sources.iter().any(|source| {
        source.source == biomcp_types::Source::Oncokb
            && source.state == SourceState::Skipped
            && source.reason == Some(biomcp_types::Reason::NoKey)
    }) {
        steps.push(crate::next::Step::new(
            StepKind::Key,
            &[biomcp_types::Source::Oncokb],
            Action::KeyHelp(biomcp_types::Source::Oncokb),
        ));
    }
    for item in items {
        if let Some(GeneResolution::Ambiguous { candidates }) = &item.answer {
            for candidate in candidates {
                if let Some(step) = get_id(candidate.hgnc_id, problems)
                    && !steps.contains(&step)
                    && steps.len() < 5
                {
                    steps.push(step);
                }
            }
        }
    }
    for item in items {
        if !item.more.is_empty()
            && let Some(GeneResolution::Resolved { gene, .. }) = &item.answer
        {
            let words = item
                .more
                .iter()
                .filter_map(|section| {
                    crate::next::checked(
                        section.to_string().parse(),
                        biomcp_types::Source::Hgnc,
                        problems,
                    )
                })
                .collect();
            if let Some(query) = crate::next::checked(
                GeneGet::with_sections(
                    vec![biomcp_types::GeneInput::HgncId(gene.hgnc_id)].into(),
                    words,
                ),
                biomcp_types::Source::Hgnc,
                problems,
            ) {
                let step = crate::next::Step::new(
                    StepKind::Section,
                    &get_sources(&query),
                    Action::Run {
                        request: Request::GetGene(query),
                        fresh: false,
                    },
                );
                if steps.len() < 5 && !steps.contains(&step) {
                    steps.push(step);
                }
            }
        }
    }
    steps
}
pub(crate) fn search(
    query: &GeneSearch,
    items: &[GeneHeader],
    page: &PageInfo,
    source: &SourceOutcome,
    problems: &mut Vec<biomcp_types::Problem>,
) -> Vec<crate::next::Step> {
    let mut steps = Vec::new();
    if matches!(
        source.state,
        SourceState::Stale | SourceState::Failed | SourceState::Syncing
    ) {
        steps.push(retry(
            &[biomcp_types::Source::Hgnc],
            Request::SearchGene(query.clone()),
        ));
        steps.push(sync());
    }
    if let Some(offset) = page.offset.checked_add(page.returned)
        && u64::from(offset) < page.total
    {
        steps.push(crate::next::Step::new(
            StepKind::Page,
            &[biomcp_types::Source::Hgnc],
            Action::Run {
                request: Request::SearchGene(query.at_offset(offset)),
                fresh: false,
            },
        ));
    }
    if let Some(header) = items.first()
        && let Some(step) = get_id(header.hgnc_id, problems)
    {
        steps.push(step);
    }
    steps
}

fn get_sources(query: &GeneGet) -> Vec<biomcp_types::Source> {
    use biomcp_types::{GeneSectionWord as Section, Source};
    let mut sources = vec![Source::Hgnc];
    for section in query.sections() {
        match section {
            Section::Protein => sources.push(Source::UniProt),
            Section::Civic => sources.push(Source::Civic),
            Section::ClinGen => sources.push(Source::ClinGen),
            Section::Gencc => sources.push(Source::Gencc),
            Section::Constraint => sources.push(Source::Gnomad),
            Section::Expression => sources.push(Source::Hpa),
            Section::Pathways => sources.push(Source::Reactome),
            Section::Oncokb => sources.push(Source::Oncokb),
            Section::All => sources.extend([
                Source::UniProt,
                Source::Civic,
                Source::ClinGen,
                Source::Gencc,
                Source::Gnomad,
                Source::Hpa,
                Source::Reactome,
                Source::Oncokb,
            ]),
        }
    }
    sources
}
