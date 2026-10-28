//! Spell each provider's typed continuation without changing source identity or page selectors.
use crate::Action;

use biomcp_types::{
    Request, Section, Source, StepKind, VariantGet, VariantHeader, VariantResolution,
};
pub(super) fn collect(candidates: &[VariantHeader], next: &mut Vec<(StepKind, Action)>) {
    for candidate in candidates {
        let Some(sections) = candidate.sections.as_deref() else {
            continue;
        };
        for section in [&sections.clinvar, &sections.erepo].into_iter().flatten() {
            if let Section::Data { data, .. } = section
                && let Some(query) = &data.next
            {
                push(
                    next,
                    StepKind::Page,
                    Request::VariantAssertions(query.clone()),
                );
            }
        }
        if let Some(query) = data(&sections.gwas).and_then(|gwas| gwas.continuation.as_ref()) {
            push(next, StepKind::Page, Request::GetVariant(query.clone()));
        }
        for query in [
            data(&sections.civic).and_then(|d| d.next.as_ref()),
            data(&sections.hotspots).and_then(|d| d.next.as_ref()),
        ]
        .into_iter()
        .flatten()
        {
            push(next, StepKind::Page, Request::GetVariant(query.clone()));
        }
        if let Some(civic) = data(&sections.civic) {
            for profile in &civic.profiles {
                for query in [&profile.evidence_page.next, &profile.assertion_page.next]
                    .into_iter()
                    .flatten()
                {
                    push(next, StepKind::Page, Request::GetVariant(query.clone()));
                }
            }
        }
        if let Some(alphagenome) = data(&sections.alphagenome) {
            for query in alphagenome
                .scorers
                .iter()
                .filter_map(|scorer| scorer.next.as_ref())
            {
                push(next, StepKind::Page, Request::GetVariant(query.clone()));
            }
        }
    }
}
fn data<T>(section: &Option<Section<T>>) -> Option<&T> {
    match section.as_ref()? {
        Section::Data { data, .. } => Some(data),
        _ => None,
    }
}
fn push(next: &mut Vec<(StepKind, Action)>, kind: StepKind, request: Request) {
    let action = Action::Run {
        request,
        fresh: false,
    };
    if !next.iter().any(|(_, old)| old == &action) {
        next.push((kind, action));
    }
}

pub(super) fn next(
    query: &VariantGet,
    resolution: VariantResolution,
    candidates: &[VariantHeader],
    page: Option<&biomcp_types::PageInfo>,
) -> Vec<(StepKind, Action)> {
    let mut next = Vec::new();
    if resolution == VariantResolution::Candidates
        || resolution == VariantResolution::Resolved
            && candidates.iter().any(|row| {
                row.source == Source::MyVariant && row.record.id.as_str() != query.input().as_str()
            })
    {
        for row in candidates {
            if let Ok(input) = row.record.id.as_str().parse()
                && let Ok(q) = VariantGet::new(
                    input,
                    query.resolver(),
                    row.identity
                        .representations
                        .iter()
                        .find_map(|r| r.build)
                        .or(query.build()),
                )
            {
                next.push((
                    StepKind::Disambiguate,
                    Action::Run {
                        request: Request::GetVariant(q),
                        fresh: false,
                    },
                ));
            }
        }
    }
    if let Some(page) = page
        && (page.offset as u64 + page.returned as u64) < page.total
    {
        next.push((
            StepKind::Page,
            Action::Run {
                request: Request::GetVariant(query.at_offset(page.offset + page.returned)),
                fresh: false,
            },
        ));
    }
    if query.sections().is_empty() && !candidates.is_empty() {
        next.push((
            StepKind::Section,
            Action::Run {
                request: Request::GetVariant(
                    query
                        .clone()
                        .with_sections(vec![biomcp_types::VariantSection::Clinvar].into()),
                ),
                fresh: false,
            },
        ));
    }
    collect(candidates, &mut next);
    if query.input().namespace().ok() == Some(biomcp_types::VariantNamespace::Rs) {
        for gene in candidates.iter().flat_map(|row| &row.identity.genes) {
            push(&mut next, StepKind::Pivot, Request::SearchArticle(
                biomcp_types::ArticleSearch::for_gene(biomcp_types::GeneInput::Text(gene.label().clone())),
            ));
        }
    }
    next
}
