//! Source coordinate metadata qualifies an unchanged native record key.
//! https://biothings.io/blog/new-data-release-201606 describes root chrom/hg19/hg38.
//! Index selection alone never establishes a returned coordinate's assembly.
use super::raw::Hit;
use biodata::Build;
use serde_json::Value;

pub(super) fn source_build(hit: &Hit) -> Option<Build> {
    let (chromosome, expression) = hit.id.as_str().strip_prefix("chr")?.split_once(":g.")?;
    if hit.chrom.as_ref()?.as_str()? != chromosome {
        return None;
    }
    let span = literal_span(expression)?;
    let mut found = None;
    for (build, value) in [(Build::Grch37, &hit.hg19), (Build::Grch38, &hit.hg38)] {
        if let Some(value) = value {
            let source = source_span(value)?;
            if source == span {
                if found.is_some() {
                    return None;
                }
                found = Some(build);
            }
        }
    }
    found
}
fn source_span(value: &Value) -> Option<(u64, u64)> {
    let start = value.get("start")?.as_u64()?;
    let end = value.get("end")?.as_u64()?;
    (start > 0 && end >= start).then_some((start, end))
}
// Compare only literal substitution positions. Other HGVS forms retain unknown assembly;
// no normalization, strand conversion, or allele transplant qualifies them.
fn literal_span(expression: &str) -> Option<(u64, u64)> {
    let length = expression.bytes().take_while(u8::is_ascii_digit).count();
    let position = expression.get(..length)?.parse::<u64>().ok()?;
    let (reference, alternate) = expression.get(length..)?.split_once('>')?;
    if position == 0
        || reference.len() != 1
        || alternate.len() != 1
        || !reference
            .bytes()
            .chain(alternate.bytes())
            .all(|b| b"ACGT".contains(&b))
    {
        return None;
    }
    Some((position, position))
}
