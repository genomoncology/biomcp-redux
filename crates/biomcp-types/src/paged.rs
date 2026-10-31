//! Shared pagination operations preserve selectors and page sizes.
/// A checked query with an independent page offset and optional retained context.
pub trait Paged: Clone {
    /// Set the source-owned offset without changing selection fields.
    fn set_page_offset(&mut self, offset: u32);
    /// Drop retained paging context while preserving selection fields.
    fn clear_page_context(&mut self) {}
    /// Continue the same selection at another offset.
    fn at_offset(&self, offset: u32) -> Self {
        let mut page = self.clone();
        page.set_page_offset(offset);
        page
    }
    /// Restart the selection without its retained collection or snapshot.
    fn restart(&self) -> Self {
        let mut page = self.at_offset(0);
        page.clear_page_context();
        page
    }
}
