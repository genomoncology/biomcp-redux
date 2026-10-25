//! Feature drug-approvals: native application candidates and independent product history.
use super::{configuration::invoke, launch::Environment, network_stub::Stub};
use libtest_mimic::Trial;
use serde_json::{Value, json};
type Check = Result<(), String>;
pub(super) fn trials() -> Vec<Trial> {
    vec![
        Trial::test("drug-approvals/literal-application-candidates", || {
            search().map_err(Into::into)
        }),
        Trial::test(
            "drug-approvals/application-history-keeps-product-scope",
            || approvals().map_err(Into::into),
        ),
    ]
}
fn setup(body: Value, selector: &str) -> Result<(Environment, Stub), String> {
    let expected = format!("GET /drug/drugsfda.json?search={selector}&limit=1 HTTP/1.1");
    let stub = Stub::for_source_status(
        "api.fda.gov",
        "application/json",
        move |request| {
            if request.lines().next() != Some(expected.as_str()) {
                return Err(format!("unexpected FDA request: {request}"));
            }
            Ok(body.to_string())
        },
        |_| 200,
    )?;
    let env = Environment::new().map_err(|e| e.to_string())?;
    let cert = env.root().join("ca.pem");
    std::fs::write(&cert, &stub.certificate).map_err(|e| e.to_string())?;
    env.config(&json!({"ca_bundle":cert,"proxy":format!("http://{}",stub.address)}))
        .map_err(|e| e.to_string())?;
    Ok((env, stub))
}
fn application() -> Value {
    json!({"application_number":"NDA021335","sponsor_name":"Synthetic sponsor",
        "products":[{"product_number":"002","brand_name":"Synthetic brand",
            "active_ingredients":[{"name":"Synthetic ingredient","strength":"10MG"}],
            "dosage_form":"TABLET","route":"ORAL","marketing_status":"Discontinued",
            "reference_drug":"Yes","reference_standard":"No"}],
        "submissions":[{"submission_type":"ORIG","submission_number":"1",
            "submission_status":"AP","submission_status_date":"20010510",
            "review_priority":"PRIORITY"}]})
}
fn search() -> Check {
    let (env, stub) = setup(
        json!({"meta":{"results":{"skip":0,"limit":1,"total":2}},"results":[application()]}),
        "products.active_ingredients.name%3A%22Synthetic%22",
    )?;
    let args = [
        "search",
        "drug",
        "Synthetic",
        "--source",
        "drugsfda",
        "--limit",
        "1",
    ]
    .map(str::to_owned)
    .to_vec();
    let value = invoke(&env, args.clone(), 0)?;
    super::schema::Registry::embedded()?.validate(&value, &args, true)?;
    let mcp = super::mcp::call_with_globals(&env, &args, false, &[])?;
    if value != mcp
        || value.pointer("/result/items/0/id") != Some(&json!("NDA021335"))
        || value.pointer("/result/items/0/identity_level") != Some(&json!("fda_application"))
        || value.pointer("/result/total") != Some(&json!(2))
    {
        return Err(format!("native application candidates changed: {value}"));
    }
    if stub.finish()?.len() != 1 {
        return Err("warm FDA search made another request".into());
    }
    Ok(())
}
fn approvals() -> Check {
    let mut app = application();
    app["submissions"]
        .as_array_mut()
        .ok_or("submissions")?
        .push(json!({
        "submission_type":"SUPPL","submission_number":"2","submission_status":"AP",
        "submission_status_date":"20021220","submission_class_code":"LABELING",
        "application_docs":[{"id":"7","url":"https://example.invalid/label.pdf",
            "date":"20030101","type":"Label"}]}));
    let (env, stub) = setup(
        json!({"meta":{"results":{"skip":0,"limit":1,"total":1}},
        "results":[app]}),
        "application_number%3A%22NDA021335%22",
    )?;
    let args = [
        "get",
        "drug",
        "NDA021335",
        "approvals",
        "--source",
        "drugsfda",
        "--product-number",
        "002",
        "--section-limit",
        "1",
    ]
    .map(str::to_owned)
    .to_vec();
    let value = invoke(&env, args.clone(), 0)?;
    super::schema::Registry::embedded()?.validate(&value, &args, true)?;
    let mcp = super::mcp::call_with_globals(&env, &args, false, &[])?;
    if value != mcp
        || value.pointer("/result/products/0/product_number") != Some(&json!("002"))
        || value.pointer("/result/approvals/data/total") != Some(&json!(2))
        || value.pointer("/result/approvals/data/records/0/submission_number") != Some(&json!("1"))
        || value
            .pointer("/result/approvals/data/records/0/event/product_number")
            .is_some()
        || value.pointer("/result/approvals/data/records/0/event/dates/0/role")
            != Some(&json!("submission_status_date"))
    {
        return Err(format!(
            "application history acquired product scope or lost rows: {value}"
        ));
    }
    if stub.finish()?.len() != 1 {
        return Err("warm approvals fetched again".into());
    }
    Ok(())
}
