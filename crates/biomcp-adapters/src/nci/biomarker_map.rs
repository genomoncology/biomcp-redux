//! Preserve native biomarker values with rejected optional fields disclosed.
use super::biomarker_raw::Page;
use crate::{Mapped, mapped::Problems};
use biomcp_types::{BiomarkerCard, BiomarkerHeader, Source};
pub(super) fn page(raw: Page) -> Mapped<Vec<BiomarkerCard>> {
    let mut problems = Vec::new();
    let value = raw
        .data
        .into_iter()
        .map(|row| {
            let mut p = Problems {
                source: Source::Nci,
                record: None,
                list: Vec::new(),
            };
            let value = BiomarkerCard {
                header: BiomarkerHeader {
                    codes: row.codes,
                    name: row.name,
                    r#type: row.r#type,
                    count: row.count,
                },
                synonyms: p.optional("synonyms", row.synonyms.map(|v| v.value().ok_or(()))),
                parent_ids: p.optional("parent_ids", row.parent_ids.map(|v| v.value().ok_or(()))),
                ancestor_ids: p.optional(
                    "ancestor_ids",
                    row.ancestor_ids.map(|v| v.value().ok_or(())),
                ),
                eligibility_criterion: p.optional(
                    "eligibility_criterion",
                    row.eligibility_criterion.map(|v| v.value().ok_or(())),
                ),
            };
            problems.extend(p.list);
            value
        })
        .collect();
    Mapped { value, problems }
}
