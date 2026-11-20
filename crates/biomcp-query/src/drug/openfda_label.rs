//! Native SPL identities and unchanged explicit label headings.
mod names;
mod read;
mod search;
mod search_page;
mod section;

use biomcp_types::Source::OpenfdaLabel as SOURCE;
pub(super) use read::get;
pub(super) use search::search;
