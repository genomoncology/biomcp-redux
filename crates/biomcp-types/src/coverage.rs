//! Source coverage for public command discovery, including implicit source sections.
use crate::Source;

/// Sources not fully represented by a command source selector.
fn implicit_sources(verb: &str, entity: &str) -> &'static [Source] {
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
        ("get", "variant") => &[MyVariant, ClinVar, Civic, CancerHotspots, AlphaGenome, Oncokb, AlleleRegistry],
        ("search", "gene") => &[Hgnc],
        ("get", "gene") => &[Hgnc, MyGene, UniProt, Civic, ClinGen, Gencc, Gnomad, Hpa, Reactome, Oncokb],
        ("article", "text" | "supplement") => &[PmcOa],
        ("article", "pdf") => &[PmcOa, SemanticScholar],
        ("article", "citation-evidence") => &[Icite],
        ("article", "recommendations") => &[SemanticScholar],
        ("article", _) | ("get", "article") => &[PubMed, PmcOa, Icite],
        ("trial", _) => &[ClinicalTrialsGov],
        ("author", _) | (_, "author") => &[SemanticScholar, Orcid],
        _ => &[],
    }
}

/// Declared source selectors plus implicit sources used by a command and its sections.
pub fn command_sources(verb: &str, entity: &str, schema: &serde_json::Value) -> std::collections::BTreeSet<String> {
    let mut sources = std::collections::BTreeSet::new();
    for key in ["source", "sources"] {
        if let Some(node) = schema.pointer(&format!("/properties/{key}")) {
            source_words(node, schema, &mut sources);
        }
    }
    if sources.is_empty() || matches!((verb, entity), ("get", "gene" | "protein" | "variant" | "article")) {
        sources.extend(implicit_sources(verb, entity).iter().map(ToString::to_string));
    }
    sources
}

fn source_words(node: &serde_json::Value, schema: &serde_json::Value, sources: &mut std::collections::BTreeSet<String>) {
    if let Some(reference) = node.get("$ref").and_then(serde_json::Value::as_str) {
        if let Some(target) = schema.pointer(reference.trim_start_matches('#')) { source_words(target, schema, sources); }
    }
    for key in ["enum", "x-extensible-enum"] {
        for value in node.get(key).and_then(serde_json::Value::as_array).into_iter().flatten().filter_map(serde_json::Value::as_str) {
            sources.insert(value.into());
        }
    }
    if let Some(items) = node.get("items") { source_words(items, schema, sources); }
    for key in ["anyOf", "oneOf"] {
        for child in node.get(key).and_then(serde_json::Value::as_array).into_iter().flatten() { source_words(child, schema, sources); }
    }
}
