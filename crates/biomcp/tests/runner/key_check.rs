//! Exercise source credential checks through the CLI and hosted HTTP.
use super::{
    key_registration::{invoke, invoke_without_keys},
    launch::Environment,
    transcript::trial,
};
use libtest_mimic::Trial;

pub(super) fn trials() -> Vec<Trial> {
    let mut trials = vec![
        trial("source-key-check/keyless", || {
            let env = Environment::new()?;
            let reply = invoke(&env, &["--offline", "key", "check", "ctgov"], "", 0)?;
            if reply
                .pointer("/result/status")
                .and_then(serde_json::Value::as_str)
                != Some("no_key_required")
            {
                return Err("ClinicalTrials.gov did not report that it needs no key".into());
            }
            Ok(())
        }),
        trial("source-key-check/hosted-caller-only", || {
            hosted().map_err(Into::into)
        }),
    ];
    trials.extend(PROVIDERS.iter().map(|&(source, field, rejection)| {
        trial(format!("source-key-check/{source}"), move || {
            provider(source, field, rejection).map_err(Into::into)
        })
    }));
    trials
}

const KEY: &str = "synthetic-key-check-credential";
const PROVIDERS: &[(&str, &str, &str)] = &[
    ("ncbi", "ncbi_api_key", "API key invalid"),
    ("nci", "nci_api_key", "Forbidden"),
    ("openfda", "openfda_api_key", "API_KEY_INVALID"),
    (
        "alphagenome",
        "alphagenome_api_key",
        "Authentication or permission denied",
    ),
    ("semantic-scholar", "semantic_scholar_api_key", "Forbidden"),
];

fn provider(source: &str, field: &str, rejection: &str) -> Result<(), String> {
    for (status, reason) in [
        ("working", None),
        ("rejected", Some(rejection)),
        ("could_not_check", Some("deadline-exceeded")),
    ] {
        let env = Environment::new().map_err(|e| e.to_string())?;
        env.config(&serde_json::json!({"keys": {field: KEY}}))
            .map_err(|e| e.to_string())?;
        let replay = fixtures().join(status);
        let args = env.globals(Some(&replay), false);
        let args = args
            .iter()
            .map(String::as_str)
            .chain(["key", "check", source])
            .collect::<Vec<_>>();
        let reply = invoke_without_keys(&env, &args, "", 0, &[KEY])?;
        check(
            reply.get("result").ok_or("missing key check result")?,
            source,
            status,
            reason,
        )?;
    }
    Ok(())
}

fn fixtures() -> std::path::PathBuf {
    super::launch::fixtures().join("replies/key-check")
}

fn check(
    reply: &serde_json::Value,
    source: &str,
    status: &str,
    reason: Option<&str>,
) -> Result<(), String> {
    if reply.to_string().contains(KEY) {
        return Err("credential leaked in the key check reply".into());
    }
    let field = |name| reply.get(name).and_then(serde_json::Value::as_str);
    if field("source") != Some(source)
        || field("status") != Some(status)
        || field("reason") != reason
    {
        return Err(format!(
            "wrong key check result for {source} ({status}): {reply}"
        ));
    }
    Ok(())
}

fn hosted() -> Result<(), String> {
    use serde_json::json;
    let path = "/internal/key-check";
    let body = r#"{"source":"ncbi"}"#;
    let mut server = super::http::Server::configured_globals(
        "127.0.0.1:0",
        Some(json!({"keys":{"ncbi_api_key":"synthetic-server-key-check"}})),
        Some(fixtures().join("working")),
        None,
        None,
        &["--hosted".into()],
    )?;
    let (status, reply) = server.request("POST", path, body)?;
    if status != 200 {
        return Err(format!("missing caller key returned HTTP {status}"));
    }
    check(&reply, "ncbi", "could_not_check", Some("no-key"))?;
    let attempted_key = json!({"source":"ncbi", "key":KEY});
    if server.request("POST", path, &attempted_key.to_string())?.0 != 400 {
        return Err("internal route accepted a credential in its body".into());
    }
    let headers = format!("x-biomcp-key-ncbi-api-key: {KEY}\r\n");
    if server
        .request_with_headers("POST", path, body, &headers.repeat(2))?
        .0
        != 400
    {
        return Err("internal route accepted duplicate credential headers".into());
    }
    let reply = super::request_keys::call(&server, &["key", "check", "ncbi"], &headers, false)?;
    if !reply.to_string().contains("usage.mcp_unavailable") {
        return Err("MCP allowed a key check".into());
    }
    let (status, reply) = server.request_with_headers("POST", path, body, &headers)?;
    if status != 200 {
        return Err(format!("caller key check returned HTTP {status}"));
    }
    check(&reply, "ncbi", "working", None)?;
    let (_, reply) = server.request("POST", path, body)?;
    check(&reply, "ncbi", "could_not_check", Some("no-key"))?;
    server.stop()?;

    let mut plain = super::http::Server::start()?;
    if plain.request_with_headers("POST", path, body, &headers)?.0 != 404 {
        return Err("ordinary HTTP exposed the internal route".into());
    }
    plain.stop()?;

    let mut exposed = super::http::Server::configured_globals(
        "0.0.0.0:0",
        None,
        None,
        None,
        Some("127.0.0.1"),
        &["--hosted".into()],
    )?;
    exposed.address.set_ip(std::net::Ipv4Addr::LOCALHOST.into());
    if exposed
        .request_with_headers("POST", path, body, &headers)?
        .0
        != 404
    {
        return Err("a non-loopback listener exposed the internal route".into());
    }
    exposed.stop()
}
