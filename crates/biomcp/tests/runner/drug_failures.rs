//! Feature drug-failures: every kept source preserves synthetic failures and publisher terms.
use super::{configuration::invoke, launch::Environment, network_stub::Stub};
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::{fs, path::Path};
type Check = Result<(), String>;
struct Source {
    name: &'static str,
    host: &'static str,
    words: &'static [&'static str],
}
const SOURCES: &[Source] = &[
    Source {
        name: "mychem",
        host: "mychem.info",
        words: &["get", "drug", "SYNTHETIC", "--source", "mychem"],
    },
    Source {
        name: "chembl",
        host: "www.ebi.ac.uk",
        words: &["get", "drug", "CHEMBL941", "--source", "chembl"],
    },
    Source {
        name: "opentargets",
        host: "api.platform.opentargets.org",
        words: &["get", "drug", "CHEMBL941", "--source", "opentargets"],
    },
    Source {
        name: "civic",
        host: "civicdb.org",
        words: &["get", "drug", "169", "civic", "--source", "civic"],
    },
    Source {
        name: "openfda-label",
        host: "api.fda.gov",
        words: &[
            "get",
            "drug",
            "492dbdb2-077e-4064-bff3-372d6af0a7a2",
            "--source",
            "openfda-label",
        ],
    },
    Source {
        name: "drugsfda",
        host: "api.fda.gov",
        words: &["get", "drug", "NDA021335", "--source", "drugsfda"],
    },
    Source {
        name: "ema",
        host: "www.ema.europa.eu",
        words: &["sync", "ema"],
    },
    Source {
        name: "ddinter",
        host: "ddinter.scbdd.com",
        words: &["sync", "ddinter"],
    },
];
pub(super) fn trials() -> Vec<Trial> {
    SOURCES
        .iter()
        .map(|source| {
            Trial::test(
                format!(
                    "drug-failures/{}/synthetic-503-429-timeout-broken",
                    source.name
                ),
                move || check(source).map_err(Into::into),
            )
        })
        .collect()
}
fn check(source: &Source) -> Check {
    if matches!(source.name, "ema" | "ddinter") {
        let env = Environment::new().map_err(|e| e.to_string())?;
        let recording = env.root().join("recording");
        super::response_cases::copy_capture(
            &format!("drug/{}-synthetic", source.name),
            &recording,
            |_, _| {},
        )
        .map_err(|e| e.to_string())?;
        return matrix(source, &recording, env.root());
    }
    let stub = Stub::for_source(source.host, "application/json", |_| {
        Ok("synthetic broken body".into())
    })?;
    let env = Environment::new().map_err(|e| e.to_string())?;
    let cert = env.root().join("ca.pem");
    fs::write(&cert, &stub.certificate).map_err(|e| e.to_string())?;
    env.config(&json!({"ca_bundle":cert,"proxy":format!("http://{}",stub.address)}))
        .map_err(|e| e.to_string())?;
    let recording = env.root().join("recording");
    let mut words = source
        .words
        .iter()
        .map(|w| (*w).to_owned())
        .collect::<Vec<_>>();
    words.extend(["--record".into(), recording.to_string_lossy().into_owned()]);
    invoke(&env, words, 4)?;
    if stub.finish()?.is_empty() {
        return Err(format!("{}: fixture was never delivered", source.name));
    }
    matrix(source, &recording, env.root())
}
fn matrix(source: &Source, recording: &Path, root: &Path) -> Check {
    for fault in ["503", "429", "timeout", "broken"] {
        let directory = root.join(fault);
        mutate(recording, &directory, fault)?;
        let replay = Environment::new().map_err(|e| e.to_string())?;
        let mut args = replay.globals(Some(&directory), false);
        args.extend(source.words.iter().map(|w| (*w).to_owned()));
        let value = invoke(&replay, args, 4)?;
        assert_failed(source, fault, &value)?;
        let args = source
            .words
            .iter()
            .map(|w| (*w).to_owned())
            .collect::<Vec<_>>();
        if super::mcp::call_with_globals(
            &replay,
            &args,
            true,
            &replay.globals(Some(&directory), false),
        )? != value
        {
            return Err("synthetic failure differs through MCP".into());
        }
    }
    Ok(())
}
fn mutate(original: &Path, destination: &Path, fault: &str) -> Check {
    fs::create_dir(destination).map_err(|e| e.to_string())?;
    for directory in fs::read_dir(original).map_err(|e| e.to_string())? {
        let directory = directory.map_err(|e| e.to_string())?;
        if !directory.path().is_dir() {
            continue;
        }
        let target = destination.join(directory.file_name());
        fs::create_dir(&target).map_err(|e| e.to_string())?;
        for file in fs::read_dir(directory.path()).map_err(|e| e.to_string())? {
            let file = file.map_err(|e| e.to_string())?;
            if file
                .path()
                .extension()
                .is_none_or(|extension| extension != "json")
            {
                continue;
            }
            let mut value: Value =
                serde_json::from_slice(&fs::read(file.path()).map_err(|e| e.to_string())?)
                    .map_err(|e| e.to_string())?;
            let exchange = value
                .pointer_mut("/exchanges/0")
                .ok_or("missing exchange")?;
            exchange["status"] = json!(match fault {
                "503" => 503,
                "429" => 429,
                _ => 200,
            });
            exchange["body"] = json!({"kind":"text","value":"synthetic broken body"});
            exchange["delay_ms"] = json!(if fault == "timeout" { 180000 } else { 0 });
            value["exchanges"] = json!(vec![exchange.clone(); 5]);
            fs::write(target.join(file.file_name()), value.to_string())
                .map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}
fn assert_failed(source: &Source, fault: &str, value: &Value) -> Check {
    let outcomes = value
        .get("sources")
        .and_then(Value::as_array)
        .ok_or("no sources")?;
    let outcome = outcomes
        .iter()
        .find(|row| row["source"] == source.name)
        .ok_or("missing failed source")?;
    let reason = match fault {
        "503" => "unavailable",
        "429" => "rate_limited",
        "timeout" => "deadline_exceeded",
        _ => "bad_reply",
    };
    if !matches!(value["status"].as_str(), Some("failed" | "partial"))
        || outcome["state"] != "failed"
        || outcome["reason"] != reason
    {
        return Err(format!(
            "{} {fault}: failure became absence: {value}",
            source.name
        ));
    }
    if outcome["terms"].as_str().is_none_or(str::is_empty)
        || outcome["terms_url"].as_str().is_none_or(str::is_empty)
        || outcome["use"].as_str().is_none()
    {
        return Err("synthetic failure lost publisher terms".into());
    }
    let full = serde_json::to_vec(value).map_err(|e| e.to_string())?.len();
    let comparable = super::source_terms::size(value, "")?;
    if full > 8192 || comparable > 8192 {
        return Err("synthetic failure exceeds size ceiling".into());
    }
    use std::io::Write;
    writeln!(
        std::io::stdout(),
        "synthetic {} {fault}: full={full} comparable={comparable}",
        source.name
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}
