use biodata::License;
use biodata::{NctId, PartialDate, Pmid, Text, TrialDocument, TrialReference};

use crate::{mapped::Problems, raw::Field};
use serde::Deserialize;
#[derive(Deserialize)]
pub(super) struct References {
    pub(super) references: Option<Field<Vec<Field<Reference>>>>,
}
#[derive(Deserialize)]
pub(super) struct Reference {
    pmid: Option<Field<Pmid>>,
    #[serde(rename = "type")]
    kind: Option<Field<Box<str>>>,
    citation: Option<Field<Box<str>>>,
}
impl Reference {
    pub(super) fn mapped(self, problems: &mut Problems) -> TrialReference {
        TrialReference {
            pmid: problems.field("references.pmid", self.pmid),
            kind: problems.required_field("references.kind", self.kind),
            citation: problems.required_field("references.citation", self.citation),
        }
    }
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Documents {
    large_document_module: Option<Field<LargeDocuments>>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct LargeDocuments {
    large_docs: Option<Field<Vec<Field<Document>>>>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Document {
    filename: Option<Field<Box<str>>>,
    type_abbrev: Option<Field<Box<str>>>,
    label: Option<Field<Box<str>>>,
    date: Option<Field<PartialDate>>,
    upload_date: Option<Field<Box<str>>>,
    size: Option<Field<u64>>,
    has_protocol: Option<Field<bool>>,
    has_sap: Option<Field<bool>>,
    has_icf: Option<Field<bool>>,
}
impl References {
    pub(super) fn rows(self, problems: &mut Problems) -> Box<[TrialReference]> {
        problems
            .fields("references", self.references)
            .into_vec()
            .into_iter()
            .map(|v| v.mapped(problems))
            .collect()
    }
}
impl Documents {
    pub(super) fn mapped(self, id: NctId, problems: &mut Problems) -> Box<[TrialDocument]> {
        problems
            .field("largeDocumentModule", self.large_document_module)
            .map_or_else(Box::default, |m| problems.fields("largeDocs", m.large_docs))
            .into_vec()
            .into_iter()
            .filter_map(|d| d.mapped(id, problems))
            .collect()
    }
}
impl Document {
    fn mapped(self, id: NctId, problems: &mut Problems) -> Option<TrialDocument> {
        let name = problems.required_field("largeDocs.filename", self.filename)?;
        let name = problems.optional(
            "largeDocs.filename",
            Some(if filename(&name) {
                Text::new(&name).map_err(|_| ())
            } else {
                Err(())
            }),
        )?;
        // Only the validated source manifest supplies the filename.
        let id = id.to_string();
        let suffix = id.get(9..).unwrap_or_default();
        let url = problems.optional(
            "largeDocs.filename",
            Some(Text::new(&format!(
                "{}/large-docs/{suffix}/{id}/{name}",
                super::super::DOCUMENT_ORIGIN
            ))),
        )?;
        Some(TrialDocument {
            filename: name,
            url,
            kind: problems.required_field("largeDocs.typeAbbrev", self.type_abbrev),
            label: problems.field("largeDocs.label", self.label),
            date: problems.field("largeDocs.date", self.date),
            uploaded: problems.field("largeDocs.uploadDate", self.upload_date),
            size: problems.field("largeDocs.size", self.size),
            has_protocol: problems.field("largeDocs.hasProtocol", self.has_protocol),
            has_sap: problems.field("largeDocs.hasSap", self.has_sap),
            has_icf: problems.field("largeDocs.hasIcf", self.has_icf),
            license: License::Other,
        })
    }
}

fn filename(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 255
        && !value.starts_with('.')
        && value.ends_with(".pdf")
        && value
            .bytes()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, b'.' | b'-' | b'_'))
}
