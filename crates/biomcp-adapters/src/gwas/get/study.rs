use crate::native_text::NativeText;
use biodata::{GwasStudyId, Pmid, Text};
use biomcp_fetch::{BadReply, Reply};
use biomcp_types::{GwasStudy, GwasTrait};
use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub(super) struct Study {
    pub accession_id: GwasStudyId,
    pubmed_id: Option<u64>,
    #[serde(deserialize_with = "crate::native_text::deserialize")]
    disease_trait: biodata::Text,
    efo_traits: Vec<super::super::raw::Trait>,
    initial_sample_size: Option<NativeText>,
    replication_sample_size: Option<NativeText>,
    discovery_ancestry: Vec<NativeText>,
    replication_ancestry: Vec<NativeText>,
    genotyping_technologies: Vec<Text>,
    snp_count: Option<u64>,
    platforms: Option<NativeText>,
    array_manufacturer: Option<Vec<NativeText>>,
    gxe: Option<bool>,
    gxg: Option<bool>,
    imputed: Option<bool>,
    pooled: Option<bool>,
    user_requested: Option<bool>,
    full_summary_stats_available: Option<bool>,
    full_summary_stats: Option<Text>,
    terms_of_license: Option<Text>,
}
impl Reply for Study {
    fn check(&self) -> Result<(), BadReply> {
        if self.pubmed_id.is_some_and(|n| n == 0) {
            Err(BadReply)
        } else {
            Ok(())
        }
    }
}
impl Study {
    pub(super) fn card(self) -> GwasStudy {
        GwasStudy {
            id: self.accession_id,
            publication: self
                .pubmed_id
                .and_then(|n| n.to_string().parse::<Pmid>().ok()),
            reported_traits: vec![self.disease_trait].into(),
            ontology_traits: self
                .efo_traits
                .into_iter()
                .map(|t| GwasTrait {
                    id: t.efo_id,
                    label: t.efo_trait,
                })
                .collect(),
            initial_sample_description: self.initial_sample_size.map(Into::into),
            replication_sample_description: self.replication_sample_size.map(Into::into),
            discovery_ancestries: self
                .discovery_ancestry
                .into_iter()
                .map(Into::into)
                .collect(),
            replication_ancestries: self
                .replication_ancestry
                .into_iter()
                .map(Into::into)
                .collect(),
            genotyping_technologies: self.genotyping_technologies.into(),
            snp_count: self.snp_count,
            platforms: self.platforms.map(Into::into),
            array_manufacturers: self
                .array_manufacturer
                .map(|values| values.into_iter().map(Into::into).collect()),
            gxe: self.gxe,
            gxg: self.gxg,
            imputed: self.imputed,
            pooled: self.pooled,
            user_requested: self.user_requested,
            summary_statistics_available: self.full_summary_stats_available,
            summary_statistics_url: self.full_summary_stats,
            license_url: self.terms_of_license,
        }
    }
}
