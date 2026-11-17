//! The registry's own "Inclusion Criteria" and "Exclusion Criteria" headings and list lines; never eligibility rules.

/// Registry items under each heading, in source order.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Lists {
    /// Items below an inclusion heading.
    pub inclusion: Vec<Box<str>>,
    /// Items below an exclusion heading.
    pub exclusion: Vec<Box<str>>,
}

/// Whether a line is an inclusion (`false`) or exclusion (`true`) heading, with any text after its colon.
pub(crate) fn heading(line: &str) -> Option<(bool, &str)> {
    let trimmed = line.trim_matches(|c: char| c.is_whitespace() || "#*_-".contains(c));
    let lower = trimmed.to_ascii_lowercase();
    let lower = lower.strip_prefix("key ").unwrap_or(&lower);
    [("exclusion criteria", true), ("inclusion criteria", false)]
        .into_iter()
        .find_map(|(prefix, excluded)| {
            let tail = lower.strip_prefix(prefix)?.trim_start();
            (tail.is_empty() || tail.starts_with(':')).then_some((excluded, tail.len()))
        })
        .map(|(excluded, tail)| {
            // ASCII lowercasing keeps byte offsets, so the tail is the original's suffix.
            let rest = trimmed.split_at(trimmed.len() - tail).1;
            (excluded, rest.trim_start_matches(':').trim())
        })
}

/// Splits one page of `text`, starting at byte `start`, into the registry's items.
///
/// The split happens only when the whole text carries both headings. An item is a
/// line below a heading together with the more deeply indented lines that follow it,
/// as the registry's nested list markers lay them out.
pub fn split(text: &str, start: usize, page: &str) -> Option<Lists> {
    let headings = text
        .lines()
        .filter_map(heading)
        .map(|(excluded, _)| excluded);
    let (mut inclusion, mut exclusion) = (false, false);
    for excluded in headings {
        *(if excluded {
            &mut exclusion
        } else {
            &mut inclusion
        }) = true;
    }
    if !(inclusion && exclusion) {
        return None;
    }
    let mut section = text
        .get(..start)?
        .lines()
        .filter_map(heading)
        .last()
        .map(|(excluded, _)| excluded);
    let mut lists = Lists::default();
    let mut item: Option<(usize, String)> = None;
    let mut close = |item: &mut Option<(usize, String)>, section: Option<bool>| {
        if let (Some((_, body)), Some(excluded)) = (item.take(), section) {
            let body = body.trim();
            if !body.is_empty() {
                let list = if excluded {
                    &mut lists.exclusion
                } else {
                    &mut lists.inclusion
                };
                list.push(body.into());
            }
        }
    };
    for line in page.split_inclusive('\n') {
        if let Some((excluded, rest)) = heading(line) {
            close(&mut item, section);
            section = Some(excluded);
            if !rest.is_empty() {
                item = Some((0, rest.to_owned()));
            }
            continue;
        }
        let indent = line.len() - line.trim_start_matches([' ', '\t']).len();
        match item.as_mut() {
            Some((_, body)) if line.trim().is_empty() => body.push_str(line),
            Some((first, body)) if indent > *first => body.push_str(line),
            _ if line.trim().is_empty() => {}
            _ => {
                close(&mut item, section);
                item = Some((indent, line.to_owned()));
            }
        }
    }
    close(&mut item, section);
    Some(lists)
}
