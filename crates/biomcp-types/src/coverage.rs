//! Source coverage for public command discovery, including implicit source sections.
use crate::Source;

/// Sources not fully represented by a command source selector.
pub fn command_implicit_sources(verb: &str, entity: &str) -> &'static [Source] {
    use Source::*;
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
        ("variant", "population") => &[Gnomad],
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
        ("article", "text" | "supplement") => &[PmcOa],
        ("article", "pdf") => &[PmcOa, SemanticScholar],
        ("article", "citation-evidence") => &[Icite],
        ("article", "recommendations") => &[SemanticScholar],
        ("get", "article") => &[PubMed, EuropePmc, PmcOa, Icite, PubTator, SemanticScholar],
        ("article", _) => &[PubMed],
        ("trial", _) => &[ClinicalTrialsGov],
        ("author", _) | (_, "author") => &[SemanticScholar, Orcid],
        (_, "study") => &[Cbioportal],
        (_, "gwas") => &[Gwas],
        _ => &[],
    }
}
