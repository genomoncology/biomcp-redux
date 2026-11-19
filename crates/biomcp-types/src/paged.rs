//! Shared pagination operations preserve selectors and page sizes.

/// The most rows one page takes from a list held whole, a stored copy or a whole record
/// (1061): the source is not asked again, and 200 narrow rows stay near 60 KB.
pub const HELD_PAGE_MAX: u32 = 200;
/// The most rows one page of a search takes when the source pages it and serves 100 at once.
pub const SEARCH_PAGE_MAX: u32 = 100;
/// A checked query with an independent page offset and optional retained context.
pub trait Paged: Clone {
    /// Native offset for offset-capable queries; unit for restart-only queries.
    type Offset: Default;
    /// Set the supported offset without changing selection fields.
    fn set_page_offset(&mut self, offset: Self::Offset);
    /// Drop retained paging context while preserving selection fields.
    fn clear_page_context(&mut self) {}
    /// Continue the same selection at another offset.
    fn at_offset(&self, offset: Self::Offset) -> Self {
        let mut page = self.clone();
        page.set_page_offset(offset);
        page
    }
    /// Restart the selection without its retained collection or snapshot.
    fn restart(&self) -> Self {
        let mut page = self.at_offset(Self::Offset::default());
        page.clear_page_context();
        page
    }
}
