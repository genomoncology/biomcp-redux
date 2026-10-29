use biodata::{PartialDate, ReportId};
use biomcp_fetch::{BadReply, Reply};
use serde::Deserialize;
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub(super) enum Response {
    Page { meta: Meta, results: Vec<Header> },
    Absent { error: Absence },
}
#[derive(Debug, Deserialize)]
pub(super) struct Meta {
    pub last_updated: Option<crate::raw::Field<PartialDate>>,
    pub results: Counts,
}
#[derive(Debug, Deserialize)]
pub(super) struct Counts {
    pub skip: u32,
    pub limit: u32,
    pub total: u64,
}
#[derive(Debug, Deserialize)]
pub(super) struct Header {
    pub safetyreportid: ReportId,
    pub safetyreportversion: Option<Box<str>>,
    pub serious: Option<Box<str>>,
    pub receivedate: Option<Box<str>>,
}
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Absence {
    code: String,
    message: String,
}
impl Absence {
    pub(super) fn valid(&self) -> bool {
        self.code == "NOT_FOUND" && self.message == "No matches found!"
    }
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
