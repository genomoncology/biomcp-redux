//! Public response contracts reject malformed identifiers without changing valid spellings.
use biomcp_types::{ClinGenValidity, CosmicRecord, NciDiseaseRef};
use serde_json::json;

#[test]
fn response_identifiers_use_their_publisher_grammar() {
    let cosmic = json!({
        "myvariant_record":"chr7:g.140453136A>T", "identity":"unverified", "cosmic_id":"COSM476"
    });
    let mondo = json!({
        "disease":"source disease", "mondo_id":"MONDO:0000001", "mode_of_inheritance":"source mode",
        "classification":"source classification", "classification_date":"2025-01-01",
        "expert_panel":"source panel", "report_url":"https://example.org/report"
    });
    let ncit = json!({"name":"source name", "nci_thesaurus_concept_id":"C3224"});
    assert!(serde_json::from_value::<CosmicRecord>(cosmic.clone()).is_ok());
    assert!(serde_json::from_value::<ClinGenValidity>(mondo.clone()).is_ok());
    assert!(serde_json::from_value::<NciDiseaseRef>(ncit.clone()).is_ok());
    for bad in ["unrecognized", "COSM0", "COSM0476"] {
        let mut value = cosmic.clone();
        value["cosmic_id"] = json!(bad);
        assert!(
            serde_json::from_value::<CosmicRecord>(value).is_err(),
            "accepted {bad}"
        );
    }
    for bad in ["unrecognized", "DOID:0000001", "MONDO:1"] {
        let mut value = mondo.clone();
        value["mondo_id"] = json!(bad);
        assert!(
            serde_json::from_value::<ClinGenValidity>(value).is_err(),
            "accepted {bad}"
        );
    }
    for bad in ["unrecognized", "MONDO:0000001", "Cbad"] {
        let mut value = ncit.clone();
        value["nci_thesaurus_concept_id"] = json!(bad);
        assert!(
            serde_json::from_value::<NciDiseaseRef>(value).is_err(),
            "accepted {bad}"
        );
    }
}

#[test]
fn native_vocabulary_members_keep_exact_publisher_values() -> Result<(), serde_json::Error> {
    use biodata::CellLineSex;
    use biomcp_types::{CivicKind, CivicSignificance, CivicStatus, NciTrialPhase};
    assert!(matches!(
        serde_json::from_value::<CellLineSex>(json!("Female"))?,
        CellLineSex::Female
    ));
    assert!(matches!(
        serde_json::from_value::<CivicKind>(json!("PREDICTIVE"))?,
        CivicKind::Predictive
    ));
    assert!(matches!(
        serde_json::from_value::<CivicStatus>(json!("ACCEPTED"))?,
        CivicStatus::Accepted
    ));
    assert!(matches!(
        serde_json::from_value::<CivicSignificance>(json!("RESISTANCE"))?,
        CivicSignificance::Resistance
    ));
    assert!(matches!(
        serde_json::from_value::<NciTrialPhase>(json!("I_II"))?,
        NciTrialPhase::Phase1And2
    ));
    for original in [" new value ", "female", "PHASE1", ""] {
        let phase = serde_json::from_value::<NciTrialPhase>(json!(original))?;
        assert!(phase.is_unknown());
        assert_eq!(serde_json::to_value(phase)?, json!(original));
    }
    Ok(())
}

#[test]
fn normalization_source_text_retains_empty_values_and_whitespace() -> Result<(), serde_json::Error>
{
    let original = json!({"category":" infos ", "code":"", "text":"\n  native text  \n"});
    let message: biomcp_types::NormalizationMessage = serde_json::from_value(original.clone())?;
    assert_eq!(serde_json::to_value(message)?, original);
    Ok(())
}
