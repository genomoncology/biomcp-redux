//! Source coverage for public command discovery, including implicit source sections.
use crate::Source;

/// Sources not fully represented by a command source selector.
pub fn command_implicit_sources(verb: &str, entity: &str) -> &'static [Source] {
    use Source::{
        AlleleRegistry, AlphaGenome, CancerHotspots, Cbioportal, Cellosaurus, Chembl, Civic,
        ClinGen, ClinVar, ClinicalTrialsGov, ComplexPortal, Erepo, EuropePmc, Gencc, Gnomad, Gwas,
        Hgnc, Hpa, Icite, InterPro, Mutalyzer, MyGene, MyVariant, Oncokb, Orcid, PmcOa, PubMed,
        PubTator, Reactome, SemanticScholar, UniProt, VariantValidator, WikiPathways,
    };
    match (verb, entity) {
        ("protein", "domains") => &[InterPro],
        ("protein", "complexes" | "complex-members") => &[ComplexPortal],
        ("protein", _) | ("search", "protein") => &[UniProt],
        ("get", "protein") => &[Hgnc, UniProt, InterPro, ComplexPortal],
        ("cell-line", _) => &[Chembl],
        ("search" | "get", "cell-line") => &[Cellosaurus],
        ("pathway", "events") => &[Reactome],
        ("pathway", _) | ("search" | "get", "pathway") => &[Reactome, WikiPathways],
        ("variant", "assertions") => &[ClinVar, Erepo],
        ("variant", "population") => &[Gnomad, MyVariant, AlleleRegistry],
        ("variant", "recurrence") => &[Cbioportal],
        ("variant", "normalize") => &[Mutalyzer, VariantValidator],
        ("get", "variant") => &[
            MyVariant,
            ClinVar,
            Erepo,
            Gnomad,
            Gwas,
            Civic,
            CancerHotspots,
            AlphaGenome,
            Oncokb,
            AlleleRegistry,
        ],
        ("search", "gene") => &[Hgnc],
        ("get", "gene") => &[
            Hgnc, MyGene, UniProt, Civic, ClinGen, Gencc, Gnomad, Hpa, Reactome, Oncokb,
        ],
        ("article", "text") => &[PubMed, PmcOa, EuropePmc],
        ("article", "supplement") => &[PubMed, PmcOa],
        ("article", "pdf") => &[SemanticScholar],
        ("article", "citation-evidence") => &[SemanticScholar, PubMed, PmcOa],
        ("article", "recommendations") => &[SemanticScholar],
        ("get", "article") => &[PubMed, EuropePmc, PmcOa, Icite, PubTator, SemanticScholar],
        ("article", _) => &[PubMed],
        ("trial", _) => &[ClinicalTrialsGov],
        ("author", _) | ("get", "author") => &[SemanticScholar, Orcid],
        (_, "study") => &[Cbioportal],
        (_, "gwas") => &[Gwas],
        _ => &[],
    }
}

/// Section publishers from their checked type owners; no renderer owns these bindings.
/// None leaves single-source command discovery to the command declaration.
pub fn command_section_sources(
    verb: &str,
    entity: &str,
    word: &str,
    candidates: &[Source],
) -> Option<Vec<Source>> {
    use crate::{
        AdverseEventSection, ArticleSection, CellLineSection, DiagnosticGetSource,
        DiagnosticSection, DiseaseSection, DrugSection, DrugSource, GeneSection, PathwaySection,
        PgxSection, PhenotypeSection, ProteinSection, TrialSection, TrialSource, VaersSection,
        VariantSection,
    };
    match (verb, entity) {
        ("get", "gene") => word.parse::<GeneSection>().ok().map(|s| vec![s.source()]),
        ("get", "variant") => word
            .parse::<VariantSection>()
            .ok()
            .map(|s| vec![s.direct_source().unwrap_or(Source::MyVariant)]),
        ("get", "protein") => word
            .parse::<ProteinSection>()
            .ok()
            .and_then(|s| s.source())
            .map(|s| vec![s]),
        ("get", "article") => word
            .parse::<ArticleSection>()
            .ok()
            .map(|s| s.sources().to_vec()),
        ("get", "cell-line") => word
            .parse::<CellLineSection>()
            .ok()
            .map(|s| s.sources().to_vec()),
        ("get", "disease") => word
            .parse::<DiseaseSection>()
            .ok()
            .map(|s| s.sources().to_vec()),
        ("get", "pathway") => word
            .parse::<PathwaySection>()
            .ok()
            .map(|s| s.sources().to_vec()),
        ("get", "phenotype") => word
            .parse::<PhenotypeSection>()
            .ok()
            .map(|s| s.sources().to_vec()),
        ("get", "pgx") => word.parse::<PgxSection>().ok().map(|s| vec![s.source()]),
        ("get", "adverse-event") => word
            .parse::<AdverseEventSection>()
            .ok()
            .map(|s| vec![s.source().source()]),
        ("search", "adverse-event") => word
            .parse::<VaersSection>()
            .ok()
            .map(|_| vec![Source::Vaers]),
        ("get", "drug") => word.parse::<DrugSection>().ok().map(|s| {
            DrugSource::ALL
                .into_iter()
                .filter(|source| source.sections().contains(&s))
                .map(DrugSource::source)
                .collect()
        }),
        ("get", "trial") => word.parse::<TrialSection>().ok().map(|s| {
            candidates
                .iter()
                .copied()
                .filter(|source| {
                    source
                        .to_string()
                        .parse::<TrialSource>()
                        .is_ok_and(|source| source.sections().contains(&s))
                })
                .collect()
        }),
        ("get", "diagnostic") => word.parse::<DiagnosticSection>().ok().map(|s| {
            candidates
                .iter()
                .copied()
                .filter(|source| {
                    source
                        .to_string()
                        .parse::<DiagnosticGetSource>()
                        .is_ok_and(|source| source.sections().contains(&s))
                })
                .collect()
        }),
        _ => None,
    }
}
