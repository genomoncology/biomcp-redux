//! Repeated trial filters preserve each source-native value at the command boundary.
use super::super::{Stub, invoke, setup};
use serde_json::json;

pub(super) fn run() -> Result<(), String> {
    let stub = Stub::for_source("clinicaltrials.gov", "application/json", |request| {
        if request.starts_with("GET /api/v2/version ") {
            return Ok(json!({"dataTimestamp":"2026-10-28T00:00:00"}).to_string());
        }
        for clause in [
            "query.cond=%28lung%20cancer%29%20OR%20%28melanoma%29",
            "query.intr=%28radiation%20therapy%29%20OR%20%28pembrolizumab%29",
            "query.term=%28quality%20of%20life%29%20OR%20%28survival%29",
            "filter.advanced=%28%28AREA%5BCondition%5DEXPANSION%5BNone%5D%22lung%20cancer%22%29%20OR%20%28AREA%5BCondition%5DEXPANSION%5BNone%5D%22melanoma%22%29%29",
        ] {
            if !request.contains(clause) {
                return Err(format!("missing OR clause {clause}: {request}"));
            }
        }
        Ok(json!({"totalCount":1,"studies":[]}).to_string())
    })?;
    let env = setup(&stub)?;
    let args: Vec<String> = ["search","trial","--condition","lung cancer","--condition","melanoma","--intervention","radiation therapy","--intervention","pembrolizumab","--keyword","quality of life","--keyword","survival","--exact-condition"].iter().map(|v| (*v).into()).collect();
    let value = invoke(&env, &args, 0)?;
    for (field, expected) in [("condition",json!(["lung cancer","melanoma"])),("intervention",json!(["radiation therapy","pembrolizumab"])),("keyword",json!(["quality of life","survival"]))] {
        if value.pointer(&format!("/query/{field}")) != Some(&expected) {
            return Err(format!("lost echoed values: {value}"));
        }
    }
    stub.finish()?;
    for field in ["condition", "intervention", "keyword"] {
        let mut value = json!({});
        value[field] = json!(["a","b","c","d","e","f"]);
        if serde_json::from_value::<biomcp_types::TrialSearch>(value).is_ok() {
            return Err(format!("accepted sixth {field}"));
        }
    }
    Ok(())
}
