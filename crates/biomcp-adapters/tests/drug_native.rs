//! Public adapter mapping contracts over retained publisher replies, without provider access.
use biodata::{DrugIdentity, DrugIdentityLevel};
use biomcp_adapters::{chembl, opentargets};
use serde_json::json;
fn identity(id: &str, name: &str, source: &str) -> DrugIdentity {
    DrugIdentity {
        id: id.parse().unwrap(),
        source: source.parse().unwrap(),
        name: name.parse().unwrap(),
        identity_level: DrugIdentityLevel::Unknown,
        r#type: None,
        url: None,
    }
}
#[test]
fn chembl_target_records_do_not_manufacture_a_gene() {
    let page: chembl::Mechanisms = serde_json::from_str(include_str!(
        "fixtures/drug/chembl-trastuzumab-mechanisms.json"
    ))
    .unwrap();
    let drug = identity("CHEMBL1201585", "TRASTUZUMAB", "chembl");
    let rows = chembl::map_targets(&drug, page.mechanisms).unwrap();
    assert_eq!(rows.len(), 1);
    assert!(rows[0].target().is_none());
    assert_eq!(rows[0].native_target().unwrap().id.as_str(), "CHEMBL1824");
    assert_eq!(
        rows[0].attribution().record.as_ref().unwrap().as_str(),
        "127"
    );
    assert_eq!(rows[0].references()[0].ref_type.as_str(), "DailyMed");
}
#[test]
fn chembl_indications_keep_each_native_disease_label_stage_and_reference() {
    let page: chembl::Indications = serde_json::from_str(include_str!(
        "fixtures/drug/chembl-imatinib-indications.json"
    ))
    .unwrap();
    let rows = chembl::map_indications(
        &identity("CHEMBL941", "IMATINIB", "chembl"),
        page.drug_indications,
    )
    .unwrap();
    let row = &rows[0];
    assert_eq!(row.disease_references[0].id.as_str(), "EFO:0000220");
    assert_eq!(row.disease_references[1].id.as_str(), "D054198");
    assert_eq!(
        row.disease_references[1].label.as_ref().unwrap().as_str(),
        "Precursor Cell Lymphoblastic Leukemia-Lymphoma"
    );
    assert_eq!(
        row.stage.as_ref().unwrap().field.as_ref(),
        "max_phase_for_ind"
    );
    assert_eq!(row.stage.as_ref().unwrap().value.as_ref(), "3.0");
    assert!(
        row.references[0]
            .ref_id
            .as_ref()
            .unwrap()
            .as_str()
            .contains("NCT00130195,NCT00137111")
    );
}
#[test]
fn opentargets_preserves_native_stage_and_all_target_members_without_authorization() {
    let value: serde_json::Value =
        serde_json::from_str(include_str!("fixtures/drug/opentargets-axicel-detail.json")).unwrap();
    let mut drug: opentargets::Drug =
        serde_json::from_value(value["data"]["drug"].clone()).unwrap();
    let identity = identity(&drug.id, &drug.name, "opentargets");
    let extra = json!({"id":"ENSG00000141736","approvedSymbol":"ERBB2","approvedName":"erb-b2 receptor tyrosine kinase 2"});
    // Synthetic second member protects complete native member context; the first remains captured CD19.
    drug.mechanisms_of_action.rows[0]
        .targets
        .push(serde_json::from_value(extra).unwrap());
    let targets = opentargets::map_targets(&identity, drug.mechanisms_of_action.rows).unwrap();
    assert_eq!(targets.len(), 2);
    assert_eq!(targets[0].target().unwrap().label().as_str(), "CD19");
    assert_eq!(targets[1].target().unwrap().label().as_str(), "ERBB2");
    let indications = opentargets::map_indications(&identity, drug.indications.rows).unwrap();
    assert_eq!(
        indications[2].stage.as_ref().unwrap().value.as_ref(),
        "APPROVAL"
    );
    assert_eq!(
        indications[2].disease_references[0].id.as_str(),
        "MONDO_0018905"
    );
    assert!(
        serde_json::to_value(&indications)
            .unwrap()
            .get("authorization")
            .is_none()
    );
}
