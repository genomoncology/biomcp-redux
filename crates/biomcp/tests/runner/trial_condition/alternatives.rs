//! Repeated trial filters preserve each source-native value at the command boundary.
use super::{Stub, invoke, setup};
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
        for count in [1, 5, 6] {
            let mut value = json!({});
            value[field] = json!(vec!["value"; count]);
            if serde_json::from_value::<biomcp_types::TrialSearch>(value).is_ok() != (count <= 5) {
                return Err(format!("wrong boundary for {count} {field} values"));
            }
        }
    }
    Ok(())
}

pub(super) fn nci() -> Result<(), String> {
    let stub = Stub::for_source("clinicaltrialsapi.cancer.gov", "application/json", |request| {
        if !request.contains("diseases.nci_thesaurus_concept_id=C3224&diseases.nci_thesaurus_concept_id=C4872&") {
            return Err(format!("NCI lost repeated native disease fields: {request}"));
        }
        Ok(json!({"total":3,"data":[]}).to_string())
    })?;
    let env = setup(&stub)?;
    let args: Vec<String> = ["search","trial","--source","nci","--condition","C3224","--condition","C4872","--count-only"].iter().map(|v| (*v).into()).collect();
    let value = invoke(&env, &args, 0)?;
    if value.pointer("/page/total") != Some(&json!(3)) || value.pointer("/query/condition") != Some(&json!(["C3224","C4872"])) {
        return Err(format!("NCI lost source-native count or query: {value}"));
    }
    let mut refused = args.clone();
    refused.extend(["--intervention".into(),"pembrolizumab".into()]);
    let value = invoke(&env, &refused, 2)?;
    if value.pointer("/error/option") != Some(&json!("--intervention")) {
        return Err(format!("NCI accepted intervention: {value}"));
    }
    if stub.finish()?.len() != 1 { return Err("refused NCI filter reached source".into()); }
    Ok(())
}

pub(super) fn mismatch() -> Result<(), String> {
    let stub = Stub::for_source("clinicaltrials.gov", "application/json", |request| {
        if request.starts_with("GET /api/v2/version ") {
            return Ok(json!({"dataTimestamp":"2026-10-28T00:00:00"}).to_string());
        }
        // One value matches every native condition; the other value does not.
        // Flattening all values into one string hides which input lost words.
        let conditions = if request.contains("pageSize=1&") {
            json!(["Lung Cancer", "Melanoma"])
        } else {
            json!(["Lung Cancer"])
        };
        Ok(json!({"totalCount":1,"studies":[{"protocolSection":{
            "identificationModule":{"nctId":"NCT00000001","briefTitle":"Synthetic native condition"},
            "statusModule":{"overallStatus":"RECRUITING","lastUpdatePostDateStruct":{"date":"2026-10-28"}},
            "designModule":{},
            "conditionsModule":{"conditions":conditions}
        }}]}).to_string())
    })?;
    let env = setup(&stub)?;
    for (limit, mismatch) in [("1", false), ("2", true)] {
        let args: Vec<String> = ["search","trial","--condition","lung cancer","--condition","melanoma","--limit",limit].iter().map(|v| (*v).into()).collect();
        let value = invoke(&env, &args, 0)?;
        if value.pointer("/sources/0/notices/0").is_some() != mismatch {
            return Err(format!("per-value mismatch was {mismatch}: {value}"));
        }
    }
    stub.finish()?;
    Ok(())
}
