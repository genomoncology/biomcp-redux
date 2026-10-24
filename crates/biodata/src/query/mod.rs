//! Checked command inputs shared by every interface.
mod gene;
mod request;
pub use gene::{GeneGet, GeneSearch};
pub use request::Request;
