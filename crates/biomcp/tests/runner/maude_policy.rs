//! Feature maude-reports: component policy precedes source cache, recorder and HTTP.
use super::maude::{Check, environment, response, row};
use super::{configuration::invoke, network_stub::Stub};
use serde_json::json;
pub(super) fn run() -> Check {
    let stub = Stub::for_source("api.fda.gov", "application/json", |_| Ok(response(row())))?;
    let env = environment(&stub)?;
    let record = env.root().join("device-recordings");
    let words = vec![
        "--record".into(),
        record.to_string_lossy().into_owned(),
        "get".into(),
        "adverse-event".into(),
        "160".into(),
        "devices".into(),
        "--source".into(),
        "maude".into(),
    ];
    invoke(&env, words, 0)?;
    for source in ["gmdn", "restricted", "maude"] {
        for replay in [false, true] {
            let mut words = vec!["--disable-sources".into(), source.into()];
            if replay {
                words.extend(["--replay".into(), record.to_string_lossy().into_owned()]);
            }
            words.extend([
                "get".into(),
                "adverse-event".into(),
                "160".into(),
                "devices".into(),
                "--source".into(),
                "maude".into(),
            ]);
            let value = invoke(&env, words, 4)?;
            if value.get("result").is_some()
                || value.pointer("/sources/0/reason") != Some(&json!("disabled"))
            {
                return Err("disabled device reply entered product".into());
            }
        }
    }
    for section in [None, Some("narrative")] {
        let mut words = vec![
            "--disable-sources".into(),
            "gmdn".into(),
            "get".into(),
            "adverse-event".into(),
            "160".into(),
        ];
        if let Some(section) = section {
            words.push(section.into());
        }
        words.extend(["--source".into(), "maude".into()]);
        let value = invoke(&env, words, 0)?;
        if value.to_string().contains("generic_name") {
            return Err("card or narrative exposed disabled GMDN".into());
        }
    }
    if stub.finish()?.len() != 1 {
        return Err("disabled or warm get touched HTTP".into());
    }
    let stub = Stub::for_source("api.fda.gov", "application/json", |_| {
        Err("disabled device search touched HTTP".into())
    })?;
    let env = environment(&stub)?;
    let value = invoke(
        &env,
        [
            "--disable-sources",
            "gmdn",
            "search",
            "adverse-event",
            "Synthetic device",
            "--source",
            "maude",
        ]
        .map(str::to_string)
        .to_vec(),
        4,
    )?;
    if value.pointer("/sources/0/reason") != Some(&json!("disabled")) || !stub.finish()?.is_empty()
    {
        return Err("disabled search touched HTTP".into());
    }
    Ok(())
}
