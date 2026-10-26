//! The keyed annotation boundary crosses the actual CLI and synthetic TLS peer.
use super::{launch::Environment, network_stub::Stub};
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::{fs, path::Path};
const KEY: &str = "synthetic-oncokb-variant-key";
const INPUT: &str = "chr7:g.140753336A>T";
const SECTION: &str = "/result/candidates/0/sections/oncokb";
pub(super) fn trials() -> Vec<Trial> {
    vec![
        Trial::test("variant/oncokb-environment-key-config-precedence", || {
            environment_key().map_err(Into::into)
        }),
        Trial::test("variant/oncokb-key-policy-wire", || {
            wire().map_err(Into::into)
        }),
        Trial::test("variant/oncokb-server-key-reload", || {
            reload().map_err(Into::into)
        }),
        Trial::test("variant/oncokb-reflected-key-never-leaves-wire", || {
            echo().map_err(Into::into)
        }),
    ]
}
pub(super) fn args() -> Vec<String> {
    [
        "get",
        "variant",
        INPUT,
        "oncokb",
        "--build",
        "GRCh38",
        "--tumor-type",
        "Synthetic tumor",
        "--section-limit",
        "50",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect()
}
pub(super) fn invoke(env: &Environment, args: &[String], exit: i32) -> Result<Value, String> {
    let output = env
        .command(env!("CARGO_BIN_EXE_biomcp"), args)
        .output()
        .map_err(|e| e.to_string())?;
    if output.status.code() != Some(exit) || !output.stderr.is_empty() {
        return Err(format!(
            "wrong annotation exit {:?}, expected {exit}",
            output.status.code()
        ));
    }
    if [output.stdout.as_slice(), output.stderr.as_slice()]
        .concat()
        .windows(KEY.len())
        .any(|b| b == KEY.as_bytes())
    {
        return Err("annotation credential leaked".into());
    }
    let value = serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;
    super::schema::Registry::embedded()?.validate(&value, args, exit == 0)?;
    Ok(value)
}
fn wire() -> Result<(), String> {
    let stub = Stub::for_sources(
        &["myvariant.info", "www.oncokb.org"],
        |_| "application/json",
        reply,
    )?;
    let env = Environment::new().map_err(|e| e.to_string())?;
    let ca = env.root().join("ca.pem");
    fs::write(&ca, &stub.certificate).map_err(|e| e.to_string())?;
    let mut config = json!({"proxy":format!("http://{}",stub.address),"ca_bundle":ca});
    env.config(&config).map_err(|e| e.to_string())?;
    let mut missing_build = args();
    missing_build.drain(4..6);
    invoke(&env, &missing_build, 2)?;
    let mut missing_tumor = args();
    missing_tumor.drain(6..8);
    invoke(&env, &missing_tumor, 2)?;
    let skipped = invoke(&env, &args(), 0)?;
    if skipped.pointer(&format!("{SECTION}/state")) != Some(&json!("skipped"))
        || skipped.pointer(&format!("{SECTION}/reason")) != Some(&json!("no_key"))
        || !skipped
            .get("next")
            .and_then(Value::as_array)
            .is_some_and(|rows| rows.iter().any(|r| r.get("kind") == Some(&json!("key"))))
    {
        return Err(format!(
            "no-key annotation did not skip with setup: {skipped}"
        ));
    }
    config
        .as_object_mut()
        .ok_or("test config object")?
        .insert("keys".into(), json!({"oncokb_api_key":KEY}));
    env.config(&config).map_err(|e| e.to_string())?;
    let mut recorded = args();
    let directory = env.root().join("recorded");
    recorded.extend(["--record".into(), directory.to_string_lossy().into()]);
    let data = invoke(&env, &recorded, 0)?;
    let size = data.to_string().len();
    if size > 2098 {
        return Err(format!(
            "annotation complete envelope exceeds measured ceiling: {size} bytes"
        ));
    }
    if data.pointer(&format!("{SECTION}/state")) != Some(&json!("data")) {
        return Err(format!("keyed annotation missing: {data}"));
    }
    let requests = stub.finish()?;
    let keyed = requests
        .iter()
        .filter(|(_, r)| r.starts_with("GET /api/v1/annotate/"))
        .collect::<Vec<_>>();
    if keyed.len() != 1 || requests.len() != 1 {
        return Err("OncoKB request without key or redundant request".into());
    }
    let request = &keyed.first().ok_or("missing annotation request")?.1;
    if !request.starts_with("GET \
        /api/v1/annotate/mutations/byHGVSg?hgvsg=7%3Ag.140753336A%3ET&referenceGenome=GRCh38&tumorType=Synth\
        etic%20tumor HTTP/1.1") || !request.lines().any(|l|l.eq_ignore_ascii_case(&format!("authorization: Bearer {KEY}"))) {
return Err(format!("source request context/header mismatch: {}",
 request.lines().next().unwrap_or_default()));}
    for file in fs::read_dir(directory.join("oncokb")).map_err(|e| e.to_string())? {
        let bytes = fs::read(file.map_err(|e| e.to_string())?.path()).map_err(|e| e.to_string())?;
        if bytes.windows(KEY.len()).any(|b| b == KEY.as_bytes()) {
            return Err("credential in recording".into());
        }
    }
    content(&data)?;
    replay_proof(&env, &directory)?;
    Ok(())
}
pub(super) fn reply(request: &str) -> Result<String, String> {
    if request.starts_with("GET /v1/variant/") {
        return Ok(json!({"_id":INPUT,"dbsnp":{"rsid":"rs113488022","gene":{"symbol":"BRAF"}},"clinvar":{"variant_id":13961}}).to_string());
    }
    Ok(json!({"query":{"hgvs":"7:g.140753336A>T",
"referenceGenome":"GRCh38",
"tumorType":"Synthetic tumor",
"hugoSymbol":"BRAF",
"alteration":"V600E"},
"variantExist":false,
"alleleExist":false,
"oncogenic":"Oncogenic",
"mutationEffect":{"knownEffect":"Neutral",
"description":"Synthetic mutation effect.",
"citations":{"pmids":["17611497"]}},
"variantSummary":" Synthetic source text Ω ",
"dataVersion":"synthetic-v1",
"lastUpdate":"10/24/2026",
"treatments":[{"level":"LEVEL_1",
"alterations":["V600E",
"V600K"],
"drugs":[{"drugName":"Synthetic drug",
"ncitCode":"C0000"}],
"pmids":["17611497"]}],
"diagnosticImplications":[],
"prognosticImplications":[]})
    .to_string())
}

fn content(data: &Value) -> Result<(), String> {
    let records = data
        .pointer(&format!("{SECTION}/data/records"))
        .and_then(Value::as_array)
        .ok_or("annotation records absent")?;
    let oncogenic = records
        .iter()
        .find(|r| r.pointer("/statement/proposition") == Some(&json!("oncogenicity")))
        .ok_or("oncogenicity missing")?;
    if oncogenic.pointer("/statement/subject")
        != Some(&json!({"level":"protein","gene":{"label":"BRAF"},"label":"V600E"}))
        || oncogenic.pointer("/statement/classification/coding/system")
            != Some(&json!("oncokb_oncogenicity"))
    {
        return Err("source protein subject or own framework changed".into());
    }
    let effect = records
        .iter()
        .find(|r| r.pointer("/statement/proposition") == Some(&json!("functional_impact")))
        .ok_or("effect missing")?;
    if effect.pointer("/statement/direction").is_some()
        || effect.pointer("/statement/classification/coding").is_some()
        || effect.pointer("/statement/citations") != Some(&json!(["17611497"]))
    {
        return Err("functional Neutral gained direction/coding or lost citations".into());
    }
    let group = records
        .iter()
        .find(|r| r.get("kind") == Some(&json!("implication")))
        .ok_or("native implication missing")?;
    if group.get("statement_unmapped") != Some(&json!(true))
        || group.get("alterations") != Some(&json!(["V600E", "V600K"]))
        || group.get("tumor_type").is_some()
        || group.get("statement").is_some()
    {
        return Err("grouped implication got invented subject/context".into());
    }
    if data.pointer(&format!("{SECTION}/data/variant_exist")) != Some(&json!(false))
        || !records
            .iter()
            .any(|r| r.get("text") == Some(&json!(" Synthetic source text Ω ")))
    {
        return Err("native curation flag or text changed".into());
    }
    Ok(())
}
fn replay_args(dir: &Path) -> Vec<String> {
    let mut a = args();
    a.extend(["--replay".into(), dir.to_string_lossy().into()]);
    a
}
fn replay_proof(env: &Environment, dir: &Path) -> Result<(), String> {
    let baseline = invoke(env, &replay_args(dir), 0)?;
    let mcp = super::mcp::call_with_globals(
        env,
        &args(),
        false,
        &["--replay".into(), dir.to_string_lossy().into()],
    )?;
    if baseline.get("result") != mcp.get("result") {
        return Err("annotation CLI/MCP parity".into());
    }
    walk(env, &baseline)?;
    failures(env, dir)?;
    let mut disabled = replay_args(dir);
    disabled.extend(["--disable-sources".into(), "oncokb".into()]);
    let disabled = invoke(env, &disabled, 0)?;
    if disabled.pointer(&format!("{SECTION}/reason")) != Some(&json!("disabled")) {
        return Err("disabled annotation restored cache/replay".into());
    }
    env.config(&json!({})).map_err(|e| e.to_string())?;
    let missing = invoke(env, &replay_args(dir), 0)?;
    if missing.pointer(&format!("{SECTION}/reason")) != Some(&json!("no_key")) {
        return Err("missing key restored annotation replay".into());
    }
    Ok(())
}
#[expect(
    clippy::indexing_slicing,
    reason = "Mutate response-only fields on actual CLI-generated synthetic recordings"
)]
fn failures(env: &Environment, dir: &Path) -> Result<(), String> {
    let entry = fs::read_dir(dir.join("oncokb"))
        .map_err(|e| e.to_string())?
        .next()
        .ok_or("missing capture")?
        .map_err(|e| e.to_string())?
        .path();
    let original: Value = serde_json::from_slice(&fs::read(&entry).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    for (kind, reason) in [
        (401, "rejected"),
        (429, "rate_limited"),
        (500, "unavailable"),
        (0, "bad_reply"),
        (1, "deadline_exceeded"),
        (2, "bad_reply"),
        (3, "bad_reply"),
        (4, "bad_reply"),
        (5, "bad_reply"),
        (6, "bad_reply"),
    ] {
        let target = env.root().join(format!("failure-{kind}"));
        fs::create_dir_all(target.join("oncokb")).map_err(|e| e.to_string())?;
        let mut record = original.clone();
        let mut exchange = record["exchanges"][0].clone();
        exchange["delay_ms"] = json!(0);
        if kind >= 400 {
            exchange["status"] = json!(kind);
            exchange["retry_after_ms"] = json!(1);
        } else if kind == 0 {
            exchange["body"] = json!({"kind":"text","value":"{"});
        } else if kind == 1 {
            exchange["delay_ms"] = json!(30001);
        } else if kind == 2 {
            exchange["body"] = json!({"kind":"json","value":{"error":"synthetic"}});
        } else if kind == 6 {
            exchange["body"]["value"]["errors"] =
                json!([{"type":"INVALID_ALTERATION","message":"Synthetic validation failure."}]);
        } else {
            let field = match kind {
                3 => "hgvs",
                4 => "referenceGenome",
                _ => "tumorType",
            };
            exchange["body"]["value"]["query"][field] = if kind == 4 {
                json!("GRCh37")
            } else {
                json!("different")
            };
        }
        record["exchanges"] = json!(vec![
            exchange;
            if kind == 429 || kind == 500 { 3 } else { 1 }
        ]);
        fs::write(
            target
                .join("oncokb")
                .join(entry.file_name().ok_or("filename")?),
            serde_json::to_vec(&record).map_err(|e| e.to_string())?,
        )
        .map_err(|e| e.to_string())?;
        let output = invoke(env, &replay_args(&target), 4)?;
        if output.pointer(&format!("{SECTION}/reason")) != Some(&json!(reason)) {
            return Err(format!(
                "annotation failure {kind} lost its reason: {output}"
            ));
        }
    }
    Ok(())
}

fn reload() -> Result<(), String> {
    for http in [false, true] {
        let assets = Environment::new().map_err(|e| e.to_string())?;
        let stub = Stub::for_source("www.oncokb.org", "application/json", reload_reply)?;
        let ca = assets.root().join("ca.pem");
        fs::write(&ca, &stub.certificate).map_err(|e| e.to_string())?;
        let mut client = super::key_reload::start(
            http,
            false,
            json!({"proxy":format!("http://{}",stub.address),"ca_bundle":ca}),
            "ONCOKB_TOKEN",
        )?;
        for (index, key) in [None, Some(KEY), Some("synthetic-oncokb-replaced-key"), None]
            .into_iter()
            .enumerate()
        {
            if let Some(key) = key {
                let commands = if index == 1 {
                    vec!["key", "set", "oncokb"]
                } else {
                    vec!["key", "set", "oncokb", "--replace"]
                };
                super::key_registration::invoke(
                    client.environment(),
                    &commands,
                    &format!("{key}\n"),
                    0,
                )?;
            } else if index == 3 {
                super::key_registration::invoke(
                    client.environment(),
                    &["key", "remove", "oncokb"],
                    "",
                    0,
                )?;
            }
            let mut args = args();
            *args.get_mut(7).ok_or("tumor argument")? = format!("Synthetic tumor {index}");
            let refs = args.iter().map(String::as_str).collect::<Vec<_>>();
            let output = client.call(index, &refs)?;
            let expected = if key.is_some() { "data" } else { "skipped" };
            if output.pointer(&format!("{SECTION}/state")) != Some(&json!(expected)) {
                return Err("persistent OncoKB call did not reload key state".into());
            }
            for marker in [KEY, "synthetic-oncokb-replaced-key"] {
                if output.to_string().contains(marker) {
                    return Err("key leaked in persistent annotation".into());
                }
            }
        }
        client.finish()?;
        let requests = stub.finish()?;
        if requests.len() != 2 {
            return Err("persistent OncoKB made keyless or missing requests".into());
        }
        for ((_, request), key) in requests.iter().zip([KEY, "synthetic-oncokb-replaced-key"]) {
            if !request
                .lines()
                .any(|l| l.eq_ignore_ascii_case(&format!("authorization: Bearer {key}")))
            {
                return Err("persistent OncoKB used stale key snapshot".into());
            }
        }
    }
    Ok(())
}

fn walk(env: &Environment, baseline: &Value) -> Result<(), String> {
    let expected = baseline
        .pointer(&format!("{SECTION}/data/records"))
        .and_then(Value::as_array)
        .ok_or("complete records")?;
    let mut rows = Vec::new();
    let mut offset = 0;
    loop {
        let mut page = args();
        *page.last_mut().ok_or("section limit")? = "1".into();
        page.extend(["--section-offset".into(), offset.to_string()]);
        let value = invoke(env, &page, 0)?;
        let data = value
            .pointer(&format!("{SECTION}/data"))
            .ok_or("warm annotation data")?;
        let items = data
            .get("records")
            .and_then(Value::as_array)
            .ok_or("warm annotation rows")?;
        if items.len() != 1 || data.pointer("/page/total") != Some(&json!(expected.len())) {
            return Err("annotation warm row bounds".into());
        }
        rows.extend(items.iter().cloned());
        if let Some(next) = data.get("next") {
            offset += 1;
            if next.get("section_offset") != Some(&json!(offset))
                || next.get("build") != Some(&json!("GRCh38"))
                || next.get("tumor_type") != Some(&json!("Synthetic tumor"))
                || next.get("section") != Some(&json!(["oncokb"]))
                || next.get("input") != Some(&json!(INPUT))
                || rows.len() >= expected.len()
            {
                return Err("annotation continuation changed context or repeated a page".into());
            }
        } else {
            break;
        }
    }
    if &rows != expected {
        return Err("annotation warm walk changed native order or lost records".into());
    }
    Ok(())
}

fn echo() -> Result<(), String> {
    let stub = Stub::for_source("www.oncokb.org", "application/json", echo_reply)?;
    let env = Environment::new().map_err(|e| e.to_string())?;
    let ca = env.root().join("ca.pem");
    fs::write(&ca, &stub.certificate).map_err(|e| e.to_string())?;
    env.config(&json!({"proxy":format!("http://{}",stub.address),"ca_bundle":ca,"keys":{"oncokb_api_key":KEY}})).map_err(|e|e.to_string())?;
    let mut command = args();
    let directory = env.root().join("echo");
    command.extend(["--record".into(), directory.to_string_lossy().into()]);
    let output = env
        .command(env!("CARGO_BIN_EXE_biomcp"), &command)
        .output()
        .map_err(|e| e.to_string())?;
    let requests = stub.finish()?;
    if requests.len() != 1 {
        return Err("echo proof did not reach exactly one authenticated TLS request".into());
    }
    let leaked_output = [output.stdout.as_slice(), output.stderr.as_slice()]
        .concat()
        .windows(KEY.len())
        .any(|b| b == KEY.as_bytes());
    let mut leaked_recording = false;
    if directory.join("oncokb").exists() {
        for file in fs::read_dir(directory.join("oncokb")).map_err(|e| e.to_string())? {
            let bytes =
                fs::read(file.map_err(|e| e.to_string())?.path()).map_err(|e| e.to_string())?;
            leaked_recording |= bytes.windows(KEY.len()).any(|b| b == KEY.as_bytes());
        }
    }
    if leaked_output || leaked_recording {
        return Err(format!(
            "reflected synthetic credential leaked: output={leaked_output}, recording={leaked_recording}"
        ));
    }
    if output.status.code() != Some(4) {
        return Err("credential-bearing reply was not refused".into());
    }
    Ok(())
}
fn echo_reply(request: &str) -> Result<String, String> {
    let mut body: Value = serde_json::from_str(&reply(request)?).map_err(|e| e.to_string())?;
    body.as_object_mut()
        .ok_or("synthetic annotation object")?
        .insert("geneSummary".into(), json!(KEY));
    Ok(body.to_string())
}

fn environment_key() -> Result<(), String> {
    for configured in [false, true] {
        let stub = Stub::for_source("www.oncokb.org", "application/json", reply)?;
        let env = Environment::new().map_err(|e| e.to_string())?;
        let ca = env.root().join("ca.pem");
        fs::write(&ca, &stub.certificate).map_err(|e| e.to_string())?;
        let mut config = json!({"proxy":format!("http://{}",stub.address),"ca_bundle":ca});
        if configured {
            config
                .as_object_mut()
                .ok_or("config object")?
                .insert("keys".into(), json!({"oncokb_api_key":KEY}));
        }
        env.config(&config).map_err(|e| e.to_string())?;
        let output = env
            .command(env!("CARGO_BIN_EXE_biomcp"), &args())
            .env(
                "ONCOKB_TOKEN",
                if configured {
                    "synthetic-environment-shadowed"
                } else {
                    KEY
                },
            )
            .output()
            .map_err(|e| e.to_string())?;
        let requests = stub.finish()?;
        if !output.status.success()
            || !output.stderr.is_empty()
            || requests.len() != 1
            || !requests.first().is_some_and(|(_, request)| {
                request
                    .lines()
                    .any(|line| line.eq_ignore_ascii_case(&format!("authorization: Bearer {KEY}")))
            })
        {
            return Err("OncoKB environment key or config precedence failed".into());
        }
        for marker in [KEY, "synthetic-environment-shadowed"] {
            if output
                .stdout
                .windows(marker.len())
                .any(|part| part == marker.as_bytes())
            {
                return Err("OncoKB environment credential leaked".into());
            }
        }
    }
    Ok(())
}

// Each key snapshot uses a distinct uncached input over both transports.
fn reload_reply(request: &str) -> Result<String, String> {
    let index = (0..4)
        .find(|index| request.contains(&format!("tumorType=Synthetic%20tumor%20{index}")))
        .ok_or("missing synthetic tumor input")?;
    Ok(reply(request)?.replace("Synthetic tumor", &format!("Synthetic tumor {index}")))
}
