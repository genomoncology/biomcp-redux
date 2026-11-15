//! Shared pagination operations preserve selectors and page sizes.
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

/// Reply row lists are written even when empty (0948), so a script reading `items` finds `[]`.
/// They stay optional in the 1.0.0 schema, as before. The schema derive writes a field's default
/// unless the field's skip test passes on it; this test always passes, so the schema files keep
/// no `default` and stay unchanged. Serialization ignores it.
#[cfg(feature = "schema")]
pub(crate) fn no_schema_default<T>(_: &[T]) -> bool {
    true
}
