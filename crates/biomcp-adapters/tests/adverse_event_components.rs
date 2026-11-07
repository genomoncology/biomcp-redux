//! Feature adverse-event-terminologies: admission and labels follow the declared terminology.
use biomcp_adapters::{adverse_event_sources::components, maude};
use biomcp_fetch::{Fetch, FetchConfig, Mode, RequestError};
use biomcp_types::{Reason, Source};
use std::time::Duration;

type Check = Result<(), Box<dyn std::error::Error>>;

fn offline() -> Result<Fetch, Box<dyn std::error::Error>> {
    Ok(Fetch::open(FetchConfig {
        mode: Mode::Offline,
        ..Default::default()
    })?)
}

#[test]
fn maude_search_is_refused_when_gmdn_is_turned_off() -> Check {
    let fetch = offline()?;
    let leg = fetch.leg(&maude::SPEC, fetch.deadline(Duration::from_secs(1)));
    components::gmdn(&leg)?;
    let off = fetch.restrict_sources([Source::Gmdn].into());
    let leg = off.leg(&maude::SPEC, off.deadline(Duration::from_secs(1)));
    let Err(RequestError::Source(error)) = components::gmdn(&leg) else {
        return Err("MAUDE search was admitted with GMDN turned off".into());
    };
    assert_eq!(
        (error.source, error.reason),
        (Source::Maude, Reason::Disabled)
    );
    Ok(())
}

#[test]
fn each_source_labels_only_its_own_terminology() -> Check {
    for (owner, component, name) in [
        (Source::Faers, Source::Meddra, "meddra"),
        (Source::Maude, Source::Gmdn, "gmdn"),
    ] {
        let labels = components::labels(owner, component)?;
        assert_eq!(labels.len(), 1);
        assert!(
            labels[0].component.as_str().eq_ignore_ascii_case(name),
            "{owner:?}"
        );
    }
    for (owner, component) in [
        (Source::Faers, Source::Gmdn),
        (Source::Maude, Source::Meddra),
    ] {
        assert!(
            components::labels(owner, component).is_err(),
            "{owner:?} labelled {component:?}, which it does not carry"
        );
    }
    Ok(())
}
