//! Admit native metadata independently from primary accessions.
use super::raw::{Accession, Comment, Name, Variation, Xref};
use biodata::{CellLineXref, PartialDate, Text};
use serde::Deserialize;
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(super) struct ReceivedRecord {
    pub accession_list: Vec<crate::raw::Field<Accession>>,
    pub name_list: Option<crate::raw::Field<Vec<crate::raw::Field<Name>>>>,
    pub category: Option<crate::raw::Field<Text>>,
    pub species_list: Option<crate::raw::Field<Vec<crate::raw::Field<CellLineXref>>>>,
    pub disease_list: Option<crate::raw::Field<Vec<crate::raw::Field<CellLineXref>>>>,
    pub age: Option<crate::raw::Field<Text>>,
    pub sex: Option<crate::raw::Field<biodata::CellLineSex>>,
    pub created: Option<crate::raw::Field<PartialDate>>,
    pub last_updated: Option<crate::raw::Field<PartialDate>>,
    pub entry_version: Option<crate::raw::Field<Text>>,
    pub comment_list: Option<crate::raw::Field<Vec<crate::raw::Field<Comment>>>>,
    pub xref_list: Option<crate::raw::Field<Vec<crate::raw::Field<Xref>>>>,
    pub sequence_variation_list: Option<crate::raw::Field<Vec<crate::raw::Field<Variation>>>>,
}
impl From<ReceivedRecord> for super::raw::Record {
    fn from(row: ReceivedRecord) -> Self {
        let mut problems = crate::mapped::Problems {
            source: biomcp_types::Source::Cellosaurus,
            record: row
                .accession_list
                .iter()
                .filter_map(crate::raw::Field::as_ref)
                .find(|a| a.kind.as_str() == "primary")
                .and_then(|a| crate::mapped::record(&a.value)),
            list: Vec::new(),
        };
        Self {
            accession_list: row.accession_list,
            name_list: problems.fields("name-list", row.name_list).into_vec(),
            category: problems.required_field("category", row.category),
            species_list: row
                .species_list
                .map(|v| crate::strict_input::Input(problems.fields("species-list", Some(v)))),
            disease_list: row
                .disease_list
                .map(|v| crate::strict_input::Input(problems.fields("disease-list", Some(v)))),
            age: problems.field("age", row.age),
            sex: problems.field("sex", row.sex),
            created: problems.field("created", row.created),
            last_updated: problems.field("last-updated", row.last_updated),
            entry_version: problems
                .field("entry-version", row.entry_version)
                .and_then(|v| {
                    problems
                        .optional(
                            "entry-version",
                            Some(v.as_str().parse::<std::num::NonZeroU32>()),
                        )
                        .map(|_| v)
                }),
            comment_list: row
                .comment_list
                .map(|v| problems.fields("comment-list", Some(v))),
            xref_list: row.xref_list.map(|v| problems.fields("xref-list", Some(v))),
            sequence_variation_list: row
                .sequence_variation_list
                .map(|v| problems.fields("sequence-variation-list", Some(v))),
            problems: problems.list,
        }
    }
}
