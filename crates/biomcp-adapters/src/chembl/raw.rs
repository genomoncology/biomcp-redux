//! Native ChEMBL records; source fields never become chemical or regulatory inference.
use biomcp_fetch::{BadReply, Reply};
use serde::Deserialize;
/// Native molecule identity returned by the molecule endpoint.
#[derive(Debug, Deserialize)]
pub struct Molecule {
    /// Exact published molecule ID.
    pub molecule_chembl_id: String,
    /// Native preferred name.
    pub pref_name: Option<String>,
    /// Native record type, including nonchemical therapies.
    pub molecule_type: String,
    /// Explicit source synonyms.
    #[serde(default)]
    pub molecule_synonyms: Vec<Synonym>,
    /// Explicit native hierarchy.
    pub molecule_hierarchy: Option<Hierarchy>,
}
/// One source alternate name.
#[derive(Debug, Deserialize)]
pub struct Synonym {
    /// Original name.
    pub molecule_synonym: String,
    /// Native name kind.
    pub syn_type: String,
}
/// Source molecule parent relationship.
#[derive(Debug, Deserialize)]
pub struct Hierarchy {
    /// Source parent ID, possibly the molecule itself.
    pub parent_chembl_id: String,
}
/// Source page metadata, independent of local row counts.
#[derive(Debug, Deserialize)]
pub struct PageMeta {
    /// Native total.
    pub total_count: u64,
    /// Native offset.
    pub offset: u32,
    /// Native page size.
    pub limit: u32,
}
/// Candidate page in source order.
#[derive(Debug, Deserialize)]
pub struct Molecules {
    /// Native candidate identities.
    pub molecules: Vec<Molecule>,
    /// Native counts and page position.
    pub page_meta: PageMeta,
}
/// Native target mechanism, which need not identify a gene.
#[derive(Debug, Deserialize)]
pub struct Mechanism {
    /// Source relation ID.
    pub mec_id: u64,
    /// Exact drug identity.
    pub molecule_chembl_id: String,
    /// Native target record, without an inferred gene mapping.
    pub target_chembl_id: Option<String>,
    /// Native action type.
    pub action_type: Option<String>,
    /// Complete mechanism wording.
    pub mechanism_of_action: String,
    /// Native source links.
    pub mechanism_refs: Vec<Reference>,
}
/// Native indication, independent of a regulatory authorization.
#[derive(Debug, Deserialize)]
pub struct Indication {
    /// Native indication ID.
    pub drugind_id: u64,
    /// Exact drug identity.
    pub molecule_chembl_id: String,
    /// Native disease ID.
    pub efo_id: Option<String>,
    /// Native disease name.
    pub efo_term: Option<String>,
    /// Native MeSH identity.
    pub mesh_id: Option<String>,
    /// Native MeSH wording.
    pub mesh_heading: Option<String>,
    /// Native clinical phase string, never an authorization.
    pub max_phase_for_ind: Option<String>,
    /// Native source links.
    pub indication_refs: Vec<Reference>,
}
/// Native source reference, retaining its original identifiers and URL.
#[derive(Debug, Deserialize)]
pub struct Reference {
    /// Original reference identity, including source-provided lists.
    pub ref_id: String,
    /// Original reference type.
    pub ref_type: String,
    /// Original source URL.
    pub ref_url: String,
}
/// Native mechanism page.
#[derive(Debug, Deserialize)]
pub struct Mechanisms {
    /// Source mechanism rows.
    pub mechanisms: Vec<Mechanism>,
    /// Source page counts.
    pub page_meta: PageMeta,
}
/// Native indication page.
#[derive(Debug, Deserialize)]
pub struct Indications {
    /// Source indication rows.
    pub drug_indications: Vec<Indication>,
    /// Source page counts.
    pub page_meta: PageMeta,
}
impl Reply for Molecule {
    fn check(&self) -> Result<(), BadReply> {
        if self.molecule_chembl_id.starts_with("CHEMBL") && !self.molecule_type.is_empty() {
            Ok(())
        } else {
            Err(BadReply)
        }
    }
}
impl Reply for Molecules {
    fn check(&self) -> Result<(), BadReply> {
        self.molecules.iter().try_for_each(Reply::check)
    }
}
impl Reply for Mechanisms {
    fn check(&self) -> Result<(), BadReply> {
        Ok(())
    }
}
impl Reply for Indications {
    fn check(&self) -> Result<(), BadReply> {
        Ok(())
    }
}
