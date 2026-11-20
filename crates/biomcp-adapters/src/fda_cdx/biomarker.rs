//! Whole names in an FDA companion cell: a biomarker, or a test or drug name (1084).
/// For each place the cell names the asked text whole, ignoring case and surrounding spaces,
/// whether the word "Not" directly precedes it. PIK3CA matches "PIK3CA, AKT1, and PTEN" and
/// "PIK3CA  " but not PIK3CA2 (1056); MSI-High in "(Not MSI-High)" is negated (1065).
fn whole(cell: &str, asked: &str) -> Vec<bool> {
    let (cell, asked) = (cell.to_uppercase(), asked.trim().to_uppercase());
    let inside = |c: char| c.is_alphanumeric() || c == '-';
    if asked.is_empty() {
        return Vec::new();
    }
    cell.match_indices(asked.as_str())
        .filter(|(at, _)| {
            !cell[..*at].chars().next_back().is_some_and(inside)
                && !cell[at + asked.len()..].chars().next().is_some_and(inside)
        })
        .map(|(at, _)| {
            let before = cell[..at].trim_end();
            before.len() < at
                && before
                    .strip_suffix("NOT")
                    .is_some_and(|rest| !rest.chars().next_back().is_some_and(inside))
        })
        .collect()
}

/// Whether the cell names the asked text whole.
pub(super) fn names(cell: &str, asked: &str) -> bool {
    !whole(cell, asked).is_empty()
}

/// Whether the cell names the asked biomarker only as "Not NAME", such as "Not MSI-High".
pub(super) fn negated(cell: &str, asked: &str) -> bool {
    let found = whole(cell, asked);
    !found.is_empty() && found.iter().all(|not| *not)
}
