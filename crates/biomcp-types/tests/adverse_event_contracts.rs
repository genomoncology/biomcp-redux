//! adverse-event-types: caller-owned no-I/O report contracts.
use biodata::{FaersReport, MaudeReport};
use biomcp_types::{AdverseEventGet, AdverseEventSearch};
use serde::{Serialize, de::DeserializeOwned};
use serde_json::{Value, json};
fn roundtrip<T: Serialize + DeserializeOwned>(value: Value) -> Value {
    serde_json::to_value(serde_json::from_value::<T>(value).unwrap()).unwrap()
}
#[test]
fn source_requests_refuse_cross_family_filters_sections_and_invented_ids() {
    assert_eq!(roundtrip::<AdverseEventSearch>(json!({"text":"synthetic"}))["source"],"faers");
    for value in [json!({"text":"x","source":"maude","reaction":"x"}),json!({"text":"x","serious":"disability"}),json!({"text":"x","received_from":"2024-02-30"}),json!({"text":"x","received_from":"2024-02-02","received_to":"2024-01-01"})] {
        assert!(serde_json::from_value::<AdverseEventSearch>(value).is_err());
    }
    for value in [json!({"id":"FAERS:26940567"}),json!({"id":"26940567","section":"devices"}),json!({"id":"50952244","source":"maude","section":"drugs"}),json!({"id":"26940567","drug_index":0}),json!({"id":"26940567","section":"unii"}),json!({"id":"26940567","version":"1"})] {
        assert!(serde_json::from_value::<AdverseEventGet>(value).is_err());
    }
}
#[test]
fn report_records_preserve_array_order_empty_strings_and_missing_reporter_qualification() {
    let drugs:Vec<_>=(0..19).map(|n|json!({"medicinalproduct":format!("synthetic-{n}"),"openfda":{"unii":["A","B"]}})).collect();
    let reactions:Vec<_>=(0..22).map(|n|json!({"term":{"label":format!("synthetic reaction {n}")},"outcome":"1"})).collect();
    let report=json!({"safetyreportid":"26940567","safetyreportversion":"1","receivedate":"20240101","receiptdate":"20240201","seriousness":{"death":"1","hospitalization":"1"},"reporter":{"country":""},"patient":{"age":"","age_unit":"801"},"drugs":drugs,"reactions":reactions});
    assert_eq!(roundtrip::<FaersReport>(report.clone()),report);
    for length in [0,5] {
        let narrative:Vec<_>=(0..length).map(|n|json!({"text_type_code":"N","text":format!("Synthetic narrative {n}")})).collect();
        let device=json!({"mdr_report_key":"50952244","devices":[{"brand_name":""},{"brand_name":"synthetic"}],"narrative":narrative});
        assert_eq!(roundtrip::<MaudeReport>(device.clone()),device);
    }
}
