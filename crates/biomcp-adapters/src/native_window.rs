//! Accepted rows and the native positions traversed before identity refusals.
/// Internal adapter result; native counts never define public retrieved counts.
#[derive(Debug)]
pub struct NativeWindow<T> {
    /// Accepted records in source order.
    pub rows: Vec<T>,
    offset: u32,
    limit: u32,
    total: Option<u64>,
    received: u32,
}
impl<T> NativeWindow<T> {
    /// Bind accepted records to counts measured before identity filtering.
    pub fn new(rows: Vec<T>, offset: u32, limit: u32, total: Option<u64>, received: u32) -> Self {
        Self {
            rows,
            offset,
            limit,
            total,
            received,
        }
    }
    /// Select native slots before removing refused records.
    pub fn local(rows: Vec<Option<T>>, offset: u32, limit: u32) -> Self {
        let total = rows.len() as u64;
        let slots: Vec<_> = rows
            .into_iter()
            .skip(offset as usize)
            .take(limit as usize)
            .collect();
        let received = slots.len() as u32;
        Self::new(
            slots.into_iter().flatten().collect(),
            offset,
            limit,
            Some(total),
            received,
        )
    }
    /// Native selector offset.
    pub fn offset(&self) -> u32 {
        self.offset
    }
    /// Requested native window size.
    pub fn limit(&self) -> u32 {
        self.limit
    }
    /// Native selector total, when the provider supplies one.
    pub fn total(&self) -> Option<u64> {
        self.total
    }
    /// Native slots examined, including refused identities.
    pub fn received(&self) -> u32 {
        self.received
    }
    /// Whether the native window contained no rows before filtering.
    pub fn provider_empty(&self) -> bool {
        self.received == 0
    }
    /// Advance by native slots; an unknown total requires a full native page.
    pub fn next_offset(&self) -> Option<u32> {
        let next = self.offset.checked_add(self.received)?;
        (!self.provider_empty()
            && self
                .total
                .map_or(self.received == self.limit, |total| u64::from(next) < total))
        .then_some(next)
    }
    /// Keep native traversal facts while mapping accepted records for a caller.
    pub fn with_rows<U>(self, rows: Vec<U>) -> NativeWindow<U> {
        NativeWindow::new(rows, self.offset, self.limit, self.total, self.received)
    }
}
