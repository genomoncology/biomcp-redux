//! Feature drug-approvals: native application candidates and independent product history.
use super::{configuration::invoke, launch::Environment, network_stub::Stub};
use libtest_mimic::Trial;
use serde_json::{Value, json};
type Check = Result<(), String>;
mod capture;
mod errors;
mod names;
pub(super) fn trials() -> Vec<Trial> {
    let mut trials = vec![
        Trial::test("drug-approvals/literal-application-candidates", || {
            search().map_err(Into::into)
        }),
        Trial::test(
            "drug-approvals/application-history-keeps-product-scope",
            || approvals().map_err(Into::into),
        ),
        Trial::test("drug-approvals/missing-product-is-unavailable", || {
            missing_product().map_err(Into::into)
        }),
        Trial::test("drug-approvals/application-card-has-no-events", || {
            card().map_err(Into::into)
        }),
        Trial::test("drug-approvals/all-candidate-gets-and-page-survive", || {
            navigation().map_err(Into::into)
        }),
    ];
    trials.extend(errors::trials());
    trials.extend(names::trials());
    trials.extend(capture::trials());
    trials
}
fn setup(body: Value, selector: &str) -> Result<(Environment, Stub), String> {
    setup_status(body.to_string(), selector, 200)
}
fn setup_status(body: String, selector: &str, status: u16) -> Result<(Environment, Stub), String> {
    let expected = format!("GET /drug/drugsfda.json?search={selector}&limit=1 HTTP/1.1");
    let stub = Stub::for_source_status(
        "api.fda.gov",
        "application/json",
        move |request| {
            if request.lines().next() != Some(expected.as_str()) {
                return Err(format!("unexpected FDA request: {request}"));
            }
            Ok(body.clone())
        },
        move |_| status,
    )?;
    let env = environment(&stub)?;
    Ok((env, stub))
}
fn environment(stub: &Stub) -> Result<Environment, String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let cert = env.root().join("ca.pem");
    std::fs::write(&cert, &stub.certificate).map_err(|e| e.to_string())?;
    env.config(&json!({"ca_bundle":cert,"proxy":format!("http://{}",stub.address)}))
        .map_err(|e| e.to_string())?;
    Ok(env)
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
    let value = both(&env, args, 0)?;
    budget(measure(&value)?, (923, 812))?;
    if value.pointer("/result/items/0/identity/id") != Some(&json!("NDA021335"))
        || value.pointer("/result/items/0/identity/identity_level")
            != Some(&json!("fda_application"))
        || value.pointer("/result/items/0/products/0/brand_name") != Some(&json!("Synthetic brand"))
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
    app.get_mut("submissions")
        .ok_or("submissions")?
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
    let value = both(&env, args, 0)?;
    budget(measure(&value)?, (1299, 1188))?;
    if value.pointer("/result/products/0/product_number") != Some(&json!("002"))
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
    let args: Vec<String> = value
        .pointer("/next/0/args")
        .and_then(Value::as_array)
        .ok_or("missing history continuation")?
        .iter()
        .map(|v| v.as_str().map(str::to_owned).ok_or("next argument"))
        .collect::<Result<_, _>>()?;
    if !args.windows(2).any(|w| w == ["--product-number", "002"]) {
        return Err("history continuation lost selected product".into());
    }
    let second = both(&env, args, 0)?;
    budget(measure(&second)?, (1271, 1160))?;
    if second.pointer("/result/approvals/data/records/0/submission_number") != Some(&json!("2"))
        || second.pointer("/result/approvals/data/records/0/application_docs/0/date")
            != Some(&json!("2003-01-01"))
        || second
            .pointer("/result/approvals/data/records/0/review_priority")
            .is_some()
        || second.get("next").is_some()
    {
        return Err(format!(
            "native history continuation lost dates or absent fields: {second}"
        ));
    }
    if stub.finish()?.len() != 1 {
        return Err("warm approvals fetched again".into());
    }
    Ok(())
}
fn both(env: &Environment, args: Vec<String>, exit: i32) -> Result<Value, String> {
    let value = invoke(env, args.clone(), exit)?;
    super::schema::Registry::embedded()?.validate(&value, &args, true)?;
    let start = args
        .iter()
        .position(|s| matches!(s.as_str(), "get" | "search"))
        .unwrap_or(0);
    let (globals, command) = args.split_at(start);
    if value != super::mcp::call_with_globals(env, command, exit >= 2, globals)? {
        return Err("FDA CLI and MCP differ".into());
    }
    let (full, comparable) = measure(&value)?;
    use std::io::Write;
    writeln!(
        std::io::stdout(),
        "FDA envelope full={full} comparable={comparable} estimated_tokens={}",
        comparable.div_ceil(4)
    )
    .map_err(|e| e.to_string())?;
    Ok(value)
}
fn measure(value: &Value) -> Result<(usize, usize), String> {
    let full = serde_json::to_vec(value).map_err(|e| e.to_string())?.len();
    let comparable = super::source_terms::size(value, "")?;
    Ok((full, comparable))
}
fn budget(actual: (usize, usize), measured: (usize, usize)) -> Check {
    let bound = (
        (measured.0 * 115).div_ceil(100),
        (measured.1 * 115).div_ceil(100),
    );
    if actual.0 > bound.0 || actual.1 > bound.1 {
        return Err(format!(
            "FDA output exceeds measured full/comparable bound: {actual:?}/{bound:?}"
        ));
    }
    Ok(())
}
fn exact_setup() -> Result<(Environment, Stub), String> {
    setup(
        json!({"meta":{"results":{"skip":0,"limit":1,"total":1}},"results":[application()]}),
        "application_number%3A%22NDA021335%22",
    )
}
fn missing_product() -> Check {
    let (env, stub) = exact_setup()?;
    let args = [
        "get",
        "drug",
        "NDA021335",
        "approvals",
        "--source",
        "drugsfda",
        "--product-number",
        "999",
    ]
    .map(str::to_owned)
    .to_vec();
    let value = both(&env, args, 0)?;
    budget(measure(&value)?, (519, 408))?;
    if value.pointer("/result/approvals/state") != Some(&json!("unavailable"))
        || value.pointer("/result/products") != Some(&json!([]))
        || value.get("page").is_some()
        || value.get("next").is_some()
    {
        return Err(format!(
            "missing product selected another product or history: {value}"
        ));
    }
    if stub.finish()?.len() != 1 {
        return Err("missing product fetched again".into());
    }
    Ok(())
}
fn card() -> Check {
    let (env, stub) = exact_setup()?;
    let value = both(
        &env,
        ["get", "drug", "NDA021335", "--source", "drugsfda"]
            .map(str::to_owned)
            .to_vec(),
        0,
    )?;
    budget(measure(&value)?, (531, 420))?;
    if value.pointer("/result/header/id") != Some(&json!("NDA021335"))
        || value.pointer("/result/approvals").is_some()
        || value.pointer("/result/submissions").is_some()
        || value.pointer("/result/header/name").is_some()
        || value.pointer("/result/retrieved").is_none()
    {
        return Err("application card invented a drug name or leaked events".into());
    }
    if stub.finish()?.len() != 1 {
        return Err("warm application card fetched again".into());
    }
    Ok(())
}
fn navigation() -> Check {
    let rows = (0..7)
        .map(|n| {
            let mut row = application();
            *row.get_mut("application_number")
                .ok_or("application number")? = json!(format!("NDA{:06}", 21330 + n));
            Ok(row)
        })
        .collect::<Result<Vec<_>, String>>()?;
    let body = json!({"meta":{"results":{"skip":0,"limit":7,"total":9}},"results":rows});
    let stub = Stub::for_source_status(
        "api.fda.gov",
        "application/json",
        move |request| {
            if request.lines().next()
                != Some(
                    "GET /drug/drugsfda.json?search=products.active_ingredients.name%3A%22Synthetic%22&limit=7 HTTP/1.1",
                )
            {
                return Err("candidate request changed".into());
            }
            Ok(body.to_string())
        },
        |_| 200,
    )?;
    let env = environment(&stub)?;
    let value = both(
        &env,
        [
            "search",
            "drug",
            "Synthetic",
            "--source",
            "drugsfda",
            "--limit",
            "7",
        ]
        .map(str::to_owned)
        .to_vec(),
        0,
    )?;
    let (full, comparable) = measure(&value)?;
    budget((full, comparable), (3503, 3392))?;
    use std::io::Write;
    writeln!(
        std::io::stdout(),
        "FDA seven-candidate navigation full={full} comparable={comparable}"
    )
    .map_err(|e| e.to_string())?;
    let next = value
        .get("next")
        .and_then(Value::as_array)
        .ok_or("candidate actions")?;
    if next.len() != 8 {
        return Err(format!("candidate gets were truncated: {next:?}"));
    }
    for (n, action) in next.iter().take(7).enumerate() {
        if action
            != &json!({"kind":"disambiguate","args":["get","drug",format!("NDA{:06}",21330+n),"--source","drugsfda"]})
        {
            return Err("candidate get changed identity".into());
        }
    }
    if next.last()
        != Some(
            &json!({"kind":"page","args":["search","drug","Synthetic","--limit","7","--offset","7","--source","drugsfda"]}),
        )
    {
        return Err("native page action lost source or offset".into());
    }
    if stub.finish()?.len() != 1 {
        return Err("warm candidates fetched again".into());
    }
    Ok(())
}
