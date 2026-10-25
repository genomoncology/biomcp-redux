use super::{Contract, generate};
use biodata::*;
use biomcp_types::*;
pub(super) fn registry() -> Vec<Contract> {
    contracts! {
        DiseaseAssociation => ("disease-association", "biodata.disease-association/1"),
        DiseaseRelation => ("disease-relation", "biodata.disease-relation/1"),
        DiseaseSynonym => ("disease-synonym", "biodata.disease-synonym/1"),
        DiseaseStatistic => ("disease-statistic", "biodata.disease-statistic/1"),
        DiseaseOntology => ("disease-ontology", "biomcp.disease-ontology/1"),
        DiseaseSearch => ("disease-search", "biomcp.disease-search/1"),
        DiseaseGet => ("disease-get", "biomcp.disease-get/1"),
        Envelope<DiseasePage> => ("disease-page", "biomcp.disease_page/1"),
        Envelope<DiseaseCard> => ("disease-card", "biomcp.disease_card/1"),
        Envelope<DiseaseSynonyms> => ("disease-synonyms", "biomcp.disease_synonyms/1"),
        Envelope<DiseaseXrefs> => ("disease-xrefs", "biomcp.disease_xrefs/1"),
        Envelope<DiseaseHierarchy> => ("disease-hierarchy", "biomcp.disease_hierarchy/1"),
        Envelope<DiseaseGenes> => ("disease-genes", "biomcp.disease_genes/1"),
        Envelope<DiseasePhenotypes> => ("disease-phenotypes", "biomcp.disease_phenotypes/1"),
        Envelope<DiseaseSites> => ("disease-sites", "biomcp.disease_sites/1"),
        Envelope<DiseaseIncidence> => ("disease-incidence", "biomcp.disease_incidence/1"),
        Envelope<DiseaseSurvival> => ("disease-survival", "biomcp.disease_survival/1"),
    }
}
