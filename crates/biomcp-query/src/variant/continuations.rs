//! Spell each provider's typed continuation without changing source identity or page selectors.
use crate::Action;

use biomcp_types::{
    Request, Section, Source, StepKind, VariantGet, VariantHeader, VariantResolution,
};
pub(super) fn collect(candidates: &[VariantHeader], next: &mut Vec<crate::next::Step>) {
    for candidate in candidates {
        let Some(sections) = candidate.sections.as_deref() else {
            continue;
        };
        for (source, section) in [
            (Source::ClinVar, &sections.clinvar),
            (Source::Erepo, &sections.erepo),
        ] {
            let Some(section) = section else { continue };
            if let Section::Data { data, .. } = section
                && let Some(query) = &data.next
            {
                push(
                    next,
                    StepKind::Page,
                    source,
                    Request::VariantAssertions(query.clone()),
                );
            }
        }
        if let Some(query) =
            data(sections.gwas.as_ref()).and_then(|gwas| gwas.continuation.as_ref())
        {
            push(
                next,
                StepKind::Page,
                Source::Gwas,
                Request::GetVariant(query.clone()),
            );
        }
        for (source, query) in [
            (
                Source::Civic,
                data(sections.civic.as_ref()).and_then(|d| d.next.as_ref()),
            ),
            (
                Source::CancerHotspots,
                data(sections.hotspots.as_ref()).and_then(|d| d.next.as_ref()),
            ),
        ] {
            let Some(query) = query else { continue };
            push(
                next,
                StepKind::Page,
                source,
                Request::GetVariant(query.clone()),
            );
        }
        if let Some(civic) = data(sections.civic.as_ref()) {
            for profile in &civic.profiles {
                for query in [&profile.evidence_page.next, &profile.assertion_page.next]
                    .into_iter()
                    .flatten()
                {
                    push(
                        next,
                        StepKind::Page,
                        Source::Civic,
                        Request::GetVariant(query.clone()),
                    );
                }
            }
        }
        if let Some(alphagenome) = data(sections.alphagenome.as_ref()) {
            for query in alphagenome
                .scorers
                .iter()
                .filter_map(|scorer| scorer.next.as_ref())
            {
                push(
                    next,
                    StepKind::Page,
                    Source::AlphaGenome,
                    Request::GetVariant(query.clone()),
                );
            }
        }
    }
}
fn data<T>(section: Option<&Section<T>>) -> Option<&T> {
    match section? {
        Section::Data { data, .. } => Some(data),
        _ => None,
    }
}
fn push(next: &mut Vec<crate::next::Step>, kind: StepKind, source: Source, request: Request) {
    let action = Action::Run {
        request,
        fresh: false,
    };
    if !next.iter().any(|step| step.1 == action) {
        next.push(crate::next::Step::new(kind, &[source], action));
    }
}

pub(super) fn next(
    query: &VariantGet,
    resolution: VariantResolution,
    candidates: &[VariantHeader],
    page: Option<&biomcp_types::PageInfo>,
    problems: &mut Vec<biomcp_types::Problem>,
) -> Vec<crate::next::Step> {
    let mut next = Vec::new();
    if resolution == VariantResolution::Candidates
        || resolution == VariantResolution::Resolved
            && candidates.iter().any(|row| {
                row.source == Source::MyVariant && row.record.id.as_str() != query.input().as_str()
                    || row
                        .sections
                        .as_ref()
                        .and_then(|s| s.population.as_ref())
                        .is_some_and(|s| {
                            matches!(
                                s,
                                Section::Unavailable {
                                    reason: biomcp_types::Reason::NoJoinId,
                                    ..
                                }
                            )
                        })
            })
    {
        for row in candidates {
            if let Some((request, source)) = candidate_request(query, resolution, row, problems) {
                push(&mut next, StepKind::Disambiguate, source, request);
            }
        }
    }
    if let Some(page) = page
        && (page.offset as u64 + page.returned as u64) < page.total
    {
        next.push(crate::next::Step::new(
            StepKind::Page,
            &[super::source(query).source],
            Action::Run {
                request: Request::GetVariant(query.at_offset(page.offset + page.returned)),
                fresh: false,
            },
        ));
    }
    if query.sections().is_empty() && !candidates.is_empty() {
        next.push(crate::next::Step::new(
            StepKind::Section,
            &[Source::ClinVar],
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
            push(
                &mut next,
                StepKind::Pivot,
                Source::PubMed,
                Request::SearchArticle(biomcp_types::ArticleSearch::for_gene(
                    biomcp_types::GeneInput::Text(gene.label().clone()),
                )),
            );
        }
    }
    next
}

fn candidate_request(
    query: &VariantGet,
    resolution: VariantResolution,
    row: &VariantHeader,
    problems: &mut Vec<biomcp_types::Problem>,
) -> Option<(Request, Source)> {
    if super::population_identity::requested(query) {
        let (request, source) = super::population_identity::choice(row, problems)?;
        return Some((
            match request {
                Request::GetVariant(q) if resolution == VariantResolution::Candidates => {
                    Request::GetVariant(q.with_sections(query.sections().into()))
                }
                request => request,
            },
            source,
        ));
    }
    let input = crate::next::checked(row.record.id.as_str().parse(), row.source, problems)?;
    let query = crate::next::checked(
        VariantGet::new(
            input,
            query.resolver(),
            row.identity
                .representations
                .iter()
                .find_map(|r| r.build)
                .or(query.build()),
        ),
        row.source,
        problems,
    )?;
    let source = super::source(&query).source;
    Some((Request::GetVariant(query), source))
}
