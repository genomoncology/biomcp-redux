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
        assert_eq!(
            parsed.to_string().parse::<CellosaurusAccession>().unwrap(),
            parsed
        );
    }
    for id in [
        "CVCL_1",
        "CVCL_12345",
        "cvcl_2119",
        "CVCL_21-9",
        "CVCL_2119 ",
    ] {
        assert!(serde_json::from_value::<CellosaurusAccession>(json!(id)).is_err());
    }
    for id in [
        "DepMap:ACH-000362",
        "ChEMBL-Cells:CHEMBL3706573",
        "HGNC:HGNC:7132",
        "NCIt:C8263",
        "ORDO:Orphanet_514",
    ] {
        assert_eq!(roundtrip::<CellLineSourceId>(json!(id)), id);
        let parsed: CellLineSourceId = id.parse().unwrap();
        assert_eq!(
            parsed.to_string().parse::<CellLineSourceId>().unwrap(),
            parsed
        );
    }
    for id in [
        "ACH-000362",
        "CHEMBL3706573",
        ":foo",
        "DepMap:",
        "DepMap:ACH 1",
    ] {
        assert!(serde_json::from_value::<CellLineSourceId>(json!(id)).is_err());
    }
    for id in ["CHEMBL3706573", "CHEMBL3308063"] {
        assert_eq!(roundtrip::<ChemblCellId>(json!(id)), id);
        let parsed: ChemblCellId = id.parse().unwrap();
        assert_eq!(parsed.to_string().parse::<ChemblCellId>().unwrap(), parsed);
    }
    for id in [
        "CHEMBL",
        "chembl3706573",
        "CHEMBL37x",
        "chembl:CHEMBL3706573",
    ] {
        assert!(serde_json::from_value::<ChemblCellId>(json!(id)).is_err());
    }
}
#[test]
fn requests_reject_bad_bounds_and_identities_before_io() {
    let q: CellLineSearch = serde_json::from_value(json!({"query":"MV4;11"})).unwrap();
    assert_eq!(q.query().as_str(), "MV4;11");
    assert_eq!(q.limit(), 5);
    assert_eq!(q.offset(), 0);
    for v in [
        json!({}),
        json!({"query":""}),
        json!({"query":"x","limit":0}),
        json!({"query":"x","limit":26}),
        json!({"query":"x","offset":-1}),
        json!({"query":"x","sort":"name"}),
    ] {
        assert!(serde_json::from_value::<CellLineSearch>(v).is_err());
    }
    for section in ["names", "identity", "xrefs", "variants", "chembl", "all"] {
        roundtrip::<CellLineGet>(
            json!({"id":"CVCL_2119","section":section,"section_limit":25,"section_offset":2}),
        );
    }
    roundtrip::<CellLineGet>(json!({"id":"DepMap:ACH-000362"}));
    for v in [
        json!({"id":"MOLM13"}),
        json!({"id":"CVCL_12"}),
        json!({"id":["CVCL_2119","CVCL_0004"]}),
        json!({"id":"CVCL_2119","section_limit":0}),
        json!({"id":"CVCL_2119","section_limit":26}),
        json!({"id":"CVCL_2119","section_offset":-1}),
        json!({"id":"CVCL_2119","section":"drug_response"}),
    ] {
        assert!(serde_json::from_value::<CellLineGet>(v).is_err());
    }
    roundtrip::<CellLineAssayCountQuery>(json!({"id":"CHEMBL3706573"}));
    for id in ["CVCL_2119", "ChEMBL-Cells:CHEMBL3706573", "MOLM13"] {
        assert!(serde_json::from_value::<CellLineAssayCountQuery>(json!({"id":id})).is_err());
    }
}

// Synthetic contract rows preserve only the multiplicity and absence observed in the
// retained capture archives. They are not native replies or curation answers.
fn xref(database: &str, accession: &str) -> Value {
    json!({"database":database,"accession":accession})
}
#[test]
fn whole_variations_keep_fusion_members_and_non_pubmed_provenance() {
    let fusion = json!({"variation_type":"Gene fusion","description":"source fusion spelling",
        "genes":[{"reference":xref("HGNC","HGNC:7132"),"hgnc":"HGNC:7132"},{"reference":xref("HGNC","HGNC:7136"),"hgnc":"HGNC:7136"}],
        "references":[],"provenance":[{"reference":{"resource_internal_ref":"PubMed=9305600"}},{"institution":"Example institution","value":"Native value"}]});
    assert_eq!(roundtrip::<CellLineVariation>(fusion.clone()), fusion);
    let variation = json!({"variation_type":"Native future label","mutation_type":"Native future mutation",
        "description":"native coordinates without assembly","note":"Complete source splice note","zygosity":"Native future zygosity",
        "genes":[{"reference":xref("HGNC","HGNC:7989"),"hgnc":"HGNC:7989"}],
        "references":[xref("ClinVar","VCV000375874")],
        "provenance":[{"xref":xref("DepMap","ACH-000336")},{"xref":xref("Cosmic-CLP","1290455")}]});
    let row = roundtrip::<CellLineVariation>(variation.clone());
    assert_eq!(row, variation);
    assert!(row.get("allele").is_none());
    assert!(row["provenance"][0].get("reference").is_none());
    let page = json!({"accession":"CVCL_1844","items":{"items":[variation],"total":3},"offset":2});
    assert_eq!(roundtrip::<CellLineVariants>(page.clone()), page);
}
#[test]
fn collections_keep_independent_totals_and_complete_qualified_text() {
    let warning = json!({"scope":"Native warning category","text":"Some initial stocks had the source problem. Subsequent stocks were corrected."});
    assert_eq!(roundtrip::<CellLineWarning>(warning.clone()), warning);
    let page = json!({"accession":"CVCL_0007","species":{"items":[xref("NCBI_TaxID","9606")],"total":1},
        "diseases":{"items":[xref("NCIt","C8263"),xref("ORDO","Orphanet_514")],"total":2},
        "warnings":{"items":[warning],"total":1},"offset":0});
    assert_eq!(roundtrip::<CellLineIdentityPage>(page.clone()), page);
    let page = json!({"accession":"CVCL_0004","items":{"items":[xref("Example","entry-1")],"total":1398},"offset":1397});
    assert_eq!(roundtrip::<CellLineXrefs>(page.clone()), page);
    let name = json!({"kind":"identifier","value":"MV4;11"});
    assert_eq!(roundtrip::<CellLineName>(name.clone()), name);
    let dates = json!({"created":"2012-04-04","updated":"2026-06-25","version":48});
    assert_eq!(roundtrip::<CellLineEntryDates>(dates.clone()), dates);
}
#[test]
fn chembl_absence_and_synthetic_zero_remain_source_local() {
    let record =
        json!({"id":"CHEMBL3706573","cell_id":5547,"cellosaurus_id":"CVCL_2119","name":"MOLM-13"});
    let got = roundtrip::<CellLineChemblRecord>(record.clone());
    assert_eq!(got, record);
    assert!(got.get("clo").is_none());
    assert!(got.get("efo").is_none());
    for count in [0, 1408] {
        let value = json!({"id":"CHEMBL3706573","assay_count":count});
        assert_eq!(roundtrip::<CellLineAssayCount>(value.clone()), value);
    }
    let skipped: Section<CellLineChembl> = Section::Skipped {
        source: Source::Chembl,
        reason: Reason::Disabled,
    };
    assert_eq!(serde_json::to_value(skipped).unwrap()["state"], "skipped");
}
