//! Checked follow-ups, source restrictions and response navigation policy.
use crate::{Action, Outcome};
use biomcp_types::{ReplyData, Source, StepKind};

/// One typed follow-up and the sources it would contact.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Step(
    /// Follow-up role.
    pub StepKind,
    /// Checked action to spell.
    pub Action,
    /// Native providers required by this follow-up.
    pub Box<[Source]>,
);
impl Step {
    /// Retain target identities supplied by the entity that creates the step.
    pub(crate) fn new(kind: StepKind, sources: &[Source], action: Action) -> Self {
        Self(kind, action, sources.into())
    }
    /// Retain a checked query and its cache policy with the declared provider requirements.
    pub(crate) fn run(
        kind: StepKind,
        sources: &[Source],
        request: biomcp_types::Request,
        fresh: bool,
    ) -> Self {
        Self::new(kind, sources, Action::Run { request, fresh })
    }
    fn is_get(&self) -> bool {
        use biomcp_types::Request::*;
        matches!(
            &self.1,
            Action::Run {
                request: GetGene(_)
                    | GetVariant(_)
                    | GetArticle(_)
                    | GetTrial(_)
                    | GetDisease(_)
                    | GetDrug(_)
                    | GetWhoMedicine(_)
                    | GetAdverseEvent(_)
                    | GetDiagnostic(_)
                    | GetPgx(_)
                    | GetPgxAnnotation(_)
                    | GetStudy(_)
                    | GetProtein(_)
                    | GetPathway(_)
                    | GetPhenotype(_)
                    | GetBiomarker(_)
                    | GetAuthor(_)
                    | GetCellLine(_)
                    | GetGwasAssociation(_)
                    | GetGwasStudy(_),
                ..
            }
        )
    }
}

/// Wrap an already checked query without changing its cache policy.
pub(crate) fn run(request: biomcp_types::Request) -> Action {
    Action::Run {
        request,
        fresh: false,
    }
}

/// Keep failed construction visible without exposing rejected field values.
pub(crate) fn checked<T, E>(
    result: Result<T, E>,
    source: Source,
    problems: &mut Vec<biomcp_types::Problem>,
) -> Option<T> {
    match result {
        Ok(value) => Some(value),
        Err(_) => {
            problems.push(biomcp_types::Problem {
                source,
                record: None,
                field: "next".into(),
                code: biomcp_types::ProblemCode::ValueUnknown,
            });
            None
        }
    }
}

/// Apply source exclusions and the shared response ordering and caps.
pub(crate) fn finish(fetch: &biomcp_fetch::Fetch, result: &mut Outcome) {
    result.next.retain(|step| {
        !matches!(step.1, Action::NetworkHelp(_))
            && !step.2.iter().any(|source| fetch.disabled(*source))
    });
    let has_more = result
        .page
        .as_ref()
        .is_some_and(|page| u64::from(page.offset) + u64::from(page.returned) < page.total);
    match navigation(&result.body) {
        Navigation::NativeOrder => append_network_steps(&mut result.next, &result.sources),
        Navigation::BoundedPma => {
            append_network_steps(&mut result.next, &result.sources);
            result.next.truncate(5);
        }
        Navigation::Bounded => network_steps(&mut result.next, &result.sources, None),
        Navigation::Selectable => network_steps(&mut result.next, &result.sources, Some(has_more)),
        Navigation::Nested(page) => network_steps(
            &mut result.next,
            &result.sources,
            Some(has_more || u64::from(page.offset) + u64::from(page.returned) < page.total),
        ),
    }
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

fn navigation(body: &biomcp_types::ReplyData) -> Navigation<'_> {
    use biomcp_types::ReplyData as Body;
    match body {
        ReplyData::BiomarkerPage { body: _, .. }
        | ReplyData::BiomarkerResult { body: _, .. }
        | ReplyData::OrganizationPage { body: _, .. }
        | ReplyData::AuthorPage { body: _, .. }
        | ReplyData::CellLinePage { body: _, .. } => Navigation::NativeOrder,
        ReplyData::AuthorCards { body: _, .. } => Navigation::BoundedPma,
        ReplyData::CellLineChembl { body: _, .. }
        | ReplyData::CellLineAssayCount { body: _, .. } => Navigation::BoundedPma,
        ReplyData::CellLineCard { body: _, .. } => Navigation::BoundedPma,
        ReplyData::GwasAssociationCard { body: _, .. }
        | ReplyData::GwasStudy { body: _, .. }
        | ReplyData::GwasLoci { body: _, .. }
        | ReplyData::GwasAncestries { body: _, .. }
        | ReplyData::GwasPage { body: _, .. }
        | ReplyData::PathwayEvents { body: _, .. }
        | ReplyData::ProteinComplexes { body: _, .. } => Navigation::NativeOrder,
        ReplyData::PathwayCard { body: card, .. } => pathway_navigation(card.as_ref()),
        ReplyData::ProteinPage { body: _, .. } => Navigation::Selectable,
        ReplyData::ProteinFeatures { body: _, .. }
        | ReplyData::ProteinIsoforms { body: _, .. }
        | ReplyData::ProteinDomains { body: _, .. }
        | ReplyData::ProteinComplexMembers { body: _, .. }
        | ReplyData::ProteinStructures { body: _, .. }
        | ReplyData::ProteinSequenceResult { body: _, .. } => Navigation::Bounded,
        ReplyData::ProteinCard {
            body: Some(card), ..
        } => protein_navigation(card),
        ReplyData::StudyProfiles { body: _, .. }
        | ReplyData::StudySampleLists { body: _, .. }
        | ReplyData::StudyFiles { body: _, .. } => Navigation::Bounded,
        ReplyData::PathwayPage { body: _, .. } | ReplyData::PhenotypePage { body: _, .. } => {
            Navigation::Selectable
        }
        ReplyData::PhenotypeTermCard { body: _, .. }
        | ReplyData::PhenotypeSynonyms { body: _, .. }
        | ReplyData::PhenotypeXrefs { body: _, .. }
        | ReplyData::PhenotypeHierarchy { body: _, .. } => Navigation::Bounded,
        ReplyData::PathwayGenes { body: _, .. } => Navigation::Bounded,
        ReplyData::PgxAnnotations {
            body: Some(value), ..
        } => pgx_navigation(value),
        ReplyData::VariantCard {
            body: Some(card), ..
        } => variant_navigation(card),
        ReplyData::DiagnosticCard {
            body: Some(biomcp_types::DiagnosticCard::PmaSubmission(_)),
            ..
        } => Navigation::BoundedPma,
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

/// Build source network guidance for housekeeping results with the same bounded policy.
pub(crate) fn network(sources: &[biomcp_types::SourceOutcome]) -> Vec<Step> {
    let mut steps = Vec::new();
    network_steps(&mut steps, sources, None);
    steps
}

fn network_steps(
    steps: &mut Vec<Step>,
    sources: &[biomcp_types::SourceOutcome],
    candidate_page: Option<bool>,
) {
    let association = steps
        .iter()
        .any(|step| step.0 == biomcp_types::StepKind::Pivot);
    if candidate_page.is_none() && !association {
        append_network_steps(steps, sources);
        steps.truncate(5);
        return;
    }
    // Some disambiguate actions are searches or relations, not candidate gets.
    let (mut candidates, mut extras): (Vec<_>, Vec<_>) =
        std::mem::take(steps).into_iter().partition(|step| {
            step.0 == biomcp_types::StepKind::Pivot
                || step.0 == biomcp_types::StepKind::Disambiguate && step.is_get()
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
            .position(|step| step.0 == biomcp_types::StepKind::Page)
    {
        candidates.push(extras.remove(index));
    }
    append_network_steps(&mut extras, sources);
    extras.truncate(3);
    candidates.extend(extras);
    *steps = candidates;
}
fn append_network_steps(steps: &mut Vec<Step>, sources: &[biomcp_types::SourceOutcome]) {
    for (reason, setting) in [
        (biomcp_types::Reason::Tls, "ca_bundle"),
        (biomcp_types::Reason::Proxy, "proxy"),
    ] {
        if sources.iter().any(|source| source.reason == Some(reason)) {
            steps.insert(
                0,
                Step::new(
                    biomcp_types::StepKind::Configure,
                    &[],
                    Action::NetworkHelp(setting.into()),
                ),
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

fn bounded_navigation(body: &biomcp_types::ReplyData) -> Navigation<'_> {
    use biomcp_types::ReplyData as Body;
    match body {
        ReplyData::StudyPage { body: _, .. }
        | ReplyData::DiagnosticCompanion { body: _, .. }
        | ReplyData::DiagnosticPage { body: _, .. }
        | ReplyData::DiagnosticGenes { body: _, .. }
        | ReplyData::DiagnosticSupplements { body: _, .. }
        | ReplyData::AdverseEventPage { body: _, .. }
        | ReplyData::DiseasePage { body: _, .. }
        | ReplyData::PhenotypeDiseases { body: _, .. }
        | ReplyData::PhenotypeGenes { body: _, .. }
        | ReplyData::DiseaseGenes { body: _, .. }
        | ReplyData::DiseasePhenotypes { body: _, .. }
        | ReplyData::DrugPage { body: _, .. }
        | ReplyData::DrugWhoPage { body: _, .. }
        | ReplyData::DrugRecallCard { body: _, .. }
        | ReplyData::DrugRecallPage { body: _, .. }
        | ReplyData::DrugFdaPage { body: _, .. }
        | ReplyData::DrugCivic { body: _, .. }
        | ReplyData::VariantPage { body: _, .. } => Navigation::Selectable,
        ReplyData::StudyCard { body: _, .. }
        | ReplyData::PgxPage { body: _, .. }
        | ReplyData::PgxCard { body: _, .. } => Navigation::Bounded,
        ReplyData::PgxRecommendations { body: _, .. }
        | ReplyData::PgxGuidelines { body: _, .. } => Navigation::Bounded,
        ReplyData::PgxDiplotype { body: _, .. } => Navigation::Bounded,
        ReplyData::PgxAnnotation { body: _, .. } | ReplyData::DiagnosticCard { body: _, .. } => {
            Navigation::Bounded
        }
        ReplyData::DiagnosticConditions { body: _, .. }
        | ReplyData::DiagnosticMethods { body: _, .. } => Navigation::Bounded,
        ReplyData::DiagnosticRegulatory { body: _, .. }
        | Body::Usage(biomcp_types::UsageError::UnsupportedAdverseEvent) => Navigation::Bounded,
        ReplyData::AdverseEventFaersCard { body: _, .. }
        | ReplyData::AdverseEventDrugs { body: _, .. } => Navigation::Bounded,
        ReplyData::AdverseEventReactions { body: _, .. } => Navigation::Bounded,
        ReplyData::AdverseEventIdentifiers { body: _, .. }
        | ReplyData::VaersSummary { body: _, .. } => Navigation::Bounded,
        ReplyData::AdverseEventMaudeCard { body: _, .. }
        | ReplyData::AdverseEventDevices { body: _, .. } => Navigation::Bounded,
        ReplyData::AdverseEventNarrative { body: _, .. }
        | ReplyData::DiseaseCard { body: _, .. } => Navigation::Bounded,
        ReplyData::DiseaseSynonyms { body: _, .. } | ReplyData::DiseaseXrefs { body: _, .. } => {
            Navigation::Bounded
        }
        ReplyData::DiseaseHierarchy { body: _, .. } | ReplyData::DiseaseSites { body: _, .. } => {
            Navigation::Bounded
        }
        ReplyData::DiseaseIncidence { body: _, .. }
        | ReplyData::DiseaseSurvival { body: _, .. } => Navigation::Bounded,
        ReplyData::DrugCard { body: _, .. }
        | ReplyData::DrugLabel { body: _, .. }
        | ReplyData::DrugWhoCard { body: _, .. } => Navigation::Bounded,
        ReplyData::DrugRegulatory { body: _, .. } | ReplyData::DrugApprovals { body: _, .. } => {
            Navigation::Bounded
        }
        ReplyData::DrugInteractions { body: _, .. } | ReplyData::DrugXrefs { body: _, .. } => {
            Navigation::Bounded
        }
        ReplyData::DrugTargets { body: _, .. } | ReplyData::DrugIndications { body: _, .. } => {
            Navigation::Bounded
        }
        ReplyData::VariantAssertions { body: _, .. }
        | ReplyData::VariantNormalization { body: _, .. } => Navigation::Bounded,
        ReplyData::NciTrialPage { body: _, .. } | ReplyData::NciTrialCards { body: _, .. } => {
            Navigation::Bounded
        }
        Body::Usage(biomcp_types::UsageError::BiomarkerTooBroad)
        | Body::Usage(biomcp_types::UsageError::OrganizationTooBroad)
        | Body::Usage(biomcp_types::UsageError::UnsupportedTrialFilter { .. })
        | Body::Usage(biomcp_types::UsageError::NciDiseaseError) => Navigation::Bounded,
        Body::Usage(biomcp_types::UsageError::VariantQueryPolicyError)
        | ReplyData::VariantPopulationResult { body: _, .. } => Navigation::Bounded,
        ReplyData::VariantRecurrence { body: _, .. }
        | ReplyData::TrialDocumentResult { body: _, .. } => Navigation::Bounded,
        ReplyData::TrialCards { body: _, .. }
        | ReplyData::TrialPage { body: _, .. }
        | ReplyData::ArticlePdfResult { body: _, .. } => Navigation::Bounded,
        Body::Usage(biomcp_types::UsageError::PdfSaveRefused(_))
        | Body::Usage(biomcp_types::UsageError::ArticleSaveRefused(_)) => Navigation::Bounded,
        ReplyData::ArticleEvidenceResult { body: _, .. }
        | ReplyData::ArticleRecommendations { body: _, .. } => Navigation::Bounded,
        ReplyData::ArticleSupplementResult { body: _, .. }
        | Body::Usage(biomcp_types::UsageError::UnsupportedFilter { .. }) => Navigation::Bounded,
        ReplyData::ArticleTextResult { body: _, .. }
        | Body::Usage(biomcp_types::UsageError::TextPositionError) => Navigation::Bounded,
        ReplyData::ArticleEdgePage { body: _, .. }
        | Body::Usage(biomcp_types::UsageError::TooManyPmids { .. }) => Navigation::Bounded,
        ReplyData::ArticleCards { body: _, .. } | Body::PoolError { .. } => Navigation::Bounded,
        ReplyData::ArticlePage { body: _, .. }
        | ReplyData::GeneCards { body: _, .. }
        | ReplyData::GenePage { body: _, .. } => Navigation::Bounded,
        _ => Navigation::Bounded,
    }
}
