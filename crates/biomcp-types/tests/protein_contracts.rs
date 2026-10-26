//! Feature protein-types: neutral native records and checked standalone protein inputs.
use biodata::*;
use biomcp_types::*;
use serde_json::{Value, json};

fn roundtrip<T: serde::Serialize + serde::de::DeserializeOwned>(
    value: Value,
) -> Result<Value, Box<dyn std::error::Error>> {
    Ok(serde_json::to_value(serde_json::from_value::<T>(value)?)?)
}

#[test]
fn stated_isoforms_and_residues_roundtrip_without_imputation()
-> Result<(), Box<dyn std::error::Error>> {
    for id in ["P15056", "P15056-2", "P00533-1", "A0A024RBG1-12"] {
        assert_eq!(roundtrip::<UniProtIsoform>(json!(id))?, id);
    }
    for id in ["P15056-0", "P15056-01", "P15056-", "P15056-2-1", "P15056.1"] {
        assert!(serde_json::from_value::<UniProtIsoform>(json!(id)).is_err());
    }
    assert_eq!(roundtrip::<UniProtIsoform>(json!("P15056"))?, "P15056");
    assert_eq!(
        roundtrip::<AminoAcids>(json!("ACDEFGHIKLMNPQRSTVWYUOX"))?,
        "ACDEFGHIKLMNPQRSTVWYUOX"
    );
    for residues in ["", "a", "B", "J", "Z", "*", "A C"] {
        assert!(serde_json::from_value::<AminoAcids>(json!(residues)).is_err());
    }
    assert_eq!(
        Error::ProteinResidueInvalid.code(),
        "protein.residue.invalid"
    );
    assert_eq!(Error::ProteinSiteOrder.code(), "protein.site.order");
    assert_eq!(
        Error::UniProtIsoformInvalid.code(),
        "id.uniprot_isoform.invalid"
    );
    for id in ["IPR000001", "IPR052999"] {
        assert_eq!(roundtrip::<InterProId>(json!(id))?, id);
    }
    for id in ["IPR1", "IPR00000x", "interpro:IPR000001"] {
        assert!(serde_json::from_value::<InterProId>(json!(id)).is_err());
    }
    assert_eq!(roundtrip::<ComplexPortalId>(json!("CPX-715"))?, "CPX-715");
    for id in ["CPX-0", "CPX-0715", "cpx-715", "CPX-715x"] {
        assert!(serde_json::from_value::<ComplexPortalId>(json!(id)).is_err());
    }
    Ok(())
}

#[test]
fn checked_requests_refuse_conflicts_and_bounds_before_io() -> Result<(), Box<dyn std::error::Error>>
{
    for value in [
        json!({"query":"EGFR"}),
        json!({"gene":"CDKN2A","reviewed":true,"limit":25,"offset":5,"pool":"retained"}),
    ] {
        let v = roundtrip::<ProteinSearch>(value)?;
        assert!(v.get("include_unreviewed").is_none());
    }
    for value in [
        json!({}),
        json!({"query":"EGFR","gene":"EGFR"}),
        json!({"query":"EGFR","limit":0}),
        json!({"gene":"BRAF","limit":26}),
        json!({"query":"EGFR","include_unreviewed":true}),
        json!({"query":null,"gene":"BRAF"}),
        json!({"query":"x","all_species":true}),
    ] {
        assert!(
            serde_json::from_value::<ProteinSearch>(value.clone()).is_err(),
            "{value}"
        );
    }
    for section in [
        "features",
        "domains",
        "isoforms",
        "complexes",
        "structures",
        "function",
        "sequence",
        "all",
    ] {
        roundtrip::<ProteinGet>(json!({"input":["P00533", "P38398-2"],"section":[section]}))?;
    }
    roundtrip::<ProteinGet>(json!({"input":["CDKN2A"]}))?;
    for value in [
        json!({"input":[]}),
        json!({"input":["P00533"],"section_limit":0}),
        json!({"input":["P00533"],"section":["associations"]}),
    ] {
        assert!(serde_json::from_value::<ProteinGet>(value).is_err());
    }
    roundtrip::<ProteinFeaturesQuery>(
        json!({"accession":"P00533","position":858,"type":"DNA binding"}),
    )?;
    assert!(
        serde_json::from_value::<ProteinFeaturesQuery>(json!({"accession":"P00533","position":0}))
            .is_err()
    );
    assert!(
        serde_json::from_value::<ProteinDomainsQuery>(json!({"accession":"P00533","position":858}))
            .is_err()
    );
    assert!(
        serde_json::from_value::<ProteinComplexMembersQuery>(json!({"accession":"P15056"}))
            .is_err()
    );
    roundtrip::<ProteinComplexMembersQuery>(json!({"accession":"P15056","complex":"CPX-715"}))?;
    roundtrip::<ProteinSequenceQuery>(json!({"isoform":"P38398-2"}))?;
    Ok(())
}

#[test]
fn uncertain_feature_bounds_keep_source_ranges() -> Result<(), Box<dyn std::error::Error>> {
    let feature = json!({"protein":"P00533-1","span":{"start":{"min":711,"max":711},"end":{"min":979}},"kind":{"label":"Domain"},"original":"L","alternative":["R"],"evidence":["ECO:0000269|PubMed:1"]});
    assert_eq!(roundtrip::<UniProtFeature>(feature.clone())?, feature);
    for bound in [json!({}), json!({"min":2,"max":1})] {
        assert!(serde_json::from_value::<Bound>(bound).is_err());
    }
    let span = UncertainSpan0::from_inclusive(
        Bound::new(Some(712), Some(714))?,
        Bound::new(Some(979), Some(979))?,
    )?;
    assert_eq!(
        serde_json::to_value(span)?,
        json!({"start":{"min":711,"max":713},"end":{"min":979,"max":979}})
    );
    Ok(())
}
