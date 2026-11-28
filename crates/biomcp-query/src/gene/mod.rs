//! HGNC identity resolution preserving source ambiguity and withdrawn identity.
mod resolve;
pub(crate) use resolve::resolve;

pub(crate) mod get;
mod next;
pub(crate) mod search;

mod sections;

/// HGNC-approved protein-coding genes can lack UniProt cross-references.
pub(crate) fn needs_uniprot_lookup(gene: &biodata::Gene) -> bool {
    gene.uniprot.is_empty() && gene.locus_group == Some(biodata::LocusGroup::ProteinCoding)
}
