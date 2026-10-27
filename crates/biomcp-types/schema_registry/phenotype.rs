use super::Contract;
use biomcp_types::*;
pub(super) fn registry() -> Vec<Contract> {
    contracts! {
        PhenotypeSearch => ("phenotype-search-query", "biomcp.phenotype-search-query/1"),
        PhenotypeGet => ("phenotype-get", "biomcp.phenotype-get/1"),
        Envelope<PhenotypePage> => ("phenotype-search", "biomcp.phenotype-search/1"),
        Envelope<PhenotypeTermCard> => ("phenotype-card", "biomcp.phenotype-card/1"),
        Envelope<PhenotypeSynonyms> => ("phenotype-synonyms", "biomcp.phenotype-synonyms/1"),
        Envelope<PhenotypeXrefs> => ("phenotype-xrefs", "biomcp.phenotype-xrefs/1"),
        Envelope<PhenotypeHierarchy> => ("phenotype-hierarchy", "biomcp.phenotype-hierarchy/1"),
        Envelope<PhenotypeDiseases> => ("phenotype-diseases", "biomcp.phenotype-diseases/3"),
        Envelope<PhenotypeGenes> => ("phenotype-genes", "biomcp.phenotype-genes/3"),
    }
}
