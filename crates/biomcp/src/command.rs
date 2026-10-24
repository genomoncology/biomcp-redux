//! Shared command interpretation and typed outcomes for CLI and MCP.
use std::{ffi::OsString, io};
use serde_json::Value;

pub(crate) struct Outcome {
    pub(crate) value: Value,
    pub(crate) status: crate::exit::Status,
    pub(crate) cli_bytes: Vec<u8>,
}

pub(crate) fn dispatch(args: &[OsString]) -> io::Result<Outcome> {
    let text = match args {
        [command, name] if command == "schema" => name.to_str().and_then(crate::schema::find),
        _ => None,
    };
    match text {
        Some(text) => Ok(Outcome { value: serde_json::from_str(text)?,
            status: crate::exit::Status::Success, cli_bytes: text.as_bytes().to_vec() }),
        None => {
            let mut bytes = serde_json::to_vec(&biodata::ErrorEnvelope::schema_usage())?;
            bytes.push(b'\n');
            Ok(Outcome { value: serde_json::from_slice(&bytes)?,
                status: crate::exit::Status::Usage, cli_bytes: bytes })
        }
    }
}
