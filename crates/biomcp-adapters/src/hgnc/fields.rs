use crate::mapped::push_problem;
use crate::mapped::record as record_text;
use biodata::{HgncId, LocusGroup, LocusType, Mane};

use biomcp_types::{Problem, Source};
use std::str::FromStr;

pub(super) struct Fields {
    pub id: Option<HgncId>,
    pub problems: Vec<Problem>,
}
impl Fields {
    pub(super) fn required<T>(&mut self, field: &str, value: Option<T>) -> Option<T> {
        if value.is_none() {
            push_problem(
                &mut self.problems,
                Source::Hgnc,
                self.id.and_then(|id| record_text(id)),
                field,
            );
        }
        value
    }
    pub(super) fn optional<T: FromStr>(&mut self, field: &str, value: &str) -> Option<T> {
        if value.is_empty() {
            return None;
        }
        self.required(field, value.parse().ok())
    }
    pub(super) fn list<T: FromStr>(&mut self, field: &str, value: &str) -> Box<[T]> {
        super::raw::parts(value)
            .filter_map(|v| self.optional(field, v))
            .collect()
    }
}
pub(super) fn group(value: &str) -> Option<LocusGroup> {
    Some(match value {
        "protein-coding gene" => LocusGroup::ProteinCoding,
        "non-coding RNA" => LocusGroup::NonCodingRna,
        "pseudogene" => LocusGroup::Pseudogene,
        "phenotype" => LocusGroup::Phenotype,
        "other" => LocusGroup::Other,
        _ => return None,
    })
}
pub(super) fn locus(value: &str) -> Option<LocusType> {
    Some(match value {
        "gene with protein product" => LocusType::GeneWithProteinProduct,
        "RNA, cluster" => LocusType::RnaCluster,
        "RNA, long non-coding" => LocusType::LongNonCodingRna,
        "RNA, micro" => LocusType::MicroRna,
        "RNA, misc" => LocusType::MiscRna,
        "RNA, ribosomal" => LocusType::RibosomalRna,
        "RNA, small nuclear" => LocusType::SmallNuclearRna,
        "RNA, small nucleolar" => LocusType::SmallNucleolarRna,
        "RNA, transfer" => LocusType::TransferRna,
        "RNA, vault" => LocusType::VaultRna,
        "RNA, Y" => LocusType::YRna,
        "phenotype only" => LocusType::PhenotypeOnly,
        "pseudogene" => LocusType::Pseudogene,
        "complex locus constituent" => LocusType::ComplexLocusConstituent,
        "endogenous retrovirus" => LocusType::EndogenousRetrovirus,
        "fragile site" => LocusType::FragileSite,
        "immunoglobulin gene" => LocusType::ImmunoglobulinGene,
        "immunoglobulin pseudogene" => LocusType::ImmunoglobulinPseudogene,
        "readthrough" => LocusType::Readthrough,
        "region" => LocusType::Region,
        "T cell receptor gene" => LocusType::TCellReceptorGene,
        "T cell receptor pseudogene" => LocusType::TCellReceptorPseudogene,
        "transposable element" => LocusType::TransposableElement,
        "unknown" => LocusType::Unknown,
        "virus integration site" => LocusType::VirusIntegrationSite,
        _ => return None,
    })
}
pub(super) fn mane(value: &str) -> Option<Mane> {
    let (ensembl, refseq) = value.split_once('|')?;
    Mane::new(
        biodata::ManeStatus::Select,
        refseq.parse().ok()?,
        ensembl.parse().ok()?,
    )
    .ok()
}
