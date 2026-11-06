//! Opaque source data cannot create protocol control; direct trust-boundary coverage.
use super::steps::{paths, remove};
use serde_json::json;
#[test]
fn opaque_source_json_and_nonproductive_cycles_stay_untouched() {
    let schema = json!({"$defs":{"Reply_Step":{"type":"object"},
        "Cycle":{"$ref":"#/$defs/Cycle"}},"type":"object","properties":{
        "owned":{"type":"object","properties":{"next":{"$ref":"#/$defs/Reply_Step"}}},
        "native":{"type":"object","additionalProperties":true},
        "cycle":{"$ref":"#/$defs/Cycle"}}});
    let native = json!({"next":{"kind":"pivot","args":["KEEP LITERAL"]},
        "alternatives":[{"kind":"pivot","args":["KEEP TOO"]}]});
    let mut value = json!({"owned":{"next":{"kind":"pivot","args":["get","gene","BRAF"]}},
        "native":native,"cycle":{"next":{"args":["KEEP CYCLE"]}}});
    assert_eq!(paths(&schema, &schema, &value), ["/owned/next"]);
    assert_eq!(
        paths(
            &schema,
            &schema["properties"]["owned"]["properties"]["next"],
            &value["owned"]["next"]
        ),
        [""]
    );
    assert!(paths(&schema, &schema["properties"]["native"], &value["native"]).is_empty());
    remove(&mut value, "/owned/next");
    assert_eq!(value["native"], native);
    assert_eq!(value["cycle"]["next"]["args"], json!(["KEEP CYCLE"]));
}
#[test]
fn overlapping_branches_visit_each_array_step_once() {
    let schema = json!({"$defs":{"Reply_Step":{"type":"object"}},"properties":{
        "next":{"items":{"anyOf":[{"$ref":"#/$defs/Reply_Step"},{"$ref":"#/$defs/Reply_Step"}]}}}});
    let value = json!({"next":[{"kind":"pivot","args":["a"]},{"kind":"pivot","args":["b"]}]});
    assert_eq!(paths(&schema, &schema, &value), ["/next/0", "/next/1"]);
}
