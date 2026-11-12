//! The reply's coordinate note leads human text as plain prose; numeric spans stay typed JSON.
use serde_json::Value;

pub(crate) fn note(value: &Value) -> Option<&str> {
    value.pointer("/result/coordinate_note")?.as_str()
}
