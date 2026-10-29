//! Admit native candidate and publication identities without discarding optional labels.
use crate::{Mapped, mapped::Problems, raw::Field};
use biodata::Text;
use biomcp_fetch::{BadReply, PathSegment, Reply};
use biomcp_types::Source;
use serde::Deserialize;

#[derive(Deserialize)]
pub(super) struct Candidates(Vec<Field<Candidate>>);
#[derive(Deserialize)]
struct Candidate {
    #[serde(rename = "_id", alias = "id")]
    id: String,
    rsid: Option<Text>,
    gene: Option<Field<Vec<Field<Text>>>>,
    #[serde(deserialize_with = "crate::native_text::deserialize")]
    name: Text,
    pmids_count: u64,
}
impl Candidate {
    fn admitted(&self) -> bool {
        PathSegment::new(&self.id).is_ok()
            && self
                .rsid
                .as_ref()
                .is_none_or(|id| super::is_rsid(id.as_str()))
    }
}
impl Candidates {
    pub(super) fn eligible(&self) -> bool {
        self.0
            .iter()
            .all(|row| row.as_ref().is_some_and(Candidate::admitted))
    }
    pub(super) fn mapped(self) -> Mapped<Vec<super::Candidate>> {
        let mut problems = Problems {
            source: Source::LitVar,
            record: None,
            list: Vec::new(),
        };
        let mut value = Vec::new();
        for row in self.0 {
            problems.record = None;
            let Some(row) = problems.field("identity", Some(row)) else {
                continue;
            };
            problems.record = crate::mapped::record(&row.id);
            if !row.admitted() {
                problems.required::<()>("identity", None);
                continue;
            }
            value.push(super::Candidate {
                id: row.id,
                rsid: row.rsid,
                gene: problems.fields("gene", row.gene).into_vec(),
                name: row.name,
                pmids_count: row.pmids_count,
            });
        }
        Mapped {
            value,
            problems: problems.list,
        }
    }
}
impl Reply for Candidates {
    fn check(&self) -> Result<(), BadReply> {
        if self.0.len() > 10_000 {
            return Err(BadReply);
        }
        Ok(())
    }
}
#[derive(Deserialize)]
pub(super) struct Publications {
    pub(super) pmids: Vec<Field<u32>>,
    pub(super) pmids_count: u64,
}
impl Publications {
    pub(super) fn eligible(&self) -> bool {
        self.pmids
            .iter()
            .all(|id| id.as_ref().is_some_and(|id| *id > 0))
    }
}
impl Reply for Publications {
    fn discovery(&self) -> Option<biomcp_fetch::Identities> {
        biomcp_fetch::Identities::bounded(
            "pmid",
            self.pmids
                .iter()
                .filter_map(Field::as_ref)
                .map(ToString::to_string)
                .collect(),
        )
    }
    fn check(&self) -> Result<(), BadReply> {
        if self.pmids.len() as u64 > self.pmids_count {
            return Err(BadReply);
        }
        Ok(())
    }
}
