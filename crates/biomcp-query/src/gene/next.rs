use crate::{Action, Bundle};

use biomcp_types::{
    GeneGet, GeneItem, GeneSearch, PageInfo, Request, SourceOutcome, SourceState, StepKind,
};
use biomcp_types::{GeneHeader, GeneResolution};
fn retry(request: Request) -> (StepKind, Action) {
    (
        StepKind::Fresh,
        Action::Run {
            request,
            fresh: true,
        },
    )
}
fn sync() -> (StepKind, Action) {
    (
        StepKind::Sync,
        Action::Sync {
            bundle: Bundle::Hgnc,
        },
    )
}
fn get_id(id: biodata::HgncId) -> Option<(StepKind, Action)> {
    let query = GeneGet::new(vec![biomcp_types::GeneInput::HgncId(id)].into()).ok()?;
    Some((
        StepKind::Disambiguate,
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
) -> Vec<(StepKind, Action)> {
    let mut steps = Vec::new();
    if sources.iter().any(|source| {
        matches!(
            source.state,
            SourceState::Stale | SourceState::Failed | SourceState::Syncing
        )
    }) {
        steps.push(retry(Request::GetGene(query.clone())));
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
            steps.push((StepKind::Sync, Action::Sync { bundle }));
        }
    }
    if sources.iter().any(|source| {
        source.source == biomcp_types::Source::Oncokb
            && source.state == SourceState::Skipped
            && source.reason == Some(biomcp_types::Reason::NoKey)
    }) {
        steps.push((StepKind::Key, Action::KeyHelp(biomcp_types::Source::Oncokb)));
    }
    for item in items {
        if let Some(GeneResolution::Ambiguous { candidates }) = &item.answer {
            for candidate in candidates {
                if let Some(step) = get_id(candidate.hgnc_id)
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
                .filter_map(|section| section.to_string().parse().ok())
                .collect();
            if let Ok(query) = GeneGet::with_sections(
                vec![biomcp_types::GeneInput::HgncId(gene.hgnc_id)].into(),
                words,
            ) {
                let step = (
                    StepKind::Section,
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
) -> Vec<(StepKind, Action)> {
    let mut steps = Vec::new();
    if matches!(
        source.state,
        SourceState::Stale | SourceState::Failed | SourceState::Syncing
    ) {
        steps.push(retry(Request::SearchGene(query.clone())));
        steps.push(sync());
    }
    if let Some(offset) = page.offset.checked_add(page.returned)
        && u64::from(offset) < page.total
    {
        steps.push((
            StepKind::Page,
            Action::Run {
                request: Request::SearchGene(query.at_offset(offset)),
                fresh: false,
            },
        ));
    }
    if let Some(header) = items.first()
        && let Some(step) = get_id(header.hgnc_id)
    {
        steps.push(step);
    }
    steps
}
