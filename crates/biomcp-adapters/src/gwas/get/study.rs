use biodata::{GwasStudyId, Pmid, Text};
use biomcp_fetch::{BadReply, Reply};
use biomcp_types::{GwasStudy, GwasTrait};
use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub(super) struct Study {
    pub accession_id: GwasStudyId,
    pubmed_id: Option<u64>,
    disease_trait: Text,
    efo_traits: Vec<super::super::raw::Trait>,
    initial_sample_size: Option<Text>,
    replication_sample_size: Option<Text>,
    discovery_ancestry: Vec<Text>,
    replication_ancestry: Vec<Text>,
    genotyping_technologies: Vec<Text>,
    snp_count: Option<u64>,
    platforms: Option<Text>,
    array_manufacturer: Option<Vec<Text>>,
    gxe: Option<bool>, gxg: Option<bool>, imputed: Option<bool>, pooled: Option<bool>, user_requested: Option<bool>,
    full_summary_stats_available: Option<bool>, full_summary_stats: Option<Text>, terms_of_license: Option<Text>,
}
impl Reply for Study {
    fn check(&self) -> Result<(), BadReply> {
        if self.pubmed_id.is_some_and(|n| n == 0) { Err(BadReply) } else { Ok(()) }
    }
}
impl Study {
    pub(super) fn card(self) -> GwasStudy {
        GwasStudy { id: self.accession_id,
            publication: self.pubmed_id.and_then(|n| n.to_string().parse::<Pmid>().ok()),
            reported_traits: vec![self.disease_trait].into(),
            ontology_traits: self.efo_traits.into_iter().map(|t| GwasTrait { id: t.efo_id, label: t.efo_trait }).collect(),
            initial_sample_description: self.initial_sample_size,
            replication_sample_description: self.replication_sample_size,
            discovery_ancestries: self.discovery_ancestry.into(), replication_ancestries: self.replication_ancestry.into(),
            genotyping_technologies: self.genotyping_technologies.into(), snp_count: self.snp_count,
            platforms: self.platforms, array_manufacturers: self.array_manufacturer.map(Into::into),
            gxe: self.gxe, gxg: self.gxg, imputed: self.imputed, pooled: self.pooled, user_requested: self.user_requested,
            summary_statistics_available: self.full_summary_stats_available,
            summary_statistics_url: self.full_summary_stats, license_url: self.terms_of_license }
    }
}
