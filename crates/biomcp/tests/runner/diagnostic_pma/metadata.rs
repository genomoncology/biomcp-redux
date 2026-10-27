use super::{Check, Environment, Value, invoke, json};
use crate::runner::network_stub::Stub;
pub(super) fn run() -> Check {
    for text in [None, Some("")] {
        let mut row = json!({"pma_number":"P150013","supplement_number":"",
            "decision_date":"20200101","date_received":"20190101","product_code":"ABC"});
        if let Some(text) = text {
            row.as_object_mut()
                .ok_or("row object")?
                .insert("ao_statement".into(), json!(text));
        }
        let body = json!({"meta":{"results":{"skip":0,"limit":100,"total":1}},"results":[row]});
        let stub = Stub::for_source_status(
            "api.fda.gov",
            "application/json",
            move |_| Ok(body.to_string()),
            |_| 200,
        )?;
        let env = Environment::new().map_err(|e| e.to_string())?;
        let ca = env.root().join("ca.pem");
        stub.retain(&env);
        std::fs::write(&ca, &stub.certificate).map_err(|e| e.to_string())?;
        env.config(&json!({"ca_bundle":ca,"proxy":format!("http://{}",stub.address)}))
            .map_err(|e| e.to_string())?;
        let args = [
            "get",
            "diagnostic",
            "P150013",
            "regulatory",
            "--source",
            "fda-pma",
        ]
        .into_iter()
        .map(String::from)
        .collect();
        let value = invoke(&env, args, 0)?;
        if value.pointer("/result/items/0/metadata/product_code") != Some(&json!("ABC"))
            || value.pointer("/result/items/0/submission/decision_date") != Some(&json!("20200101"))
            || value.pointer("/result/items/0/metadata/date_received") != Some(&json!("20190101"))
            || value.pointer("/result/items/0/text") != text.map(|v| json!(v)).as_ref()
            || value
                .pointer("/result/items")
                .and_then(Value::as_array)
                .map(Vec::len)
                != Some(1)
        {
            return Err(format!(
                "regulatory metadata or native text presence lost: {value}"
            ));
        }
        stub.finish()?;
    }
    Ok(())
}
