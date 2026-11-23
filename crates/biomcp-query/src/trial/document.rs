//! Registry manifest documents as downloadable files or readable text.
mod access;
mod text;
pub(crate) use access::run;
pub use access::{Download, download};
