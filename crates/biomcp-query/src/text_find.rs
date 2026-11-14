//! Literal find paging shared by article text and FDA document text.
use biomcp_types::PageInfo;
/// Most hits returned in one find page.
const HITS: usize = 25;
/// Case-insensitive literal containment against lower-cased needles.
pub(crate) fn contains(text: &str, needles: &[String]) -> bool {
    let lower = text.to_lowercase();
    needles.iter().any(|needle| lower.contains(needle))
}
/// One find page: the selected hits, their page and the following offset when more remain.
pub(crate) struct Page<'a, T> {
    pub(crate) hits: &'a [T],
    pub(crate) info: PageInfo,
    pub(crate) following: Option<u32>,
}
/// Take whole hits from `offset` until 25 hits or `budget` bytes; the first hit always fits.
/// An offset past the last hit is a position error, answered with `None`.
pub(crate) fn page<T>(
    hits: &[T],
    offset: u32,
    budget: usize,
    cost: impl Fn(&T) -> Option<usize>,
) -> Option<Page<'_, T>> {
    let start = usize::try_from(offset).ok()?;
    let rest = hits.get(start..)?;
    let mut size = 0;
    let mut taken = 0;
    for hit in rest.iter().take(HITS) {
        let bytes = cost(hit)?;
        if taken > 0 && size + bytes > budget {
            break;
        }
        size += bytes;
        taken += 1;
    }
    let returned = u32::try_from(taken).ok()?;
    let total = u64::try_from(hits.len()).ok()?;
    let following = offset.checked_add(returned)?;
    Some(Page {
        hits: &rest[..taken],
        info: PageInfo {
            offset,
            returned,
            total,
            pool: None,
            truncated: None,
        },
        following: (returned > 0 && u64::from(following) < total).then_some(following),
    })
}
