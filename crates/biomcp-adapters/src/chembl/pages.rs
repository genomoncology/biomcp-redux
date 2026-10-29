//! Preserve native page counts before refusing row identities.
use super::raw::{Indication, Mechanism, Molecule, PageMeta};
use biomcp_fetch::{BadReply, Reply};
use serde::Deserialize;
/// Native source page with independent row diagnostics.
#[derive(Debug, Deserialize)]
#[serde(from = "ReceivedMolecules")]
pub struct Molecules {
    /// Accepted native rows.
    pub molecules: Vec<Molecule>,
    /// Native page metadata.
    pub page_meta: PageMeta,
    /// Native count before local identity refusals.
    pub received: u32,
    /// Rejected row identities.
    pub problems: Vec<biomcp_types::Problem>,
}
#[derive(Deserialize)]
struct ReceivedMolecules {
    molecules: Vec<crate::raw::Field<Molecule>>,
    page_meta: PageMeta,
}
impl From<ReceivedMolecules> for Molecules {
    fn from(page: ReceivedMolecules) -> Self {
        let received = page.molecules.len() as u32;
        let mut problems = crate::mapped::Problems {
            source: biomcp_types::Source::Chembl,
            record: None,
            list: Vec::new(),
        };
        let mut rows = Vec::new();
        for row in page.molecules {
            problems.record = None;
            if let Some(row) = problems.field("molecule_chembl_id", Some(row)) {
                if row.molecule_chembl_id.starts_with("CHEMBL")
                    && row.molecule_chembl_id.parse::<biodata::DrugId>().is_ok()
                {
                    rows.push(row);
                } else {
                    problems.record = crate::mapped::record(&row.molecule_chembl_id);
                    problems.required::<()>("molecule_chembl_id", None);
                }
            }
        }
        Self {
            molecules: rows,
            page_meta: page.page_meta,
            received,
            problems: problems.list,
        }
    }
}
impl Reply for Molecules {
    fn check(&self) -> Result<(), BadReply> {
        Ok(())
    }
}
/// Native source page with independent row diagnostics.
#[derive(Debug, Deserialize)]
#[serde(from = "ReceivedMechanisms")]
pub struct Mechanisms {
    pub(super) molecules: Vec<String>,
    /// Accepted native rows.
    pub mechanisms: Vec<Mechanism>,
    /// Native page metadata.
    pub page_meta: PageMeta,
    /// Native count before local identity refusals.
    pub received: u32,
    /// Rejected row identities.
    pub problems: Vec<biomcp_types::Problem>,
}
#[derive(Deserialize)]
struct ReceivedMechanisms {
    mechanisms: Vec<super::scope::Row<Mechanism>>,
    page_meta: PageMeta,
}
impl From<ReceivedMechanisms> for Mechanisms {
    fn from(page: ReceivedMechanisms) -> Self {
        let received = page.mechanisms.len() as u32;
        let mut problems = crate::mapped::Problems {
            source: biomcp_types::Source::Chembl,
            record: None,
            list: Vec::new(),
        };
        let mut rows = Vec::new();
        let mut molecules = Vec::new();
        for row in page.mechanisms {
            molecules.push(row.scope.molecule_chembl_id);
            problems.record = None;
            if let Some(row) = problems.field("mec_id", Some(row.value)) {
                if row.mec_id > 0 {
                    rows.push(row);
                } else {
                    problems.record = crate::mapped::record(row.mec_id);
                    problems.required::<()>("mec_id", None);
                }
            }
        }
        Self {
            molecules,
            mechanisms: rows,
            page_meta: page.page_meta,
            received,
            problems: problems.list,
        }
    }
}
impl Reply for Mechanisms {
    fn check(&self) -> Result<(), BadReply> {
        Ok(())
    }
}
/// Native source page with independent row diagnostics.
#[derive(Debug, Deserialize)]
#[serde(from = "ReceivedIndications")]
pub struct Indications {
    pub(super) molecules: Vec<String>,
    /// Accepted native rows.
    pub drug_indications: Vec<Indication>,
    /// Native page metadata.
    pub page_meta: PageMeta,
    /// Native count before local identity refusals.
    pub received: u32,
    /// Rejected row identities.
    pub problems: Vec<biomcp_types::Problem>,
}
#[derive(Deserialize)]
struct ReceivedIndications {
    drug_indications: Vec<super::scope::Row<Indication>>,
    page_meta: PageMeta,
}
impl From<ReceivedIndications> for Indications {
    fn from(page: ReceivedIndications) -> Self {
        let received = page.drug_indications.len() as u32;
        let mut problems = crate::mapped::Problems {
            source: biomcp_types::Source::Chembl,
            record: None,
            list: Vec::new(),
        };
        let mut rows = Vec::new();
        let mut molecules = Vec::new();
        for row in page.drug_indications {
            molecules.push(row.scope.molecule_chembl_id);
            problems.record = None;
            if let Some(row) = problems.field("drugind_id", Some(row.value)) {
                if row.drugind_id > 0 {
                    rows.push(row);
                } else {
                    problems.record = crate::mapped::record(row.drugind_id);
                    problems.required::<()>("drugind_id", None);
                }
            }
        }
        Self {
            molecules,
            drug_indications: rows,
            page_meta: page.page_meta,
            received,
            problems: problems.list,
        }
    }
}
impl Reply for Indications {
    fn check(&self) -> Result<(), BadReply> {
        Ok(())
    }
}
