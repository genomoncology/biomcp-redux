//! The registry's own "Inclusion Criteria" and "Exclusion Criteria" headings and list lines; never eligibility rules.

/// One page of registry criteria, with any named arms in source order.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Page {
    /// Criteria outside a named arm.
    pub unscoped: Split,
    /// Named arms with text or headings on this page.
    pub arms: Vec<Arm>,
}

/// Criteria under a registry arm heading.
#[derive(Debug, PartialEq, Eq)]
pub struct Arm {
    /// The registry title without its heading markup or inline criteria label.
    pub heading: Box<str>,
    /// Criteria within this arm, which may continue on another page.
    pub criteria: Split,
}

/// Registry text within one scope, split at its criteria headings.
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
            (
                excluded,
                rest.strip_prefix(':').unwrap_or(rest).trim_start(),
            )
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

/// A Markdown arm heading, optionally ending in an inline inclusion or exclusion label.
pub fn arm_heading(line: &str) -> Option<(&str, Option<bool>)> {
    let line = line.trim_start();
    let markup = line.strip_prefix('\\').unwrap_or(line);
    let title = trim(markup.strip_prefix('#')?);
    let title = title.trim_end_matches(':').trim_end();
    let lower = title.to_ascii_lowercase();
    for (suffix, excluded) in [
        (" exclusion criteria", true),
        (" inclusion criteria", false),
    ] {
        if let Some(name) = lower.strip_suffix(suffix)
            && name.ends_with(" arm")
        {
            return title.get(..name.len()).map(|name| (name, Some(excluded)));
        }
    }
    lower.ends_with(" arm").then_some((title, None))
}

fn section(line: &str) -> Option<(bool, &str)> {
    match arm_heading(line) {
        Some((_, excluded)) => excluded.map(|excluded| (excluded, "")),
        None => heading(line),
    }
}

/// Whether the registry marks arms, or unambiguously marks both criteria lists.
pub fn splits(text: &str) -> bool {
    text.lines().any(|line| arm_heading(line).is_some()) || splits_lists(text)
}

fn splits_lists(text: &str) -> bool {
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

/// Splits a page of unscoped criteria into the registry's own inclusion and exclusion items.
pub fn split(text: &str, start: usize, page: &str) -> Option<Split> {
    splits_lists(text)
        .then(|| split_items(text, start, page))
        .flatten()
}

/// Splits a page beginning at byte `start` into registry arms and items.
pub fn split_page(text: &str, start: usize, page: &str) -> Option<Page> {
    let end = start.checked_add(page.len())?;
    let mut arms = text
        .split_inclusive('\n')
        .scan(0, |offset, line| {
            let position = *offset;
            *offset += line.len();
            Some((position, line))
        })
        .filter_map(|(position, line)| arm_heading(line).map(|(title, _)| (position, Some(title))))
        .peekable();
    if arms.peek().is_none() {
        return split(text, start, page).map(|unscoped| Page {
            unscoped,
            arms: Vec::new(),
        });
    }
    let boundaries = arms.chain(std::iter::once((text.len(), None)));
    let mut out = Page::default();
    let (mut block_start, mut title) = (0, None);
    for (block_end, next_title) in boundaries {
        let left = start.max(block_start);
        let right = end.min(block_end);
        if left < right {
            let criteria = split_scope(
                text.get(block_start..block_end)?,
                left - block_start,
                text.get(left..right)?,
            )?;
            if let Some(title) = title {
                out.arms.push(Arm {
                    heading: Box::from(title),
                    criteria,
                });
            } else {
                out.unscoped = criteria;
            }
        }
        if block_end >= end {
            break;
        }
        (block_start, title) = (block_end, next_title);
    }
    Some(out)
}

fn split_scope(text: &str, start: usize, page: &str) -> Option<Split> {
    if !text.lines().any(|line| {
        heading(line).is_none() && arm_heading(line).is_none() && other_criteria_heading(line)
    }) {
        return split_items(text, start, page);
    }
    // Keep ambiguous text in its own scope without reassigning any criterion.
    let heading_end = if arm_heading(text.lines().next()?).is_some() {
        text.find('\n').map_or(text.len(), |offset| offset + 1)
    } else {
        0
    };
    let preamble = page
        .get(heading_end.saturating_sub(start)..)
        .unwrap_or_default()
        .trim();
    Some(Split {
        preamble: (!preamble.is_empty()).then(|| preamble.into()),
        ..Split::default()
    })
}

// An item includes any more deeply indented lines.
fn split_items(text: &str, start: usize, page: &str) -> Option<Split> {
    let before = text.get(..start)?;
    let starts_on_line = before.is_empty() || before.ends_with('\n');
    let line_start = before.rfind('\n').map_or(0, |index| index + 1);
    let whole_line = text.get(line_start..)?.lines().next();
    let continued_heading = whole_line.filter(|_| !starts_on_line).and_then(section);
    let mut excluded = continued_heading
        .or_else(|| text.get(..line_start)?.lines().rev().find_map(section))
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
        if let Some((_, inline)) = arm_heading(
            whole_line
                .filter(|_| !complete_start)
                .unwrap_or(source_line),
        ) {
            excluded = inline;
            if let Some(inline) = inline {
                list(&mut out, inline);
            }
            continue;
        }
        if complete_start
            && (complete_end || heading(source_line).is_some())
            && let Some((new_section, rest)) = heading(line)
        {
            close(&mut out, &mut item, excluded);
            excluded = Some(new_section);
            list(&mut out, new_section);
            if !rest.is_empty() {
                item = Some((0, rest.to_owned()));
            }
            continue;
        }
        if excluded.is_none() {
            preamble.push_str(line);
            continue;
        }
        let indent = line.len() - line.trim_start_matches([' ', '\t']).len();
        match item.as_mut() {
            Some((_, body)) if line.trim().is_empty() => body.push_str(line),
            Some((first, body)) if indent > *first => body.push_str(line),
            _ if line.trim().is_empty() => {}
            _ => {
                close(&mut out, &mut item, excluded);
                item = Some((indent, line.to_owned()));
            }
        }
    }
    close(&mut out, &mut item, excluded);
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
