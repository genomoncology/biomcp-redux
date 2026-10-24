use super::compatibility::check;
use serde_json::{Value, json};

fn contract(properties: Value) -> Value {
    json!({"$id":"biodata.example/1", "type":"object", "properties":properties,
        "required":["name"], "additionalProperties":false})
}

#[test]
fn compatibility_preserves_paths_types_and_required_fields() {
    let old = contract(json!({"name":{"$ref":"#/$defs/Text"}}));
    let mut old = old;
    old["$defs"] = json!({"Text":{"type":"string"}});
    let cases = [
        ("optional addition", contract(json!({"name":{"type":"string"},"extra":{"type":"string"}})), true),
        ("removal", contract(json!({})), false),
        ("rename", contract(json!({"label":{"type":"string"}})), false),
        ("retyping", contract(json!({"name":{"type":"object"}})), false),
        ("unresolved reference", contract(json!({"name":{"$ref":"#/$defs/Missing"}})), false),
        ("unsupported form", contract(json!({"name":{"oneOf":[{"type":"string"}]}})), false),
        ("external reference", contract(json!({"name":{"$ref":"https://example.org/text"}})), false),
    ];
    for (name, new, expected) in cases {
        assert_eq!(check(&old, &new).is_ok(), expected, "{name}");
    }
    let mut new = contract(json!({"name":{"type":"string"},"extra":{"type":"string"}}));
    new["required"] = json!(["name", "extra"]);
    assert!(check(&old, &new).is_err(), "required addition breaks old inputs");
    // This fixture deliberately documents a breaking rename under a new major.
    let mut new = contract(json!({"label":{"type":"object"}}));
    new["$id"] = json!("biodata.example/2");
    assert!(check(&old, &new).is_ok());
    assert!(check(&new, &old).is_err(), "major cannot decrease");
}

#[test]
fn compatibility_checks_enums_and_string_constraints() {
    let old = json!({"$id":"biodata.example/1", "type":"string", "enum":["first","second"]});
    for (values, compatible) in [(json!(["first","second","third"]), true), (json!(["first"]), false)] {
        let mut new = old.clone();
        new["enum"] = values;
        assert_eq!(check(&old, &new).is_ok(), compatible);
    }
    let old = json!({"$id":"biodata.example/1", "type":"string", "minLength":1});
    let mut new = old.clone();
    new["minLength"] = json!(2);
    assert!(check(&old, &new).is_err());
}
