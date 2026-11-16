//! Saved work is stored and read back by any 1.x build, so its schemas only add optional fields.
use super::{
    compatibility::check_reviewed,
    policy::{ContractKind, Reviews},
};
use serde_json::{Value, json};

#[test]
fn saved_schemas_accept_only_added_optional_fields() {
    let saved = |id: &str, extra: Value, required: Value, kinds: Value| {
        json!({"$id":id, "type":"object", "required":required, "properties":{
            "name":{"type":"string"}, "kind":{"type":"string", "enum":kinds}, "extra":extra
        }})
    };
    let old = json!({"$id":"biomcp.saved-example/1", "type":"object", "required":["name"],
        "properties":{"name":{"type":"string"}, "kind":{"type":"string", "enum":["note"]}}});
    let added = saved(
        "biomcp.saved-example/1",
        json!({"type":"string", "description":"Added later."}),
        json!(["name"]),
        json!(["note"]),
    );
    let reply = |schema: &Value, other: &Value| {
        check_reviewed(schema, other, ContractKind::Reply, &Reviews::default())
    };
    assert!(
        reply(&old, &added).is_ok(),
        "an added optional field passes"
    );
    for (name, new) in [
        (
            "required addition",
            saved(
                "biomcp.saved-example/1",
                json!({"type":"string"}),
                json!(["name", "extra"]),
                json!(["note"]),
            ),
        ),
        (
            "enum widening",
            saved(
                "biomcp.saved-example/1",
                json!({"type":"string"}),
                json!(["name"]),
                json!(["note", "tag"]),
            ),
        ),
        (
            "major bump",
            saved(
                "biomcp.saved-example/2",
                json!({"type":"string"}),
                json!(["name"]),
                json!(["note"]),
            ),
        ),
    ] {
        assert!(reply(&old, &new).is_err(), "{name} is not additive");
        let mut other = old.clone();
        other["$id"] = json!("biomcp.example/1");
        let mut changed = new.clone();
        let major = changed["$id"]
            .as_str()
            .unwrap_or_default()
            .replace("saved-", "");
        changed["$id"] = json!(major);
        assert!(
            reply(&other, &changed).is_ok(),
            "{name} still passes outside saved work"
        );
    }
}

#[test]
fn saved_schemas_take_open_list_growth_and_reviewed_pattern_widenings() {
    let schema = |sources: Value, pattern: &str| {
        json!({"$id":"biomcp.saved-example/1", "type":"object", "properties":{
            "source":{"type":"string", "x-extensible-enum":sources},
            "id":{"type":"string", "pattern":pattern}
        }})
    };
    let old = schema(json!(["civic"]), "^[0-9]{4}$");
    let reply = |new: &Value, reviews: &Reviews| {
        check_reviewed(&old, new, ContractKind::Reply, reviews).is_ok()
    };
    let none = Reviews::default();
    assert!(
        reply(&schema(json!(["civic", "gtr"]), "^[0-9]{4}$"), &none),
        "an open list such as the shared sources may grow"
    );
    assert!(
        !reply(&schema(json!(["gtr"]), "^[0-9]{4}$"), &none),
        "an open list may not lose a value"
    );
    let widened = schema(json!(["civic"]), "^[0-9]{4,5}$");
    let reviews: Reviews = serde_json::from_value(json!({"pattern_widenings":[{
        "schema":"biomcp.saved-example/1", "path":"/id",
        "from":"^[0-9]{4}$", "to":"^[0-9]{4,5}$", "review":"BioData widened the shared ID."
    }]}))
    .unwrap_or_default();
    assert!(
        !reply(&widened, &none),
        "an unreviewed pattern change fails"
    );
    assert!(
        reply(&widened, &reviews),
        "a reviewed pattern widening passes"
    );
    let narrowings: Reviews = serde_json::from_value(json!({"pattern_narrowings":[{
        "from":"^[0-9]{4}$", "to":"^[0-9]{4,5}$", "review":"x",
        "sites":[{"schema":"biomcp.saved-example/1", "paths":["/id"]}]
    }]}))
    .unwrap_or_default();
    assert!(
        !reply(&widened, &narrowings),
        "saved schemas take no other reviewed exception"
    );
}
