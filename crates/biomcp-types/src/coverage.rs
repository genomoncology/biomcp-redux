//! Source coverage for public command discovery, including implicit source sections.
use crate::Source;

/// Sources reachable through a public command, including its optional sections.
pub fn command_sources(verb: &str, entity: &str) -> &'static [Source] {
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
        ("search", "variant") => &[ClinVar, MyVariant],
        ("get", "variant") => &[MyVariant, ClinVar, Civic, CancerHotspots, AlphaGenome, Oncokb, AlleleRegistry],
        ("search", "gene") => &[Hgnc],
        ("get", "gene") => &[Hgnc, MyGene, UniProt, Civic, ClinGen, Gencc, Gnomad, Hpa, Reactome, Oncokb],
        ("article", "text" | "supplement") => &[PmcOa],
        ("article", "pdf") => &[PmcOa, SemanticScholar],
        ("article", "citation-evidence") => &[Icite],
        ("article", "recommendations") => &[SemanticScholar],
        ("article", _) | ("get", "article") => &[PubMed, PmcOa, Icite],
        ("search", "article") => &[PubMed, EuropePmc, LitVar, LitSense, PubTator, SemanticScholar],
        ("trial", _) => &[ClinicalTrialsGov],
        ("search" | "get", "trial") => &[ClinicalTrialsGov, Nci],
        (_, "biomarker" | "organization") => &[Nci],
        ("author", _) | (_, "author") => &[SemanticScholar, Orcid],
        (_, "gwas") => &[Gwas],
        (_, "phenotype") => &[Hpo, Monarch],
        (_, "study") => &[Cbioportal],
        (_, "pgx") => &[Cpic, ClinPgx],
        (_, "diagnostic") => &[Gtr, WhoIvd, FdaCdx, FdaPma, Fda510k, FdaDeviceRecall],
        (_, "adverse-event") => &[Faers, Vaers, Maude],
        (_, "disease") => &[Ols, MyDisease, Monarch, Seer],
        (_, "drug") => &[Mychem, Chembl, Opentargets, Civic, OpenfdaLabel, Drugsfda, Ema, WhoPq, Ddinter, FdaDrugRecall],
        _ => &[],
    }
}
