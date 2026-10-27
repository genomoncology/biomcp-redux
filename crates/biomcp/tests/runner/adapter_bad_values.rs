//! Features variant and drug-ema: rejected optional source text keeps the reply usable.
use super::{configuration::invoke, network_stub::Stub, response_cases::checked_steps};
use libtest_mimic::Trial;
use serde_json::{Value, json};

pub(super) fn trials() -> Vec<Trial> {
    vec![
        Trial::test("variant/empty-clinvar-text-keeps-assertion", || {
            clinvar().map_err(Into::into)
        }),
        Trial::test("drug-ema/empty-text-keeps-medicine", || {
            ema().map_err(|e| e.to_string().into())
        }),
    ]
}
fn check(value: &Value, field: &str, absent: &str) -> Result<(), String> {
    let problems = value
        .get("problems")
        .and_then(Value::as_array)
        .ok_or("missing problems")?;
    if problems.len() != 1
        || problems.first().and_then(|p| p.get("field")) != Some(&json!(field))
        || value.pointer(absent).is_some_and(|v| !v.is_null())
    {
        return Err(format!(
            "bad {field} survived or lost its single problem: {value}"
        ));
    }
    Ok(())
}
fn clinvar() -> Result<(), String> {
    let stub = Stub::for_source("eutils.ncbi.nlm.nih.gov", "application/xml", |request| {
        super::variant_assertion_wire::clinvar_body(request).map(|xml| {
            xml.replace(
                "<ReviewStatus>criteria provided, single submitter</ReviewStatus>",
                "<ReviewStatus></ReviewStatus>",
            )
        })
    })?;
    let env = super::variant::source_environment(&stub)?;
    let args = [
        "variant",
        "assertions",
        "clinvar:128042",
        "--source",
        "clinvar",
        "--assertion",
        "SCV000611258",
        "--version",
        "1",
    ]
    .map(str::to_owned);
    let value = invoke(&env, args.to_vec(), 0)?;
    if value.pointer("/result/assertions/0/statement/attribution/record")
        != Some(&json!("SCV000611258"))
    {
        return Err(format!("assertion lost: {value}"));
    }
    check(
        &value,
        "review_status",
        "/result/assertions/0/statement/strength",
    )?;
    if stub.finish()?.len() != 1 {
        return Err("unexpected ClinVar requests".into());
    }
    Ok(())
}
fn ema() -> Result<(), Box<dyn std::error::Error>> {
    let value = checked_steps(
        "drug/ema-synthetic",
        |source, capture| {
            if source != "ema" {
                return;
            }
            let Some(body) = capture.pointer_mut("/exchanges/0/body/value") else {
                return;
            };
            let Some(text) = body.as_str() else {
                return;
            };
            let Ok(mut rows) = serde_json::from_str::<Value>(text) else {
                return;
            };
            if let Some(rows) = rows.get_mut("data").and_then(Value::as_array_mut) {
                for row in rows {
                    if row.get("ema_product_number") == Some(&json!("EMEA/H/C/004124")) {
                        if let Some(field) = row.get_mut("therapeutic_indication") {
                            *field = json!("");
                        }
                    }
                }
            }
            *body = json!(rows.to_string());
        },
        json!([
            {"run":["sync","ema"], "exit":0},
            {"run":["get","drug","EMEA/H/C/004124","regulatory","--source","ema"], "exit":0,
             "expect":{"/result/identity/id":"EMEA/H/C/004124"}}
        ]),
    )?;
    check(
        &value,
        "therapeutic_indication",
        "/result/regulatory/data/records/0/indication",
    )?;
    Ok(())
}
