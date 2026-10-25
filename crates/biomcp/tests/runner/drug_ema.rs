//! Feature drug-ema: malformed source documents preserve the last complete local bundle.
use super::{configuration::invoke, launch::Environment};
use libtest_mimic::Trial;
use serde_json::Value;

pub(super) fn trials() -> Vec<Trial> {
    vec![Trial::test("drug-ema/malformed-source-keeps-last-good-bundle", || {
        malformed().map_err(Into::into)
    })]
}
fn malformed() -> Result<(), String> {
    let capture: Value = serde_json::from_str(include_str!(
        "../../../../tests/replies/drug/ema-synthetic/ema/1854254ba57f4597bd03970bf3664f62fac42eda059bf0022aaf32367297a317.json"
    )).map_err(|e| e.to_string())?;
    let body: Value = serde_json::from_str(capture["exchanges"][0]["body"]["value"].as_str().ok_or("missing body")?).map_err(|e| e.to_string())?;
    for fault in ["total", "duplicate", "date", "metadata"] {
        let env = Environment::new().map_err(|e| e.to_string())?;
        let replay = env.root().join("replay");
        let source = replay.join("ema");
        std::fs::create_dir_all(&source).map_err(|e| e.to_string())?;
        let path = source.join("1854254ba57f4597bd03970bf3664f62fac42eda059bf0022aaf32367297a317.json");
        std::fs::write(&path, capture.to_string()).map_err(|e| e.to_string())?;
        let call = |args: &[&str], exit| {
            let mut command = env.globals(Some(&replay), false);
            command.extend(args.iter().map(|s| (*s).into()));
            invoke(&env, command, exit)
        };
        call(&["sync", "ema"], 0)?;
        let get = ["get", "drug", "EMEA/H/C/004124", "--source", "ema"];
        let before = call(&get, 0)?;
        let mut bad = body.clone();
        match fault {
            "total" => bad["meta"]["total_records"] = 5.into(),
            "duplicate" => bad["data"][1]["ema_product_number"] = bad["data"][0]["ema_product_number"].clone(),
            "date" => bad["data"][1]["marketing_authorisation_date"] = "31/02/2016".into(),
            _ => bad["meta"]["timestamp"] = "".into(),
        }
        let mut changed = capture.clone();
        changed["exchanges"][0]["body"]["value"] = bad.to_string().into();
        std::fs::write(&path, changed.to_string()).map_err(|e| e.to_string())?;
        let failed = call(&["sync", "ema"], 4)?;
        if failed.pointer("/sources/0/reason").and_then(Value::as_str) != Some("bad_reply") {
            return Err(format!("{fault}: wrong source error: {failed}"));
        }
        if call(&get, 0)? != before {
            return Err(format!("{fault}: invalid sync changed the usable card"));
        }
    }
    Ok(())
}
