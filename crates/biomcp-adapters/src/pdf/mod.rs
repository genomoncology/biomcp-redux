//! Publisher downloads and bounded document embedded-text access.
mod child;
mod chunks;
mod cmap;
mod download;
mod embedded;
mod fonts;
mod garbled;
mod lexer;
mod numbers;
mod page;
mod position;
pub mod publisher;
mod scan;
mod state;
mod symbol;
mod text;
mod tounicode;
mod words;
mod xref;
pub use crate::text_parts::escaped_bytes;
pub use child::{MAX_OUTPUT, pages, serve};
pub(crate) use download::pdf;
pub use download::{MAX_BYTES, download, refused};
pub use embedded::{Embedded, embedded};
pub use garbled::garbled;
pub use symbol::is_private_use;
pub(crate) use text::from_download;
pub use text::{
    FRAGMENT_BYTES, LABEL_PAGES, MAX_EXPANDED_BYTES, MAX_PAGES, fragment_starts, label,
    missing_notice, part, read, refusal, whole,
};
