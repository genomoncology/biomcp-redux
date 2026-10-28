//! Feature source-controls: disease services and carried components share publisher policy.
use super::{configuration::invoke, launch::Environment};
use libtest_mimic::Trial;
use serde_json::Value;
type Check = Result<(), String>;
pub(super) fn trials() -> Vec<Trial> {
    vec![
        Trial::test("disease-registry/component-health", || {
            health::run().map_err(Into::into)
        }),
        Trial::test("disease-registry/publisher-declarations", || {
            declarations().map_err(Into::into)
        }),
        Trial::test("disease-registry/disabled-health-union", || {
            disabled_health().map_err(Into::into)
        }),
        Trial::test("disease-registry/component-policy", || {
            components().map_err(Into::into)
        }),
        Trial::test("disease-registry/enabled-health-native-probes", || {
            enabled_health().map_err(Into::into)
        }),
        Trial::test("disease-registry/disabled-before-cache-and-replay", || {
            admission::run().map_err(Into::into)
        }),
    ]
}
#[path = "disease_registry/health.rs"]
mod health;
fn declarations() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let health = invoke(&env, vec!["health".into(), "--offline".into()], 0)?;
    let rows = health
        .pointer("/result/sources")
        .and_then(Value::as_array)
        .ok_or("missing health")?;
    for (name, usage, url) in [
        (
            "ols",
            "attribution",
            "https://www.ebi.ac.uk/about/terms-of-use/",
        ),
        (
            "mydisease",
            "attribution",
            "https://docs.mydisease.info/en/latest/doc/data.html",
        ),
        (
            "monarch",
            "attribution",
            "https://monarch-app.monarchinitiative.org/Licensing/",
        ),
        (
            "seer",
            "open",
            "https://seer.cancer.gov/about/using-website.html",
        ),
    ] {
        let row = rows
            .iter()
            .find(|r| r["source"] == name)
            .ok_or_else(|| format!("missing {name}"))?;
        if row["use"] != usage
            || row["terms_url"] != url
            || row["disabled"] != false
            || row["terms"].as_str().is_none_or(str::is_empty)
        {
            return Err(format!("wrong declaration: {row}"));
        }
    }
    for name in ["oncotree", "medlineplus", "umls", "disgenet", "reporter"] {
        if rows.iter().any(|r| r["source"] == name) {
            return Err(format!("dropped route declared: {name}"));
        }
    }
    Ok(())
}

fn components() -> Check {
    use biomcp_adapters::{disease_sources::components, registry};
    use biomcp_fetch::{Fetch, FetchConfig, Mode};
    use biomcp_types::DiseaseOntology;
    use std::time::Duration;
    for (switch, blocked) in [
        ("mondo", "mondo"),
        ("doid", "doid"),
        ("ncit", "ncit"),
        ("ols", "all"),
        ("restricted", "none"),
    ] {
        let fetch = Fetch::open(FetchConfig {
            mode: Mode::Offline,
            disabled_sources: registry::disabled(&[switch.into()]).map_err(|e| e.to_string())?,
            disabled_uses: registry::disabled_uses(&[switch.into()]),
            ..Default::default()
        })
        .map_err(|e| e.to_string())?;
        let leg = fetch.leg(
            &biomcp_adapters::ols::SPEC,
            fetch.deadline(Duration::from_secs(1)),
        );
        for &ontology in DiseaseOntology::ALL {
            let admitted = components::ontology(&leg, ontology);
            if admitted.is_err() != (blocked == "all" || blocked == ontology.as_str()) {
                return Err(format!("{switch} admitted wrong ontology: {ontology:?}"));
            }
        }
    }
    outcome_labels()?;
    component_labels()
}
fn outcome_labels() -> Check {
    use biomcp_adapters::registry;
    use biomcp_types::{Reason, Source, SourceState};
    for source in [
        Source::Ols,
        Source::MyDisease,
        Source::Monarch,
        Source::Seer,
    ] {
        for (state, reason) in [
            (SourceState::Ok, None),
            (SourceState::Skipped, Some(Reason::Disabled)),
            (SourceState::Failed, Some(Reason::Unavailable)),
        ] {
            let mut outcome = registry::outcome(source, state, reason);
            outcome.count = Some(0);
            if outcome.terms.as_ref() != registry::terms(source).ok_or("missing terms")?.name {
                return Err("empty/skipped/failed outcome lost terms".into());
            }
        }
    }
    Ok(())
}
fn component_labels() -> Check {
    use biomcp_adapters::disease_sources::components;
    use biomcp_types::Source;
    for (source, name, usage, url) in [
        (
            Source::Ols,
            "mondo",
            "attribution",
            "https://mondo.monarchinitiative.org/pages/download/",
        ),
        (
            Source::Ols,
            "doid",
            "open",
            "https://www.disease-ontology.org/resources/do-resources/",
        ),
        (
            Source::Ols,
            "ncit",
            "attribution",
            "https://evs.nci.nih.gov/ftp1/NCI_Thesaurus/ThesaurusTermsofUse.pdf",
        ),
        (
            Source::Monarch,
            "hpo",
            "attribution",
            "https://human-phenotype-ontology.github.io/license.html",
        ),
        (
            Source::Monarch,
            "orphanet",
            "attribution",
            "https://sciences.orphadata.com/faq/",
        ),
        (
            Source::MyDisease,
            "omim",
            "restricted",
            "https://omim.org/help/agreement",
        ),
    ] {
        let c = components::get(source, name).ok_or("missing component")?;
        if c.terms.usage.as_str() != usage || c.terms.url != url || c.terms.name.is_empty() {
            return Err(format!("incorrect {name} terms"));
        }
    }
    Ok(())
}
const PROBES: &[(&str, &str, &str)] = &[
    (
        "hpo",
        "ontology.jax.org",
        "GET /api/hp/terms/HP:0001250 HTTP/1.1",
    ),
    (
        "ols",
        "www.ebi.ac.uk",
        "GET /ols4/api/ontologies?size=1 HTTP/1.1",
    ),
    ("mydisease", "mydisease.info", "GET /v1/metadata HTTP/1.1"),
    (
        "monarch",
        "api-v3.monarchinitiative.org",
        "GET /v3/api/association?object=MONDO%3A0018875&subject_category=biolink%3AGene&limit=1 HTTP/1.1",
    ),
    (
        "seer",
        "seer.cancer.gov",
        "GET /statistics-network/explorer/source/content_writers/get_var_formats.php HTTP/1.1",
    ),
];
fn enabled_health() -> Check {
    use biomcp_adapters::registry;
    use serde_json::json;
    let env = Environment::new().map_err(|e| e.to_string())?;
    let hosts = PROBES.iter().map(|(_, host, _)| *host).collect::<Vec<_>>();
    let stub = super::network_stub::Stub::for_sources(
        &hosts,
        |_| "application/json",
        |_| Ok("{}".into()),
    )?;
    let cert = env.root().join("private-ca.pem");
    stub.retain(&env);
    std::fs::write(&cert, &stub.certificate).map_err(|e| e.to_string())?;
    env.config(&json!({"ca_bundle":cert,"proxy":format!("http://{}",stub.address)}))
        .map_err(|e| e.to_string())?;
    let disabled = registry::SOURCES
        .iter()
        .filter(|s| {
            !PROBES
                .iter()
                .any(|(name, _, _)| s.source.to_string() == *name)
        })
        .map(|s| s.source.to_string())
        .collect::<Vec<_>>()
        .join(",");
    let health = invoke(
        &env,
        vec!["health".into(), "--disable-sources".into(), disabled],
        0,
    )?;
    let rows = health
        .pointer("/result/sources")
        .and_then(Value::as_array)
        .ok_or("missing health")?;
    let requests = stub.finish()?;
    if requests.len() != PROBES.len() {
        return Err(format!("wrong probe count: {requests:?}"));
    }
    for &(name, host, first_line) in PROBES {
        let row = rows
            .iter()
            .find(|r| r["source"] == name)
            .ok_or("missing source")?;
        if row["state"] != "ok" || row["disabled"] != false {
            return Err(format!("enabled probe failed: {row}"));
        }
        if requests
            .iter()
            .filter(|(connect, request)| {
                connect.starts_with(&format!("CONNECT {host}:443 "))
                    && request.lines().next() == Some(first_line)
            })
            .count()
            != 1
        {
            return Err(format!("wrong {name} probe: {requests:?}"));
        }
    }
    Ok(())
}
mod admission;

fn disabled_health() -> Check {
    use biomcp_adapters::registry;
    use serde_json::json;
    let env = Environment::new().map_err(|e| e.to_string())?;
    let listener = std::net::TcpListener::bind("127.0.0.1:0").map_err(|e| e.to_string())?;
    listener.set_nonblocking(true).map_err(|e| e.to_string())?;
    let replay = env.root().join("empty-replay");
    std::fs::create_dir(&replay).map_err(|e| e.to_string())?;
    env.config(&json!(
        {"disable_sources":["ols","mydisease"],"proxy":format!("http://{}",listener.local_addr().map_err(|e| e.to_string())?)}
    ))
    .map_err(|e| e.to_string())?;
    let disabled = registry::SOURCES
        .iter()
        .filter(|s| {
            !matches!(
                s.source,
                biomcp_types::Source::Ols | biomcp_types::Source::MyDisease
            )
        })
        .map(|s| s.source.to_string())
        .collect::<Vec<_>>()
        .join(",");
    for replay_mode in [false, true] {
        let mut args = vec![
            "health".into(),
            "--disable-sources".into(),
            disabled.clone(),
        ];
        if replay_mode {
            args.extend(["--replay".into(), replay.to_string_lossy().into()]);
        }
        let health = invoke(&env, args, 0)?;
        let rows = health
            .pointer("/result/sources")
            .and_then(Value::as_array)
            .ok_or("missing health")?;
        for row in rows {
            if row["state"] != "skipped"
                || row["reason"] != "disabled"
                || row["disabled"] != true
                || row["terms"].as_str().is_none_or(str::is_empty)
            {
                return Err(format!("disabled source executed: {row}"));
            }
            if let Some(components) = row["components"].as_array()
                && components.iter().any(|c| c["disabled"] != true)
            {
                return Err(format!("provider left component enabled: {row}"));
            }
        }
    }
    if listener.accept().is_ok() {
        return Err("disabled health opened a connection".into());
    }
    Ok(())
}
