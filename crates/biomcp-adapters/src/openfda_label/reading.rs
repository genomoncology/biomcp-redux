//! How many large label pages one process decodes at once.
use std::num::NonZeroUsize;

/// Default for [`set_pages_at_once`]. Decoding, checking and caching a large label page took
/// about five times its size in a hosted server: a page near the 32 MiB page bound cost about
/// 140 MiB. Three such pages at once stay under half of a 1 GiB task.
pub const PAGES_AT_ONCE: usize = 3;
/// A body at least this large waits for a turn; smaller pages and single labels never wait.
pub(super) const LARGE_PAGE: usize = 1024 * 1024;

pub(super) static PAGES: biomcp_fetch::Turns = biomcp_fetch::Turns::new(PAGES_AT_ONCE);

/// Set how many large label pages this process decodes at once.
pub fn set_pages_at_once(limit: NonZeroUsize) {
    PAGES.set_limit(limit.get());
}
