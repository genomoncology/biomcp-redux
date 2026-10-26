use super::{Contract, generate};
use biomcp_types::*;
pub(super) fn registry() -> Vec<Contract> {
    contracts! {
        Envelope<PhenotypePage> => ("phenotype-search", "biomcp.phenotype-search/1"),
        Envelope<PhenotypeTermCard> => ("phenotype-card", "biomcp.phenotype-card/1"),
        Envelope<PhenotypeSynonyms> => ("phenotype-synonyms", "biomcp.phenotype-synonyms/1"),
        Envelope<PhenotypeXrefs> => ("phenotype-xrefs", "biomcp.phenotype-xrefs/1"),
        Envelope<PhenotypeHierarchy> => ("phenotype-hierarchy", "biomcp.phenotype-hierarchy/1"),
        Envelope<PhenotypeDiseases> => ("phenotype-diseases", "biomcp.phenotype-diseases/1"),
        Envelope<PhenotypeGenes> => ("phenotype-genes", "biomcp.phenotype-genes/1"),
    }
}
