//! variant-assertions: optional NCBI credentials cross only the outgoing wire.
use super::{launch::Environment, network_stub::Stub};
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::{fs, path::Path, time::Duration};

const KEY: &str = "synthetic-clinvar-ncbi-key";
const COMMAND: &[&str] = &["get", "variant", "clinvar:13961", "clinvar"];

pub(super) fn trials() -> Vec<Trial> {
    vec![Trial::test(
        "variant-assertions/optional-ncbi-key-is-outgoing-only",
        || configured().map_err(Into::into),
    )]
}

fn configured() -> Result<(), String> {
    // The shared limiter owns timing. Check its declared contract here rather
    // than imposing an upper wall-clock bound on a child sharing a busy host.
    let quota = biomcp_adapters::clinvar::SPEC.quota;
    if quota.interval(false) != Duration::from_nanos(333_333_334)
        || quota.interval(true) != Duration::from_millis(100)
    {
        return Err("ClinVar changed its public or optional-key rate interval".into());
    }
    let mut baseline = None;
    for mode in 0..3 {
        let value = wire(mode)?;
        if let Some(baseline) = &baseline {
            if baseline != &value {
                return Err("an optional NCBI key changed the ClinVar reply".into());
            }
        } else {
            baseline = Some(value);
        }
    }
    Ok(())
}

fn wire(mode: u8) -> Result<Value, String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let stub = Stub::for_source("eutils.ncbi.nlm.nih.gov", "application/xml", reply)?;
    let ca = env.root().join("ca.pem");
    stub.retain(&env);
    fs::write(&ca, &stub.certificate).map_err(|e| e.to_string())?;
    let mut config = json!({"proxy":format!("http://{}",stub.address),"ca_bundle":ca});
    if mode == 1 {
        config
            .as_object_mut()
            .ok_or("config must be an object")?
            .insert("keys".into(), json!({"ncbi_api_key":KEY}));
    }
    env.config(&config).map_err(|e| e.to_string())?;
    let capture = env.root().join("capture");
    let mut command = env.command(env!("CARGO_BIN_EXE_biomcp"), &args("--record", &capture));
    if mode == 2 {
        command.env("NCBI_API_KEY", KEY);
    }
    let output = command.output().map_err(|e| e.to_string())?;
    let value = checked(&output)?;
    let requests = stub.finish()?;
    let [(_, request)] = requests.as_slice() else {
        return Err("ClinVar must make exactly one local stub request".into());
    };
    let target = request
        .lines()
        .next()
        .and_then(|line| line.split_whitespace().nth(1))
        .ok_or("missing HTTP request target")?;
    let keys = target
        .split_once('?')
        .ok_or("missing ClinVar query")?
        .1
        .split('&')
        .filter_map(|pair| pair.split_once('='))
        .filter(|(name, _)| *name == "api_key")
        .map(|(_, value)| value.to_owned())
        .collect::<Vec<_>>();
    if keys
        != if mode == 0 {
            vec![]
        } else {
            vec![KEY.to_owned()]
        }
        || request.matches(KEY).count() != usize::from(mode != 0)
    {
        return Err("the selected NCBI credential did not cross only the query boundary".into());
    }
    recordings(&capture)?;
    // A fresh keyless child must replay every mode's capture under one identity.
    let keyless = Environment::new().map_err(|e| e.to_string())?;
    let replay = keyless
        .command(env!("CARGO_BIN_EXE_biomcp"), &args("--replay", &capture))
        .output()
        .map_err(|e| e.to_string())?;
    if checked(&replay)? != value {
        return Err("the optional key changed canonical replay identity or reply".into());
    }
    Ok(value)
}

fn args(flag: &str, path: &Path) -> Vec<String> {
    COMMAND
        .iter()
        .map(|s| (*s).to_owned())
        .chain([flag.to_owned(), path.to_string_lossy().into_owned()])
        .collect()
}

fn checked(output: &std::process::Output) -> Result<Value, String> {
    for bytes in [&output.stdout, &output.stderr] {
        if bytes.windows(KEY.len()).any(|s| s == KEY.as_bytes()) {
            return Err("NCBI credential entered command output".into());
        }
    }
    if !output.status.success() || !output.stderr.is_empty() {
        return Err(format!(
            "ClinVar command failed with {:?}",
            output.status.code()
        ));
    }
    let value: Value = serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;
    super::schema::Registry::embedded()?.validate(
        &value,
        &COMMAND.iter().map(|s| (*s).to_owned()).collect::<Vec<_>>(),
        true,
    )?;
    if value.pointer("/sources/0/state") != Some(&json!("ok")) {
        return Err("the successful native ClinVar reply is missing".into());
    }
    Ok(value)
}

fn recordings(capture: &Path) -> Result<(), String> {
    let files = fs::read_dir(capture.join("clinvar"))
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    if files.len() != 1 {
        return Err("expected exactly one ClinVar recording".into());
    }
    for file in files {
        let bytes = fs::read(file.path()).map_err(|e| e.to_string())?;
        if bytes.windows(KEY.len()).any(|s| s == KEY.as_bytes())
            || bytes.windows(b"api_key".len()).any(|s| s == b"api_key")
        {
            return Err("NCBI credential entered a recording".into());
        }
    }
    Ok(())
}

fn reply(_: &str) -> Result<String, String> {
    let root = super::launch::fixtures().join("replies/variant/assertions/domains/clinvar");
    let file = fs::read_dir(root)
        .map_err(|e| e.to_string())?
        .next()
        .ok_or("missing synthetic ClinVar fixture")?
        .map_err(|e| e.to_string())?;
    let value: Value = serde_json::from_slice(&fs::read(file.path()).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    value
        .pointer("/exchanges/0/body/value")
        .and_then(Value::as_str)
        .map(str::to_owned)
        .ok_or("missing synthetic ClinVar XML".into())
}
