use biodata::License;
use biodata::{NctId, PartialDate, Pmid, Text, TrialDocument, TrialReference};

use biomcp_fetch::BadReply;
use serde::Deserialize;
#[derive(Deserialize)]
pub(super) struct References {
    pub(super) references: Option<Box<[Reference]>>,
}
#[derive(Deserialize)]
pub(super) struct Reference {
    pmid: Option<Pmid>,
    #[serde(rename = "type")]
    kind: Box<str>,
    citation: Box<str>,
}
impl Reference {
    pub(super) fn mapped(self) -> TrialReference {
        TrialReference {
            pmid: self.pmid,
            kind: self.kind,
            citation: self.citation,
        }
    }
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Documents {
    large_document_module: Option<LargeDocuments>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct LargeDocuments {
    large_docs: Option<Box<[Document]>>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Document {
    filename: Box<str>,
    type_abbrev: Box<str>,
    label: Option<Box<str>>,
    date: Option<PartialDate>,
    upload_date: Option<Box<str>>,
    size: Option<u64>,
    has_protocol: Option<bool>,
    has_sap: Option<bool>,
    has_icf: Option<bool>,
}
impl Documents {
    fn rows(&self) -> &[Document] {
        self.large_document_module
            .as_ref()
            .and_then(|m| m.large_docs.as_deref())
            .unwrap_or(&[])
    }
    pub(super) fn check(&self) -> Result<(), BadReply> {
        if self.rows().iter().any(|d| !filename(&d.filename)) {
            return Err(BadReply);
        }
        Ok(())
    }
    pub(super) fn mapped(self, id: NctId) -> Result<Box<[TrialDocument]>, BadReply> {
        self.large_document_module
            .and_then(|m| m.large_docs)
            .unwrap_or_default()
            .into_vec()
            .into_iter()
            .map(|d| {
                // Only the validated source manifest supplies the filename. Caller text never enters this URL.
                let id = id.to_string();
                let suffix = id.get(9..).unwrap_or_default();
                Ok(TrialDocument {
                    url: Text::new(&format!(
                        "{}/large-docs/{suffix}/{id}/{}",
                        super::super::DOCUMENT_ORIGIN,
                        d.filename
                    ))
                    .map_err(|_| BadReply)?,
                    filename: Text::new(&d.filename).map_err(|_| BadReply)?,
                    kind: d.type_abbrev,
                    label: d.label,
                    date: d.date,
                    uploaded: d.upload_date,
                    size: d.size,
                    has_protocol: d.has_protocol,
                    has_sap: d.has_sap,
                    has_icf: d.has_icf,
                    license: License::Other,
                })
            })
            .collect()
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
