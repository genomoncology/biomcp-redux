//! Whole names in an FDA companion cell: a biomarker, or a test or drug name (1084).
/// Whether the cell names the asked biomarker whole, ignoring case and surrounding spaces, so
/// PIK3CA matches "PIK3CA, AKT1, and PTEN" and "PIK3CA  " but not PIK3CA2 (1056).
pub(super) fn names(cell: &str, asked: &str) -> bool {
    let (cell, asked) = (cell.to_uppercase(), asked.trim().to_uppercase());
    let inside = |c: char| c.is_alphanumeric() || c == '-';
    !asked.is_empty()
        && cell.match_indices(asked.as_str()).any(|(at, _)| {
            !cell[..at].chars().next_back().is_some_and(inside)
                && !cell[at + asked.len()..].chars().next().is_some_and(inside)
        })
}
