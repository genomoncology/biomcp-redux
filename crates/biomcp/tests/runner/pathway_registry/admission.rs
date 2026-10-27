//! Feature source-controls: disabled pathways precede cache, bundle, replay and HTTP.
//! Synthetic endpoint reads exercise the public boundary; pathway dispatch remains deferred.
use super::{Check, Environment};
use biomcp_adapters::registry;
use biomcp_fetch::{
    Certificates, Fetch, FetchConfig, Freshness, Mode, Network, RequestError, Secret, SourceSpec,
};
use biomcp_types::Reason;
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
fn specs() -> Result<Vec<&'static SourceSpec>, String> {
    super::SOURCES
        .iter()
        .map(|&(name, _, _)| {
            registry::SOURCES
                .iter()
                .copied()
                .find(|s| s.source.to_string() == name)
                .ok_or_else(|| format!("missing {name}"))
        })
        .collect()
}
fn get(fetch: &Fetch, spec: &'static SourceSpec) -> Result<Synthetic, RequestError> {
    fetch
        .leg(spec, fetch.deadline(spec.deadline))
        .json_checked_cache(
            &spec.get("/registry-stub").fresh(Freshness::Days(1)),
            &[],
            |_| true,
        )
}
fn policy(
    mode: Mode,
    cache: Option<std::path::PathBuf>,
    network: Network,
    switch: &str,
) -> Result<Fetch, String> {
    let names = if switch.is_empty() {
        vec![]
    } else {
        vec![switch.into()]
    };
    Fetch::open(FetchConfig {
        mode,
        cache,
        network,
        disabled_sources: registry::disabled(&names).map_err(|e| e.to_string())?,
        disabled_uses: registry::disabled_uses(&names),
        ..Default::default()
    })
    .map_err(|e| e.to_string())
}
fn assert_routes(fetch: &Fetch, switch: &str, absent: bool) -> Check {
    for spec in specs()? {
        let blocked = spec.source.to_string() == switch || switch == "open";
        let result = get(fetch, spec);
        if blocked {
            if !matches!(result, Err(RequestError::Source(e)) if e.reason == Reason::Disabled && e.source == spec.source)
            {
                return Err(format!(
                    "{switch} admitted {}: {:?}",
                    spec.source,
                    result.err()
                ));
            }
            bundle_blocked(fetch, spec)?;
        } else if !absent {
            result.map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}
impl biomcp_fetch::Row for Synthetic {
    fn keys(&self) -> biomcp_fetch::Keys {
        biomcp_fetch::Keys {
            exact: vec![],
            words: vec![],
        }
    }
}
fn bundle_blocked(fetch: &Fetch, spec: &'static SourceSpec) -> Check {
    let bundle = biomcp_fetch::Bundle::<Synthetic> {
        spec,
        name: "pathway-registry-stub",
        path: "/absent-bundle",
        format: biomcp_fetch::RowFormat::Tsv,
        fresh: Freshness::Days(1),
        row: std::marker::PhantomData,
    };
    let leg = fetch.leg(spec, fetch.deadline(spec.deadline));
    let read = leg.stored_rows(&bundle, biomcp_fetch::Find::Words("stub"));
    if !matches!(read, Err(RequestError::Source(e)) if e.reason == Reason::Disabled) {
        return Err("disabled source read a stored bundle".into());
    }
    if !matches!(fetch.sync(&bundle), Err(RequestError::Source(e)) if e.reason == Reason::Disabled)
    {
        return Err("disabled source synchronized a bundle".into());
    }
    Ok(())
}
pub(super) fn run() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let stub = crate::runner::network_stub::Stub::for_sources(
        &["reactome.org", "www.wikipathways.org"],
        |_| "application/json",
        |_| Ok(r#"{"present":true}"#.into()),
    )?;
    stub.retain(&env);
    let network = Network::new(
        None,
        Some(Secret::new(format!("http://{}", stub.address))),
        "",
        Certificates::from_pem(stub.certificate.as_bytes()).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    let record = env.root().join("actual-local-record");
    let cache = env.root().join("replies.sqlite");
    let original = policy(
        Mode::Record {
            dir: record.clone(),
        },
        Some(cache.clone()),
        network.clone(),
        "",
    )?;
    assert_routes(&original, "", false)?;
    drop(original);
    for switch in ["reactome", "wikipathways", "open"] {
        for mode in [
            Mode::Live { fresh: false },
            Mode::Replay {
                dir: record.clone(),
            },
        ] {
            let fetch = policy(mode, Some(cache.clone()), network.clone(), switch)?;
            assert_routes(&fetch, switch, false)?;
        }
    }
    std::fs::remove_dir_all(&record).map_err(|e| e.to_string())?;
    std::fs::create_dir(&record).map_err(|e| e.to_string())?;
    for switch in ["reactome", "wikipathways", "open"] {
        let fetch = policy(
            Mode::Replay {
                dir: record.clone(),
            },
            None,
            network.clone(),
            switch,
        )?;
        assert_routes(&fetch, switch, true)?;
    }
    if stub.finish()?.len() != super::SOURCES.len() {
        return Err("disabled or warm read reached HTTP".into());
    }
    Ok(())
}
