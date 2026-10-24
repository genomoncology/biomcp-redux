//! Shared command interpretation and typed outcomes for CLI and MCP.
use std::{ffi::OsString, io};
use serde_json::Value;

pub(crate) struct Outcome {
    pub(crate) value: Value,
    pub(crate) status: crate::exit::Status,
    pub(crate) schema_text: Option<&'static str>,
}

pub(crate) fn dispatch(args: &[OsString]) -> io::Result<Outcome> {
    let text = match args {
        [command, name] if command == "schema" => name.to_str().and_then(crate::schema::find),
        _ => None,
    };
    match text {
        Some(text) => Ok(Outcome { value: serde_json::from_str(text)?,
            status: crate::exit::Status::Success, schema_text: Some(text) }),
        None => Ok(Outcome { value: serde_json::to_value(biodata::ErrorEnvelope::schema_usage())?,
            status: crate::exit::Status::Usage, schema_text: None }),
    }
}
