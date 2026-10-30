//! Feature drug-ema: source failures preserve bundles; rejected rows retain valid siblings.
mod navigation;
use super::{configuration::invoke, launch::Environment};
use crate::runner::transcript::trial as transcript_trial;
use libtest_mimic::Trial;
use serde_json::{Value, json};

pub(super) fn trials() -> Vec<Trial> {
    vec![
        transcript_trial("drug-ema/malformed-source-keeps-last-good-bundle", || {
            malformed().map_err(Into::into)
        }),
        transcript_trial("drug-ema/five-candidates-keep-continuation", || {
            navigation::check().map_err(Into::into)
        }),
    ]
}
fn malformed() -> Result<(), String> {
    let (capture, body) = retained()?;
    for fault in ["total", "duplicate", "metadata", "identity"] {
        let env = Environment::new().map_err(|e| e.to_string())?;
        let replay = env.root().join("replay");
        let source = replay.join("ema");
        std::fs::create_dir_all(&source).map_err(|e| e.to_string())?;
        let path =
            source.join("1854254ba57f4597bd03970bf3664f62fac42eda059bf0022aaf32367297a317.json");
        std::fs::write(&path, capture.to_string()).map_err(|e| e.to_string())?;
        let call = |args: &[&str], exit| {
            let mut command = env.globals(Some(&replay), false);
            command.extend(args.iter().map(|s| (*s).into()));
            invoke(&env, command, exit)
        };
        call(&["sync", "ema"], 0)?;
        let id = if fault == "identity" {
            "EMEA/H/C/000406"
        } else {
            "EMEA/H/C/004124"
        };
        let get = ["get", "drug", id, "--source", "ema"];
        let before = call(&get, 0)?;
        let mut bad = body.clone();
        let (pointer, value) = match fault {
            "total" => ("/meta/total_records", 5.into()),
            "identity" => ("/data/1/ema_product_number", "".into()),
            "duplicate" => (
                "/data/1/ema_product_number",
                bad.pointer("/data/0/ema_product_number")
                    .ok_or("missing first identity")?
                    .clone(),
            ),
            _ => ("/meta/timestamp", "".into()),
        };
        replace(&mut bad, pointer, value)?;
        let mut changed = capture.clone();
        replace(
            &mut changed,
            "/exchanges/0/body/value",
            bad.to_string().into(),
        )?;
        std::fs::write(&path, changed.to_string()).map_err(|e| e.to_string())?;
        if fault == "metadata" {
            call(&["sync", "ema"], 0)?;
            let value = call(&get, 0)?;
            if value.get("result") != before.get("result")
                || value.get("problems")
                    != Some(&json!([{
                        "source":"ema", "record":id, "field":"_bundle_meta.timestamp", "code":"value.unknown"
                    }]))
                || value.pointer("/sources/0/state") != Some(&json!("ok"))
            {
                return Err(format!(
                    "optional EMA timestamp changed native card: {value}"
                ));
            }
            continue;
        }
        if fault == "identity" {
            rejected_identity(&call(&["sync", "ema"], 0)?)?;
            rejected_identity(&call(&["sync", "ema"], 0)?)?;
            if call(&get, 0)? != before {
                return Err("row rejection changed a valid sibling card".into());
            }
            continue;
        }
        let failed = call(&["sync", "ema"], 4)?;
        if failed.pointer("/sources/0/reason").and_then(Value::as_str) != Some("bad-reply") {
            return Err(format!("{fault}: wrong source error: {failed}"));
        }
        if call(&get, 0)? != before {
            return Err(format!("{fault}: invalid sync changed the usable card"));
        }
    }
    Ok(())
}

fn retained() -> Result<(Value, Value), String> {
    let capture: Value = serde_json::from_str(&crate::runner::launch::fixture_text(
        "replies/drug/ema-synthetic/ema/1854254ba57f4597bd03970bf3664f62fac42eda059bf0022aaf32367297a317.json",
    )?)
    .map_err(|e| e.to_string())?;
    let body: Value = serde_json::from_str(
        capture
            .pointer("/exchanges/0/body/value")
            .and_then(Value::as_str)
            .ok_or("missing body")?,
    )
    .map_err(|e| e.to_string())?;
    Ok((capture, body))
}

fn rejected_identity(value: &Value) -> Result<(), String> {
    if value.pointer("/result/rows") != Some(&json!(5))
        || value.get("problems")
            != Some(&json!([{
                "source": "ema", "field": "ema.rows.2.decode", "code": "value.unknown"
            }]))
    {
        return Err(format!("wrong rejected identity disclosure: {value}"));
    }
    Ok(())
}

fn replace(value: &mut Value, pointer: &str, replacement: Value) -> Result<(), String> {
    *value
        .pointer_mut(pointer)
        .ok_or_else(|| format!("missing fixture field {pointer}"))? = replacement;
    Ok(())
}
