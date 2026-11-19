//! Publisher downloads and bounded FDA document embedded-text access.
mod download;
mod embedded;
mod fonts;
mod garbled;
mod page;
pub mod publisher;
mod state;
mod text;
mod words;
pub(crate) use download::pdf;
pub use download::{MAX_BYTES, download, refused};
pub use embedded::{Embedded, embedded};
pub use text::{
    FRAGMENT_BYTES, MAX_EXPANDED_BYTES, MAX_PAGES, fragment_starts, read, refusal, whole,
};
