use crate::raw::Field;
use biodata::DrugRecallNumber;
use biomcp_fetch::{BadReply, Reply};
use serde::{Deserialize, de::DeserializeOwned};
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub(super) enum Response<T> {
    Page { meta: Meta, results: Vec<T> },
    Absent { error: Absence },
}
#[derive(Debug, Deserialize)]
pub(super) struct Meta {
    pub results: Counts,
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
impl<T: DeserializeOwned> Reply for Response<T> {
    fn check(&self) -> Result<(), BadReply> {
        match self {
            Self::Page { meta, results }
                if meta.results.limit > 0
                    && results.len() as u64
                        == meta
                            .results
                            .total
                            .saturating_sub(meta.results.skip.into())
                            .min(meta.results.limit.into()) =>
            {
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
            (Self::Page { .. }, 200) | (Self::Absent { .. }, 404)
        ) {
            Ok(())
        } else {
            Err(BadReply)
        }
    }
}
#[derive(Debug, Deserialize)]
pub(super) struct Record {
    pub recall_number: DrugRecallNumber,
    pub classification: Option<Field<String>>,
    pub status: Option<Field<String>>,
    pub recalling_firm: Option<Field<String>>,
    pub product_description: Option<Field<String>>,
    pub recall_initiation_date: Option<Field<String>>,
    pub report_date: Option<Field<String>>,
    pub reason_for_recall: Option<Field<String>>,
    pub distribution_pattern: Option<Field<String>>,
    pub code_info: Option<Field<String>>,
    pub voluntary_mandated: Option<Field<String>>,
}
