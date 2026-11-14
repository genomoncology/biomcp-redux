//! E-utilities einfo, esearch and esummary replies read by the gene counts.
use biomcp_fetch::{BadReply, Reply};
use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Deserialize)]
pub(super) struct Info {
    pub(super) einforesult: InfoResult,
}
#[derive(Deserialize)]
pub(super) struct InfoResult {
    pub(super) dbinfo: Vec<DbInfo>,
}
#[derive(Deserialize)]
pub(super) struct DbInfo {
    pub(super) dbname: String,
    pub(super) dbbuild: String,
    pub(super) lastupdate: String,
}
impl Reply for Info {
    fn check(&self) -> Result<(), BadReply> {
        match self.einforesult.dbinfo.as_slice() {
            [info]
                if info.dbname == "clinvar"
                    && !info.dbbuild.trim().is_empty()
                    && !info.lastupdate.trim().is_empty() =>
            {
                Ok(())
            }
            _ => Err(BadReply),
        }
    }
    fn data_date(&self) -> Option<String> {
        self.einforesult
            .dbinfo
            .first()
            .map(|info| info.lastupdate.clone())
    }
}

#[derive(Deserialize)]
pub(super) struct Search {
    pub(super) esearchresult: Found,
}
#[derive(Deserialize)]
pub(super) struct Found {
    pub(super) count: String,
    pub(super) retstart: String,
    pub(super) idlist: Vec<String>,
    #[serde(rename = "ERROR")]
    pub(super) error: Option<serde::de::IgnoredAny>,
    pub(super) errorlist: Option<serde::de::IgnoredAny>,
}
impl Found {
    pub(super) fn total(&self) -> Result<u64, BadReply> {
        self.count.parse().map_err(|_| BadReply)
    }
    pub(super) fn ids(&self) -> Result<Vec<u64>, BadReply> {
        self.idlist
            .iter()
            .map(|id| id.parse().map_err(|_| BadReply))
            .collect()
    }
}
impl Reply for Search {
    /// An unknown property or status phrase arrives as an error list with a zero count.
    fn check(&self) -> Result<(), BadReply> {
        let found = &self.esearchresult;
        if found.error.is_some() || found.errorlist.is_some() {
            return Err(BadReply);
        }
        found.total()?;
        found.ids()?;
        found.retstart.parse::<u64>().map_err(|_| BadReply)?;
        Ok(())
    }
}

#[derive(Deserialize)]
pub(super) struct Summary {
    pub(super) result: SummaryResult,
}
#[derive(Deserialize)]
pub(super) struct SummaryResult {
    pub(super) uids: Vec<String>,
    #[serde(flatten)]
    pub(super) records: BTreeMap<String, SummaryRecord>,
}
#[derive(Deserialize)]
pub(super) struct SummaryRecord {
    pub(super) germline_classification: Germline,
}
#[derive(Deserialize)]
pub(super) struct Germline {
    pub(super) review_status: String,
}
impl Reply for Summary {
    fn check(&self) -> Result<(), BadReply> {
        let result = &self.result;
        if result.uids.len() != result.records.len()
            || result
                .uids
                .iter()
                .any(|uid| !result.records.contains_key(uid))
        {
            return Err(BadReply);
        }
        Ok(())
    }
}
