use super::{Environment, Stub, Value, call, fs, invoke, json, setup};
use std::path::Path;
pub(super) fn run() -> Result<(), String> {
    for command in ["domains", "complexes"] {
        let stub = Stub::for_source("www.ebi.ac.uk", "application/json", |request| {
            Ok(if request.contains("/interpro/") {
                json!({"count":0,"next":null,"results":[]})
            } else {
                json!({"totalNumberOfResults":0,"elements":[]})
            }
            .to_string())
        })?;
        let seed = setup(&stub)?;
        let record = seed.root().join("recorded");
        call(
            &seed,
            &[
                "protein",
                command,
                "P38398",
                "--record",
                record.to_str().ok_or("record path")?,
            ],
            0,
        )?;
        stub.finish()?;
        for (kind, reason) in [
            ("429", "rate-limited"),
            ("500", "unavailable"),
            ("timeout", "deadline-exceeded"),
            ("broken", "bad-reply"),
            ("error", "bad-reply"),
        ] {
            let env = Environment::new().map_err(|e| e.to_string())?;
            let replay = env.root().join("recordings");
            copy(&record, &replay, kind)?;
            let mut words = env.globals(Some(&replay), false);
            words.extend(["protein", command, "P38398"].map(str::to_owned));
            let value = invoke(&env, words, 4)?;
            let (cli_ceiling, mcp_ceiling) = relation_ceilings(command, kind)?;
            super::card_failures::bounded(&value, cli_ceiling, mcp_ceiling)?;
            let args = ["protein", command, "P38398"].map(str::to_owned);
            let mcp = super::super::mcp::call_with_globals(
                &env,
                &args,
                true,
                &env.globals(Some(&replay), false),
            )?;

            if value != mcp {
                return Err("synthetic relation CLI/MCP failures differ".into());
            }
            super::card_failures::bounded(&mcp, cli_ceiling, mcp_ceiling)?;
            if value.pointer("/status") != Some(&json!("failed"))
                || value.pointer("/sources/0/reason") != Some(&json!(reason))
                || value.pointer("/sources/0/terms") != Some(&json!("CC0"))
                || value.get("result").is_some()
            {
                return Err(format!(
                    "{command}/{kind} became an empty collection: {value}"
                ));
            }
        }
    }
    Ok(())
}
fn copy(original: &Path, target: &Path, kind: &str) -> Result<(), String> {
    for source in fs::read_dir(original).map_err(|e| e.to_string())? {
        let source = source.map_err(|e| e.to_string())?;
        let directory = target.join(source.file_name());
        fs::create_dir_all(&directory).map_err(|e| e.to_string())?;
        for path in fs::read_dir(source.path()).map_err(|e| e.to_string())? {
            let path = path.map_err(|e| e.to_string())?;
            let mut record: Value =
                serde_json::from_slice(&fs::read(path.path()).map_err(|e| e.to_string())?)
                    .map_err(|e| e.to_string())?;
            let mut exchange = record
                .pointer("/exchanges/0")
                .cloned()
                .ok_or("record exchange")?;
            let fields = exchange.as_object_mut().ok_or("exchange object")?;
            match kind {
                "429" => {
                    fields.insert("status".into(), json!(429));
                    fields.insert("retry_after_ms".into(), json!(0));
                }
                "500" => {
                    fields.insert("status".into(), json!(500));
                }
                "timeout" => {
                    fields.insert("delay_ms".into(), json!(9000));
                }
                "broken" => {
                    fields.insert("body".into(), json!({"kind":"text","value":"{"}));
                }
                _ => {
                    fields.insert(
                        "body".into(),
                        json!({"kind":"json","value":{"message":"source error"}}),
                    );
                }
            }

            record.as_object_mut().ok_or("record object")?.insert(
                "exchanges".into(),
                json!([exchange.clone(), exchange.clone(), exchange]),
            );
            fs::write(
                directory.join(path.file_name()),
                serde_json::to_vec(&record).map_err(|e| e.to_string())?,
            )
            .map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

// Complete envelopes: j15006; InterPro timeout guidance: j19312. Fifteen percent headroom.
fn relation_ceilings(command: &str, kind: &str) -> Result<(usize, usize), String> {
    Ok(match (command, kind) {
        ("domains", "429") => (528, 527),
        ("domains", "500") => (527, 526),
        ("domains", "timeout") => (975, 973),
        ("domains", "broken") => (525, 524),
        ("domains", "error") => (525, 524),
        ("complexes", "429") => (526, 525),
        ("complexes", "500") => (525, 524),
        ("complexes", "timeout") => (532, 531),
        ("complexes", "broken") => (523, 521),
        ("complexes", "error") => (523, 521),
        _ => return Err("missing measured protein case ceiling".into()),
    })
}
