//! Feature cell-line-types: source-owned cell identities and standalone contracts before I/O.
use biodata::*;
use biomcp_types::*;
use serde_json::{Value, json};
fn roundtrip<T: serde::Serialize + serde::de::DeserializeOwned>(v: Value) -> Value {
    serde_json::to_value(serde_json::from_value::<T>(v).unwrap()).unwrap()
}
#[test]
fn checked_cell_identifiers_keep_their_exact_namespace() {
    for id in ["CVCL_2119", "CVCL_A794", "CVCL_E3VV", "CVCL_ZZZZ"] {
        assert_eq!(roundtrip::<CellosaurusAccession>(json!(id)), id);
        let parsed: CellosaurusAccession = id.parse().unwrap();
        assert_eq!(parsed.to_string().parse::<CellosaurusAccession>().unwrap(), parsed);
    }
    for id in ["CVCL_1", "CVCL_12345", "cvcl_2119", "CVCL_21-9", "CVCL_2119 "] {
        assert!(serde_json::from_value::<CellosaurusAccession>(json!(id)).is_err());
    }
    for id in ["DepMap:ACH-000362", "ChEMBL-Cells:CHEMBL3706573", "HGNC:HGNC:7132", "NCIt:C8263", "ORDO:Orphanet_514"] {
        assert_eq!(roundtrip::<CellLineSourceId>(json!(id)), id);
        let parsed: CellLineSourceId = id.parse().unwrap();
        assert_eq!(parsed.to_string().parse::<CellLineSourceId>().unwrap(), parsed);
    }
    for id in ["ACH-000362", "CHEMBL3706573", ":foo", "DepMap:", "DepMap:ACH 1"] {
        assert!(serde_json::from_value::<CellLineSourceId>(json!(id)).is_err());
    }
    for id in ["CHEMBL3706573", "CHEMBL3308063"] {
        assert_eq!(roundtrip::<ChemblCellId>(json!(id)), id);
        let parsed: ChemblCellId = id.parse().unwrap();
        assert_eq!(parsed.to_string().parse::<ChemblCellId>().unwrap(), parsed);
    }
    for id in ["CHEMBL", "chembl3706573", "CHEMBL37x", "chembl:CHEMBL3706573"] {
        assert!(serde_json::from_value::<ChemblCellId>(json!(id)).is_err());
    }
}
#[test]
fn requests_reject_bad_bounds_and_identities_before_io() {
    let q: CellLineSearch = serde_json::from_value(json!({"query":"MV4;11"})).unwrap();
    assert_eq!(q.query().as_str(), "MV4;11"); assert_eq!(q.limit(),5); assert_eq!(q.offset(),0);
    for v in [json!({}),json!({"query":""}),json!({"query":"x","limit":0}),json!({"query":"x","limit":26}),json!({"query":"x","offset":-1}),json!({"query":"x","sort":"name"})] {
        assert!(serde_json::from_value::<CellLineSearch>(v).is_err());
    }
    for section in ["names","identity","xrefs","variants","chembl","all"] {
        roundtrip::<CellLineGet>(json!({"id":"CVCL_2119","section":section,"section_limit":25,"section_offset":2}));
    }
    roundtrip::<CellLineGet>(json!({"id":"DepMap:ACH-000362"}));
    for v in [json!({"id":"MOLM13"}),json!({"id":"CVCL_12"}),json!({"id":["CVCL_2119","CVCL_0004"]}),json!({"id":"CVCL_2119","section_limit":0}),json!({"id":"CVCL_2119","section_limit":26}),json!({"id":"CVCL_2119","section_offset":-1}),json!({"id":"CVCL_2119","section":"drug_response"})] {
        assert!(serde_json::from_value::<CellLineGet>(v).is_err());
    }
    roundtrip::<CellLineAssayCountQuery>(json!({"id":"CHEMBL3706573"}));
    for id in ["CVCL_2119", "ChEMBL-Cells:CHEMBL3706573", "MOLM13"] {
        assert!(serde_json::from_value::<CellLineAssayCountQuery>(json!({"id":id})).is_err());
    }
}
