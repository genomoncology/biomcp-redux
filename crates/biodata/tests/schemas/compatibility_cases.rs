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
        (
            "optional addition",
            contract(json!({"name":{"type":"string"},"extra":{"type":"string"}})),
            true,
        ),
        ("removal", contract(json!({})), false),
        (
            "rename",
            contract(json!({"label":{"type":"string"}})),
            false,
        ),
        (
            "retyping",
            contract(json!({"name":{"type":"object"}})),
            false,
        ),
        (
            "unresolved reference",
            contract(json!({"name":{"$ref":"#/$defs/Missing"}})),
            false,
        ),
        (
            "unsupported form",
            contract(json!({"name":{"oneOf":[{"type":"string"}]}})),
            false,
        ),
        (
            "external reference",
            contract(json!({"name":{"$ref":"https://example.org/text"}})),
            false,
        ),
    ];
    for (name, new, expected) in cases {
        assert_eq!(check(&old, &new).is_ok(), expected, "{name}");
    }
    let mut new = contract(json!({"name":{"type":"string"},"extra":{"type":"string"}}));
    new["required"] = json!(["name", "extra"]);
    assert!(
        check(&old, &new).is_err(),
        "required addition breaks old inputs"
    );
    // This fixture deliberately documents a breaking rename under a new major.
    let mut new = contract(json!({"label":{"type":"object"}}));
    new["$id"] = json!("biodata.example/2");
    assert!(check(&old, &new).is_ok());
    assert!(check(&new, &old).is_err(), "major cannot decrease");
}

#[test]
fn compatibility_checks_enums_and_string_constraints() {
    let old = json!({"$id":"biodata.example/1", "type":"string", "enum":["first","second"]});
    for (values, compatible) in [
        (json!(["first", "second", "third"]), true),
        (json!(["first"]), false),
    ] {
        let mut new = old.clone();
        new["enum"] = values;
        assert_eq!(check(&old, &new).is_ok(), compatible);
    }
    let old = json!({"$id":"biodata.example/1", "type":"string", "minLength":1});
    let mut new = old.clone();
    new["minLength"] = json!(2);
    assert!(check(&old, &new).is_err());
}

#[test]
fn compatibility_refuses_assertion_siblings_on_composed_schemas() {
    let old = json!({"$id":"biodata.example/1", "type":"string", "enum":["first","second"]});
    // These assertions intersect; overwriting the referenced enum would hide
    // the removal of "second" from the accepted values.
    let new = json!({"$id":"biodata.example/1", "$ref":"#/$defs/Value",
        "$defs":{"Value":{"type":"string","enum":["first"]}},
        "enum":["first","second"]});
    assert!(check(&old, &new).is_err());

    let old = json!({"$id":"biodata.example/1", "type":["string","null"]});
    let new = json!({"$id":"biodata.example/1",
        "anyOf":[{"type":"string"},{"type":"null"}], "type":"string"});
    assert!(check(&old, &new).is_err(), "a sibling can exclude null");
}

#[test]
fn compatibility_preserves_patterns_and_rejects_tighter_integer_bounds() {
    let old = json!({"$id":"biodata.example/1", "type":"string", "pattern":"^a$"});
    for pattern in [json!("^b$"), json!(null)] {
        let mut new = old.clone();
        if pattern.is_null() { new.as_object_mut().unwrap().remove("pattern"); }
        else { new["pattern"] = pattern; }
        assert!(check(&old, &new).is_err());
    }
    let old = json!({"$id":"biodata.example/1", "type":"integer", "minimum":1,"maximum":100});
    for (minimum, maximum, expected) in [(0,101,true),(1,100,true),(2,100,false),(1,99,false)] {
        let mut new = old.clone(); new["minimum"]=json!(minimum); new["maximum"]=json!(maximum);
        assert_eq!(check(&old,&new).is_ok(), expected);
    }
    for unsupported in [json!({"minimum":-1}), json!({"maximum":1.5}), json!({"format":"int64"}), json!({"exclusiveMinimum":0})] {
        let mut new=old.clone();
        new.as_object_mut().unwrap().extend(unsupported.as_object().unwrap().clone());
        assert!(check(&old,&new).is_err());
    }
}
