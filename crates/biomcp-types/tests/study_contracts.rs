//! study-types: public no-I/O contracts from retained cBioPortal metadata shapes.
use biodata::{Study, StudyId, StudySampleList};
use biomcp_types::{StudyGet, StudySearch};
use serde_json::{Value, json};

fn roundtrip<T: serde::Serialize + serde::de::DeserializeOwned>(v: Value) -> Result<Value, serde_json::Error> {
    serde_json::to_value(serde_json::from_value::<T>(v)?)
}
fn header() -> Value {
    json!({"studyId":"brca_tcga_pan_can_atlas_2018","name":"Synthetic study","cancerTypeId":"brca","allSampleCount":1084,"pmids":["29625048","29596782","29625048"],"referenceGenome":"hg19"})
}
fn study() -> Value {
    json!({"header":header(),"description":"Synthetic source description","sequencedSampleCount":1066,"cnaSampleCount":1080,"structuralVariantCount":0,"treatmentCount":0,"resourceCounts":[],"importDate":"2026-08-20 21:46:46","groups":"","readPermission":true,"publicStudy":true})
}
#[test]
fn exact_ids_roundtrip_without_case_or_punctuation_repair() -> Result<(), Box<dyn std::error::Error>> {
    for value in ["brca_tcga_pan_can_atlas_2018", "Mixed-Study.1"] {
        let id: StudyId = value.parse()?;
        assert_eq!(id.as_str(), value);
        assert_eq!(id.to_string(), value);
        assert_eq!(roundtrip::<StudyId>(json!(value))?, value);
    }
    assert_ne!("BRCA".parse::<StudyId>()?, "brca".parse::<StudyId>()?);
    for value in ["", " brca", "brca ", "../brca", "a/b", "a?x", "a%2Fb", "a\nb"] {
        assert!(value.parse::<StudyId>().is_err());
        assert!(serde_json::from_value::<StudyGet>(json!({"id":value})).is_err());
    }
    Ok(())
}
#[test]
fn native_counts_and_publication_multiplicity_survive_without_member_ids() -> Result<(), Box<dyn std::error::Error>> {
    let v = study();
    assert_eq!(roundtrip::<Study>(v.clone())?, v);
    let mut zero = v;
    zero["patientCount"] = json!(0);
    assert_eq!(roundtrip::<Study>(zero.clone())?, zero);
    let list = json!({"sampleListId":"brca_tcga_pan_can_atlas_2018_all","studyId":"brca_tcga_pan_can_atlas_2018","name":"Synthetic list","description":"Synthetic description","category":"all_cases_in_study","sampleCount":1084});
    assert_eq!(roundtrip::<StudySampleList>(list.clone())?, list);
    let mut absent = list.clone();
    absent.as_object_mut().ok_or("object")?.remove("sampleCount");
    assert_eq!(roundtrip::<StudySampleList>(absent.clone())?, absent);
    let mut forbidden = list;
    forbidden["sampleIds"] = json!(["synthetic-member"]);
    assert!(serde_json::from_value::<StudySampleList>(forbidden).is_err());
    Ok(())
}
#[test]
fn literal_search_and_bounded_section_requests_validate_before_io() -> Result<(), Box<dyn std::error::Error>> {
    let q: StudySearch = serde_json::from_value(json!({"text":"breast AND lung","cancer_type":"mixed","limit":2,"offset":3,"pool":"snapshot"}))?;
    assert_eq!(q.text().map(biodata::Text::as_str), Some("breast AND lung"));
    assert_eq!(q.cancer_type().map(|v| v.as_str()), Some("mixed"));
    assert_eq!((q.limit(), q.offset()), (2,3));
    assert_eq!(q.at_offset(5).pool(), q.pool());
    assert!(serde_json::from_value::<StudySearch>(json!({"cancer_type":"brca"})).is_ok());
    for v in [json!({}),json!({"text":" "}),json!({"text":"x","limit":0}),json!({"text":"x","limit":51}),json!({"text":"x","offset":-1})] {
        assert!(serde_json::from_value::<StudySearch>(v).is_err());
    }
    for section in ["profiles","sample-lists","files","all"] {
        let q: StudyGet = serde_json::from_value(json!({"id":"brca_tcga_pan_can_atlas_2018","section":section,"section_limit":2}))?;
        assert_eq!(q.id().as_str(), "brca_tcga_pan_can_atlas_2018");
        if section == "all" { assert!(q.at_section_offset(2).is_err()); }
        else { assert_eq!(q.at_section_offset(2)?.section_offset(),2); }
    }
    for v in [json!({"id":"brca","section":"counts"}),json!({"id":"brca","gene":"TP53"}),json!({"id":"brca","section_limit":0}),json!({"id":"brca","section_limit":null}),json!({"id":"brca","section":"all","section_offset":1})] {
        assert!(serde_json::from_value::<StudyGet>(v).is_err());
    }
    Ok(())
}
