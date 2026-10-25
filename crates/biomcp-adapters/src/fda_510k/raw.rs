use biomcp_fetch::{BadReply, Reply};
use serde::Deserialize;
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub(super) enum Response {
    Page { meta: Meta, results: Vec<Record> },
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
impl Reply for Response {
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
    pub k_number: biodata::DiagnosticDeviceId,
    pub device_name: Option<Box<str>>,
    pub applicant: Option<Box<str>>,
    pub decision_code: Option<Box<str>>,
    pub decision_description: Option<Box<str>>,
    pub decision_date: Option<Box<str>>,
    pub date_received: Option<Box<str>>,
    pub product_code: Option<Box<str>>,
}
impl Record {
    pub(super) fn mapped(self) -> biomcp_types::DiagnosticClearance {
        biomcp_types::DiagnosticClearance {
            k_number: self.k_number,
            device_name: self.device_name,
            applicant: self.applicant,
            decision_code: self.decision_code,
            decision_description: self.decision_description,
            decision_date: self.decision_date,
            date_received: self.date_received,
            product_code: self.product_code,
        }
    }
}
