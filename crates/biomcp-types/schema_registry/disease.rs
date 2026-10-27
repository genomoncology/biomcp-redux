use super::Contract;
use biodata::*;
use biomcp_types::*;
pub(super) fn registry() -> Vec<Contract> {
    contracts! {
        DiseaseAssociation => ("disease-association", "biodata.disease-association/4"),
        DiseaseRelation => ("disease-relation", "biodata.disease-relation/2"),
        DiseaseEndpoint => ("disease-endpoint", "biodata.disease-endpoint/1"),
        DiseaseSynonym => ("disease-synonym", "biodata.disease-synonym/1"),
        DiseaseStatistic => ("disease-statistic", "biodata.disease-statistic/1"),
        DiseaseOntology => ("disease-ontology", "biomcp.disease-ontology/1"),
        DiseaseSearch => ("disease-search", "biomcp.disease-search/1"),
        DiseaseGet => ("disease-get", "biomcp.disease-get/2"),
        Envelope<DiseasePage> => ("disease-page", "biomcp.disease-page/1"),
        Envelope<DiseaseCard> => ("disease-card", "biomcp.disease-card/2"),
        Envelope<DiseaseSynonyms> => ("disease-synonyms", "biomcp.disease-synonyms/2"),
        Envelope<DiseaseXrefs> => ("disease-xrefs", "biomcp.disease-xrefs/2"),
        Envelope<DiseaseHierarchy> => ("disease-hierarchy", "biomcp.disease-hierarchy/2"),
        Envelope<DiseaseGenes> => ("disease-genes", "biomcp.disease-genes/5"),
        Envelope<DiseasePhenotypes> => ("disease-phenotypes", "biomcp.disease-phenotypes/5"),
        Envelope<DiseaseSites> => ("disease-sites", "biomcp.disease-sites/2"),
        Envelope<DiseaseIncidence> => ("disease-incidence", "biomcp.disease-incidence/3"),
        Envelope<DiseaseSurvival> => ("disease-survival", "biomcp.disease-survival/3"),
    }
}
