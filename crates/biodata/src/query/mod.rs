//! Checked command inputs shared by every interface.
mod gene;
mod request;
pub use gene::{GeneGet, GeneSearch, GeneSection, GeneSectionWord};
pub use request::Request;
