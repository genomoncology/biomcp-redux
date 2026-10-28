//! Feature drug-failures: every kept source preserves synthetic failures and publisher terms.
use super::{configuration::invoke, launch::Environment, network_stub::Stub};
use crate::runner::transcript::trial as transcript_trial;
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::{fs, path::Path};
type Check = Result<(), String>;
struct Source {
    name: &'static str,
    host: &'static str,
    words: &'static [&'static str],
    measured: (usize, usize),
}
const SOURCES: &[Source] = &[
    Source {
        name: "mychem",
        measured: (1621, 659),
        host: "mychem.info",
        words: &["get", "drug", "SYNTHETIC", "--source", "mychem"],
    },
    Source {
        name: "chembl",
        measured: (347, 240),
        host: "www.ebi.ac.uk",
        words: &["get", "drug", "CHEMBL941", "--source", "chembl"],
    },
    Source {
        name: "opentargets",
        measured: (394, 255),
        host: "api.platform.opentargets.org",
        words: &["get", "drug", "CHEMBL941", "--source", "opentargets"],
    },
    Source {
        name: "civic",
        measured: (336, 252),
        host: "civicdb.org",
        words: &["get", "drug", "169", "civic", "--source", "civic"],
    },
    Source {
        name: "openfda-label",
        measured: (399, 288),
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
        measured: (357, 246),
        host: "api.fda.gov",
        words: &["get", "drug", "NDA021335", "--source", "drugsfda"],
    },
    Source {
        name: "ema",
        measured: (316, 144),
        host: "www.ema.europa.eu",
        words: &["sync", "ema"],
    },
    Source {
        name: "ddinter",
        measured: (255, 152),
        host: "ddinter.scbdd.com",
        words: &["sync", "ddinter"],
    },
];
pub(super) fn trials() -> Vec<Trial> {
    let mut trials = SOURCES
        .iter()
        .map(|source| {
            transcript_trial(
                format!(
                    "drug-failures/{}/synthetic-503-429-timeout-broken",
                    source.name
                ),
                move || check(source).map_err(Into::into),
            )
        })
        .collect::<Vec<_>>();
    trials.push(transcript_trial(
        "drug-failures/ema-record-broken-body",
        || ema_record().map_err(Into::into),
    ));
    trials
}
fn ema_record() -> Check {
    let stub = Stub::for_source("www.ema.europa.eu", "application/json", |_| {
        Ok("synthetic broken body".into())
    })?;
    let env = Environment::new().map_err(|e| e.to_string())?;
    let cert = env.root().join("ca.pem");
    stub.retain(&env);
    fs::write(&cert, &stub.certificate).map_err(|e| e.to_string())?;
    env.config(&json!({"ca_bundle":cert,"proxy":format!("http://{}",stub.address)}))
        .map_err(|e| e.to_string())?;
    let args = vec![
        "sync".into(),
        "ema".into(),
        "--record".into(),
        env.root().join("record").to_string_lossy().into_owned(),
    ];
    let output = env
        .command(env!("CARGO_BIN_EXE_biomcp"), &args)
        .output()
        .map_err(|e| e.to_string())?;
    let requests = stub.finish()?;
    if output.status.code() != Some(4) {
        return Err(format!(
            "EMA malformed recording {:?}, requests={}, stdout={}",
            output.status,
            requests.len(),
            String::from_utf8_lossy(&output.stdout)
        ));
    }
    let value: Value = serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;
    let source = SOURCES
        .iter()
        .find(|source| source.name == "ema")
        .ok_or("EMA source")?;
    assert_failed(source, "broken", &value)
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
    stub.retain(&env);
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
            let exchanges = json!(vec![exchange.clone(); 5]);
            *value.get_mut("exchanges").ok_or("missing exchanges")? = exchanges;
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
    let (full_bound, comparable_bound) = source.measured;
    if full > (full_bound * 115).div_ceil(100)
        || comparable > (comparable_bound * 115).div_ceil(100)
    {
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
