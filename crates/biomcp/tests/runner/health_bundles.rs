//! Feature source-controls: publisher reachability and stored readiness are independent.
use super::{configuration::invoke, launch::Environment, network_stub::Stub};
use biomcp_adapters::registry;
use libtest_mimic::Trial;
use serde_json::{Value, json};
type Check = Result<(), String>;
const SOURCES: &[&str] = &[
    "cpic", "clinpgx", "gtr", "who-ivd", "who-pq", "fda-cdx", "fda-pma", "fda-510k", "vaers",
];
const BUNDLES: &[&str] = &["gtr", "who-ivd", "who-pq", "fda-cdx"];
pub(super) fn trials() -> Vec<Trial> {
    vec![
        Trial::test("sources/health-publisher-probes-record-and-replay", || {
            reachable().map_err(Into::into)
        }),
        Trial::test("sources/health-local-bundles-ignore-abandoned-sync", || {
            local().map_err(Into::into)
        }),
        Trial::test("sources/health-publisher-failure-is-one-request", || {
            failure().map_err(Into::into)
        }),
    ]
}
fn disabled(enabled: &[&str]) -> String {
    registry::SOURCES
        .iter()
        .filter(|s| !enabled.contains(&s.source.to_string().as_str()))
        .map(|s| s.source.to_string())
        .collect::<Vec<_>>()
        .join(",")
}
fn row<'a>(v: &'a Value, source: &str) -> Result<&'a Value, String> {
    v.pointer("/result/sources")
        .and_then(Value::as_array)
        .and_then(|rows| rows.iter().find(|r| r["source"] == source))
        .ok_or_else(|| format!("missing {source}: {v}"))
}
fn configure(env: &Environment, stub: &Stub) -> Check {
    let cert = env.root().join("private-ca.pem");
    std::fs::write(&cert, &stub.certificate).map_err(|e| e.to_string())?;
    env.config(&json!({"ca_bundle":cert,"proxy":format!("http://{}", stub.address)}))
        .map_err(|e| e.to_string())
}
fn reachable() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let stub = Stub::for_sources(
        &[
            "api.cpicpgx.org",
            "api.clinpgx.org",
            "ftp.ncbi.nlm.nih.gov",
            "extranet.who.int",
            "www.fda.gov",
            "api.fda.gov",
            "wonder.cdc.gov",
        ],
        |_| "text/plain",
        |request| {
            let path = request.split_whitespace().nth(1).ok_or("no request path")?;
            if ![
                "/v1/gene?limit=1",
                "/v1/data/clinicalAnnotation?location.genes.symbol=DPYD&view=min",
                "/pub/GTR/data/",
                "/prequal/vitro-diagnostics/prequalified/in-vitro-diagnostics",
                "/prequal/medicines/prequalified/finished-pharmaceutical-products",
                "/about-fda/about-website/website-policies",
                "/device/pma.json?limit=1",
                "/device/510k.json?limit=1",
                "/datause.html",
            ]
            .contains(&path)
            {
                return Err(format!(
                    "health fetched undeclared endpoint or bundle: {request}"
                ));
            }
            Ok("publisher reachable".into())
        },
    )?;
    configure(&env, &stub)?;
    let recording = env.root().join("recording");
    for mode in ["--record", "--replay"] {
        let v = invoke(
            &env,
            vec![
                "health".into(),
                mode.into(),
                recording.to_string_lossy().into(),
                "--disable-sources".into(),
                disabled(SOURCES),
            ],
            0,
        )?;
        for source in SOURCES {
            let r = row(&v, source)?;
            if r["state"] != "ok"
                || r.get("reason").is_some()
                || BUNDLES.contains(source) && r["bundle"] != "missing"
            {
                return Err(format!("wrong health {r}"));
            }
        }
    }
    if stub.finish()?.len() != SOURCES.len() {
        return Err("probe retried or replay reached publisher".into());
    }
    Ok(())
}
fn local() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let empty = env.root().join("empty");
    std::fs::create_dir(&empty).map_err(|e| e.to_string())?;
    let mut init = env.globals(Some(&empty), false);
    init.extend(["health".into(), "--disable-sources".into(), disabled(&[])]);
    invoke(&env, init, 0)?;
    let mut args = env.globals(None, true);
    args.extend([
        "health".into(),
        "--disable-sources".into(),
        disabled(BUNDLES),
    ]);
    invoke(&env, args.clone(), 0)?;
    let cache = rusqlite::Connection::open(env.root().join("case-cache/replay.sqlite"))
        .map_err(|e| e.to_string())?;
    for source in BUNDLES {
        cache
            .execute(
                "INSERT INTO bundle VALUES(?1,10,1,9223372036854775807,NULL,NULL)",
                [source],
            )
            .map_err(|e| e.to_string())?;
        cache.execute("INSERT INTO bundle_sync VALUES(?1,'finished-process',0,'https://unused.invalid','unused',0,NULL,NULL)", [source]).map_err(|e|e.to_string())?;
    }
    for expected in ["fresh", "stale"] {
        let v = invoke(&env, args.clone(), 0)?;
        for source in BUNDLES {
            let r = row(&v, source)?;
            if r["bundle"] != expected
                || r["state"] != if expected == "fresh" { "ok" } else { "stale" }
                || r.get("latency_ms").is_some()
                || r.get("reason").is_some()
            {
                return Err(format!("wrong local bundle state {r}"));
            }
        }
        cache
            .execute("UPDATE bundle SET fresh_until=0", [])
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}
fn failure() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let stub = Stub::for_source_status(
        "api.fda.gov",
        "application/json",
        |_| Ok("{}".into()),
        |_| 503,
    )?;
    configure(&env, &stub)?;
    let v = invoke(
        &env,
        vec![
            "health".into(),
            "--disable-sources".into(),
            disabled(&["fda-pma"]),
        ],
        0,
    )?;
    let r = row(&v, "fda-pma")?;
    if r["state"] != "failed" || r["reason"] != "unavailable" {
        return Err(format!("publisher failure disappeared {r}"));
    }
    if stub.finish()?.len() != 1 {
        return Err("health retried failed publisher".into());
    }
    Ok(())
}
