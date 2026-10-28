//! Features variant and drug-ema: rejected optional source text keeps the reply usable.
use super::super::{configuration::invoke, network_stub::Stub, response_cases::checked_steps};
use libtest_mimic::Trial;
use serde_json::{Value, json};
#[path = "adapter_repeated_references.rs"]
mod repeated_references;
#[path = "adapter_reactome_bad_values.rs"]
mod reactome_bad_values;

pub(super) fn trials() -> Vec<Trial> {
    vec![
        Trial::test("pathway-reactome/invalid-metadata-shapes-keep-siblings", || reactome_bad_values::check().map_err(Into::into)),
        Trial::test(
            "pathway-reactome/repeated-reference-invalid-fields-are-disclosed",
            || repeated_references::check().map_err(Into::into),
        ),
        Trial::test(
            "pathway-reactome/invalid-optional-fields-keep-records",
            || reactome().map_err(Into::into),
        ),
        Trial::test("variant/invalid-clinvar-condition-id-is-disclosed", || {
            clinvar_condition_id().map_err(Into::into)
        }),
        Trial::test("variant/invalid-rcv-condition-names-own-record", || {
            clinvar_rcv().map_err(Into::into)
        }),
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
        super::super::variant_assertion_wire::clinvar_body(request).map(|xml| {
            xml.replace(
                "<ReviewStatus>criteria provided, single submitter</ReviewStatus>",
                "<ReviewStatus></ReviewStatus>",
            )
        })
    })?;
    let env = super::super::variant::source_environment(&stub)?;
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
    for (field, invalid, absent) in [
        (
            "therapeutic_indication",
            "",
            "/result/regulatory/data/records/0/indication",
        ),
        (
            "marketing_authorisation_date",
            "31/02/2016",
            "/result/regulatory/data/records/0/dates/1",
        ),
    ] {
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
                        if row.get("ema_product_number") == Some(&json!("EMEA/H/C/004124"))
                            && let Some(fields) = row.as_object_mut()
                        {
                            fields.insert(field.into(), json!(invalid));
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
        check(&value, field, absent)?;
    }
    Ok(())
}

fn reactome() -> Result<(), String> {
    for (section, field, absent, present) in [
        (
            "summary",
            "references.title",
            "/result/pathway/references/0/title",
            "/result/pathway/references/0/db_id",
        ),
        (
            "events",
            "events.name",
            "/result/items/0/name",
            "/result/items/0/db_id",
        ),
    ] {
        let stub = Stub::for_source("reactome.org", "application/json", |request| {
            Ok(if request.contains("/database/version") {
                json!(97)
            } else if request.contains("/containedEvents") {
                json!([{"dbId":123,"displayName":""}])
            } else {
                json!({
                    "stId":"R-HSA-5673001", "displayName":"Synthetic pathway",
                    "speciesName":"Homo sapiens", "schemaClass":"Pathway",
                    "stIdVersion":"R-HSA-5673001.1",
                    "literatureReference":[{"dbId":123,"title":""}]
                })
            }
            .to_string())
        })?;
        let env = super::super::variant::source_environment(&stub)?;
        let args = if section == "summary" {
            vec!["get", "pathway", "R-HSA-5673001", "summary"]
        } else {
            vec!["pathway", "events", "R-HSA-5673001"]
        };
        let value = invoke(&env, args.into_iter().map(str::to_owned).collect(), 0)?;
        if value.pointer(present) != Some(&json!(123)) {
            return Err(format!("record lost: {value}"));
        }
        check(&value, field, absent)?;
        stub.finish()?;
    }
    Ok(())
}

fn clinvar_rcv() -> Result<(), String> {
    let stub = Stub::for_source("eutils.ncbi.nlm.nih.gov", "application/xml", |request| {
        super::super::variant_assertion_wire::clinvar_body(request).map(|xml| {
            xml.replace(
                "<ClinicalAssertionList>",
                concat!(
                    "<RCVList><RCVAccession Accession=\"RCV000123456\" Version=\"1\">",
                    "<ClassifiedConditionList><ClassifiedCondition DB=\"MedGen\" ID=\"C123\">",
                    "</ClassifiedCondition></ClassifiedConditionList><RCVClassifications>",
                    "<GermlineClassification><Description>Pathogenic</Description>",
                    "</GermlineClassification></RCVClassifications></RCVAccession>",
                    "</RCVList><ClinicalAssertionList>",
                ),
            )
        })
    })?;
    let env = super::super::variant::source_environment(&stub)?;
    let args = [
        "variant",
        "assertions",
        "clinvar:128042",
        "--source",
        "clinvar",
        "--assertion",
        "RCV000123456",
        "--version",
        "1",
    ];
    let value = invoke(&env, args.into_iter().map(str::to_owned).collect(), 0)?;
    check(
        &value,
        "conditions.label",
        "/result/assertions/0/detail/conditions/0/label",
    )?;
    if value.pointer("/problems/0/record") != Some(&json!("RCV000123456"))
        || value.pointer("/result/assertions/0/statement/attribution/record")
            != Some(&json!("RCV000123456"))
    {
        return Err(format!("RCV field problem lost its record: {value}"));
    }
    stub.finish()?;
    Ok(())
}

fn clinvar_condition_id() -> Result<(), String> {
    let stub = Stub::for_source("eutils.ncbi.nlm.nih.gov", "application/xml", |request| {
        super::super::variant_assertion_wire::clinvar_body(request)
            .map(|xml| xml.replace("ID=\"114480\"", "ID=\"invalid:\""))
    })?;
    let env = super::super::variant::source_environment(&stub)?;
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
    ];
    let value = invoke(&env, args.into_iter().map(str::to_owned).collect(), 0)?;
    check(
        &value,
        "conditions.id",
        "/result/assertions/0/detail/conditions/0/id",
    )?;
    if value.pointer("/problems/0/record") != Some(&json!("SCV000611258"))
        || value.pointer("/result/assertions/0/statement/attribution/record")
            != Some(&json!("SCV000611258"))
    {
        return Err(format!(
            "condition identity problem lost its record: {value}"
        ));
    }
    stub.finish()?;
    Ok(())
}
