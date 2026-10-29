use super::Contract;
use biomcp_types::*;
pub(super) fn registry() -> Vec<Contract> {
    contracts! {
        PhenotypeSearch => ("phenotype-search-query", "biomcp.phenotype-search-query/1", query),
        PhenotypeGet => ("phenotype-get", "biomcp.phenotype-get/1", query),
        Envelope<PhenotypePage> => ("phenotype-search", "biomcp.phenotype-search/2"),
        Envelope<PhenotypeTermCard> => ("phenotype-card", "biomcp.phenotype-card/3"),
        Envelope<PhenotypeSynonyms> => ("phenotype-synonyms", "biomcp.phenotype-synonyms/2"),
        Envelope<PhenotypeXrefs> => ("phenotype-xrefs", "biomcp.phenotype-xrefs/3"),
        Envelope<PhenotypeHierarchy> => ("phenotype-hierarchy", "biomcp.phenotype-hierarchy/3"),
        Envelope<PhenotypeDiseases> => ("phenotype-diseases", "biomcp.phenotype-diseases/5"),
        Envelope<PhenotypeGenes> => ("phenotype-genes", "biomcp.phenotype-genes/5"),
    }
}
