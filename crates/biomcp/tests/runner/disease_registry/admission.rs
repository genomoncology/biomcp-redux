//! Feature source-controls: prohibit data admission before HTTP, cache and replay.
use super::{Check, Environment};
use biomcp_adapters::{
    disease_sources::{self, components},
    registry,
};
use biomcp_fetch::{
    Certificates, Fetch, FetchConfig, Freshness, Mode, Network, RequestError, Secret, SourceSpec,
};
use biomcp_types::{DiseaseOntology, Reason, Source};
use std::time::Duration;
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
const SPECS: &[&SourceSpec] = &[
    &disease_sources::OLS,
    &disease_sources::MYDISEASE,
    &disease_sources::MONARCH,
    &disease_sources::SEER,
];
fn get(fetch: &Fetch, spec: &'static SourceSpec) -> Result<Synthetic, RequestError> {
    let leg = fetch.leg(spec, fetch.deadline(Duration::from_secs(3)));
    let (path, params) = match spec.source {
        Source::Ols => {
            let ontology = components::ontology(&leg, DiseaseOntology::default())?;
            (
                "/api/search",
                vec![("ontology", ontology.as_str().to_owned())],
            )
        }
        Source::MyDisease => (
            "/query",
            vec![(
                "fields",
                components::allowed_fields(&leg, "_id,mondo.label")?,
            )],
        ),
        Source::Monarch => {
            components::monarch(&leg)?;
            ("/v3/api/association", vec![])
        }
        _ => ("/get_var_formats.php", vec![]),
    };
    let endpoint = spec.get(path).fresh(Freshness::Days(1));
    let args = params
        .iter()
        .map(|(key, value)| (*key, biomcp_fetch::Arg::Text(value)))
        .collect::<Vec<_>>();
    leg.json_checked_cache(&endpoint, &args, |_| true)
}
fn policy(
    mode: Mode,
    cache: Option<std::path::PathBuf>,
    network: Network,
    switch: &str,
) -> Result<Fetch, String> {
    Fetch::open(FetchConfig {
        mode,
        cache,
        network,
        disabled_sources: registry::disabled(&[switch.into()]).map_err(|e| e.to_string())?,
        disabled_uses: registry::disabled_uses(&[switch.into()]),
        ..Default::default()
    })
    .map_err(|e| e.to_string())
}
fn assert_policy(fetch: &Fetch, switch: &str) -> Check {
    let blocked = match switch {
        "ols" | "mondo" => Source::Ols,
        "mydisease" => Source::MyDisease,
        "hpo" | "orphanet" | "monarch" => Source::Monarch,
        "seer" => Source::Seer,
        _ => return Err("unknown blocking test".into()),
    };
    for &spec in SPECS {
        let result = get(fetch, spec);
        let should_skip =
            spec.source == blocked || (switch == "mondo" && spec.source == Source::MyDisease);
        if should_skip {
            if !matches!(result, Err(RequestError::Source(e)) if e.reason == Reason::Disabled && e.source == spec.source)
            {
                return Err(format!(
                    "{switch} supplied prohibited reply from {}: {:?}",
                    spec.source,
                    result.err()
                ));
            }
        } else {
            result.map_err(|e| format!("{switch} removed independent {}: {e}", spec.source))?;
        }
    }
    Ok(())
}
pub(super) fn run() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let hosts = [
        "www.ebi.ac.uk",
        "mydisease.info",
        "api-v3.monarchinitiative.org",
        "seer.cancer.gov",
    ];
    let stub = crate::runner::network_stub::Stub::for_sources(
        &hosts,
        |_| "application/json",
        |_| Ok(r#"{"present":true}"#.into()),
    )?;
    let network = Network::new(
        None,
        Some(Secret::new(format!("http://{}", stub.address))),
        "",
        Certificates::from_pem(stub.certificate.as_bytes()).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    let record = env.root().join("actual-local-record");
    let cache = env.root().join("replies.sqlite");
    let original = Fetch::open(FetchConfig {
        mode: Mode::Record {
            dir: record.clone(),
        },
        cache: Some(cache.clone()),
        network: network.clone(),
        ..Default::default()
    })
    .map_err(|e| e.to_string())?;
    for &spec in SPECS {
        get(&original, spec).map_err(|e| e.to_string())?;
    }
    drop(original);
    for switch in [
        "ols",
        "mondo",
        "mydisease",
        "monarch",
        "hpo",
        "orphanet",
        "seer",
    ] {
        for mode in [
            Mode::Live { fresh: false },
            Mode::Replay {
                dir: record.clone(),
            },
        ] {
            let fetch = policy(mode, Some(cache.clone()), network.clone(), switch)?;
            assert_policy(&fetch, switch)?;
        }
    }
    // An OMIM payload switch cannot turn cited OMIM/Orphanet identifiers into payloads.
    for switch in ["omim", "restricted", "noncommercial"] {
        let fetch = policy(
            Mode::Replay {
                dir: record.clone(),
            },
            None,
            network.clone(),
            switch,
        )?;
        get(&fetch, &disease_sources::MONARCH)
            .map_err(|e| format!("{switch} removed reviewed HPO annotations: {e}"))?;
    }
    projection(&network, &record)?;
    absent_replay(&network, &record)?;
    if stub.finish()?.len() != 4 {
        return Err("disabled or warm request reached source".into());
    }
    Ok(())
}
fn absent_replay(network: &Network, record: &std::path::Path) -> Check {
    std::fs::remove_dir_all(record).map_err(|e| e.to_string())?;
    std::fs::create_dir(record).map_err(|e| e.to_string())?;
    for switch in [
        "ols",
        "mydisease",
        "monarch",
        "seer",
        "hpo",
        "orphanet",
        "mondo",
    ] {
        let fetch = policy(
            Mode::Replay {
                dir: record.to_owned(),
            },
            None,
            network.clone(),
            switch,
        )?;
        let spec = if switch == "mondo" {
            &disease_sources::OLS
        } else if matches!(switch, "hpo" | "orphanet") {
            &disease_sources::MONARCH
        } else {
            SPECS
                .iter()
                .copied()
                .find(|s| s.source.to_string() == switch)
                .ok_or("missing source")?
        };
        if !matches!(get(&fetch, spec), Err(RequestError::Source(e)) if e.reason == Reason::Disabled)
        {
            return Err(format!("{switch} touched absent replay"));
        }
    }
    Ok(())
}

fn projection(network: &Network, record: &std::path::Path) -> Check {
    let fields = "_id,_score,mondo.label,mondo.xrefs.omim,disease_ontology.name,hpo.phenotype_related_to_disease,orpha\
        net.label,omim.description,disgenet.associations";
    for (switch, absent) in [
        ("mondo", "mondo"),
        ("doid", "disease_ontology"),
        ("hpo", "hpo"),
        ("orphanet", "orphanet"),
        ("omim", "omim"),
        ("restricted", "omim"),
    ] {
        let fetch = policy(
            Mode::Replay {
                dir: record.to_owned(),
            },
            None,
            network.clone(),
            switch,
        )?;
        let leg = fetch.leg(
            &disease_sources::MYDISEASE,
            fetch.deadline(Duration::from_secs(1)),
        );
        let selected = components::allowed_fields(&leg, fields).map_err(|e| e.to_string())?;
        let expected = fields
            .split(',')
            .filter(|f| !f.starts_with(absent) && !f.starts_with("disgenet"))
            .collect::<Vec<_>>()
            .join(",");
        if selected != expected {
            return Err(format!(
                "{switch} failed pre-request projection: {selected}"
            ));
        }
    }
    Ok(())
}
