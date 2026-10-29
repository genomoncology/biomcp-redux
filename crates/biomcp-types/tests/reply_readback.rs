//! Contract checks for concrete reply readers.
use biomcp_types::{Envelope, Help, ResponseBody, SchemaCatalog, Status, reply_registry};
use serde_json::{Value, json};

#[test]
fn envelope_checks_body_identity_and_accepts_added_reply_fields()
-> Result<(), Box<dyn std::error::Error>> {
    let mut wire = serde_json::to_value(Envelope::<Help>::new(
        Status::Complete,
        serde_json::from_value(json!({}))?,
        Some(Help { commands: vec![] }),
    ))?;
    wire["future"] = json!({"anything": true});
    wire["result"]["future"] = json!(17);
    let read: Envelope<Help> = serde_json::from_value(wire.clone())?;
    let rewritten = serde_json::to_value(read)?;
    assert_eq!(rewritten["schema"], Help::SCHEMA);
    assert!(rewritten.get("future").is_none());
    assert!(serde_json::from_value::<Envelope<SchemaCatalog>>(wire.clone()).is_err());
    wire.as_object_mut().ok_or("object")?.remove("schema");
    assert!(serde_json::from_value::<Envelope<Help>>(wire.clone()).is_err());
    wire["schema"] = json!(42);
    assert!(serde_json::from_value::<Envelope<Help>>(wire).is_err());
    Ok(())
}

#[test]
fn registered_reply_ids_are_unique_and_read_their_concrete_envelope()
-> Result<(), Box<dyn std::error::Error>> {
    let registry = reply_registry();
    let mut identities = std::collections::BTreeSet::new();
    for reply in registry {
        assert!(identities.insert(reply.schema), "{}", reply.schema);
    }
    let reply = registry
        .iter()
        .find(|entry| entry.schema == SchemaCatalog::SCHEMA)
        .ok_or("catalog")?;
    let bytes = serde_json::to_vec(&Envelope::<SchemaCatalog>::new(
        Status::Complete,
        (),
        Some(SchemaCatalog { schemas: vec![] }),
    ))?;
    assert_eq!((reply.read_write)(&bytes)?, bytes);
    let mut wire: Value = serde_json::from_slice(&bytes)?;
    wire["schema"] = json!(Help::SCHEMA);
    assert!((reply.read_write)(&serde_json::to_vec(&wire)?).is_err());
    Ok(())
}
