//! Encode AlphaGenome requests and admit complete bounded provider replies.
mod envelope;
mod envelope_budget;
mod preflight;
mod proto;
mod registration;
mod request;
mod request_variant;
mod score;
mod score_envelope;
mod score_metadata;
mod score_reply;
mod tensor;
mod wire;
use biomcp_fetch::grpc::{CodecError, Limits, StreamDecoder};
pub use envelope::Envelope;
pub use registration::{PREDICT_VARIANT, SCORE_VARIANT, SPEC};
pub use request::{DnaseRequest, SCHEMA};
pub use score::{SCORE_SCHEMA, ScoreRequest};
pub use score_envelope::{SCORE_LIMITS, ScoreEnvelope};
pub use score_reply::{ScoreGroup, ScoreReply};
pub use tensor::{DataType, Tensor, TensorStream};

mod execute;
pub use execute::score;
