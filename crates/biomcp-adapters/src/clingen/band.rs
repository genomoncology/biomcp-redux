//! Cytoband containment for ClinGen region spans such as 15q11.2-q13.1.
//! Bands compare as digit sequences on one arm: q11.2 is [1,1,2], q12 is [1,2], q13.1 is [1,3,1].

/// One band: chromosome, arm (`p` or `q`, absent for a whole chromosome) and its digits.
#[derive(Debug, PartialEq, Eq)]
pub(super) struct Band<'a> {
    pub(super) chromosome: &'a str,
    arm: Option<char>,
    digits: Vec<u8>,
}
/// Read a band such as 16p11.2, 15q or X; `None` for any other spelling.
pub(super) fn parse(text: &str) -> Option<Band<'_>> {
    let (chromosome, rest) = text.split_at(text.find(['p', 'q']).unwrap_or(text.len()));
    if chromosome.is_empty() || !chromosome.chars().all(|c| c.is_ascii_alphanumeric()) {
        return None;
    }
    on_arm(chromosome, rest)
}
fn on_arm<'a>(chromosome: &'a str, rest: &str) -> Option<Band<'a>> {
    let mut chars = rest.chars();
    let arm = chars.next();
    if arm.is_some_and(|arm| arm != 'p' && arm != 'q') {
        return None;
    }
    let mut digits = Vec::new();
    for c in chars.filter(|c| *c != '.') {
        digits.push(u8::try_from(c.to_digit(10)?).unwrap_or_default());
    }
    Some(Band {
        chromosome,
        arm,
        digits,
    })
}
/// The two ends of a published span; a single band is both ends.
fn ends(cytoband: &str) -> Option<(Band<'_>, Band<'_>)> {
    let (first, last) = cytoband.split_once('-').unwrap_or((cytoband, cytoband));
    let first = parse(first)?;
    let last = if last.starts_with(['p', 'q']) {
        on_arm(first.chromosome, last)?
    } else {
        parse(last)?
    };
    Some((first, last))
}
/// A query band lies inside a published span, or it and either end band are one a prefix of the other.
pub(super) fn contains(cytoband: &str, query: &Band<'_>) -> bool {
    let Some((first, last)) = ends(cytoband) else {
        return false;
    };
    if !first.chromosome.eq_ignore_ascii_case(query.chromosome) {
        return false;
    }
    let Some(arm) = query.arm else {
        return true;
    };
    // 17q21 holds 17q21.3, and 17q21.31 lies within a region labelled 17q21.3.
    let prefix = |end: &Band<'_>| {
        end.arm == Some(arm)
            && (end.digits.starts_with(&query.digits) || query.digits.starts_with(&end.digits))
    };
    if prefix(&first) || prefix(&last) {
        return true;
    }
    match (first.arm, last.arm) {
        (Some(a), Some(b)) if a == b => {
            let (low, high) = if first.digits <= last.digits {
                (&first.digits, &last.digits)
            } else {
                (&last.digits, &first.digits)
            };
            a == arm && *low <= query.digits && query.digits <= *high
        }
        _ => false,
    }
}
