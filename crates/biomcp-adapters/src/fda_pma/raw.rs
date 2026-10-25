use biodata::DeviceSubmission;
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
    pub pma_number: biodata::PmaNumber,
    pub supplement_number: Option<biodata::PmaSupplementCell>,
    pub trade_name: Option<Box<str>>,
    pub applicant: Option<Box<str>>,
    pub generic_name: Option<Box<str>>,
    pub decision_date: Option<Box<str>>,
    pub decision_code: Option<Box<str>>,
    pub date_received: Option<Box<str>>,
    pub fed_reg_notice_date: Option<Box<str>>,
    pub advisory_committee: Option<Box<str>>,
    pub advisory_committee_description: Option<Box<str>>,
    pub product_code: Option<Box<str>>,
    pub supplement_type: Option<Box<str>>,
    pub supplement_reason: Option<Box<str>>,
    pub ao_statement: Option<Box<str>>,
    pub docket_number: Option<Box<str>>,
    pub expedited_review_flag: Option<Box<str>>,
}
impl Record {
    pub(super) fn mapped(self) -> DeviceSubmission {
        DeviceSubmission {
            pma_number: self.pma_number,
            supplement_number: self.supplement_number,
            trade_name: self.trade_name,
            applicant: self.applicant,
            generic_name: self.generic_name,
            decision_date: self.decision_date,
            decision_code: self.decision_code,
            date_received: self.date_received,
            fed_reg_notice_date: self.fed_reg_notice_date,
            advisory_committee: self.advisory_committee,
            advisory_committee_description: self.advisory_committee_description,
            product_code: self.product_code,
            supplement_type: self.supplement_type,
            supplement_reason: self.supplement_reason,
            ao_statement: self.ao_statement,
            docket_number: self.docket_number,
            expedited_review_flag: self.expedited_review_flag,
        }
    }
}
