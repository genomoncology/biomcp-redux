//! FDA Drug Safety Communications: FDA's index page, a shipped seed of what each page links,
//! and one communication's page read live.
pub(crate) mod decl;
mod html;
mod page;
mod read;
mod seed;
mod spec;
pub use read::{Communications, communication, communications, text_hint};
pub use spec::SPEC;
pub(crate) use spec::{INDEX_PATH, SITE};
