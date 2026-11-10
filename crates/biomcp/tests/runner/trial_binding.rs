//! Registry continuation capabilities bind selectors and consumed native positions.
use super::{configuration::invoke, http::Server, launch::Environment, mcp, network_stub::Stub};
use serde_json::{Value, json};
use std::sync::mpsc;

pub(super) fn trials() -> Vec<libtest_mimic::Trial> {
    [false, true]
        .into_iter()
        .flat_map(|meta| [false, true].map(move |broad| (meta, broad)))
        .map(|(meta, broad)| {
            super::transcript::trial(
                format!(
                    "trial-search/bound-continuation-{}-{}",
                    if broad { "broader" } else { "native" },
                    if meta { "mcp" } else { "cli" }
                ),
                move || check(meta, broad).map_err(Into::into),
            )
        })
        .chain([super::transcript::trial(
            "trial-search/hosted-continuation-crosses-independent-stores",
            || cross_store().map_err(Into::into),
        )])
        .collect()
}

fn registry(broad: bool) -> Result<(Stub, mpsc::Receiver<String>), String> {
    let (observed, calls) = mpsc::channel();
    let stub = Stub::for_source("clinicaltrials.gov", "application/json", move |request| {
        observed
            .send(request.lines().next().unwrap_or_default().to_owned())
            .map_err(|_| "request observer closed")?;
        if request.contains("/version") {
            return Ok(json!({"dataTimestamp":"2026-11-10T00:00:00Z"}).to_string());
        }
        if broad && !request.contains("query.spons=") {
            return Ok(json!({"totalCount":0,"studies":[]}).to_string());
        }
        let position = if request.contains("pageToken=native-four") {
            4
        } else if request.contains("pageToken=native-two") {
            2
        } else {
            0
        };
        let row = |id: &str| {
            json!({"protocolSection":{"identificationModule":{"nctId":id,
            "briefTitle":"Synthetic bound cursor"},"statusModule":{"overallStatus":"RECRUITING"},
            "conditionsModule":{"conditions":["Other condition"]}}})
        };
        let rows = match position {
            0 => vec![row("NCT00000001"), row("invalid")],
            2 => vec![row("NCT00000003"), row("NCT00000004")],
            _ => vec![row("invalid"), row("invalid")],
        };
        Ok(
            json!({"totalCount":8,"studies":rows,"nextPageToken":match position {
            0=>"native-two",2=>"native-four",_=>"native-six"}})
            .to_string(),
        )
    })?;
    Ok((stub, calls))
}

fn config(stub: &Stub, assets: &Environment) -> Result<Value, String> {
    let ca = assets.root().join("registry-ca.pem");
    std::fs::write(&ca, &stub.certificate).map_err(|e| e.to_string())?;
    Ok(
        json!({"proxy":format!("http://{}",stub.address),"ca_bundle":ca,
        "disable_sources":["mychem","chembl","openfda-label"]}),
    )
}

fn setup(broad: bool) -> Result<(Environment, Stub, mpsc::Receiver<String>), String> {
    let (stub, calls) = registry(broad)?;
    let env = Environment::new().map_err(|e| e.to_string())?;
    stub.retain(env.shared_root());
    env.config(&config(&stub, &env)?)
        .map_err(|e| e.to_string())?;
    Ok((env, stub, calls))
}

fn cross_store() -> Result<(), String> {
    let (stub, calls) = registry(false)?;
    let assets = Environment::new().map_err(|e| e.to_string())?;
    let config = config(&stub, &assets)?;
    let start = || {
        Server::configured_globals(
            "127.0.0.1:0",
            Some(config.clone()),
            None,
            None,
            None,
            &["--hosted".into()],
        )
    };
    let mut first = start()?;
    let mut other = start()?;
    stub.retain(first.environment().shared_root());
    stub.retain(other.environment().shared_root());
    if first.environment().root() == other.environment().root() {
        return Err("the two hosted cores must have independent stores".into());
    }
    let initial = hosted_call(
        &first,
        &[
            "search",
            "trial",
            "--sponsor",
            "Synthetic Registry",
            "--limit",
            "2",
            "--exact-term",
        ],
    )?;
    let next = super::action(&initial, "page")?;
    let words: Vec<_> = next.iter().map(String::as_str).collect();
    let initial_calls = calls.try_iter().count();
    let same = hosted_call(&first, &words)?;
    let native = calls
        .try_iter()
        .filter(|request| request.contains("pageToken=native-two"))
        .count();
    if initial_calls == 0 || native != 1 || same.pointer("/page/offset") != Some(&json!(2)) {
        return Err(format!(
            "same-task control failed, initial={initial_calls} continued={native}: {same}"
        ));
    }
    first.stop()?;
    let followed = hosted_call(&other, &words)?;
    let cross_calls: Vec<_> = calls.try_iter().collect();
    other.stop()?;
    stub.finish()?;
    if followed.pointer("/page/offset") != Some(&json!(2))
        || cross_calls
            .iter()
            .filter(|request| request.contains("pageToken=native-two"))
            .count()
            != 1
    {
        return Err(format!(
            "cross-task continuation failed, initial={initial_calls} same-task-cursor={native} cross-task-requests={}: {followed}",
            cross_calls.len()
        ));
    }
    Ok(())
}

fn hosted_call(server: &Server, args: &[&str]) -> Result<Value, String> {
    let frame = mcp::request(
        json!(1),
        "tools/call",
        json!({"name":"biomcp","arguments":{"args":args}}),
    );
    let (status, reply) = server.request_with_headers(
        "POST",
        "/mcp",
        &frame.to_string(),
        "x-biomcp-use: restricted\r\n",
    )?;
    if status != 200 {
        return Err(format!("hosted HTTP status {status}: {reply}"));
    }
    reply
        .pointer("/result/structuredContent")
        .cloned()
        .ok_or_else(|| format!("missing hosted content: {reply}"))
}

fn call(env: &Environment, args: &[String], meta: bool, error: bool) -> Result<Value, String> {
    if meta {
        mcp::call_with_globals(env, args, error, &[])
    } else {
        invoke(env, args.to_vec(), if error { 2 } else { 0 })
    }
}
fn replace(args: &[String], key: &str, value: &str) -> Result<Vec<String>, String> {
    let mut changed = args.to_vec();
    let index = changed
        .iter()
        .position(|arg| arg == key)
        .ok_or("missing continuation selector")?
        + 1;
    *changed.get_mut(index).ok_or("missing selector value")? = value.into();
    Ok(changed)
}

fn check(meta: bool, broad: bool) -> Result<(), String> {
    let (env, stub, calls) = setup(broad)?;
    let args: Vec<_> = [
        "search",
        "trial",
        "--sponsor",
        "Synthetic Registry",
        "--condition",
        "pancreatic",
        "--phase",
        "1",
        "--limit",
        "2",
        "--exact-term",
    ]
    .map(str::to_owned)
    .into();
    let first = call(&env, &args, meta, false)?;
    let next = super::action(&first, "page")?;
    if first.pointer("/page/returned") != Some(&json!(1)) {
        return Err(format!("first page lacks bound native position: {first}"));
    }
    for changed in [
        replace(&next, "--offset", "40")?,
        replace(&next, "--phase", "2")?,
        replace(&next, "--sponsor", "Other")?,
        next.iter()
            .cloned()
            .chain(["--exact-condition".into()])
            .collect(),
        replace(&next, "--next-page", "native-two")?,
        replace(&next, "--next-page", "biomcp:ctgov-names:v1:native-two")?,
        replace(
            &next,
            "--next-page",
            &format!("biomcp:ctgov-page:v1:{}", "0".repeat(64)),
        )?,
    ] {
        refused(&env, &changed, meta, &calls, true)?;
    }
    let mut excluded = next.clone();
    excluded.extend(["--disable-sources".into(), "ctgov".into()]);
    refused(&env, &excluded, meta, &calls, false)?;
    let restart = refused(&env, &replace(&next, "--offset", "40")?, meta, &calls, true)?;
    call(&env, &restart, meta, false)?;
    let second = call(&env, &next, meta, false)?;
    if second.pointer("/page/offset") != Some(&json!(2))
        || second.pointer("/page/returned") != Some(&json!(2))
    {
        return Err(format!(
            "valid bound cursor lost consumed position: {second}"
        ));
    }
    let exact = super::action(&second, "disambiguate")?;
    if exact
        .iter()
        .any(|word| matches!(word.as_str(), "--offset" | "--next-page"))
    {
        return Err(format!(
            "exact-condition restart retained position: {exact:?}"
        ));
    }
    call(&env, &exact, meta, false)?;
    let third = call(&env, &super::action(&second, "page")?, meta, false)?;
    let fourth = super::action(&third, "page")?;
    if third.pointer("/page/offset") != Some(&json!(4))
        || third.pointer("/page/returned") != Some(&json!(0))
        || !fourth.windows(2).any(|pair| pair == ["--offset", "6"])
    {
        return Err(format!(
            "all-refused page did not consume native rows: {third}"
        ));
    }
    stub.finish()?;
    Ok(())
}

fn refused(
    env: &Environment,
    args: &[String],
    meta: bool,
    calls: &mpsc::Receiver<String>,
    restart_allowed: bool,
) -> Result<Vec<String>, String> {
    for _ in calls.try_iter() {}
    let reply = call(env, args, meta, true)?;
    if calls.try_iter().next().is_some()
        || reply.pointer("/error/option") != Some(&json!("--next-page"))
        || !reply
            .pointer("/error/message")
            .and_then(Value::as_str)
            .is_some_and(|message| message.starts_with("this trial continuation cannot verify"))
    {
        return Err(format!(
            "unbound cursor reached registry or lost guidance: {reply}"
        ));
    }
    if !restart_allowed {
        if reply.get("next").is_some() {
            return Err(format!(
                "disabled source kept an executable next step: {reply}"
            ));
        }
        return Ok(Vec::new());
    }
    let fresh = super::action(&reply, "configure")?;
    if !fresh.iter().any(|word| word == "--fresh")
        || fresh
            .iter()
            .any(|word| matches!(word.as_str(), "--offset" | "--next-page"))
    {
        return Err(format!("invalid fresh recovery: {fresh:?}"));
    }
    Ok(fresh)
}
