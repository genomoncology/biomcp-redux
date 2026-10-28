//! Feature source-controls: component admission precedes cache, replay and HTTP.
//! Synthetic endpoint reads exercise the registry boundary before report dispatch exists.
use super::{Check, Environment};
use biomcp_adapters::{
    adverse_event_sources::{self, components},
    registry,
};
use biomcp_fetch::{
    Certificates, Fetch, FetchConfig, Freshness, Mode, Network, RequestError, Secret, SourceSpec,
};
use biomcp_types::{Reason, Source};
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
// Each owning request uses the corresponding admission rule; ordinary routes remain independent.
const ROUTES: &[(&SourceSpec, &str, bool)] = &[
    (&adverse_event_sources::FAERS, "search", false),
    (&adverse_event_sources::FAERS, "reaction-filter", true),
    (&adverse_event_sources::FAERS, "card", false),
    (&adverse_event_sources::FAERS, "drugs", false),
    (&adverse_event_sources::FAERS, "reactions", true),
    (&adverse_event_sources::MAUDE, "positional-search", true),
    (&adverse_event_sources::MAUDE, "card", false),
    (&adverse_event_sources::MAUDE, "devices", true),
    (&adverse_event_sources::MAUDE, "narrative", false),
];
fn get(
    fetch: &Fetch,
    spec: &'static SourceSpec,
    path: &'static str,
    component: bool,
) -> Result<Synthetic, RequestError> {
    let leg = fetch.leg(spec, fetch.deadline(spec.deadline));
    if component {
        if spec.source == Source::Faers {
            components::meddra(&leg)?;
        } else {
            components::gmdn(&leg)?;
        }
    }
    leg.json_checked_cache(&spec.get(path).fresh(Freshness::Days(1)), &[], |_| true)
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
fn blocked(spec: &SourceSpec, component: bool, switch: &str) -> bool {
    spec.source.to_string() == switch
        || component
            && (switch == "restricted"
                || spec.source == Source::Faers && switch == "meddra"
                || spec.source == Source::Maude && switch == "gmdn")
}
fn assert_routes(fetch: &Fetch, switch: &str, absent: bool) -> Check {
    for &(spec, path, component) in ROUTES {
        let result = get(fetch, spec, path, component);
        if blocked(spec, component, switch) {
            if !matches!(result, Err(RequestError::Source(e)) if e.reason == Reason::Disabled && e.source == spec.source)
            {
                return Err(format!(
                    "{switch} admitted {} {path}: {:?}",
                    spec.source,
                    result.err()
                ));
            }
        } else if !absent {
            result.map_err(|e| format!("{switch} blocked allowed {} {path}: {e}", spec.source))?;
        }
    }
    Ok(())
}
pub(super) fn run() -> Check {
    if components::GMDN_FIELDS != ["device.generic_name"]
        || components::MEDDRA_FIELDS
            != [
                "patient.reaction.reactionmeddrapt",
                "patient.reaction.reactionmeddraversionpt",
            ]
    {
        return Err("field ownership changed without policy review".into());
    }
    let env = Environment::new().map_err(|e| e.to_string())?;
    let stub = crate::runner::network_stub::Stub::for_sources(
        &["api.fda.gov"],
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
    for switch in ["faers", "maude", "meddra", "gmdn", "restricted"] {
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
    for switch in ["faers", "maude", "meddra", "gmdn", "restricted"] {
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
    if stub.finish()?.len() != ROUTES.len() {
        return Err("disabled or warm read reached HTTP".into());
    }
    Ok(())
}
