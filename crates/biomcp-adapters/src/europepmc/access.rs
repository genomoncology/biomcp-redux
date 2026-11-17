//! Open-access flags and licenses Europe PMC states for a page of PubMed papers.
use super::{SEARCH, yes_no};
use crate::{
    Mapped,
    mapped::{Problems, record},
    pmc_oa,
    raw::Field,
};
use biodata::{License, Pmid};
use biomcp_fetch::{Arg, BadReply, Input, Leg, Read, Reply, RequestError};
use biomcp_types::Source;
use serde::Deserialize;
use serde_json::Value;

/// Europe PMC's largest search page; a search page holds at most 50 papers.
const MAX_IDS: usize = 100;
/// The only row fields kept, so no abstract text reaches the cache or a recording.
const KEPT: [&str; 4] = ["source", "pmid", "isOpenAccess", "license"];

/// What Europe PMC states about one paper's access.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Access {
    /// The paper.
    pub pmid: Pmid,
    /// Europe PMC's open-access flag, when it states one.
    pub open_access: Option<bool>,
    /// The license Europe PMC states, when it states one.
    pub license: Option<License>,
}

/// One core search for the given PubMed papers; a paper Europe PMC lacks is left out.
pub fn access(leg: &Leg<'_>, ids: &[Pmid]) -> Result<Mapped<Vec<Access>>, RequestError> {
    let mut wanted: Vec<Pmid> = Vec::new();
    for id in ids {
        if !wanted.contains(id) && wanted.len() < MAX_IDS {
            wanted.push(*id);
        }
    }
    if wanted.is_empty() {
        return Ok(Mapped {
            value: Vec::new(),
            problems: Vec::new(),
            refusals: crate::Refusal::None,
        });
    }
    let names: Vec<String> = wanted.iter().map(|id| format!("EXT_ID:{id}")).collect();
    let query = format!("SRC:MED AND ({})", names.join(" OR "));
    let raw = leg
        .call(
            &SEARCH,
            Input::Params(&[
                ("query", Arg::Text(&query)),
                ("format", Arg::Text("json")),
                ("resultType", Arg::Text("core")),
                ("pageSize", Arg::Num(MAX_IDS as u64)),
            ]),
            Read::<Raw>::json().filter_body(&fields),
        )?
        .value;
    let mut problems = Vec::new();
    let mut found = Vec::new();
    for row in raw.result_list.result.into_iter().filter_map(Field::value) {
        let Some(pmid) = row.pmid.and_then(Field::value) else {
            continue;
        };
        if row.source != "MED" || !wanted.contains(&pmid) || found.iter().any(|a: &Access| a.pmid == pmid) {
            continue;
        }
        let mut p = Problems {
            source: Source::EuropePmc,
            record: record(pmid),
            list: Vec::new(),
        };
        let open_access = match p.field("open_access", row.is_open_access) {
            Some(flag) => p.required("open_access", yes_no(&flag)),
            None => None,
        };
        let license = p
            .field("license", row.license)
            .filter(|code| !code.trim().is_empty())
            .map(|code| pmc_oa::license(&code));
        problems.extend(p.list);
        found.push(Access {
            pmid,
            open_access,
            license,
        });
    }
    Ok(Mapped {
        value: found,
        problems,
        refusals: crate::Refusal::None,
    })
}

/// Keep the hit count and each row's access fields; drop everything else.
fn fields(body: &[u8]) -> Result<Vec<u8>, BadReply> {
    let root: Value = serde_json::from_slice(body).map_err(|_| BadReply)?;
    let hits = root.get("hitCount").cloned().ok_or(BadReply)?;
    let rows = root
        .pointer("/resultList/result")
        .and_then(Value::as_array)
        .ok_or(BadReply)?;
    let rows: Vec<Value> = rows
        .iter()
        .map(|row| {
            let mut kept = serde_json::Map::new();
            if let Some(row) = row.as_object() {
                for name in KEPT {
                    if let Some(value) = row.get(name) {
                        kept.insert(name.to_owned(), value.clone());
                    }
                }
            }
            Value::Object(kept)
        })
        .collect();
    let trimmed = serde_json::json!({
        "hitCount": hits,
        "resultList": { "result": rows },
    });
    serde_json::to_vec(&trimmed).map_err(|_| BadReply)
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Raw {
    hit_count: u64,
    result_list: Rows,
}
#[derive(Deserialize)]
struct Rows {
    result: Vec<Field<Row>>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Row {
    source: String,
    pmid: Option<Field<Pmid>>,
    is_open_access: Option<Field<String>>,
    license: Option<Field<String>>,
}
impl Reply for Raw {
    fn check(&self) -> Result<(), BadReply> {
        if self.result_list.result.len() > MAX_IDS
            || self.result_list.result.len() as u64 > self.hit_count
        {
            return Err(BadReply);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::fields;

    #[test]
    fn trimming_keeps_only_access_fields() {
        let body = br#"{"version":"6.9","hitCount":1,"resultList":{"result":[{"id":"1","source":"MED","pmid":"1","isOpenAccess":"Y","license":"cc by","abstractText":"secret"}]}}"#;
        let kept: serde_json::Value = serde_json::from_slice(&fields(body).unwrap()).unwrap();
        assert_eq!(
            kept,
            serde_json::json!({"hitCount":1,"resultList":{"result":[{"source":"MED","pmid":"1","isOpenAccess":"Y","license":"cc by"}]}})
        );
    }
}
