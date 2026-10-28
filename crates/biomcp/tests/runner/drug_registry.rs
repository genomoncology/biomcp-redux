//! Feature source-controls: drug declarations and admission use the shared registry.
use crate::runner::transcript::trial as transcript_trial;
use super::{configuration::invoke, launch::Environment};
use libtest_mimic::Trial;
use serde_json::{Value, json};
type Check = Result<(), String>;
pub(super) fn trials() -> Vec<Trial> {
    vec![
        transcript_trial("drug-registry/publisher-declarations", || {
            declarations().map_err(Into::into)
        }),
        transcript_trial("drug-registry/disabled-health-no-io", || {
            disabled_health().map_err(Into::into)
        }),
        transcript_trial("drug-registry/enabled-health-native-probes", || {
            enabled_health().map_err(Into::into)
        }),
        transcript_trial("drug-registry/component-projection", || {
            projection().map_err(Into::into)
        }),
        transcript_trial("drug-registry/disabled-before-cache-and-replay", || {
            admission().map_err(Into::into)
        }),
    ]
}
fn declarations() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let health = invoke(&env, vec!["health".into(), "--offline".into()], 0)?;
    let rows = health
        .pointer("/result/sources")
        .and_then(Value::as_array)
        .ok_or("missing health")?;
    for (name, usage, url) in [
        (
            "mychem",
            "attribution",
            "https://docs.mychem.info/en/latest/doc/data_source.html",
        ),
        (
            "chembl",
            "attribution",
            "https://chembl.github.io/chembl-licensing/",
        ),
        (
            "opentargets",
            "open",
            "https://platform-docs.opentargets.org/licence",
        ),
        ("civic", "open", "https://civicdb.org/releases/licensing"),
        ("openfda-label", "open", "https://open.fda.gov/terms/"),
        ("drugsfda", "open", "https://open.fda.gov/terms/"),
        (
            "ema",
            "attribution",
            "https://www.ema.europa.eu/en/about-us/about-website/legal-notice",
        ),
        (
            "ddinter",
            "noncommercial",
            "https://ddinter.scbdd.com/terms/",
        ),
    ] {
        let row = rows
            .iter()
            .find(|r| r["source"] == name)
            .ok_or_else(|| format!("missing {name}"))?;
        if row["use"] != usage || row["terms_url"] != url || row["disabled"] != false {
            return Err(format!("wrong declaration: {row}"));
        }
        if row["terms"].as_str().is_none_or(|s| s.is_empty()) {
            return Err("missing terms".into());
        }
    }
    for name in ["who", "fda-orphan", "dgidb", "pharmacodb", "cvx"] {
        if rows.iter().any(|r| r["source"] == json!(name)) {
            return Err(format!("dropped {name} declared"));
        }
    }
    Ok(())
}

fn disabled_health() -> Check {
    use biomcp_adapters::registry;
    let env = Environment::new().map_err(|e| e.to_string())?;
    let listener = std::net::TcpListener::bind("127.0.0.1:0").map_err(|e| e.to_string())?;
    listener.set_nonblocking(true).map_err(|e| e.to_string())?;
    let other_sources = registry::SOURCES
        .iter()
        .filter(|s| s.source != biomcp_types::Source::Ddinter)
        .map(|s| s.source.to_string())
        .collect::<Vec<_>>();
    // An empty existing directory is a replay poison: any admitted probe fails locally.
    let replay = env.root().join("empty-replay");
    std::fs::create_dir(&replay).map_err(|e| e.to_string())?;
    for (command, configured) in [("ddinter", false), ("noncommercial", false), ("", true)] {
        let mut disabled = other_sources.clone();
        if !command.is_empty() {
            disabled.push(command.into());
        }
        env.config(
            &json!({"disable_sources": if configured {vec!["ddinter"]} else {vec![]},
            "proxy":format!("http://{}", listener.local_addr().map_err(|e| e.to_string())?)}),
        )
        .map_err(|e| e.to_string())?;
        for replay_mode in [false, true] {
            let mut args = vec![
                "health".into(),
                "--disable-sources".into(),
                disabled.join(","),
            ];
            if replay_mode {
                args.extend(["--replay".into(), replay.to_string_lossy().into()]);
            }
            let health = invoke(&env, args, 0)?;
            let rows = health
                .pointer("/result/sources")
                .and_then(Value::as_array)
                .ok_or("missing health")?;
            if rows
                .iter()
                .any(|row| row["state"] != "skipped" || row["reason"] != "disabled")
            {
                return Err(format!("disabled source executed: {rows:?}"));
            }
        }
    }
    if listener.accept().is_ok() {
        return Err("disabled health opened a connection".into());
    }
    Ok(())
}
fn projection() -> Check {
    use biomcp_adapters::{drug_sources::components, registry};
    use biomcp_fetch::{Fetch, FetchConfig, Mode};
    use biomcp_types::Source;
    use std::time::Duration;
    let fields = "_id,drugbank.name,chembl.pref_name,drugcentral.synonyms,gtopdb.name,unii.unii,ndc.product_ndc,openfda.spl_set_id,umls.cui";
    for (switch, absent) in [
        ("drugbank-open", "drugbank"),
        ("chembl", "chembl"),
        ("drugcentral", "drugcentral"),
        ("gtopdb", "gtopdb"),
        ("unii", "unii"),
        ("ndc", "ndc"),
        ("openfda-xref", "openfda"),
    ] {
        let fetch = Fetch::open(FetchConfig {
            mode: Mode::Offline,
            disabled_sources: registry::disabled(&[switch.into()]).map_err(|e| e.to_string())?,
            ..Default::default()
        })
        .map_err(|e| e.to_string())?;
        let leg = fetch.leg(
            &biomcp_adapters::drug_sources::MYCHEM,
            fetch.deadline(Duration::from_secs(1)),
        );
        let projected = components::allowed_fields(&leg, fields);
        let expected = fields
            .split(',')
            .filter(|field| !field.starts_with(absent) && !field.starts_with("umls"))
            .collect::<Vec<_>>()
            .join(",");
        if projected != expected
            || !components::disabled(&fetch, absent)
            || !components::disabled(&fetch, "umls")
        {
            return Err(format!("bad {switch} projection: {projected}"));
        }
    }
    let fetch = Fetch::open(FetchConfig {
        mode: Mode::Offline,
        disabled_sources: [Source::Mychem].into(),
        ..Default::default()
    })
    .map_err(|e| e.to_string())?;
    if components::COMPONENTS.iter().any(|c| !c.disabled(&fetch)) {
        return Err("mychem did not disable all fields".into());
    }
    component_labels()
}
fn component_labels() -> Check {
    use biomcp_adapters::drug_sources::components;
    for (name, usage, url) in [
        (
            "drugbank",
            "open",
            "https://go.drugbank.com/releases/latest#open-data",
        ),
        (
            "chembl",
            "attribution",
            "https://chembl.github.io/chembl-licensing/",
        ),
        (
            "drugcentral",
            "attribution",
            "http://drugcentral.org/privacy",
        ),
        (
            "gtopdb",
            "attribution",
            "https://www.guidetopharmacology.org/download.jsp",
        ),
        ("unii", "open", "https://www.nlm.nih.gov/web_policies.html"),
        (
            "ndc",
            "open",
            "https://www.fda.gov/about-fda/about-website/website-policies",
        ),
        ("openfda", "open", "https://open.fda.gov/terms/"),
    ] {
        let terms = components::terms(name).ok_or_else(|| format!("missing {name}"))?;
        if terms.usage.as_str() != usage || terms.url != url || terms.name.is_empty() {
            return Err(format!("wrong terms for {name}"));
        }
    }
    if components::terms("umls").is_some() {
        return Err("unknown component gained terms".into());
    }
    Ok(())
}
#[derive(serde::Deserialize)]
struct Synthetic {
    present: bool,
}
impl biomcp_fetch::Reply for Synthetic {
    fn check(&self) -> Result<(), biomcp_fetch::BadReply> {
        if self.present {
            Ok(())
        } else {
            Err(biomcp_fetch::BadReply)
        }
    }
}
fn admission() -> Check {
    use biomcp_adapters::{drug_sources, registry};
    use biomcp_fetch::{Certificates, Fetch, FetchConfig, Freshness, Mode, Network, Secret};
    use biomcp_types::Reason;
    use std::time::Duration;
    let env = Environment::new().map_err(|e| e.to_string())?;
    let stub =
        super::network_stub::Stub::for_source("ddinter.scbdd.com", "application/json", |_| {
            Ok(r#"{"present":true}"#.into())
        })?;
    let record = env.root().join("actual-local-record");
    let cache = env.root().join("replies.sqlite");
    let network = Network::new(
        None,
        Some(Secret::new(format!("http://{}", stub.address))),
        "",
        Certificates::from_pem(stub.certificate.as_bytes()).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    let fetch = Fetch::open(FetchConfig {
        mode: Mode::Record {
            dir: record.clone(),
        },
        cache: Some(cache.clone()),
        network: network.clone(),
        ..Default::default()
    })
    .map_err(|e| e.to_string())?;
    let endpoint = drug_sources::DDINTER
        .get("/terms/")
        .fresh(Freshness::Days(1));
    let get = |fetch: &Fetch| {
        fetch
            .leg(
                &drug_sources::DDINTER,
                fetch.deadline(Duration::from_secs(3)),
            )
            .json_checked_cache::<Synthetic>(&endpoint, &[], |_| true)
    };
    get(&fetch).map_err(|e| e.to_string())?;
    drop(fetch);
    let fetch = Fetch::open(FetchConfig {
        mode: Mode::Live { fresh: false },
        cache: Some(cache),
        network,
        ..Default::default()
    })
    .map_err(|e| e.to_string())?;
    get(&fetch).map_err(|e| e.to_string())?;
    for disabled in ["ddinter", "noncommercial"] {
        let restricted = fetch
            .restrict_sources(registry::disabled(&[disabled.into()]).map_err(|e| e.to_string())?);
        if !matches!(get(&restricted), Err(biomcp_fetch::RequestError::Source(e)) if e.reason == Reason::Disabled)
        {
            return Err("disabled source restored a warm reply".into());
        }
    }
    drop(fetch);
    // Remove recorded source files. A prohibited read must still report disabled,
    // rather than the replay miss caused by touching the absent source recording.
    std::fs::remove_dir_all(&record).map_err(|e| e.to_string())?;
    std::fs::create_dir(&record).map_err(|e| e.to_string())?;
    let replay = Fetch::open(FetchConfig {
        mode: Mode::Replay { dir: record },
        disabled_sources: registry::disabled(&["ddinter".into()]).map_err(|e| e.to_string())?,
        ..Default::default()
    })
    .map_err(|e| e.to_string())?;
    if !matches!(get(&replay), Err(biomcp_fetch::RequestError::Source(e)) if e.reason == Reason::Disabled)
    {
        return Err("disabled source read replay".into());
    }
    if stub.finish()?.len() != 1 {
        return Err("prohibited request reached the local source".into());
    }
    outcomes()
}
fn outcomes() -> Check {
    use biomcp_adapters::registry;
    use biomcp_types::{DrugSource, Reason, SourceState};
    for drug_source in DrugSource::ALL {
        let source = drug_source.source();
        let spec = registry::SOURCES
            .iter()
            .find(|s| s.source == source)
            .ok_or("unregistered drug source")?;
        for (state, reason, count) in [
            (SourceState::Ok, None, Some(1)),
            (SourceState::Ok, None, Some(0)),
            (SourceState::Skipped, Some(Reason::Disabled), None),
            (SourceState::Failed, Some(Reason::Unavailable), None),
        ] {
            let mut outcome = registry::outcome(source, state, reason);
            outcome.count = count;
            if outcome.usage != spec.terms.usage
                || outcome.terms.as_ref() != spec.terms.name
                || outcome.terms_url.as_ref() != spec.terms.url
            {
                return Err("outcome lost source rights".into());
            }
        }
    }
    Ok(())
}

const HEALTH_PROBES: &[(&str, &str, &str)] = &[
    ("mychem", "mychem.info", "GET /v1/metadata HTTP/1.1"),
    (
        "chembl",
        "www.ebi.ac.uk",
        "GET /chembl/api/data/status.json HTTP/1.1",
    ),
    (
        "opentargets",
        "api.platform.opentargets.org",
        "POST /api/v4/graphql HTTP/1.1",
    ),
    (
        "openfda-label",
        "api.fda.gov",
        "GET /drug/label.json?limit=1 HTTP/1.1",
    ),
    (
        "drugsfda",
        "api.fda.gov",
        "GET /drug/drugsfda.json?limit=1 HTTP/1.1",
    ),
    (
        "ema",
        "www.ema.europa.eu",
        "GET /en/about-us/about-website/legal-notice HTTP/1.1",
    ),
    ("ddinter", "ddinter.scbdd.com", "GET /terms/ HTTP/1.1"),
];

fn enabled_health() -> Check {
    use biomcp_adapters::registry;
    let env = Environment::new().map_err(|e| e.to_string())?;
    let probes = HEALTH_PROBES;
    let hosts = probes.iter().map(|(_, host, _)| *host).collect::<Vec<_>>();
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
            !probes
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
    if requests.len() != probes.len() {
        return Err(format!(
            "expected one probe per source, got {}",
            requests.len()
        ));
    }
    for &(name, host, first_line) in probes {
        let row = rows
            .iter()
            .find(|row| row["source"] == name)
            .ok_or("missing source")?;
        if row["state"] != "ok" || row["disabled"] != false {
            return Err(format!("enabled probe failed: {row}"));
        }
        let matched = requests
            .iter()
            .filter(|(connect, request)| {
                connect.starts_with(&format!("CONNECT {host}:443 "))
                    && request.lines().next() == Some(first_line)
            })
            .collect::<Vec<_>>();
        if matched.len() != 1 {
            return Err(format!("wrong native probe for {name}: {requests:?}"));
        }
        if name == "opentargets"
            && matched
                .first()
                .is_none_or(|(_, request)| !request.contains("{ __typename }"))
        {
            return Err("Open Targets probe lost its bounded GraphQL query".into());
        }
    }
    Ok(())
}
