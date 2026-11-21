//! The few HTML facts read from FDA's pages: text without tags, one attribute, and dates.
use biodata::PartialDate;
use biomcp_types::ProblemCode;

pub(super) type Date = Option<Result<PartialDate, ProblemCode>>;

/// Visible text of a fragment: tags, scripts and styles removed, character references decoded,
/// and runs of whitespace joined into one space.
pub(super) fn visible(fragment: &str) -> String {
    let mut out = String::with_capacity(fragment.len() / 2);
    let mut rest = fragment;
    while let Some(open) = rest.find('<') {
        out.push_str(rest.get(..open).unwrap_or_default());
        out.push(' ');
        let tag = rest.get(open..).unwrap_or_default();
        let skip = ["<script", "<style"]
            .iter()
            .find(|name| {
                tag.get(..name.len())
                    .is_some_and(|t| t.eq_ignore_ascii_case(name))
            })
            .map(|name| format!("</{}>", name.trim_start_matches('<')));
        rest = match skip.and_then(|end| tag.find(&end).map(|at| at + end.len())) {
            Some(end) => tag.get(end..).unwrap_or_default(),
            None => tag
                .find('>')
                .map_or("", |close| tag.get(close + 1..).unwrap_or_default()),
        };
    }
    out.push_str(rest);
    let decoded = crate::source_text::decode_entities(&out);
    decoded.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// One double-quoted attribute value from an opening tag.
pub(super) fn attribute<'a>(tag: &'a str, name: &str) -> Option<&'a str> {
    let key = format!(" {name}=\"");
    let start = tag.find(&key)? + key.len();
    let value = tag.get(start..)?;
    value.get(..value.find('"')?)
}

/// A full date FDA typed as month, day and four-digit year, separated by `-` or `/`. A
/// two-digit year or any other form is not read, so no century is guessed.
pub(super) fn date(text: &str) -> Date {
    let mut parts = text.trim().split(['-', '/']);
    let (month, day, year) = (parts.next()?, parts.next()?, parts.next()?);
    if parts.next().is_some()
        || year.len() != 4
        || !(1..=2).contains(&month.len())
        || !(1..=2).contains(&day.len())
    {
        return None;
    }
    let number = |part: &str| {
        part.bytes().try_fold(0u16, |n, b| {
            b.is_ascii_digit().then(|| n * 10 + u16::from(b - b'0'))
        })
    };
    let (month, day, year) = (number(month)?, number(day)?, number(year)?);
    let (month, day) = (
        u8::try_from(month).unwrap_or(0),
        u8::try_from(day).unwrap_or(0),
    );
    Some(PartialDate::new(year, Some(month), Some(day)).map_err(|_| ProblemCode::ValueUnknown))
}
