//! Carry active request restrictions through every generated command.
use std::io;
pub(super) fn next(
    result: &biomcp_query::Outcome,
    globals: &crate::grammar::Globals,
) -> io::Result<Vec<biomcp_types::Step>> {
    let mut steps = result
        .next
        .iter()
        .map(|(kind, action)| crate::spell::step(*kind, action))
        .collect::<Result<Vec<_>, _>>()?;
    let has_more = result
        .page
        .as_ref()
        .is_some_and(|page| u64::from(page.offset) + u64::from(page.returned) < page.total);
    match navigation(&result.body) {
        Navigation::NativeOrder => append_network_steps(&mut steps, &result.sources),
        Navigation::BoundedPma => {
            append_network_steps(&mut steps, &result.sources);
            steps.truncate(5);
        }
        Navigation::Bounded => network_steps(&mut steps, &result.sources, None),
        Navigation::Selectable => network_steps(&mut steps, &result.sources, Some(has_more)),
        Navigation::Nested(page) => network_steps(
            &mut steps,
            &result.sources,
            Some(has_more || u64::from(page.offset) + u64::from(page.returned) < page.total),
        ),
    }
    Ok(source_steps(globals, steps))
}

#[derive(Debug)]
enum Navigation<'a> {
    NativeOrder,
    Bounded,
    Selectable,
    Nested(&'a biomcp_types::PgxPageInfo),
    // PMA cards retain section-first order even when they contain a pivot.
    BoundedPma,
}

fn navigation(body: &biomcp_types::QueryBody) -> Navigation<'_> {
    use biomcp_types::QueryBody as Body;
    match body {
        Body::OrganizationPage(_) | Body::AuthorPage(_) | Body::CellLinePage(_) => {
            Navigation::NativeOrder
        }
        Body::AuthorCards(_) => Navigation::BoundedPma,
        Body::CellLineChembl(_) | Body::CellLineAssayCount(_) => Navigation::BoundedPma,
        Body::CellLineCard(_) => Navigation::BoundedPma,
        Body::GwasAssociation(_)
        | Body::GwasStudy(_)
        | Body::GwasLoci(_)
        | Body::GwasAncestries(_)
        | Body::GwasPage(_)
        | Body::PathwayEvents(_)
        | Body::ProteinComplexes(_) => Navigation::NativeOrder,
        Body::PathwayCard(card) => pathway_navigation(card.as_ref()),
        Body::ProteinPage(_) => Navigation::Selectable,
        Body::ProteinFeatures(_)
        | Body::ProteinIsoforms(_)
        | Body::ProteinDomains(_)
        | Body::ProteinComplexMembers(_)
        | Body::ProteinStructures(_)
        | Body::ProteinSequence(_) => Navigation::Bounded,
        Body::ProteinCard(card) => protein_navigation(card),
        Body::StudyProfiles(_) | Body::StudySampleLists(_) | Body::StudyFiles(_) => {
            Navigation::Bounded
        }
        Body::PathwayPage(_) | Body::PhenotypePage(_) => Navigation::Selectable,
        Body::PhenotypeCard(_)
        | Body::PhenotypeSynonyms(_)
        | Body::PhenotypeXrefs(_)
        | Body::PhenotypeHierarchy(_) => Navigation::Bounded,
        Body::PathwayGenes(_) => Navigation::Bounded,
        Body::PgxAnnotations(value) => pgx_navigation(value),
        Body::VariantCard(card) => variant_navigation(card),
        Body::DiagnosticCard(Some(biomcp_types::DiagnosticCard::PmaSubmission(_))) => {
            Navigation::BoundedPma
        }
        _ => bounded_navigation(body),
    }
}

fn candidate_navigation(candidates: bool) -> Navigation<'static> {
    if candidates {
        Navigation::Selectable
    } else {
        Navigation::Bounded
    }
}

fn source_steps(
    globals: &crate::grammar::Globals,
    steps: Vec<biomcp_types::Step>,
) -> Vec<biomcp_types::Step> {
    let mut prefix: Vec<Box<str>> = Vec::new();
    if let Some(path) = &globals.config {
        prefix.extend([
            "--config".into(),
            path.to_string_lossy().into_owned().into(),
        ]);
    }
    if !globals.disable_sources.is_empty() {
        prefix.extend([
            "--disable-sources".into(),
            globals.disable_sources.join(",").into(),
        ]);
    }
    if prefix.is_empty() {
        return steps;
    }
    steps
        .into_iter()
        .map(|mut step| {
            let mut args = step.args.into_vec();
            args.splice(0..0, prefix.iter().cloned());
            step.args = args.into();
            step
        })
        .collect()
}

pub(super) fn network_steps(
    steps: &mut Vec<biomcp_types::Step>,
    sources: &[biomcp_types::SourceOutcome],
    candidate_page: Option<bool>,
) {
    let association = steps
        .iter()
        .any(|step| step.kind == biomcp_types::StepKind::Pivot);
    if candidate_page.is_none() && !association {
        append_network_steps(steps, sources);
        steps.truncate(5);
        return;
    }
    // Some disambiguate actions are searches or relations, not candidate gets.
    let (mut candidates, mut extras): (Vec<_>, Vec<_>) =
        std::mem::take(steps).into_iter().partition(|step| {
            step.kind == biomcp_types::StepKind::Pivot
                || step.kind == biomcp_types::StepKind::Disambiguate
                    && step.args.first().is_some_and(|arg| arg.as_ref() == "get")
        });
    if candidates.is_empty() {
        append_network_steps(&mut extras, sources);
        extras.truncate(5);
        *steps = extras;
        return;
    }
    // A section may itself have page actions. Only the response page is protected.
    if (candidate_page == Some(true) || association)
        && let Some(index) = extras
            .iter()
            .position(|step| step.kind == biomcp_types::StepKind::Page)
    {
        candidates.push(extras.remove(index));
    }
    append_network_steps(&mut extras, sources);
    extras.truncate(3);
    candidates.extend(extras);
    *steps = candidates;
}
fn append_network_steps(
    steps: &mut Vec<biomcp_types::Step>,
    sources: &[biomcp_types::SourceOutcome],
) {
    for (reason, setting) in [
        (biomcp_types::Reason::Tls, "ca_bundle"),
        (biomcp_types::Reason::Proxy, "proxy"),
    ] {
        if sources.iter().any(|source| source.reason == Some(reason)) {
            steps.insert(
                0,
                biomcp_types::Step {
                    kind: biomcp_types::StepKind::Configure,
                    args: vec!["help".into(), "network".into(), setting.into()].into(),
                },
            );
        }
    }
}

fn pathway_navigation(card: Option<&biomcp_types::PathwayCard>) -> Navigation<'_> {
    if card.is_some_and(|card| card.events.is_some() && card.genes.is_none()) {
        Navigation::NativeOrder
    } else {
        Navigation::Bounded
    }
}
fn protein_navigation(card: &biomcp_types::ProteinCard) -> Navigation<'_> {
    if card.items.iter().any(|item| {
        item.sections
            .as_ref()
            .is_some_and(|s| s.domains.is_some() || s.complexes.is_some())
    }) {
        return Navigation::NativeOrder;
    }
    candidate_navigation(card.items.iter().any(|item| !item.candidates.is_empty()))
}

fn variant_navigation(card: &biomcp_types::VariantCard) -> Navigation<'static> {
    candidate_navigation(card.resolution == biomcp_types::VariantResolution::Candidates)
}

fn pgx_navigation(value: &biomcp_types::PgxAnnotations) -> Navigation<'_> {
    match value {
        biomcp_types::PgxAnnotations::Clinical { page, .. } => Navigation::Nested(page),
        biomcp_types::PgxAnnotations::Guideline { .. }
        | biomcp_types::PgxAnnotations::Label { .. } => Navigation::Bounded,
    }
}

fn bounded_navigation(body: &biomcp_types::QueryBody) -> Navigation<'_> {
    use biomcp_types::QueryBody as Body;
    match body {
        Body::StudyPage(_)
        | Body::DiagnosticCompanion(_)
        | Body::DiagnosticPage(_)
        | Body::DiagnosticGenes(_)
        | Body::DiagnosticSupplements(_)
        | Body::AdverseEventPage(_)
        | Body::DiseasePage(_)
        | Body::PhenotypeDiseases(_)
        | Body::PhenotypeGenes(_)
        | Body::DiseaseGenes(_)
        | Body::DiseasePhenotypes(_)
        | Body::DrugPage(_)
        | Body::DrugWhoPage(_)
        | Body::DrugFdaPage(_)
        | Body::DrugCivic(_)
        | Body::VariantPage(_) => Navigation::Selectable,
        Body::StudyCard(_) | Body::PgxPage(_) | Body::PgxCard(_) => Navigation::Bounded,
        Body::PgxRecommendations(_) | Body::PgxGuidelines(_) => Navigation::Bounded,
        Body::PgxDiplotype(_) => Navigation::Bounded,
        Body::PgxAnnotation(_) | Body::DiagnosticCard(_) => Navigation::Bounded,
        Body::DiagnosticConditions(_) | Body::DiagnosticMethods(_) => Navigation::Bounded,
        Body::DiagnosticRegulatory(_) | Body::UnsupportedAdverseEvent => Navigation::Bounded,
        Body::AdverseEventFaersCard(_) | Body::AdverseEventDrugs(_) => Navigation::Bounded,
        Body::AdverseEventReactions(_) => Navigation::Bounded,
        Body::AdverseEventIdentifiers(_) | Body::VaersSummary(_) => Navigation::Bounded,
        Body::AdverseEventMaudeCard(_) | Body::AdverseEventDevices(_) => Navigation::Bounded,
        Body::AdverseEventNarrative(_) | Body::DiseaseCard(_) => Navigation::Bounded,
        Body::DiseaseSynonyms(_) | Body::DiseaseXrefs(_) => Navigation::Bounded,
        Body::DiseaseHierarchy(_) | Body::DiseaseSites(_) => Navigation::Bounded,
        Body::DiseaseIncidence(_) | Body::DiseaseSurvival(_) => Navigation::Bounded,
        Body::DrugCard(_) | Body::DrugLabel(_) | Body::DrugWhoCard(_) => Navigation::Bounded,
        Body::DrugRegulatory(_) | Body::DrugApprovals(_) => Navigation::Bounded,
        Body::DrugInteractions(_) | Body::DrugXrefs(_) => Navigation::Bounded,
        Body::DrugTargets(_) | Body::DrugIndications(_) => Navigation::Bounded,
        Body::VariantAssertions(_) | Body::VariantNormalization(_) => Navigation::Bounded,
        Body::NciTrialPage(_) | Body::NciTrialCards(_) => Navigation::Bounded,
        Body::OrganizationTooBroad
        | Body::UnsupportedTrialFilter { .. }
        | Body::NciDiseaseError => Navigation::Bounded,
        Body::VariantQueryPolicyError | Body::VariantPopulation(_) => Navigation::Bounded,
        Body::VariantRecurrence(_) | Body::TrialDocument(_) => Navigation::Bounded,
        Body::TrialCards(_) | Body::TrialPage(_) | Body::ArticlePdf(_) => Navigation::Bounded,
        Body::PdfSaveRefused(_) | Body::ArticleSaveRefused(_) => Navigation::Bounded,
        Body::ArticleEvidence(_) | Body::ArticleRecommendations(_) => Navigation::Bounded,
        Body::ArticleSupplement(_) | Body::UnsupportedFilter { .. } => Navigation::Bounded,
        Body::ArticleText(_) | Body::TextPositionError => Navigation::Bounded,
        Body::ArticleEdges(_) | Body::TooManyPmids { .. } => Navigation::Bounded,
        Body::ArticleCards(_) | Body::PoolError { .. } => Navigation::Bounded,
        Body::ArticlePage(_) | Body::GeneCards(_) | Body::GenePage(_) => Navigation::Bounded,
        _ => Navigation::Bounded,
    }
}
