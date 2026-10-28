use biodata::InterProId;
use biomcp_fetch::{BadReply, Reply};
use serde::Deserialize;
use std::collections::BTreeMap;
#[derive(Debug, Deserialize)]
pub(super) struct Page {
    pub count: u64,
    pub next: Option<String>,
    pub results: Vec<Entry>,
}
impl Reply for Page {
    fn check(&self) -> Result<(), BadReply> {
        if self.results.len() as u64 > self.count || self.results.len() > 25 {
            return Err(BadReply);
        }
        for row in &self.results {
            if row.metadata.source_database != "interpro" {
                return Err(BadReply);
            }
            for protein in &row.proteins {
                if protein.protein_length == Some(0) {
                    return Err(BadReply);
                }
                for location in &protein.entry_protein_locations {
                    if location.fragments.iter().any(|f| {
                        f.start == 0
                            || f.end < f.start
                            || protein.protein_length.is_some_and(|n| f.end > n)
                    }) {
                        return Err(BadReply);
                    }
                }
            }
        }
        Ok(())
    }
}
impl Page {
    pub(super) fn admitted(&self, id: &biodata::UniProtIsoform) -> bool {
        self.results.iter().all(|e| {
            !e.proteins.is_empty()
                && e.proteins.iter().all(|p| {
                    p.accession == id.as_str().to_ascii_lowercase()
                        && p.organism.as_deref() == Some("9606")
                })
        })
    }
}
#[derive(Debug, Deserialize)]
pub(super) struct Entry {
    pub metadata: Metadata,
    pub proteins: Vec<Protein>,
}
#[derive(Debug, Deserialize)]
pub(super) struct Metadata {
    pub accession: InterProId,
    pub name: Option<crate::raw::Field<String>>,
    pub source_database: String,
    #[serde(rename = "type")]
    pub entry_type: Option<crate::raw::Field<String>>,
    pub member_databases: Option<crate::raw::Field<BTreeMap<String, crate::raw::Field<BTreeMap<String, crate::raw::Field<String>>>>>>>,
}
#[derive(Debug, Deserialize)]
pub(super) struct Protein {
    pub accession: String,
    pub protein_length: Option<u32>,
    pub organism: Option<String>,
    pub entry_protein_locations: Vec<Location>,
}
#[derive(Debug, Deserialize)]
pub(super) struct Location {
    pub fragments: Vec<Fragment>,
}
#[derive(Debug, Deserialize)]
pub(super) struct Fragment {
    pub start: u32,
    pub end: u32,
    #[serde(rename = "dc-status")]
    pub dc_status: Option<crate::raw::Field<String>>,
}
