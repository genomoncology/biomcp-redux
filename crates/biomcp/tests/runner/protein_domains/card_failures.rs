//! Feature protein-domains-complexes: synthetic identity and optional failures remain explicit.
use super::super::mcp;
use super::*;
pub(super) fn trials() -> Vec<Trial> {
    let mut trials = Vec::new();
    for source in ["uniprot", "interpro", "complex-portal"] {
        for fault in ["429", "500", "timeout", "broken", "error"] {
            trials.push(Trial::test(
                format!("protein-domains-complexes/synthetic-card-{source}-{fault}"),
                move || failure(source, fault).map_err(Into::into),
            ));
        }
    }
    trials
}
fn failure(source: &str, fault: &str) -> Result<(), String> {
    let stub = Stub::for_sources(
        &["rest.uniprot.org", "www.ebi.ac.uk"],
        |_| "application/json",
        super::cards::reply,
    )?;
    let seed = setup(&stub)?;
    let original = seed.root().join("recorded");
    let section = match source {
        "interpro" => "domains",
        "complex-portal" => "complexes",
        _ => "function",
    };
    call(
        &seed,
        &[
            "get",
            "protein",
            "P38398",
            section,
            "--record",
            original.to_str().ok_or("record path")?,
        ],
        0,
    )?;
    stub.finish()?;
    let env = Environment::new().map_err(|e| e.to_string())?;
    let replay = env.root().join("replay");
    copy(&original, &replay, source, fault)?;
    let args: Vec<String> = ["get", "protein", "P38398", section]
        .map(str::to_owned)
        .into();
    let globals = env.globals(Some(&replay), false);
    let mut words = globals.clone();
    words.extend(args.clone());
    let cli = invoke(&env, words, if source == "uniprot" { 4 } else { 0 })?;
    let mcp = mcp::call_with_globals(&env, &args, source == "uniprot", &globals)?;

    let (cli_ceiling, mcp_ceiling) = card_ceilings(source, fault)?;
    for value in [&cli, &mcp] {
        let expected = if source == "uniprot" {
            "failed"
        } else {
            "partial"
        };
        let outcome = value
            .get("sources")
            .and_then(Value::as_array)
            .ok_or("source outcomes")?
            .iter()
            .find(|s| s.get("source") == Some(&json!(source)))
            .ok_or("source outcome")?;
        if value.get("status") != Some(&json!(expected))
            || value.pointer("/result/items/0/status") != Some(&json!(expected))
            || outcome.get("state") != Some(&json!("failed"))
            || outcome
                .get("terms")
                .and_then(Value::as_str)
                .is_none_or(str::is_empty)
            || value
                .get("problems")
                .and_then(Value::as_array)
                .is_none_or(Vec::is_empty)
            || !value
                .get("next")
                .and_then(Value::as_array)
                .ok_or("fresh steps")?
                .iter()
                .any(|v| v.get("kind") == Some(&json!("fresh")))
        {
            return Err(format!("{source}/{fault} hid the source failure: {value}"));
        }
        bounded(value, cli_ceiling, mcp_ceiling)?;
    }
    Ok(())
}
pub(super) fn bounded(value: &Value, cli: usize, mcp: usize) -> Result<(), String> {
    let bytes = serde_json::to_vec(value).map_err(|e| e.to_string())?.len();
    if bytes + 1 > cli || bytes > mcp {
        return Err(format!(
            "protein response exceeds CLI/MCP ceilings: {bytes}"
        ));
    }
    Ok(())
}
fn copy(
    from: &std::path::Path,
    to: &std::path::Path,
    source: &str,
    fault: &str,
) -> Result<(), String> {
    for folder in fs::read_dir(from).map_err(|e| e.to_string())? {
        let folder = folder.map_err(|e| e.to_string())?;
        let target = to.join(folder.file_name());
        fs::create_dir_all(&target).map_err(|e| e.to_string())?;
        for file in fs::read_dir(folder.path()).map_err(|e| e.to_string())? {
            let file = file.map_err(|e| e.to_string())?;
            let mut value: Value =
                serde_json::from_slice(&fs::read(file.path()).map_err(|e| e.to_string())?)
                    .map_err(|e| e.to_string())?;
            if folder.file_name() == source {
                let exchange = value
                    .pointer_mut("/exchanges/0")
                    .and_then(Value::as_object_mut)
                    .ok_or("seed exchange")?;
                match fault {
                    "429" | "500" => {
                        exchange.insert(
                            "status".into(),
                            json!(fault.parse::<u16>().map_err(|e| e.to_string())?),
                        );
                        exchange.insert("retry_after_ms".into(), json!(0));
                    }
                    "timeout" => {
                        exchange.insert("delay_ms".into(), json!(120001));
                    }
                    _ => {
                        exchange.insert(
                            "body".into(),
                            json!({"kind":"text",
"value":if fault == "broken" { "{" } else { "{\"error\":\"synthetic source error\"}" }}),
                        );
                    }
                }
                let exchange = Value::Object(exchange.clone());
                *value.get_mut("exchanges").ok_or("exchanges")? =
                    json!([exchange.clone(), exchange.clone(), exchange]);
            }
            fs::write(target.join(file.file_name()), value.to_string())
                .map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

// Complete synthetic envelopes measured in j15006, with fifteen percent headroom.
fn card_ceilings(source: &str, fault: &str) -> Result<(usize, usize), String> {
    Ok(match (source, fault) {
        ("uniprot", "429") => (701, 700),
        ("uniprot", "500") => (699, 697),
        ("uniprot", "timeout") => (712, 711),
        ("uniprot", "broken") => (694, 693),
        ("uniprot", "error") => (694, 693),
        ("interpro", "429") => (1494, 1493),
        ("interpro", "500") => (1492, 1491),
        ("interpro", "timeout") => (1506, 1505),
        ("interpro", "broken") => (1487, 1486),
        ("interpro", "error") => (1487, 1486),
        ("complex-portal", "429") => (1497, 1495),
        ("complex-portal", "500") => (1494, 1493),
        ("complex-portal", "timeout") => (1508, 1507),
        ("complex-portal", "broken") => (1490, 1489),
        ("complex-portal", "error") => (1490, 1489),
        _ => return Err("missing measured protein case ceiling".into()),
    })
}
