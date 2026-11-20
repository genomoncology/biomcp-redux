//! Current-label phrase search options and excerpt values.
use super::{DrugSection, DrugSource};
use crate::QueryInvalid;
use biodata::{DrugId, Text};
use serde::{Deserialize, Serialize};

const MAX_LABELS: usize = 5;
const MAX_SECTIONS: usize = 5;

pub(super) fn requested(sections: &[DrugSection], finds: &[Text]) -> bool {
    sections == [DrugSection::Label] && !finds.is_empty()
}

pub(super) fn check(
    source: Option<DrugSource>,
    (limit, offset): (Option<u32>, Option<u32>),
    (finds, every, headings): (&[Text], bool, Option<&Text>),
) -> Result<(), QueryInvalid> {
    if source != Some(DrugSource::OpenfdaLabel) || every {
        return Err(QueryInvalid::new(&[
            "section",
            "source",
            "find",
            "every_version",
        ]));
    }
    if limit.is_some() || offset.is_some() {
        return Err(QueryInvalid::new(&[
            "find",
            "section_limit",
            "section_offset",
        ]));
    }
    let Some(headings) = headings else {
        return Err(QueryInvalid::new(&["find", "label_section"]));
    };
    let names = headings
        .as_str()
        .split(',')
        .map(str::trim)
        .collect::<Vec<_>>();
    if names.len() > MAX_SECTIONS || names.iter().any(|name| name.is_empty()) {
        return Err(QueryInvalid::new(&["label_section"]));
    }
    if finds.iter().any(|term| term.as_str().chars().count() > 150) {
        return Err(QueryInvalid::new(&["find"]));
    }
    Ok(())
}

pub(super) fn batch(
    input: &Text,
    finds: &[Text],
    sections: &[DrugSection],
) -> Result<Option<Box<[DrugId]>>, QueryInvalid> {
    if !requested(sections, finds) || !input.as_str().contains(',') {
        return Ok(None);
    }
    let ids = input
        .as_str()
        .split(',')
        .map(|id| id.trim().parse::<DrugId>())
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| QueryInvalid::new(&["input"]))?;
    if ids.len() > MAX_LABELS || ids.iter().any(|id| !DrugSource::OpenfdaLabel.accepts(id)) {
        return Err(QueryInvalid::new(&["input"]));
    }
    let mut seen = std::collections::BTreeSet::new();
    if !ids
        .iter()
        .all(|id| seen.insert(id.to_string().to_ascii_lowercase()))
    {
        return Err(QueryInvalid::new(&["input"]));
    }
    Ok(Some(ids.into()))
}

/// Bounded excerpts from one readable current-label section.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct DrugLabelMatches {
    /// Matching paragraphs or table rows; an empty list means the readable section had no match.
    pub excerpts: Box<[DrugLabelExcerpt]>,
    /// The section had more matching paragraphs or rows than this reply carries.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub truncated: bool,
}

/// One paragraph or table row containing the requested literal text.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct DrugLabelExcerpt {
    /// Wording around the match, with visible ellipses where clipped.
    pub text: Box<str>,
    /// Column headings from the same table, when preserved by the source.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub table_header: Option<Box<str>>,
    /// Caption from the same table, when preserved by the source.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub table_caption: Option<Box<str>>,
}
