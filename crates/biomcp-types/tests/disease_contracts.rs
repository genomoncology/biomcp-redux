//! Disease public wire contracts precede provider implementations.
use biodata::{DiseaseAssociation, Statement};
use biomcp_types::{DiseaseCard, DiseaseGet, DiseaseOntology, DiseaseSearch};
use serde::{Serialize, de::DeserializeOwned};
use serde_json::{Value, json};

fn roundtrip<T: DeserializeOwned + Serialize>(value: Value) -> Value {
    serde_json::to_value(serde_json::from_value::<T>(value).unwrap()).unwrap()
}

#[test]
fn discovery_defaults_to_one_mondo_ontology_and_refuses_unknown_choices() {
    let value = roundtrip::<DiseaseSearch>(json!({"text":"breast cancer"}));
    assert_eq!(value["ontology"], "mondo");
    assert_eq!(value["source"], "ols");
    for ontology in ["mondo", "doid", "ncit"] {
        assert_eq!(roundtrip::<DiseaseOntology>(json!(ontology)), ontology);
    }
    for ontology in ["mondo,doid", "mesh", "MONDO", ""] {
        assert!(serde_json::from_value::<DiseaseOntology>(json!(ontology)).is_err());
    }
}

#[test]
fn exact_identity_and_statistics_selectors_cannot_be_mixed() {
    for id in ["MONDO:0007254", "DOID:1612", "NCIT:C4872", "OncoTree:BRCA", "ICD10CM:C50.9"] {
        assert_eq!(roundtrip::<DiseaseGet>(json!({"id":id}))["id"], id);
    }
    for value in [
        json!({"id":"MONDO:0007254","source":"seer","section":"incidence"}),
        json!({"id":"47","source":"seer","section":"survival","period":"5"}),
        json!({"id":"breast cancer"}),
        json!({"id":"MONDO:0007254","section":"all"}),
        json!({"id":"MONDO:0007254","limit":5}),
    ] {
        assert!(serde_json::from_value::<DiseaseGet>(value.clone()).is_err(), "{value}");
    }
    assert!(serde_json::from_value::<DiseaseGet>(json!({"id":"47","source":"seer","section":"incidence","period":"5"})).is_ok());
}

#[test]
fn card_refuses_hidden_associations_statistics_and_related_entities() {
    let card = json!({"requested_id":"MONDO:0007254","returned_id":"MONDO:0007254","concept":{"id":"MONDO:0007254","label":"breast cancer"},"attribution":{"source":"synthetic"},"sections":[],"components":[]});
    roundtrip::<DiseaseCard>(card.clone());
    for field in ["genes", "phenotypes", "incidence", "survival", "trials", "drugs"] {
        let mut invalid = card.clone();
        invalid[field] = json!([]);
        assert!(serde_json::from_value::<DiseaseCard>(invalid).is_err());
    }
}

#[test]
fn association_keeps_parent_request_and_native_child_without_becoming_statement() {
    let row = json!({"requested_id":"MONDO:0019003","subject":{"id":"HGNC:9967","label":"RET"},"object":{"id":"MONDO:0008234","label":"MEN2A"},"predicate":"biolink:gene_associated_with_condition","original_subject":"NCBIGene:5979","original_object":"Orphanet:247698","attribution":{"source":"synthetic"},"evidence":[],"qualifiers":[],"subject_closure":[],"object_closure":["MONDO:0019003"]});
    assert_eq!(roundtrip::<DiseaseAssociation>(row.clone()), row);
    assert!(serde_json::from_value::<Statement>(row).is_err());
}
