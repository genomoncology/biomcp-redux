use crate::native_text::NativeText;
use biodata::{DrugId, Text};
use biomcp_fetch::{BadReply, Reply};
use serde::Deserialize;

/// A source SPL revision; the label fields retain their native array boundaries.
#[derive(Debug, Deserialize)]
#[serde(from = "super::wire::Record")]
pub struct Record {
    /// Native revision identifier, distinct from its SPL set.
    pub id: DrugId,
    /// Native SPL set identifier.
    pub set_id: DrugId,
    /// Source-stated revision number.
    pub version: Option<Text>,
    /// Native YYYYMMDD revision date.
    pub effective_time: Option<biodata::PartialDate>,
    /// Rejected native nonidentity fields.
    pub problems: Vec<biomcp_types::Problem>,
    pub(super) openfda: Names,
    pub(super) spl_product_data_elements: Option<Vec<String>>,
    pub(super) package_label_principal_display_panel: Option<Vec<String>>,
    pub(super) boxed_warning: Option<Vec<String>>,
    pub(super) dosage_and_administration: Option<Vec<String>>,
    pub(super) drug_interactions: Option<Vec<String>>,
    pub(super) indications_and_usage: Option<Vec<String>>,
    pub(super) warnings_and_cautions: Option<Vec<String>>,
}
#[derive(Debug, Deserialize)]
pub(super) struct Names {
    pub brand_name: Option<Vec<NativeText>>,
    pub generic_name: Option<Vec<NativeText>>,
    pub spl_set_id: Option<Vec<DrugId>>,
    pub application_number: Option<Vec<NativeText>>,
}
impl Record {
    /// Only native fields the consumer explicitly supports; absent remains absent.
    pub fn parts(&self, heading: &str) -> Option<&[String]> {
        match heading {
            "boxed_warning" => &self.boxed_warning,
            "dosage_and_administration" => &self.dosage_and_administration,
            "drug_interactions" => &self.drug_interactions,
            "indications_and_usage" => &self.indications_and_usage,
            "warnings_and_cautions" => &self.warnings_and_cautions,
            _ => return None,
        }
        .as_deref()
        .filter(|parts| !parts.is_empty())
    }
    pub(super) fn check(&self) -> Result<(), BadReply> {
        use biomcp_types::DrugSource;
        if !DrugSource::OpenfdaLabel.accepts(&self.id)
            || !DrugSource::OpenfdaLabel.accepts(&self.set_id)
            || self
                .openfda
                .spl_set_id
                .iter()
                .flatten()
                .any(|id| id != &self.set_id)
        {
            return Err(BadReply);
        }
        Ok(())
    }
}
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub(super) enum Response {
    Success { meta: Meta, results: Vec<Record> },
    Absent { error: Absence },
}
#[derive(Debug, Deserialize)]
pub(super) struct Meta {
    pub results: Counts,
    #[serde(rename = "last_updated")]
    _last_updated: Option<biodata::PartialDate>,
}
#[derive(Debug, Deserialize)]
pub(super) struct Counts {
    pub skip: u32,
    pub limit: u32,
    pub total: u64,
}
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Absence {
    code: String,
    message: String,
}
impl Reply for Response {
    fn check(&self) -> Result<(), BadReply> {
        match self {
            Self::Success { meta, results } => {
                let counts = &meta.results;
                if counts.limit == 0
                    || results.len() as u64
                        != counts
                            .total
                            .saturating_sub(counts.skip.into())
                            .min(counts.limit.into())
                {
                    return Err(BadReply);
                }
                for record in results {
                    record.check()?;
                }
                Ok(())
            }
            Self::Absent { error }
                if error.code == "NOT_FOUND" && error.message == "No matches found!" =>
            {
                Ok(())
            }
            _ => Err(BadReply),
        }
    }
    fn check_status(&self, status: u16) -> Result<(), BadReply> {
        if matches!(
            (self, status),
            (Self::Success { .. }, 200) | (Self::Absent { .. }, 404)
        ) {
            Ok(())
        } else {
            Err(BadReply)
        }
    }
}
