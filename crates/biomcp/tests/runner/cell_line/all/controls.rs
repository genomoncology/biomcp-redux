//! Synthetic composites preserve failure, empty, skip and identity boundaries.
use super::{
    Environment, MEASURE, Stub, Trial, Value, fs, invoke, json, parity, path, reply, setup, stub,
    surface,
};
pub(super) fn trials() -> Vec<Trial> {
    let mut trials = vec![Trial::test(
        "cell-line-all/retired-zero-provider-requests",
        || retired().map_err(Into::into),
    )];
    trials.push(Trial::test(
        "cell-line-all/paging-controls-zero-provider-requests",
        || paging().map_err(Into::into),
    ));
    for source in ["cellosaurus", "chembl"] {
        trials.push(Trial::test(
            format!("cell-line-all/policy/{source}"),
            move || policy(source).map_err(Into::into),
        ));
        trials.push(Trial::test(
            format!("cell-line-all/verification/{source}"),
            move || verification(source).map_err(Into::into),
        ));
        for fault in [
            "429", "500", "broken", "html", "timeout", "disabled", "release", "empty", "identity",
        ] {
            if source == "cellosaurus" && fault == "empty" {
                continue;
            }
            trials.push(Trial::test(
                format!("cell-line-all/synthetic/{source}/{fault}"),
                move || fault_case(source, fault).map_err(Into::into),
            ));
        }
    }
    trials
}
fn retired() -> Result<(), String> {
    let stub = stub()?;
    let env = setup(&stub)?;
    let commands = [
        "cell-line drug-response CVCL_2119 --dataset GDSC1 --limit 5 --offset 0",
        "get cell-line CVCL_2119 drug_response",
        "get cell-line CVCL_2119 drug-response",
        "get cell-line CVCL_2119 --section drug_response",
        "get cell-line CVCL_2119 --section=drug-response",
        "gene cell-lines FLT3 --group leukemia --limit 5 --offset 0",
        "drug cell-lines imatinib --cell-line CVCL_2119 --dataset GDSC1",
        "get drug imatinib cell_lines",
        "get drug imatinib cell-lines",
        "get drug imatinib --section=cell_lines",
    ];
    for (index, command) in commands.iter().enumerate() {
        let words = command.split_whitespace().collect::<Vec<_>>();
        let value = parity(&format!("all/retired/{index}"), &env, &words, 2)?;
        path(&value, "/error/code", json!("usage.retired"))?;
        let expected = if index < 5 {
            "PharmacoDB drug response is deferred until it publishes a data reuse grant"
        } else if index == 5 {
            "gene cell-lines is deferred until an exact HPA cell identity join exists"
        } else {
            "drug cell-lines is retired: cell-line screening is outside the cancer-care drug slice."
        };
        path(&value, "/error/message", json!(expected))?;
    }
    if !stub.finish()?.is_empty() {
        return Err("retirement called a provider".into());
    }
    Ok(())
}
fn seed(source: &str, fault: &str) -> Result<(Environment, std::path::PathBuf), String> {
    let stub = stub()?;
    let env = setup(&stub)?;
    let directory = env.root().join("seed");
    invoke(
        &env,
        &[
            "get",
            "cell-line",
            "CVCL_2119",
            "all",
            "--record",
            directory.to_str().ok_or("path")?,
        ],
        0,
    )?;
    if stub.finish()?.len() != 4 {
        return Err("all seed fanout".into());
    }
    if fault != "disabled" {
        mutate(&directory.join(source), source, fault)?;
    }
    Ok((env, directory))
}
fn mutate(directory: &std::path::Path, source: &str, fault: &str) -> Result<(), String> {
    for entry in fs::read_dir(directory).map_err(|e| e.to_string())? {
        let file = entry.map_err(|e| e.to_string())?.path();
        let mut record: Value =
            serde_json::from_slice(&fs::read(&file).map_err(|e| e.to_string())?)
                .map_err(|e| e.to_string())?;
        let url = record.get("url").and_then(Value::as_str).ok_or("url")?;
        let release = url.contains("release-info") || url.contains("status.json");
        if release != (fault == "release") {
            continue;
        }
        let body = match fault {
            "broken" => "{".into(),
            "html" => "<html>verification</html>".into(),
            "empty" => json!({"cell_lines":[],"page_meta":{"limit":5,"offset":0,"total_count":0,"next":null,"previous":null}}).to_string(),
            "identity" => identity(&record, source)?,
            _ => "{}".into(),
        };
        let exchange = json!(
            {"status":if fault == "release" {500} else {fault.parse::<u16>().unwrap_or(200)},
            "content_type":if fault=="html" {"text/html"} else {"application/json"},"delay_ms":if fault=="timeout" {30000} else {0},
            "retry_after_ms":null,"body":{"kind":"text","value":body}}
        );
        *record.get_mut("exchanges").ok_or("exchanges")? =
            json!([exchange.clone(), exchange.clone(), exchange]);
        fs::write(file, record.to_string()).map_err(|e| e.to_string())?;
    }
    Ok(())
}
fn identity(record: &Value, source: &str) -> Result<String, String> {
    let mut body = record
        .pointer("/exchanges/0/body/value")
        .ok_or("body")?
        .clone();
    if let Some(text) = body.as_str() {
        body = serde_json::from_str(text).map_err(|e| e.to_string())?;
    }
    let pointer = if source == "cellosaurus" {
        "/Cellosaurus/cell-line-list/0/accession-list/0/value"
    } else {
        "/cell_lines/0/cellosaurus_id"
    };
    *body.pointer_mut(pointer).ok_or("identity field")? = json!("CVCL_0004");
    Ok(body.to_string())
}
fn fault_case(source: &str, fault: &str) -> Result<(), String> {
    let (env, directory) = seed(source, fault)?;
    let mut words = vec!["get", "cell-line", "CVCL_2119", "all"];
    if fault == "disabled" {
        words.extend(["--disable-sources", source]);
    }
    let exit = if source == "cellosaurus" && fault != "release" {
        4
    } else {
        0
    };
    let mut replay = words.clone();
    replay.extend(["--replay", directory.to_str().ok_or("path")?]);
    let output = surface::invoke_measured(&env, &replay, exit)?;
    let phase = format!("all/synthetic/{source}/{fault}");
    let value = surface::compared(&phase, &env, &words, exit, MEASURE, &directory, output)?;
    let raw = fs::read(env.root().join("mcp-raw").join(&phase).join("mcp.stdout"))
        .map_err(|e| e.to_string())?;
    surface::measure_bytes(&phase, "mcp-rpc", raw.len(), MEASURE)?;
    assertions(source, fault, exit, &value)
}
fn assertions(source: &str, fault: &str, exit: i32, value: &Value) -> Result<(), String> {
    let index = usize::from(source == "chembl");
    path(value, &format!("/sources/{index}/source"), json!(source))?;
    path(
        value,
        &format!("/sources/{index}/state"),
        json!(if fault == "disabled" {
            "skipped"
        } else if fault == "empty" {
            "ok"
        } else {
            "failed"
        }),
    )?;
    let status = if exit == 4 {
        "failed"
    } else if fault == "empty" {
        "complete"
    } else {
        "partial"
    };
    path(value, "/status", json!(status))?;
    path(value, "/result/status", json!(status))?;
    if value
        .pointer(&format!("/sources/{index}/terms_url"))
        .and_then(Value::as_str)
        .is_none_or(str::is_empty)
    {
        return Err("source outcome lost publisher terms".into());
    }
    release_facts(source, fault, value)?;
    if fault != "disabled"
        && fault != "empty"
        && source == "chembl"
        && value
            .get("problems")
            .and_then(Value::as_array)
            .is_none_or(Vec::is_empty)
    {
        return Err("partial composite lost its source problem".into());
    }

    if source == "chembl" && fault != "release" {
        path(
            value,
            "/result/sections/chembl/state",
            json!(if fault == "empty" {
                "empty"
            } else if fault == "disabled" {
                "skipped"
            } else {
                "unavailable"
            }),
        )?;
    }
    if fault != "disabled"
        && fault != "empty"
        && !value
            .get("next")
            .and_then(Value::as_array)
            .ok_or("next")?
            .iter()
            .any(|v| v.get("kind") == Some(&json!("fresh")))
    {
        return Err("failure lost fresh step".into());
    }
    Ok(())
}

fn release_facts(source: &str, fault: &str, value: &Value) -> Result<(), String> {
    if fault == "release" {
        let prefix = if source == "cellosaurus" {
            "/result"
        } else {
            "/result/sections/chembl/data"
        };
        if value.pointer(&format!("{prefix}/release")).is_some()
            || value
                .pointer(&format!("{prefix}/fetched"))
                .and_then(Value::as_u64)
                .is_none()
        {
            return Err("failed release fabricated metadata or hid retrieval time".into());
        }
    }
    Ok(())
}

fn paging() -> Result<(), String> {
    let stub = stub()?;
    let env = setup(&stub)?;
    for (index, controls) in [
        vec!["--section-limit", "1"],
        vec!["--section-limit", "5"],
        vec!["--section-offset", "25"],
        vec!["--section-limit", "1", "--section-offset", "25"],
    ]
    .iter()
    .enumerate()
    {
        let mut words = vec!["get", "cell-line", "CVCL_2119", "all"];
        words.extend(controls);
        let value = parity(&format!("all/paging-refused/{index}"), &env, &words, 2)?;
        path(&value, "/error/code", json!("usage.arguments"))?;
        path(
            &value,
            "/error/message",
            json!(
                "all returns default first pages; use get cell-line <ID> <section> with --section-limit and --section-offset to page one section"
            ),
        )?;
    }
    if !stub.finish()?.is_empty() {
        return Err("all paging refusal called a provider".into());
    }
    Ok(())
}
fn matches_source(request: &str, source: &str) -> bool {
    if source == "chembl" {
        request.contains("/chembl/api/data/cell_line.json?")
    } else {
        request.contains("/cell-line/CVCL_2119?")
    }
}
fn verification(source: &'static str) -> Result<(), String> {
    let stub = Stub::for_sources(
        &["api.cellosaurus.org", "www.ebi.ac.uk"],
        move |request| {
            if matches_source(request, source) {
                "text/html"
            } else {
                "application/json"
            }
        },
        move |request| {
            if matches_source(request, source) {
                Ok("<html><a href=\"/verify\">Verify your browser</a><form action=\"/verify\"></form></html>".into())
            } else {
                reply(request)
            }
        },
    )?;
    let env = setup(&stub)?;
    let value = parity(
        &format!("all/verification/{source}"),
        &env,
        &["get", "cell-line", "CVCL_2119", "all"],
        if source == "cellosaurus" { 4 } else { 0 },
    )?;
    path(
        &value,
        "/status",
        json!(if source == "cellosaurus" {
            "failed"
        } else {
            "partial"
        }),
    )?;
    let requests = stub.finish()?;
    if requests
        .iter()
        .filter(|(_, r)| matches_source(r, source))
        .count()
        != 1
        || requests.iter().any(|(_, r)| r.contains("/verify"))
    {
        return Err("HTML verification was retried or followed".into());
    }
    Ok(())
}

fn policy(source: &str) -> Result<(), String> {
    let stub = stub()?;
    let env = setup(&stub)?;
    let words = ["get", "cell-line", "CVCL_2119", "all"];
    let (_, bytes) = surface::invoke_measured(&env, &words, 0)?;
    surface::measure_bytes(&format!("all/policy/{source}/prime"), "cli", bytes, MEASURE)?;
    let mut disabled = words.to_vec();
    disabled.extend(["--disable-sources", source]);
    let (value, bytes) =
        surface::invoke_measured(&env, &disabled, if source == "cellosaurus" { 4 } else { 0 })?;
    surface::measure_bytes(&format!("all/policy/{source}/warm"), "cli", bytes, MEASURE)?;
    path(
        &value,
        &format!("/sources/{}/state", usize::from(source == "chembl")),
        json!("skipped"),
    )?;
    if source == "cellosaurus" {
        if value.pointer("/result/header").is_some() {
            return Err("disabled Cellosaurus returned cached identity".into());
        }
    } else {
        path(&value, "/result/sections/chembl/state", json!("skipped"))?;
    }
    if stub.finish()?.len() != 4 {
        return Err("disabled source reached the network instead of checked cache policy".into());
    }
    Ok(())
}
