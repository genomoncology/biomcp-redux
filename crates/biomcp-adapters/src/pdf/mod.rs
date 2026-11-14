//! Publisher downloads and bounded FDA document embedded-text access.
mod download;
mod text;
pub(crate) use download::pdf;
pub use download::{MAX_BYTES, download};
pub use text::{
    FRAGMENT_BYTES, MAX_EXPANDED_BYTES, MAX_PAGES, fragment_starts, read, refusal, whole,
};
