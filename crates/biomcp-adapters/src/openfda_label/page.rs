use super::{raw::Record, subsection};
use crate::text_parts::{escaped_bytes, split};
use biomcp_fetch::RequestError;
use biomcp_types::{DrugLabelFragment, DrugLabelText};
/// Fragments keep a fixed size so offsets mean the same text whatever the reply budget.
const FRAGMENT_BYTES: usize = 6144;
/// One selected subsection or why the source text could not identify it.
#[derive(Debug)]
pub enum SubsectionPage {
    /// One complete heading, paged within its subtree.
    Data(DrugLabelText),
    /// No numbered heading matches the selector.
    Missing,
    /// Multiple supported headings match the selector.
    Ambiguous(Vec<String>),
}

struct Fragment {
    meta: DrugLabelFragment,
    text: String,
}
impl Record {
    /// Page whole text fragments until the next would pass `budget` escaped bytes of text.
    /// A budget of at least one fragment (6,144) always returns one; a section sharing a
    /// reply's spent budget may return none and continues at the same offset.
    /// Offsets and totals count fragments, never characters, sentences or source array elements.
    pub(super) fn label(
        &self,
        heading: &str,
        offset: u32,
        limit: u32,
        budget: usize,
    ) -> Result<Option<DrugLabelText>, RequestError> {
        self.parts(heading)
            .map(|parts| {
                self.label_page(
                    heading,
                    parts
                        .iter()
                        .enumerate()
                        .map(|(index, part)| (index, 0, part.as_str())),
                    offset,
                    limit,
                    budget,
                )
            })
            .transpose()
    }

    fn label_page<'a>(
        &self,
        heading: &str,
        parts: impl Iterator<Item = (usize, usize, &'a str)>,
        offset: u32,
        limit: u32,
        budget: usize,
    ) -> Result<DrugLabelText, RequestError> {
        if !(1..=50).contains(&limit) {
            return Err(bad_reply());
        }
        let all = fragments(parts)?;
        let total = all.len() as u64;
        let mut text = String::new();
        let mut selected = Vec::new();
        let mut bytes = 0;
        for fragment in all.into_iter().skip(offset as usize).take(limit as usize) {
            let size = escaped_bytes(&fragment.text);
            if bytes + size > budget {
                break;
            }
            bytes += size;
            text.push_str(&fragment.text);
            selected.push(fragment.meta);
        }
        let remaining = total.saturating_sub(u64::from(offset) + selected.len() as u64);
        Ok(DrugLabelText {
            label: self.reference()?,
            heading: heading.parse().map_err(|_| bad_reply())?,
            remaining: (remaining > 0).then_some(remaining),
            search: None,
            text: text.into(),
            retrieved: None,
            fragments: selected.into(),
            offset,
            total: Some(total),
        })
    }
}
impl super::Record {
    /// Select one numbered subtree before applying the same fragment limits as a whole field.
    pub fn label_subsection(
        &self,
        field: &str,
        asked: &str,
        offset: u32,
        limit: u32,
        budget: usize,
    ) -> Result<SubsectionPage, RequestError> {
        let Some(parts) = self.0.parts(field) else {
            return Ok(SubsectionPage::Missing);
        };
        let headings = subsection::headings(parts);
        let mut matching = headings
            .iter()
            .enumerate()
            .filter(|(_, h)| h.matches(asked));
        let Some((index, heading)) = matching.next() else {
            return Ok(SubsectionPage::Missing);
        };
        let others: Vec<_> = matching.map(|(_, h)| h.printed()).collect();
        if !others.is_empty() {
            let mut titles = vec![heading.printed()];
            titles.extend(others);
            return Ok(SubsectionPage::Ambiguous(titles));
        }
        let end = headings
            .iter()
            .skip(index + 1)
            .find(|h| !heading.contains(h));
        let slice = parts
            .iter()
            .enumerate()
            .skip(heading.part)
            .take_while(|(index, _)| end.is_none_or(|end| *index <= end.part))
            .filter_map(|(index, part)| {
                let start = if index == heading.part { heading.at } else { 0 };
                let stop = end
                    .filter(|end| index == end.part)
                    .map_or(part.len(), |end| end.at);
                let (preceding, from_start) = part.split_at(start);
                let (text, _) = from_start.split_at(stop - start);
                let last = end.map_or(parts.len() - 1, |end| end.part);
                if index == last {
                    let text = text.trim_end();
                    (!text.is_empty()).then(|| (index, preceding.chars().count(), text))
                } else {
                    Some((index, preceding.chars().count(), text))
                }
            });
        self.0
            .label_page(&heading.printed(), slice, offset, limit, budget)
            .map(SubsectionPage::Data)
    }
}

fn bad_reply() -> RequestError {
    biomcp_fetch::bad_reply(biomcp_types::Source::OpenfdaLabel, Some(200))
}

fn fragments<'a>(
    parts: impl Iterator<Item = (usize, usize, &'a str)>,
) -> Result<Vec<Fragment>, RequestError> {
    let mut fragments = Vec::new();
    for (source_part, start, part) in parts {
        let mut source_offset = u32::try_from(start).map_err(|_| bad_reply())?;
        let pieces = if part.is_empty() {
            vec![String::new()]
        } else {
            split(part, FRAGMENT_BYTES)
        };
        for text in pieces {
            let characters = u32::try_from(text.chars().count()).map_err(|_| bad_reply())?;
            fragments.push(Fragment {
                meta: DrugLabelFragment {
                    source_part: u32::try_from(source_part).map_err(|_| bad_reply())?,
                    source_offset,
                    characters,
                },
                text,
            });
            source_offset = source_offset
                .checked_add(characters)
                .ok_or_else(bad_reply)?;
        }
    }
    Ok(fragments)
}
