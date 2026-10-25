//! Feature maude-reports: a card's section actions retain its original checked record.
use super::{
    configuration::invoke,
    maude::{Check, environment, response, row},
    network_stub::Stub,
};
use serde_json::{Value, json};
pub(super) fn run() -> Check {
    let mut requests = 0;
    let stub = Stub::for_sources(
        &["api.fda.gov"],
        |_| "application/json",
        move |_| {
            requests += 1;
            let mut report = row();
            *report
                .pointer_mut("/device/0/brand_name")
                .ok_or("synthetic brand")? = json!(if requests == 1 {
                "original"
            } else {
                "refreshed"
            });
            Ok(response(report))
        },
    )?;
    let env = environment(&stub)?;
    let words = ["get", "adverse-event", "160", "--source", "maude"]
        .map(str::to_owned)
        .to_vec();
    let card = invoke(&env, words.clone(), 0)?;
    let old_section = card
        .get("next")
        .and_then(Value::as_array)
        .and_then(|v| {
            v.iter().find(|v| {
                v.get("args")
                    .and_then(Value::as_array)
                    .is_some_and(|v| v.contains(&json!("devices")))
            })
        })
        .and_then(|v| v.get("args"))
        .ok_or("device section action")?;
    let mut fresh = vec!["--fresh".into()];
    fresh.extend(words);
    let refreshed = invoke(&env, fresh, 0)?;
    if card.pointer("/result/snapshot") == refreshed.pointer("/result/snapshot") {
        return Err("changed native projection kept the same snapshot".into());
    }
    let old_section: Vec<String> =
        serde_json::from_value(old_section.clone()).map_err(|e| e.to_string())?;
    let original = invoke(&env, old_section, 0)?;
    if original.pointer("/result/devices/records/0/brand_name") != Some(&json!("original"))
        || original.pointer("/result/snapshot") != card.pointer("/result/snapshot")
    {
        return Err(format!(
            "card action mixed refreshed report rows: {original}"
        ));
    }
    let missing = invoke(
        &env,
        vec![
            "get".into(),
            "adverse-event".into(),
            "160".into(),
            "devices".into(),
            "--source".into(),
            "maude".into(),
            "--snapshot".into(),
            "0".repeat(64),
        ],
        3,
    )?;
    if missing.pointer("/next/0/kind") != Some(&json!("restart")) || missing.get("result").is_some()
    {
        return Err("missing snapshot refetched or invented rows".into());
    }
    if stub.finish()?.len() != 2 {
        return Err("snapshot continuation reached source".into());
    }
    Ok(())
}
