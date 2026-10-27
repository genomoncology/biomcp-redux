use crate::raw::Field;
use biodata::Pmid;
use biomcp_fetch::{BadReply, Reply};
use serde::Deserialize;
use std::collections::BTreeMap;
#[derive(Debug, Deserialize)]
pub(super) struct Search {
    pub esearchresult: SearchResult,
}
#[derive(Debug, Deserialize)]
pub(super) struct SearchResult {
    #[serde(deserialize_with = "count")]
    pub count: u64,
    pub idlist: Vec<Pmid>,
    pub errorlist: Option<ErrorList>,
    #[serde(default)]
    pub warninglist: BTreeMap<String, Vec<String>>,
    #[serde(rename = "ERROR")]
    error: Option<serde::de::IgnoredAny>,
}
#[derive(Debug, Deserialize)]
pub(super) struct ErrorList {
    pub phrasesnotfound: Option<Vec<String>>,
}
impl Reply for Search {
    fn discovery(&self) -> Option<biomcp_fetch::Identities> {
        biomcp_fetch::Identities::bounded(
            "pmid",
            self.esearchresult
                .idlist
                .iter()
                .map(ToString::to_string)
                .collect(),
        )
    }
    fn check(&self) -> Result<(), BadReply> {
        let r = &self.esearchresult;
        let count = r.count;
        let unique: std::collections::BTreeSet<_> = r.idlist.iter().collect();
        if r.error.is_some()
            || r.idlist.len() > 9999
            || r.idlist.len() as u64 > count
            || unique.len() != r.idlist.len()
        {
            return Err(BadReply);
        }
        Ok(())
    }
}
#[derive(Debug, Deserialize)]
pub(super) struct Summary {
    pub result: BTreeMap<String, SummaryValue>,
}
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub(super) enum SummaryValue {
    Ids(Vec<String>),
    Row(Box<SummaryRow>),
    Invalid(serde::de::IgnoredAny),
}
#[derive(Debug, Deserialize)]
pub(super) struct SummaryRow {
    pub uid: Option<Field<Pmid>>,
    pub title: Option<Field<String>>,
    pub fulljournalname: Option<Field<String>>,
    pub pubdate: Option<Field<String>>,
    pub epubdate: Option<Field<String>>,
    pub pubtype: Option<Field<Vec<Field<String>>>>,
    pub authors: Option<Field<Vec<Field<Author>>>>,
    pub articleids: Option<Field<Vec<Field<ArticleId>>>>,
    pub error: Option<serde::de::IgnoredAny>,
}
#[derive(Debug, Deserialize)]
pub(super) struct Author {
    pub name: String,
}
#[derive(Debug, Deserialize)]
pub(super) struct ArticleId {
    pub idtype: String,
    pub value: String,
}
impl Reply for Summary {
    fn input_parameter() -> Option<(&'static str, &'static str)> {
        Some(("pmid", "id"))
    }
    fn check(&self) -> Result<(), BadReply> {
        match self.result.get("uids") {
            Some(SummaryValue::Ids(ids)) if ids.len() <= 50 => Ok(()),
            _ => Err(BadReply),
        }
    }
}

fn count<'de, D: serde::Deserializer<'de>>(d: D) -> Result<u64, D::Error> {
    String::deserialize(d)?
        .parse()
        .map_err(|_| serde::de::Error::custom("invalid PubMed count"))
}
