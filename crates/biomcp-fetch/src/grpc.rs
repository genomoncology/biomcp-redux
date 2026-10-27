//! Decode provider tensor payloads under explicit allocation limits without network access.
mod bounds;
mod preflight;
mod tensor;

pub use bounds::Limits;
pub use tensor::{DataType, Tensor, TensorStream};

// The complete pinned closure is retained for future RPC adapters. Its messages
// remain private; callers pass encoded tensor payloads through the checked codec.
#[allow(
    dead_code,
    reason = "Retain the complete pinned generated provider import closure"
)]
mod proto {
    include!("grpc/generated/mod.rs");
}

/// A codec failure never contains provider bytes or metadata.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CodecError {
    /// The supplied bounds cannot describe a finite supported allocation.
    Limits,
    /// The message violates the schema, ordering or configured resource bounds.
    BadReply,
}
impl std::fmt::Display for CodecError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Limits => "grpc.limits",
            Self::BadReply => "grpc.bad_reply",
        })
    }
}
impl std::error::Error for CodecError {}
