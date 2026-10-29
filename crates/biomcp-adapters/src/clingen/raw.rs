use crate::native_text::NativeText;
use biodata::{HgncId, Text};

use biomcp_fetch::{KeyKind, Keys, RequestError, Row};
use biomcp_types::ClinGenValidity;
use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub(super) struct Validity {
    #[serde(rename = "GENE ID (HGNC)")]
    id: HgncId,
    #[serde(rename = "DISEASE LABEL")]
    disease: Option<NativeText>,
    #[serde(rename = "DISEASE ID (MONDO)")]
    mondo_id: Option<biodata::MondoId>,
    #[serde(rename = "MOI")]
    mode_of_inheritance: Option<NativeText>,
    #[serde(rename = "CLASSIFICATION")]
    classification: Option<Text>,
    #[serde(rename = "CLASSIFICATION DATE")]
    classification_date: Option<Text>,
    #[serde(rename = "GCEP")]
    expert_panel: Option<NativeText>,
    #[serde(rename = "ONLINE REPORT")]
    report_url: Option<Text>,
}
impl Validity {
    pub(super) fn checked(self) -> Result<ClinGenValidity, RequestError> {
        Ok(ClinGenValidity {
            disease: self.disease.ok_or_else(super::bad)?.into(),
            mondo_id: self.mondo_id.ok_or_else(super::bad)?,
            mode_of_inheritance: self.mode_of_inheritance.ok_or_else(super::bad)?.into(),
            classification: self.classification.ok_or_else(super::bad)?,
            classification_date: self.classification_date.ok_or_else(super::bad)?,
            expert_panel: self.expert_panel.ok_or_else(super::bad)?.into(),
            report_url: self.report_url.ok_or_else(super::bad)?,
        })
    }
}
impl Row for Validity {
    fn keys(&self) -> Keys {
        keys(self.id)
    }
}
#[derive(Debug, Deserialize)]
pub(super) struct Dosage {
    #[serde(rename = "HGNC ID")]
    id: HgncId,
    #[serde(rename = "HAPLOINSUFFICIENCY")]
    pub haploinsufficiency: Option<Text>,
    #[serde(rename = "TRIPLOSENSITIVITY")]
    pub triplosensitivity: Option<Text>,
    #[serde(rename = "DATE")]
    pub date: Option<Text>,
}
impl Row for Dosage {
    fn keys(&self) -> Keys {
        keys(self.id)
    }
}
fn keys(id: HgncId) -> Keys {
    Keys {
        exact: vec![(KeyKind::Id, id.to_string().into())],
        words: Vec::new(),
    }
}
