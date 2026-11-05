//! Public source reports reject contradictory wire states and retain valid statuses.
use biomcp_types::{Reason, Source, SourceHealth, SourceOutcome, SourceStatus, VariantHotspots};
use serde_json::json;

#[test]
fn source_status_is_atomic_across_updates_clones_and_wire_input() -> Result<(), serde_json::Error> {
    let mut source = SourceOutcome::new(
        Source::FdaCdx,
        biomcp_types::Use::Open,
        "terms".into(),
        "https://example.org".into(),
        SourceStatus::Ok,
    );
    for status in [
        SourceStatus::Ok,
        SourceStatus::Stale(Some(Reason::Unavailable)),
        SourceStatus::Failed(Some(Reason::NoJoinId)),
        SourceStatus::Skipped(Some(Reason::Unsupported)),
        SourceStatus::Syncing,
    ] {
        source.set_status(status);
        let health = SourceHealth::new(
            Source::FdaCdx,
            "terms".parse().unwrap(),
            "https://example.org".parse().unwrap(),
            biomcp_types::Use::Open,
            false,
            status,
        );
        let health: SourceHealth = serde_json::from_value(serde_json::to_value(health)?)?;
        assert_eq!(
            (health.state, health.reason),
            (source.state, source.reason),
            "health probes retain the same canonical status"
        );
        let wire = serde_json::to_value(source.clone())?;
        let restored: SourceOutcome = serde_json::from_value(wire)?;
        assert_eq!(
            restored.status(),
            status,
            "valid source status survives serialization"
        );
    }
    source.set_status(SourceStatus::Ok);
    for reason in ["unsupported", "no-join-id"] {
        let mut wire = serde_json::to_value(&source)?;
        wire["reason"] = json!(reason);
        let mut health = wire.clone();
        health["disabled"] = json!(false);
        assert!(
            serde_json::from_value::<SourceHealth>(health).is_err(),
            "health answer cannot import reason {reason}"
        );
        assert!(
            serde_json::from_value::<SourceOutcome>(wire).is_err(),
            "current answer cannot import reason {reason}"
        );
    }
    Ok(())
}

#[test]
fn hotspot_data_cannot_import_a_whole_section_failure_reason() -> Result<(), serde_json::Error> {
    let wire = json!({"records":[],"page":{"offset":0,"returned":0,"total":0}});
    let data: VariantHotspots = serde_json::from_value(wire.clone())?;
    assert!(
        serde_json::to_value(data)?.get("reason").is_none(),
        "data has no failure reason"
    );
    let mut contradiction = wire;
    contradiction["reason"] = json!("no-join-id");
    assert!(
        serde_json::from_value::<VariantHotspots>(contradiction).is_err(),
        "hotspot data rejects a failure reason"
    );
    Ok(())
}
