//! Publisher downloads and bounded FDA document embedded-text access.
mod child;
mod chunks;
mod cmap;
mod download;
mod embedded;
mod fonts;
mod garbled;
mod lexer;
mod page;
pub mod publisher;
mod scan;
mod state;
mod text;
mod words;
mod xref;
pub use child::{MAX_OUTPUT, pages, serve};
pub(crate) use download::pdf;
pub use download::{MAX_BYTES, download, refused};
pub use embedded::{Embedded, embedded};
pub use garbled::garbled;
pub use text::{
    FRAGMENT_BYTES, LABEL_PAGES, MAX_EXPANDED_BYTES, MAX_PAGES, fragment_starts, label,
    missing_notice, read, refusal, whole,
};
