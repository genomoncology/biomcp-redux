//! The registry's own "Inclusion Criteria" and "Exclusion Criteria" headings and list lines; never eligibility rules.

/// One page of the registry text, split at its own headings.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Split {
    /// This page's text before the first heading, trimmed.
    pub preamble: Option<Box<str>>,
    /// Items below an inclusion heading, when this page holds that heading or such an item.
    pub inclusion: Option<Vec<Box<str>>>,
    /// Items below an exclusion heading, when this page holds that heading or such an item.
    pub exclusion: Option<Vec<Box<str>>>,
}

/// Whether a line is an inclusion (`false`) or exclusion (`true`) heading, with any text after its colon.
pub(crate) fn heading(line: &str) -> Option<(bool, &str)> {
    let trimmed = line.trim_start_matches(|c: char| c.is_whitespace() || "#*_-".contains(c));
    let lower = trimmed.to_ascii_lowercase();
    let lower = lower.strip_prefix("key ").unwrap_or(&lower);
    [("exclusion criteria", true), ("inclusion criteria", false)]
        .into_iter()
        .find_map(|(prefix, excluded)| {
            let tail = lower.strip_prefix(prefix)?.trim_start();
            if tail.starts_with(':') {
                return Some((excluded, tail.len()));
            }
            trim(tail).is_empty().then_some((excluded, 0))
        })
        .map(|(excluded, tail)| {
            // ASCII lowercasing keeps byte offsets, so the tail is the original's suffix.
            let rest = trimmed.split_at(trimmed.len() - tail).1;
            (excluded, rest.strip_prefix(':').unwrap_or(rest).trim_start())
        })
}

fn trim(line: &str) -> &str {
    line.trim_matches(|c: char| c.is_whitespace() || "#*_-".contains(c))
}

/// A line ending in "criteria:" that is not one of the two headings, such as a cohort's
/// heading, could start a block the split would misfile. A line that starts with a list
/// marker is an item, such as "6. Any of the following cardiac criteria:".
fn other_criteria_heading(line: &str) -> bool {
    let trimmed = trim(line);
    trimmed
        .strip_suffix(':')
        .is_some_and(|head| head.trim_end().to_ascii_lowercase().ends_with("criteria"))
        && !list_marker(trimmed)
}

/// A number or a single letter followed by "." or ")", or a bullet; `trim` already removed
/// "*" and "-".
fn list_marker(line: &str) -> bool {
    if line.starts_with(['•', '·', '+', '○', '▪']) {
        return true;
    }
    let digits = line.len() - line.trim_start_matches(|c: char| c.is_ascii_digit()).len();
    let label = match digits {
        0 => line
            .chars()
            .next()
            .filter(char::is_ascii_alphabetic)
            .map_or(0, |_| 1),
        n => n,
    };
    label > 0
        && line
            .get(label..)
            .is_some_and(|rest| rest.starts_with(['.', ')']))
}

/// Whether the whole text carries both headings and no other line ends in "criteria:".
pub fn splits(text: &str) -> bool {
    let (mut inclusion, mut exclusion) = (false, false);
    for line in text.lines() {
        match heading(line) {
            Some((true, _)) => exclusion = true,
            Some((false, _)) => inclusion = true,
            None if other_criteria_heading(line) => return false,
            None => {}
        }
    }
    inclusion && exclusion
}

/// Splits one page of `text`, starting at byte `start`, into the registry's items.
///
/// An item is a line below a heading together with the more deeply indented lines
/// that follow it, as the registry's nested list markers lay them out.
pub fn split(text: &str, start: usize, page: &str) -> Option<Split> {
    if !splits(text) {
        return None;
    }
    let before = text.get(..start)?;
    let starts_on_line = before.is_empty() || before.ends_with('\n');
    let line_start = before.rfind('\n').map_or(0, |index| index + 1);
    let whole_line = text.get(line_start..)?.lines().next();
    let continued_heading = whole_line.filter(|_| !starts_on_line).and_then(heading);
    let mut section = continued_heading
        .or_else(|| text.get(..line_start)?.lines().rev().find_map(heading))
        .map(|(excluded, _)| excluded);
    let mut out = Split::default();
    let mut preamble = String::new();
    let mut item: Option<(usize, String)> = None;
    let close = |out: &mut Split, item: &mut Option<(usize, String)>, section: Option<bool>| {
        if let (Some((_, body)), Some(excluded)) = (item.take(), section) {
            let body = body.trim();
            if !body.is_empty() {
                list(out, excluded).push(body.into());
            }
        }
    };
    let remaining = text.get(start..)?;
    for (index, (line, source_line)) in page
        .split_inclusive('\n')
        .zip(remaining.split_inclusive('\n'))
        .enumerate()
    {
        let complete_start = index > 0 || starts_on_line;
        let complete_end = line.len() == source_line.len();
        if complete_start
            && (complete_end || heading(source_line).is_some())
            && let Some((excluded, rest)) = heading(line)
        {
            close(&mut out, &mut item, section);
            section = Some(excluded);
            list(&mut out, excluded);
            if !rest.is_empty() {
                item = Some((0, rest.to_owned()));
            }
            continue;
        }
        if section.is_none() {
            preamble.push_str(line);
            continue;
        }
        let indent = line.len() - line.trim_start_matches([' ', '\t']).len();
        match item.as_mut() {
            Some((_, body)) if line.trim().is_empty() => body.push_str(line),
            Some((first, body)) if indent > *first => body.push_str(line),
            _ if line.trim().is_empty() => {}
            _ => {
                close(&mut out, &mut item, section);
                item = Some((indent, line.to_owned()));
            }
        }
    }
    close(&mut out, &mut item, section);
    let preamble = preamble.trim();
    out.preamble = (!preamble.is_empty()).then(|| preamble.into());
    Some(out)
}

/// The section's list, present from its heading or first item on this page.
fn list(out: &mut Split, excluded: bool) -> &mut Vec<Box<str>> {
    if excluded {
        out.exclusion.get_or_insert_with(Vec::new)
    } else {
        out.inclusion.get_or_insert_with(Vec::new)
    }
}

#[cfg(test)]
mod tests {
    use super::split;

    #[test]
    fn partial_lines_use_their_actual_source_heading() {
        let text = concat!(
            "Inclusion Criteria:\nExclusion Criteria are mentioned\n",
            "* Item mentions Exclusion Criteria: here\nExclusion Criteria: continued item"
        );
        let first = "Inclusion Criteria:\nExclusion Criteria";
        let page = split(text, 0, first).expect("split");
        assert!(page.exclusion.is_none());
        let suffix = "Exclusion Criteria: here\n";
        let suffix_start = text.find(suffix).expect("suffix in fixture");
        let page = split(text, suffix_start, suffix).expect("split");
        assert_eq!(
            page.inclusion.as_deref(),
            Some([Box::from("Exclusion Criteria: here")].as_slice())
        );
        assert!(page.exclusion.is_none());
        let heading_end = text.len() - "continued item".len();
        let page = split(text, heading_end, "continued item").expect("split");
        assert_eq!(
            page.exclusion.as_deref(),
            Some([Box::from("continued item")].as_slice())
        );
    }
}
