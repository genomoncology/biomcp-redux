//! The few HTML facts read from FDA's pages: text without tags, one attribute, and dates.
use biodata::PartialDate;

/// Visible text of a fragment: tags, scripts and styles removed, character references decoded,
/// and runs of whitespace joined into one space.
pub(super) fn text(fragment: &str) -> String {
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
pub(super) fn date(text: &str) -> Option<PartialDate> {
    let mut parts = text.trim().split(['-', '/']);
    let (month, day, year) = (parts.next()?, parts.next()?, parts.next()?);
    if parts.next().is_some() || year.len() != 4 || month.len() > 2 || day.len() > 2 {
        return None;
    }
    let number = |part: &str| {
        (!part.is_empty() && part.bytes().all(|b| b.is_ascii_digit()))
            .then(|| part.parse::<u16>().ok())
            .flatten()
    };
    let (month, day, year) = (number(month)?, number(day)?, number(year)?);
    PartialDate::new(year, u8::try_from(month).ok(), u8::try_from(day).ok()).ok()
}

#[cfg(test)]
mod tests {
    use super::{attribute, date, text};
    #[test]
    fn reads_text_attributes_and_full_dates() {
        assert_eq!(text("<p>A&amp;B <script>x</script>c</p>"), "A&B c");
        assert_eq!(attribute(r#"<a href="/x" title="y">"#, "title"), Some("y"));
        assert_eq!(
            date("9-1-2021").map(|d| d.to_string()),
            Some("2021-09-01".into())
        );
        assert_eq!(
            date("09/01/2021").map(|d| d.to_string()),
            Some("2021-09-01".into())
        );
        assert_eq!(date("9-1-21"), None);
    }
}
