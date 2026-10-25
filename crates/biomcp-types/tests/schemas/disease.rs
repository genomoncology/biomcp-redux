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

#[test]
fn request_metadata_keeps_positional_text_single_ontology_and_section_paging() {
    let search = generate::<DiseaseSearch>();
    assert_eq!(search["properties"]["text"]["x-positional"], 0);
    assert_eq!(search["properties"]["ontology"]["default"], "mondo");
    assert_eq!(
        search["$defs"]["DiseaseOntology"]["enum"],
        serde_json::json!(["mondo", "doid", "ncit"])
    );
    let get = generate::<DiseaseGet>();
    let fields = get["properties"].as_object().unwrap();
    assert!(fields.contains_key("section_limit"));
    assert!(fields.contains_key("section_offset"));
    assert!(!fields.contains_key("limit"));
    assert!(!fields.contains_key("offset"));
    assert_eq!(fields["id"]["x-positional"], 0);
}

fn envelope<T>(query: serde_json::Value, body: serde_json::Value)
where
    T: ResponseBody + schemars::JsonSchema + serde::de::DeserializeOwned,
    T::Query: schemars::JsonSchema + serde::de::DeserializeOwned,
{
    let body: T = serde_json::from_value(body).unwrap();
    let query = serde_json::from_value(query).unwrap();
    let value = serde_json::to_value(Envelope::new(Status::Complete, query, Some(body))).unwrap();
    assert_eq!(value["schema"], T::SCHEMA);
    let validator = jsonschema::draft202012::new(&generate::<Envelope<T>>()).unwrap();
    assert!(validator.is_valid(&value), "{value}");
}

#[test]
fn all_ten_synthetic_response_envelopes_follow_their_generated_schema() {
    use serde_json::json;
    let search = json!({"text":"synthetic cancer"});
    let get = json!({"id":"MONDO:0007254"});
    let empty = json!({"items":[],"components":[]});
    envelope::<DiseasePage>(search, empty.clone());
    envelope::<DiseaseCard>(
        get.clone(),
        json!({"requested_id":"MONDO:0007254","returned_id":"MONDO:0007254","concept":{"id":"MONDO:0007254","label":"synthetic cancer"},"definition":"Complete synthetic definition.","obsolete":true,"consider":["MONDO:0800444","MONDO:0800445"],"attribution":{"source":"synthetic"},"sections":["synonyms","xrefs","hierarchy"],"components":[]}),
    );
    envelope::<DiseaseSynonyms>(
        get.clone(),
        json!({"items":[{"value":"synthetic synonym","scope":"oboInOwl:hasExactSynonym","language":"en","attribution":{"source":"synthetic"}}],"components":[]}),
    );
    let relations = json!({"items":[{"subject":{"id":"MONDO:0007254","label":"synthetic"},"predicate":"skos:closeMatch","object":{"id":"DOID:1612","label":"native target"},"attribution":{"source":"synthetic"}}],"components":[]});
    envelope::<DiseaseXrefs>(get.clone(), relations.clone());
    envelope::<DiseaseHierarchy>(get.clone(), relations);
    envelope::<DiseaseGenes>(get.clone(), empty.clone());
    envelope::<DiseasePhenotypes>(get, empty.clone());
    envelope::<DiseaseSites>(
        json!({"source":"seer"}),
        json!({"items":[{"code":"47","label":"Synthetic site","active":true}],"components":[]}),
    );
    envelope::<DiseaseIncidence>(
        json!({"id":"47","source":"seer","section":"incidence","period":"5"}),
        empty.clone(),
    );
    envelope::<DiseaseSurvival>(
        json!({"id":"55","source":"seer","section":"survival","interval":"5"}),
        empty,
    );
}
