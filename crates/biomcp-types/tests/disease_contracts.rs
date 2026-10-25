//! Disease public wire contracts precede provider implementations.
use biodata::{DiseaseAssociation, Statement};
use biomcp_types::{DiseaseCard, DiseaseGet, DiseaseOntology, DiseaseSearch};
use serde::{Serialize, de::DeserializeOwned};
use serde_json::{Value, json};

fn roundtrip<T: DeserializeOwned + Serialize>(value: Value) -> Result<Value, serde_json::Error> {
    serde_json::to_value(serde_json::from_value::<T>(value)?)
}

#[test]
fn discovery_defaults_to_one_mondo_ontology_and_refuses_unknown_choices() {
    let value = roundtrip::<DiseaseSearch>(json!({"text":"breast cancer"})).unwrap();
    assert_eq!(value["ontology"], "mondo");
    assert_eq!(value["source"], "ols");
    for ontology in ["mondo", "doid", "ncit"] {
        assert_eq!(roundtrip::<DiseaseOntology>(json!(ontology)).unwrap(), ontology);
    }
    for ontology in ["mondo,doid", "mesh", "MONDO", ""] {
        assert!(serde_json::from_value::<DiseaseOntology>(json!(ontology)).is_err());
    }
}

#[test]
fn exact_identity_and_statistics_selectors_cannot_be_mixed() {
    for id in [
        "MONDO:0007254",
        "DOID:1612",
        "NCIT:C4872",
        "OncoTree:BRCA",
        "ICD10CM:C50.9",
    ] {
        assert_eq!(roundtrip::<DiseaseGet>(json!({"id":id})).unwrap()["id"], id);
    }
    for value in [
        json!({"id":"MONDO:0007254","source":"seer","section":"incidence"}),
        json!({"id":"47","source":"seer","section":"survival","period":"5"}),
        json!({"id":"47","source":"seer","section":"survival","ontology":"mondo"}),
        json!({"id":"MONDO:0007254","ontology":"mondo"}),
        json!({"id":"breast cancer"}),
        json!({"id":"MONDO:0007254","section":"all"}),
        json!({"id":"MONDO:0007254","limit":5}),
    ] {
        assert!(
            serde_json::from_value::<DiseaseGet>(value.clone()).is_err(),
            "{value}"
        );
    }
    assert!(
        serde_json::from_value::<DiseaseGet>(
            json!({"id":"47","source":"seer","section":"incidence","period":"5"})
        )
        .is_ok()
    );
}

#[test]
fn card_refuses_hidden_associations_statistics_and_related_entities() {
    let card = json!({"requested_id":"MONDO:0007254","returned_id":"MONDO:0007254","concept":{"id":"MONDO:0007254","label":"breast cancer"},"attribution":{"source":"synthetic"},"sections":[],"components":[]});
    roundtrip::<DiseaseCard>(card.clone()).unwrap();
    for field in [
        "genes",
        "phenotypes",
        "incidence",
        "survival",
        "trials",
        "drugs",
    ] {
        let mut invalid = card.clone();
        invalid[field] = json!([]);
        assert!(serde_json::from_value::<DiseaseCard>(invalid).is_err());
    }
}

#[test]
fn association_keeps_parent_request_and_native_child_without_becoming_statement() {
    let row = json!({"requested_id":"MONDO:0019003","subject":{"id":"HGNC:9967","label":"RET"},"object":{"id":"MONDO:0008234","label":"MEN2A"},"predicate":"biolink:gene_associated_with_condition","original_subject":"NCBIGene:5979","original_object":"Orphanet:247698","attribution":{"source":"synthetic"},"evidence":[],"qualifiers":[],"subject_closure":[],"object_closure":["MONDO:0019003"]});
    assert_eq!(roundtrip::<DiseaseAssociation>(row.clone()).unwrap(), row);
    assert!(serde_json::from_value::<Statement>(row).is_err());
}

#[test]
fn native_statistics_keep_requested_and_returned_dimensions_and_missing_observations() {
    let value = json!({"requested_site":"55","requested_selectors":[{"name":"dataset","code":"4"}],"returned_selectors":[{"name":"site","code":"55"},{"name":"year_range","code":"31"}],"year":2022,"measure":"rel_rate","modeled":{"value":93.690345},"attribution":{"source":"synthetic"}});
    let result = roundtrip::<biodata::DiseaseStatistic>(value.clone()).unwrap();
    assert_eq!(result, value);
    assert!(result.get("observed").is_none());
    assert_eq!(result["requested_selectors"][0]["code"], "4");
    assert_eq!(result["returned_selectors"][1]["code"], "31");
}

#[test]
fn every_source_and_named_section_roundtrips_with_closed_spellings() {
    use biomcp_types::{DiseaseSection, DiseaseSource};
    for spelling in ["ols", "mydisease", "monarch", "seer"] {
        let source: DiseaseSource = serde_json::from_value(json!(spelling)).unwrap();
        assert_eq!(source.as_str(), spelling);
        assert_eq!(roundtrip::<DiseaseSource>(json!(spelling)).unwrap(), spelling);
    }
    for spelling in [
        "synonyms",
        "xrefs",
        "hierarchy",
        "genes",
        "phenotypes",
        "incidence",
        "survival",
    ] {
        let section: DiseaseSection = serde_json::from_value(json!(spelling)).unwrap();
        assert_eq!(section.as_str(), spelling);
        assert_eq!(roundtrip::<DiseaseSection>(json!(spelling)).unwrap(), spelling);
    }
    for spelling in ["all", "clinical_features", "pathways", "civic", "shortages"] {
        assert!(serde_json::from_value::<DiseaseSection>(json!(spelling)).is_err());
        assert!(serde_json::from_value::<DiseaseSource>(json!(spelling)).is_err());
    }
}

#[test]
fn association_component_terms_stay_bound_to_the_native_row() {
    let row = json!({"association":{"requested_id":"MONDO:0019003","subject":{"id":"HGNC:9967","label":"RET"},"object":{"id":"MONDO:0008234","label":"MEN2A"},"predicate":"biolink:gene_associated_with_condition","attribution":{"source":"synthetic","record":"edge-1","version":"2026-01-01"},"evidence":[],"qualifiers":[],"subject_closure":[],"object_closure":["MONDO:0019003"]},"components":[{"component":"hpo","terms":{"usage":"attribution","name":"Synthetic attribution terms","url":"https://example.invalid/terms"}}]});
    assert_eq!(
        roundtrip::<biomcp_types::DiseaseAssociationItem>(row.clone()).unwrap(),
        row
    );
    assert!(serde_json::from_value::<DiseaseAssociation>(row).is_err());
}
