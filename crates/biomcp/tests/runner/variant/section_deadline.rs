//! Independent source requests rendezvous before either section reply is released.
use super::super::{launch::Environment, network_stub};
use biomcp_fetch::{Certificates, Fetch, FetchConfig, Network, Secret};
use biomcp_types::{Request, VariantGet, VariantResolver, VariantSection};
use serde_json::Value;
use std::time::Duration;
mod peer;

pub(super) fn check() -> Result<(), Box<dyn std::error::Error>> {
    let env = Environment::new()?;
    let inputs = env.root().join("replay");
    let peer = peer::Peer::start(&inputs)?;
    let network = Network::new(
        None,
        Some(Secret::new(format!("http://{}", peer.address))),
        "",
        Certificates::from_pem(peer.certificate.as_bytes())?,
    )?;
    let fetch = Fetch::open(FetchConfig {
        network,
        ..FetchConfig::default()
    })?;
    let query = VariantGet::new(
        "chr7:g.140753336A>T".parse()?,
        VariantResolver::Car,
        Some(biodata::Build::GRCh38),
    )?
    .with_sections(Box::new([
        VariantSection::Hotspots,
        VariantSection::Clinvar,
    ]));
    // Below both existing source caps: the held baseline exhausts the public request budget.
    let outcome = biomcp_query::run(
        &biomcp_query::Ctx {
            fetch: &fetch,
            budget: Duration::from_secs(5),
            ncbi_email: None,
            fresh: false,
        },
        &Request::GetVariant(query),
    );
    let events = peer.finish();
    network_stub::retain_replay_owner(env.shared_root().as_ref())?;
    let events = events?;
    let biomcp_types::QueryBody::VariantCard(card) = &outcome.body else {
        return Err(format!("variant request returned a different body: {outcome:?}").into());
    };
    let body = serde_json::to_value(card)?;
    for section in ["clinvar", "hotspots"] {
        if body.pointer(&format!("/candidates/0/sections/{section}/state"))
            != Some(&Value::from("data"))
        {
            let detail = format!("{outcome:?}; peer={events:?}");
            return Err(
                format!("{section} did not complete under one request deadline: {detail}").into(),
            );
        }
    }
    if outcome.status() != biomcp_types::Status::Complete
        || outcome.local.is_some()
        || outcome.sources.iter().any(|source| source.reason.is_some())
    {
        return Err(format!("independent sections failed: {outcome:?}; peer={events:?}").into());
    }
    if outcome.sources.iter().map(|s| s.source).collect::<Vec<_>>()
        != [
            biomcp_types::Source::MyVariant,
            biomcp_types::Source::ClinVar,
            biomcp_types::Source::CancerHotspots,
        ]
    {
        return Err(format!("declared section order changed: {outcome:?}").into());
    }
    peer::rendezvous(&events)?;
    Ok(())
}
