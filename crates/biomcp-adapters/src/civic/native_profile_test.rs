//! Native GraphQL profile fields captured 2026-10-29T15:21:33.888679Z.
//! Response SHA256 0b3ad5ceecc72a9799d7256f20c63cc3f7bae4dbdcd8ee03972c71da771caee9.
//! Query name:"AND", evidenceStatusFilter:ALL, first:20; no clinical claim fields.
use super::{drug_map::context, drug_raw};
use crate::mapped::Problems;
use biomcp_types::Source;
use serde_json::{Value, json};

#[test]
fn native_boolean_profile_keeps_source_ids_and_exclusions() -> Result<(), Box<dyn std::error::Error>>
{
    let row = native()?;
    let mut cx = Problems {
        source: Source::Civic,
        record: None,
        list: Vec::new(),
    };
    let (profile, _) = context(
        Some(crate::raw::Field::Value(serde_json::from_value::<
            drug_raw::Profile,
        >(row)?)),
        &mut cx,
    )
    .ok_or("profile refused")?;
    let value = serde_json::to_value(profile)?;
    assert!(cx.list.is_empty());
    assert_eq!(value.pointer("/record/id"), Some(&json!("6153")));
    assert_eq!(
        value.pointer("/raw_name"),
        Some(&json!("#VID276 AND NOT #VID442 AND NOT #VID499"))
    );
    assert_eq!(
        value.pointer("/subject/expression/tree/operator"),
        Some(&json!("and"))
    );
    assert_eq!(
        value.pointer("/subject/expression/tree/operands/1/operator"),
        Some(&json!("not"))
    );
    assert_eq!(
        value.pointer("/subject/expression/tree/operands/1/operand/record"),
        Some(&json!("civic:442"))
    );
    assert_eq!(
        value.pointer("/subject/members/2/record"),
        Some(&json!("civic:499"))
    );
    assert_eq!(
        value.pointer("/subject/members/2/feature_record"),
        Some(&json!("civic-feature:62024"))
    );
    assert_eq!(
        value.pointer("/subject/members/2/feature"),
        Some(&json!("v::ALK"))
    );
    // This source body proves native identity and Boolean structure only.
    assert!(value.get("statements").is_none());
    Ok(())
}
#[test]
fn malformed_profile_expression_keeps_native_identity() -> Result<(), Box<dyn std::error::Error>> {
    let mut row = native()?;
    row["rawName"] = json!(42);
    let mut cx = Problems {
        source: Source::Civic,
        record: None,
        list: Vec::new(),
    };
    let raw: drug_raw::Profile = serde_json::from_value(row)?;
    let (profile, subject) =
        context(Some(crate::raw::Field::Value(raw)), &mut cx).ok_or("profile refused")?;
    assert_eq!(profile.record.id.as_str(), "6153");
    assert!(subject.is_none());
    assert_eq!(cx.list.len(), 1);
    assert_eq!(
        cx.list.first().map(|problem| problem.field.as_ref()),
        Some("rawName")
    );
    Ok(())
}
fn native() -> Result<Value, Box<dyn std::error::Error>> {
    let body: Value = serde_json::from_str(include_str!(
        "../../tests/fixtures/civic-profile-members.json"
    ))?;
    Ok(body
        .pointer("/data/molecularProfiles/nodes")
        .and_then(Value::as_array)
        .ok_or("native nodes missing")?
        .iter()
        .find(|row| row["id"] == json!(6153))
        .ok_or("native profile missing")?
        .clone())
}
