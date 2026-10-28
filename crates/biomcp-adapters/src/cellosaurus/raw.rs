use biodata::{CellLineXref, CellosaurusAccession, PartialDate, Text};
use biomcp_fetch::{BadReply, Reply};
use serde::Deserialize;
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub(super) enum Response {
    Records {
        #[serde(rename = "Cellosaurus")]
        data: Data,
    },
    Missing {
        code: u16,
        message: String,
    },
}
#[derive(Debug, Deserialize)]
pub(super) struct Data {
    #[serde(rename = "cell-line-list")]
    pub rows: Vec<Record>,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(super) struct Record {
    pub accession_list: Vec<Accession>,
    pub name_list: Vec<Name>,
    pub category: Text,
    pub species_list: Option<Box<[CellLineXref]>>,
    pub disease_list: Option<Box<[CellLineXref]>>,
    pub age: Option<Text>,
    pub sex: Option<biodata::CellLineSex>,
    pub created: Option<PartialDate>,
    pub last_updated: Option<PartialDate>,
    pub entry_version: Option<Text>,
    pub comment_list: Option<Box<[Comment]>>,
    pub xref_list: Option<Box<[Xref]>>,
    pub sequence_variation_list: Option<Box<[Variation]>>,
}
// Native xrefs can carry additional publisher fields, unlike the public projection.
#[derive(Debug, Deserialize)]
pub(super) struct Xref {
    pub database: Text,
    pub accession: Text,
    pub label: Option<Text>,
    pub url: Option<Text>,
    pub category: Option<Text>,
    pub iri: Option<Text>,
    pub discontinued: Option<Text>,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(super) struct Variation {
    pub variation_type: Text,
    pub mutation_type: Option<Text>,
    pub mutation_description: Option<Text>,
    pub variation_note: Option<Text>,
    pub zygosity_type: Option<Text>,
    pub xref_list: Option<Box<[Xref]>>,
    pub source_list: Option<Box<[Provenance]>>,
}
#[derive(Debug, Deserialize)]
pub(super) struct Provenance {
    pub reference: Option<Reference>,
    pub institution: Option<Text>,
    pub value: Option<Text>,
    pub xref: Option<Xref>,
}
#[derive(Debug, Deserialize)]
pub(super) struct Reference {
    #[serde(rename = "resource-internal-ref")]
    pub resource_internal_ref: Text,
}
#[derive(Debug, Deserialize)]
pub(super) struct Accession {
    #[serde(rename = "type")]
    pub kind: Text,
    pub value: CellosaurusAccession,
}
#[derive(Debug, Deserialize)]
pub(super) struct Name {
    #[serde(rename = "type")]
    pub kind: Text,
    pub value: Text,
}
#[derive(Debug, Deserialize)]
pub(super) struct Comment {
    pub category: Text,
    pub value: Text,
}
impl Record {
    pub(super) fn accession(&self) -> Result<&CellosaurusAccession, BadReply> {
        let mut rows = self
            .accession_list
            .iter()
            .filter(|a| a.kind.as_str() == "primary");
        let first = rows.next().ok_or(BadReply)?;
        if rows.next().is_some() {
            return Err(BadReply);
        }
        Ok(&first.value)
    }
    pub(super) fn name(&self) -> Result<&Text, BadReply> {
        let mut rows = self
            .name_list
            .iter()
            .filter(|a| a.kind.as_str() == "identifier");
        let first = rows.next().ok_or(BadReply)?;
        if rows.next().is_some() {
            return Err(BadReply);
        }
        Ok(&first.value)
    }
}
impl Reply for Response {
    fn check(&self) -> Result<(), BadReply> {
        match self {
            Self::Records { data } => {
                for row in &data.rows {
                    row.accession()?;
                    row.name()?;
                    if row
                        .entry_version
                        .as_ref()
                        .is_some_and(|v| v.as_str().parse::<std::num::NonZeroU32>().is_err())
                    {
                        return Err(BadReply);
                    }
                }
                Ok(())
            }
            Self::Missing { code: 404, message }
                if message.starts_with("Item not found, ac: CVCL_") =>
            {
                Ok(())
            }
            _ => Err(BadReply),
        }
    }
    fn check_status(&self, status: u16) -> Result<(), BadReply> {
        match (self, status) {
            (Self::Records { .. }, 200) | (Self::Missing { code: 404, .. }, 404) => Ok(()),
            _ => Err(BadReply),
        }
    }
}
#[derive(Debug, Deserialize)]
pub(super) struct ReleaseReply {
    #[serde(rename = "Cellosaurus")]
    pub data: ReleaseData,
}
#[derive(Debug, Deserialize)]
pub(super) struct ReleaseData {
    pub header: ReleaseHeader,
}
#[derive(Debug, Deserialize)]
pub(super) struct ReleaseHeader {
    pub release: biomcp_types::CellLineRelease,
}
impl Reply for ReleaseReply {
    fn check(&self) -> Result<(), BadReply> {
        Ok(())
    }
}
