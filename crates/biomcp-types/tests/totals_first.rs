//! 0939: every paged drug and adverse-event reply, and the envelope, states its total before its
//! rows, so an agent that reads only the start of a long reply sees that the rows are one page.
use biomcp_types::ResponseBody;
use serde::{Serialize, de::DeserializeOwned};
use serde_json::{Value, json};

type Outcome = Result<(), Box<dyn std::error::Error>>;

/// Decode synthetic wire JSON, write it again, and require each `first` key before `rows`.
fn totals_first<T: Serialize + DeserializeOwned>(
    wire: Value,
    first: &[&str],
    rows: &str,
) -> Outcome {
    let text = serde_json::to_string(&serde_json::from_value::<T>(wire)?)?;
    let at = |key: &str| {
        text.find(&format!("\"{key}\":"))
            .ok_or(format!("{key} missing: {text}"))
    };
    let rows = at(rows)?;
    for key in first {
        assert!(at(key)? < rows, "{key} follows the rows: {text}");
    }
    Ok(())
}
fn header(id: &str, source: &str) -> Value {
    json!({"id":id,"source":source,"name":"synthetic drug","identity_level":"unknown"})
}

#[test]
fn drug_search_pages_state_their_total_before_their_rows() -> Outcome {
    use biomcp_types::{DrugFdaPage, DrugPage, DrugRecallPage, DrugWhoPage};
    totals_first::<DrugPage>(
        json!({"items":[header("CHEMBL1", "chembl")],"total":42}),
        &["total"],
        "items",
    )?;
    totals_first::<DrugFdaPage>(
        json!({"items":[{"id":"NDA000001","identity":header("NDA000001", "drugsfda")}],"total":42}),
        &["total"],
        "items",
    )?;
    let who = json!({"reference":"SY001","inn_dosage_form_strength":"synthetic","product_type":"synthetic",
        "therapeutic_area":"synthetic","applicant":"synthetic","dosage_form":"synthetic","basis_of_listing":"synthetic",
        "date_of_prequalification":""});
    totals_first::<DrugWhoPage>(json!({"items":[who],"total":42}), &["total"], "items")?;
    totals_first::<DrugRecallPage>(
        json!({"count_unit":"recall-records","items":[{"recall_number":"D-0001-2024"}],"total":42}),
        &["total"],
        "items",
    )
}
#[test]
fn drug_civic_and_section_pages_state_their_total_before_their_rows() -> Outcome {
    let claim = json!({"claim":{"record":{"namespace":"unknown","id":"EID1"},"kind":"PREDICTIVE","status":"ACCEPTED"},
        "therapies":[header("5", "civic")]});
    totals_first::<biomcp_types::DrugCivicPage>(
        json!({"records":[claim],"page":{"returned":1,"total":169}}),
        &["page"],
        "records",
    )?;
    totals_first::<biomcp_types::DrugSectionReply<Value>>(
        json!({"status":"complete","result":{"records":[{"synthetic":1}]},"page":{"offset":0,"returned":1,"total":614}}),
        &["page"],
        "result",
    )
}
#[test]
fn adverse_event_pages_state_their_total_before_their_rows() -> Outcome {
    use biomcp_types::{
        AdverseEventIdentifierRecords, AdverseEventPage, AdverseEventRecords, Envelope,
    };
    totals_first::<AdverseEventRecords<Value>>(
        json!({"records":[{"index":18}],"offset":18,"total":19}),
        &["offset", "total"],
        "records",
    )?;
    totals_first::<AdverseEventIdentifierRecords>(
        json!({"drug_index":0,"records":["synthetic"],"offset":0,"total":3}),
        &["offset", "total"],
        "records",
    )?;
    // A search's total lives in the envelope's page, which now precedes the result.
    totals_first::<Envelope<AdverseEventPage>>(
        json!({"schema":AdverseEventPage::SCHEMA,"status":"complete","query":{"text":"synthetic"},
        "result":{"source_query":"synthetic","items":[{"id":"1","source":"faers","version":"1"}],"unit":"reports"},
        "page":{"offset":0,"returned":1,"total":105020}}),
        &["page"],
        "result",
    )
}
