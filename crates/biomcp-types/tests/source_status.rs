//! Public source reports reject contradictory wire states and retain valid statuses.
use biomcp_types::{Reason, Source, SourceOutcome, SourceStatus, VariantHotspots};
use serde_json::json;

#[test]
fn source_status_is_atomic_across_updates_clones_and_wire_input() {
    let mut source = SourceOutcome::new(Source::FdaCdx, biomcp_types::Use::Open, "terms".into(), "https://example.org".into(), SourceStatus::Ok);
    for status in [SourceStatus::Ok, SourceStatus::Stale(Some(Reason::Unavailable)), SourceStatus::Failed(Some(Reason::NoJoinId)), SourceStatus::Skipped(Some(Reason::Unsupported)), SourceStatus::Syncing] {
        source.set_status(status);
        let wire = serde_json::to_value(source.clone()).unwrap();
        let restored: SourceOutcome = serde_json::from_value(wire).unwrap();
        assert_eq!(restored.status(), status);
    }
    source.set_status(SourceStatus::Ok);
    for reason in ["unsupported", "no-join-id"] {
        let mut wire = serde_json::to_value(&source).unwrap();
        wire["reason"] = json!(reason);
        assert!(serde_json::from_value::<SourceOutcome>(wire).is_err());
    }
}

#[test]
fn hotspot_data_cannot_import_a_whole_section_failure_reason() {
    let wire = json!({"records":[],"page":{"offset":0,"returned":0,"total":0}});
    let data: VariantHotspots = serde_json::from_value(wire.clone()).unwrap();
    assert!(serde_json::to_value(data).unwrap().get("reason").is_none());
    let mut contradiction = wire;
    contradiction["reason"] = json!("no-join-id");
    assert!(serde_json::from_value::<VariantHotspots>(contradiction).is_err());
}
