//! FDA Drug Safety Communications: FDA's index page, a shipped seed of what each page links,
//! and one communication's page read live.
pub(crate) mod decl;
pub(crate) mod html;
mod page;
mod read;
mod seed;
mod spec;
pub use read::{Communications, communication, communications, text_hint};
pub use seed::SearchRow;
pub(crate) use spec::{INDEX_PATH, SITE};
pub use spec::{SPEC, index_url};
