//! Map native scoring metadata without interpreting biological meaning.
use super::proto;
use biomcp_fetch::BadReply;
use biomcp_types::{
    AlphaGenomeBiosample, AlphaGenomeGene, AlphaGenomeOntology, AlphaGenomeTrack,
    AlphaGenomeVariant,
};
use serde::de::{DeserializeOwned, value::StrDeserializer};
fn native<T: DeserializeOwned>(name: &str) -> Result<T, BadReply> {
    T::deserialize(StrDeserializer::<serde::de::value::Error>::new(name)).map_err(|_| BadReply)
}
pub(super) fn variant(value: &proto::Variant) -> Result<AlphaGenomeVariant, BadReply> {
    Ok(AlphaGenomeVariant {
        chromosome: value.chromosome.parse().map_err(|_| BadReply)?,
        position: value.position,
        reference_bases: value.reference_bases.parse().map_err(|_| BadReply)?,
        alternate_bases: value.alternate_bases.parse().map_err(|_| BadReply)?,
    })
}
pub(super) fn gene(value: proto::GeneScorerMetadata) -> Result<AlphaGenomeGene, BadReply> {
    Ok(AlphaGenomeGene {
        gene_id: value.gene_id.parse().map_err(|_| BadReply)?,
        name: value.name,
        strand: value
            .strand
            .map(|v| {
                native(
                    proto::Strand::try_from(v)
                        .map_err(|_| BadReply)?
                        .as_str_name(),
                )
            })
            .transpose()?,
        kind: value.r#type,
        junction_start: value.junction_start,
        junction_end: value.junction_end,
    })
}
pub(super) fn track(value: proto::TrackMetadata) -> Result<AlphaGenomeTrack, BadReply> {
    if value.nonzero_mean.is_some_and(|v| !v.is_finite()) {
        return Err(BadReply);
    }
    let ontology_term = value
        .ontology_term
        .map(|v| {
            Ok(AlphaGenomeOntology {
                ontology_type: native(
                    proto::OntologyType::try_from(v.ontology_type)
                        .map_err(|_| BadReply)?
                        .as_str_name(),
                )?,
                id: v.id,
            })
        })
        .transpose()?;
    let biosample = value
        .biosample
        .map(|v| {
            Ok(AlphaGenomeBiosample {
                kind: native(
                    proto::BiosampleType::try_from(v.r#type)
                        .map_err(|_| BadReply)?
                        .as_str_name(),
                )?,
                name: v.name,
                stage: v.stage,
            })
        })
        .transpose()?;
    Ok(AlphaGenomeTrack {
        name: value.name,
        strand: native(
            proto::Strand::try_from(value.strand)
                .map_err(|_| BadReply)?
                .as_str_name(),
        )?,
        ontology_term,
        biosample,
        assay: value.assay,
        histone_mark_code: value.histone_mark_code,
        transcription_factor_code: value.transcription_factor_code,
        gtex_tissue: value.gtex_tissue,
        data_source: value.data_source,
        endedness: value
            .endedness
            .map(|v| {
                native(
                    proto::Endedness::try_from(v)
                        .map_err(|_| BadReply)?
                        .as_str_name(),
                )
            })
            .transpose()?,
        genetically_modified: value.genetically_modified,
        nonzero_mean: value.nonzero_mean.map(f64::from),
    })
}
