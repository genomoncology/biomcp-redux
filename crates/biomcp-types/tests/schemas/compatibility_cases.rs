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
    let new = json!({"$id":"biodata.example/1", "anyOf":[{"type":"string"},{"type":"null"}], "type":"string"});
    assert!(check(&old, &new).is_err(), "a sibling can exclude null");
}

#[test]
fn compatibility_preserves_patterns_and_rejects_tighter_integer_bounds() {
    let old = json!({"$id":"biodata.example/1", "type":"string", "pattern":"^a$"});
    for pattern in [json!("^b$"), json!(null)] {
        let mut new = old.clone();
        if pattern.is_null() {
            new.as_object_mut().unwrap().remove("pattern");
        } else {
            new["pattern"] = pattern;
        }
        assert!(check(&old, &new).is_err());
    }
    let old = json!({"$id":"biodata.example/1", "type":"integer", "minimum":1,"maximum":100});
    for (minimum, maximum, expected) in [
        (0, 101, true),
        (1, 100, true),
        (2, 100, false),
        (1, 99, false),
    ] {
        let mut new = old.clone();
        new["minimum"] = json!(minimum);
        new["maximum"] = json!(maximum);
        assert_eq!(check(&old, &new).is_ok(), expected);
    }
    for unsupported in [
        json!({"maximum":1.5}),
        json!({"format":"int32"}),
        json!({"exclusiveMinimum":0}),
    ] {
        let mut new = old.clone();
        new.as_object_mut()
            .unwrap()
            .extend(unsupported.as_object().unwrap().clone());
        assert!(check(&old, &new).is_err());
    }
}

#[test]
fn compatibility_tracks_array_items_and_each_tagged_variant() {
    let old = json!({"$id":"biodata.example/1","type":"array","items":{"type":"string"}});
    let mut new = old.clone();
    new["items"]["type"] = json!("integer");
    assert!(check(&old, &new).is_err());
    fn branch(tag: &str, fields: Value) -> Value {
        let mut properties = fields.as_object().unwrap().clone();
        properties.insert("resolution".into(), json!({"type":"string","const":tag}));
        json!({"type":"object","properties":properties,"required":["resolution"],"additionalProperties":false})
    }
    let old = json!({"$id":"biodata.example/1","oneOf":[branch("a", json!({"name":{"type":"string"}})), branch("b",json!({"name":{"type":"string"}}))]});
    let reordered = json!({"$id":"biodata.example/1","oneOf":[branch("b",json!({"name":{"type":"string"}})),branch("a",json!({"name":{"type":"string"}}))]});
    assert!(check(&old, &reordered).is_ok());
    let removed = json!({"$id":"biodata.example/1","oneOf":[branch("a",json!({})),branch("b",json!({"name":{"type":"string"}}))]});
    assert!(check(&old, &removed).is_err());
    let retyped = json!({"$id":"biodata.example/1","oneOf":[branch("a",json!({"name":{"type":"integer"}})),branch("b",json!({"name":{"type":"string"}}))]});
    assert!(check(&old, &retyped).is_err());
}

#[test]
fn compatibility_preserves_consumed_boolean_and_array_bounds() {
    let old = json!({"$id":"biodata.example/1","type":"array","minItems":1,"maxItems":20,"items":{"type":"boolean"}});
    assert!(check(&old, &old).is_ok());
    for (key, value) in [("minItems", json!(2)), ("maxItems", json!(19))] {
        let mut new = old.clone();
        new[key] = value;
        assert!(check(&old, &new).is_err());
    }
    let mut relaxed = old.clone();
    relaxed["minItems"] = json!(0);
    relaxed["maxItems"] = json!(21);
    assert!(check(&old, &relaxed).is_ok());
}

#[test]
fn selector_unions_accept_only_required_lists_and_detect_narrowing() {
    let base = json!(
        {"$id":"biodata.example/1","type":"object","properties":{"query":{"type":"string"},"band":{"type":"string"}},
        "anyOf":[{"required":["query"]},{"required":["band"]}]}
    );
    assert!(check(&base, &base).is_ok());
    let mut narrow = base.clone();
    narrow["anyOf"] = json!([{"required":["query"]}]);
    assert!(check(&base, &narrow).is_err());
    assert!(check(&narrow, &base).is_ok());
    let referenced = |contract: &Value| json!({"$id":"biodata.example/1", "$defs":{"Search":contract}, "$ref":"#/$defs/Search"});
    assert!(check(&referenced(&base), &referenced(&narrow)).is_err());
    for branches in [
        json!([]),
        json!([{"required":[]}]),
        json!([{"required":["missing"]}]),
        json!([{"required":["query"], "type":"object"}]),
        json!([{"properties":{}}]),
    ] {
        let mut invalid = base.clone();
        invalid["anyOf"] = branches;
        assert!(check(&invalid, &invalid).is_err());
    }
}

#[test]
fn compatibility_checks_consumed_decimal_bounds_and_section_annotation() {
    let old = json!({"$id":"biodata.example/1","type":"number","format":"double","minimum":-2.5,"maximum":3.25});
    assert!(check(&old, &old).is_ok());
    for (minimum, maximum, expected) in [(-3.0, 4.0, true), (-2.0, 3.25, false), (-2.5, 3.0, false)]
    {
        let mut new = old.clone();
        new["minimum"] = json!(minimum);
        new["maximum"] = json!(maximum);
        assert_eq!(check(&old, &new).is_ok(), expected);
    }
    for name in ["x-trailing-section", "x-comma-separated"] {
        let mut annotation =
            json!({"$id":"biodata.example/1","type":"array","items":{"type":"string"}});
        annotation[name] = json!(true);
        assert!(check(&annotation, &annotation).is_ok());
        annotation[name] = json!("all");
        assert!(check(&annotation, &annotation).is_err());
    }
}

#[test]
fn compatibility_preserves_markdown_rendering_contract() {
    let plain = json!({"$id":"biodata.example/1", "type":"string"});
    let mut markdown = plain.clone();
    markdown["format"] = json!("markdown");
    assert!(check(&plain, &markdown).is_ok());
    assert!(check(&markdown, &plain).is_err());
    assert!(check(&markdown, &markdown).is_ok());
}

#[test]
fn cli_flag_annotations_are_metadata_with_checked_shape() {
    let old = contract(json!({"name":{"type":"string"}}));
    let new = contract(
        json!({"name":{"type":"string", "x-cli-flags":{"include-retracted":"include", "exclude-retracted":"exclude"}}}),
    );
    assert!(check(&old, &new).is_ok());
    for flags in [
        json!(true),
        json!({}),
        json!({"--bad":"include"}),
        json!({"include":false}),
        json!({"include":""}),
    ] {
        let invalid = contract(json!({"name":{"type":"string", "x-cli-flags":flags}}));
        assert!(check(&old, &invalid).is_err());
    }
}

#[test]
fn compatibility_checks_unique_array_items_without_waiving_versions() {
    let old = contract(json!({"rows":{"type":"array","items":{"type":"string"}}}));
    let mut unique = old.clone();
    unique["properties"]["rows"]["uniqueItems"] = json!(true);
    assert!(check(&unique, &unique).is_ok());
    assert!(check(&old, &unique).is_err());
    assert!(check(&unique, &old).is_ok());
    unique["$id"] = json!("biodata.example/2");
    assert!(check(&old, &unique).is_ok());
    unique["properties"]["rows"]["uniqueItems"] = json!("true");
    assert!(check(&unique, &unique).is_err());
}

#[test]
fn compatibility_checks_native_gene_maps_signed_positions_and_impossible_elements() {
    let old = json!(
        {"$id":"biodata.example/1","type":"object","properties":{"genes":{"type":"object","propertyNames":{"type":"string",
        "pattern":"^[A-Z]+$"},"additionalProperties":{"type":"string"}},"position":{"type":"integer","format":"int64","minimum":-1},
        "empty":{"type":"array","items":false}}}
    );
    assert!(super::compatibility::check(&old, &old).is_ok());
    for changed in [
        json!({"type":"integer"}),
        json!({"type":"string","minLength":2}),
    ] {
        let mut new = old.clone();
        new["properties"]["genes"]["additionalProperties"] = changed;
        assert!(super::compatibility::check(&old, &new).is_err());
    }
    let mut changed = old.clone();
    changed["properties"]["position"]["minimum"] = json!(0);
    assert!(super::compatibility::check(&old, &changed).is_err());
    let mut changed = old.clone();
    changed["properties"]["genes"]["propertyNames"]["pattern"] = json!("^[A-Z]{2}$");
    assert!(super::compatibility::check(&old, &changed).is_err());
    let pattern = json!({"$id":"biodata.example/1","type":"object","patternProperties":{"^[A-Z]+$":{"type":"string"}},"additionalProperties":false});
    assert!(super::compatibility::check(&pattern, &pattern).is_ok());
    let mut changed = pattern.clone();
    changed["patternProperties"]["^[A-Z]+$"] = json!({"type":"integer"});
    assert!(super::compatibility::check(&pattern, &changed).is_err());
    let mut changed = pattern.clone();
    changed["patternProperties"]["X"] = json!({"type":"string"});
    assert!(super::compatibility::check(&pattern, &changed).is_err());
    let impossible = json!({"$id":"biodata.example/1","type":"array","items":{"not":{}}});
    assert!(super::compatibility::check(&impossible, &impossible).is_ok());
    for negation in [
        json!({"not":{"type":"string"}}),
        json!({"not":{},"type":"string"}),
    ] {
        let mut changed = impossible.clone();
        changed["items"] = negation;
        assert!(super::compatibility::check(&impossible, &changed).is_err());
    }
    let unrestricted_keys = json!({"$id":"biodata.example/1","type":"object","propertyNames":true});
    let mut no_keys = unrestricted_keys.clone();
    no_keys["propertyNames"] = json!(false);
    assert!(
        check(&unrestricted_keys, &no_keys).is_err(),
        "false propertyNames forbids every nonempty object"
    );
    let loose = json!({"$id":"biodata.example/1","type":"object","additionalProperties":true});
    for changed in [
        json!({"$id":"biodata.example/1","type":"object","additionalProperties":{"type":"string"}}),
        json!({"$id":"biodata.example/1","type":"object","propertyNames":{"type":"string","pattern":"X"}}),
    ] {
        assert!(super::compatibility::check(&loose, &changed).is_err());
    }
}

#[test]
fn compatibility_preserves_standard_string_format_annotations_and_assertions() {
    let plain = json!({"$id":"biodata.example/1", "type":"string", "pattern":"^x"});
    for format in ["date", "date-time", "uri", "uri-reference"] {
        let mut annotated = plain.clone();
        annotated["format"] = json!(format);
        // Draft 2020-12 uses the format annotation vocabulary by default.
        assert!(check(&plain, &annotated).is_ok());
        assert!(check(&annotated, &annotated).is_ok());
        assert!(check(&annotated, &plain).is_err());
        let mut narrowed = annotated.clone();
        narrowed["minLength"] = json!(2);
        assert!(check(&plain, &narrowed).is_err());
        narrowed = annotated.clone();
        narrowed["pattern"] = json!("^xx");
        assert!(check(&plain, &narrowed).is_err());
        narrowed = annotated.clone();
        narrowed["type"] = json!("integer");
        assert!(check(&narrowed, &narrowed).is_err());
        narrowed = annotated.clone();
        narrowed["format"] = json!(if format == "uri" {
            "uri-reference"
        } else {
            "uri"
        });
        assert!(check(&annotated, &narrowed).is_err());
    }
}
