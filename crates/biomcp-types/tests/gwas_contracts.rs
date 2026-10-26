//! Public GWAS value and request contracts before command registration.
use biodata::{GwasAssociationId, GwasStudyId, GwasTraitId, ScientificPValue};
use biomcp_types::{GwasGet, GwasSearch};
use serde_json::json;

#[test]
fn identities_keep_native_namespaces_and_refuse_malformed_values() {
    for value in ["GCST000035", "GCST90000001"] {
        assert!(serde_json::from_value::<GwasStudyId>(json!(value)).is_ok());
    }
    for value in ["GCST", "gcst000035", "GCST-35", "GCST000000"] {
        assert!(serde_json::from_value::<GwasStudyId>(json!(value)).is_err());
    }
    assert!(serde_json::from_value::<GwasAssociationId>(json!("association:6110")).is_ok());
    assert!(serde_json::from_value::<GwasAssociationId>(json!("association:0")).is_err());
    for value in ["MONDO_0004989", "EFO_0000305", "HP:0001250"] {
        assert!(serde_json::from_value::<GwasTraitId>(json!(value)).is_ok());
    }
    assert!(serde_json::from_value::<GwasTraitId>(json!("0004989")).is_err());
}

#[test]
fn exact_probability_keeps_underflow_and_threshold_equality()
-> Result<(), Box<dyn std::error::Error>> {
    for value in ["2e-76", "2e-28539", "1.250e-300"] {
        let number: ScientificPValue = value.parse()?;
        let wire = serde_json::to_value(&number)?;
        let decoded: ScientificPValue = serde_json::from_value(wire)?;
        assert_eq!(decoded, number);
    }
    let tiny: ScientificPValue = "2e-28539".parse()?;
    let ordinary: ScientificPValue = "2e-76".parse()?;
    assert!(tiny < ordinary);
    assert!(ordinary <= "2e-76".parse()?);
    for value in ["0e-76", "-2e-76", "2e1", "NaN", "2e", "2"] {
        assert!(value.parse::<ScientificPValue>().is_err());
    }
    Ok(())
}

#[test]
fn selectors_and_sections_reject_invalid_requests_before_io()
-> Result<(), Box<dyn std::error::Error>> {
    for request in [
        json!({}),
        json!({"gene":"FGFR2","include_child_traits":true}),
        json!({"trait":"breast carcinoma","extended_geneset":true}),
        json!({"gene":"FGFR2","limit":0}),
        json!({"gene":"FGFR2","sort":"other"}),
    ] {
        assert!(serde_json::from_value::<GwasSearch>(request).is_err());
    }
    let search: GwasSearch = serde_json::from_value(json!({"trait":"breast carcinoma"}))?;
    assert_eq!((search.limit(), search.offset()), (3, 0));
    assert_eq!(search.at_offset(3).offset(), 3);
    for request in [
        json!({"kind":"association","id":"association:6110","section":"ancestries"}),
        json!({"kind":"study","id":"GCST000035","section":"loci"}),
    ] {
        assert!(serde_json::from_value::<GwasGet>(request).is_err());
    }
    for request in [
        json!({"kind":"association","id":"association:6110","section":"loci"}),
        json!({"kind":"study","id":"GCST000035","section":"associations"}),
        json!({"kind":"study","id":"GCST000035","section":"ancestries"}),
    ] {
        assert!(serde_json::from_value::<GwasGet>(request).is_ok());
    }
    Ok(())
}

#[test]
fn one_association_record_preserves_arrays_unknown_frequency_and_effects()
-> Result<(), Box<dyn std::error::Error>> {
    let value = json!({"id":6110,"study":"GCST000035","traits":["Whole synthetic trait","Second trait"],"ontology_traits":[{"id":"MONDO_0004989","label":"Native label"}],"alleles":[{"rs_id":"rs2981582","allele":"A"},{"rs_id":"rs1","allele":"T"}],"p_value":0.0,"exact_p_value":{"mantissa":"2","exponent":-28539},"odds_ratio":1.26,"beta":"source beta text","risk_frequency":"NR","publication":"17529967","author":"Native author","mapped_genes":["FGFR2","intergenic"],"locations":["10:123"],"ci_lower":1.1,"ci_upper":1.4});
    let row: biomcp_types::GwasAssociation = serde_json::from_value(value.clone())?;
    assert_eq!(serde_json::to_value(&row)?, value);
    let card: biomcp_types::GwasAssociationCard =
        serde_json::from_value(json!({"association":value.clone()}))?;
    assert_eq!(card.association, row);
    let page: biomcp_types::GwasPage =
        serde_json::from_value(json!({"items":[value],"page":{"limit":3,"offset":0,"total":1}}))?;
    assert_eq!(page.items.first(), Some(&row));
    Ok(())
}

#[cfg(feature = "schema")]
#[test]
fn consumer_schemas_are_standalone_and_keep_exact_probability_contracts()
-> Result<(), Box<dyn std::error::Error>> {
    use biomcp_types::*;
    use schemars::schema_for;
    for schema in [
        schema_for!(Envelope<GwasPage>),
        schema_for!(Envelope<GwasAssociationCard>),
        schema_for!(Envelope<GwasLoci>),
        schema_for!(Envelope<GwasStudy>),
        schema_for!(Envelope<GwasAncestries>),
    ] {
        jsonschema::draft202012::new(&serde_json::to_value(schema)?)?;
    }
    let schema = serde_json::to_value(schema_for!(ScientificPValue))?;
    let validator = jsonschema::draft202012::new(&schema)?;
    assert!(validator.is_valid(&json!({"mantissa":"2","exponent":-28539})));
    assert!(!validator.is_valid(&json!({"mantissa":"0","exponent":-76})));
    Ok(())
}
