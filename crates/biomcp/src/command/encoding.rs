//! Choose one reply representation for its consumer before serialization.
use serde_json::Value;
use std::{ffi::OsString, io};

#[derive(Clone, Copy)]
pub(crate) enum Encoding {
    Json,
    Markdown,
    Mcp,
}

enum Payload {
    Bytes(Vec<u8>),
    Value(Value),
}
pub(crate) struct Outcome {
    payload: Payload,
    pub(crate) status: crate::exit::Status,
}
impl Outcome {
    pub(crate) fn bytes(&self) -> io::Result<&[u8]> {
        match &self.payload {
            Payload::Bytes(bytes) => Ok(bytes),
            Payload::Value(_) => Err(io::Error::other("MCP reply has no CLI bytes")),
        }
    }
    pub(crate) fn value(&self) -> io::Result<&Value> {
        match &self.payload {
            Payload::Value(value) => Ok(value),
            Payload::Bytes(_) => Err(io::Error::other("CLI reply has no MCP value")),
        }
    }
    pub(crate) fn value_mut(&mut self) -> io::Result<&mut Value> {
        match &mut self.payload {
            Payload::Value(value) => Ok(value),
            Payload::Bytes(_) => Err(io::Error::other("CLI reply has no MCP value")),
        }
    }
}
impl Encoding {
    pub(crate) fn cli(
        root: &clap::Command,
        settings: &crate::config::Settings,
        args: &[OsString],
    ) -> Self {
        if crate::grammar::plain_housekeeping(root, args) {
            return Self::Json;
        }
        let (format, _) = crate::grammar::presentation(root, args).unwrap_or_default();
        match format.or_else(|| crate::config::format(settings)) {
            Some(crate::grammar::Format::Md) => Self::Markdown,
            Some(crate::grammar::Format::Json) | None => Self::Json,
        }
    }
    pub(crate) fn usage(self, mut error: biomcp_types::UsageError) -> io::Result<Outcome> {
        if matches!(self, Self::Mcp)
            && let biomcp_types::UsageError::Arguments(arguments) = &mut error
        {
            for step in &mut arguments.next {
                // The local source list is a CLI command. MCP exposes the source list as help.
                if matches!(step.args.as_ref(), [verb, leaf] if &**verb == "source" && &**leaf == "list")
                {
                    step.args = ["help".into(), "sources".into()].into();
                }
            }
        }
        self.encode(
            biomcp_types::ErrorEnvelope::from(error),
            crate::exit::Status::Usage,
        )
    }
    pub(crate) fn encode(
        self,
        reply: impl serde::Serialize,
        status: crate::exit::Status,
    ) -> io::Result<Outcome> {
        let payload = match self {
            Self::Json => {
                let mut bytes = serde_json::to_vec(&reply)?;
                bytes.push(b'\n');
                Payload::Bytes(bytes)
            }
            Self::Markdown => Payload::Bytes(crate::view::markdown(&serde_json::to_value(reply)?)?),
            Self::Mcp => Payload::Value(serde_json::to_value(reply)?),
        };
        Ok(Outcome { payload, status })
    }
    pub(crate) fn schema(self, text: &str) -> io::Result<Outcome> {
        let payload = match self {
            Self::Mcp => Payload::Value(serde_json::from_str(text)?),
            Self::Json | Self::Markdown => Payload::Bytes(text.as_bytes().to_vec()),
        };
        Ok(Outcome {
            payload,
            status: crate::exit::Status::Success,
        })
    }
}
