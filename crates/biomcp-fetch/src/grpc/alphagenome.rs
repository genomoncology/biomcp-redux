//! Encode the reviewed human DNASE request without exposing generated provider types.
mod envelope;
use super::{Limits, proto};
use crate::{BadReply, InputError};
pub use envelope::Envelope;
use prost::Message;

/// Pinned publisher RPC path; the API version is not a model release.
pub const PREDICT_VARIANT: &str =
    "/google.gdm.gdmscience.alphagenome.v1main.DnaModelService/PredictVariant";
/// Pinned protobuf identity used by checked cache and recording admission.
pub const SCHEMA: &str =
    "alphagenome-55943112b83439e0c2512110f1ae607d4b5b492a-predict-variant-dnase";

/// A validated explicit request, with one-based variant and zero-based half-open interval.
/// This request selects human DNASE, one UBERON term and the ALL_FOLDS checkpoint.
#[derive(Debug)]
pub struct DnaseRequest {
    request: proto::PredictVariantRequest,
    bytes: Vec<u8>,
}
impl DnaseRequest {
    /// Encode the selected request shape. Coordinates and alleles are supplied unchanged.
    /// The caller retains its assembly provenance; the publisher has no build request field.
    pub fn human_all_folds(
        chromosome: &str,
        interval: std::ops::Range<i64>,
        position: i64,
        reference: &str,
        alternate: &str,
        uberon: i64,
    ) -> Result<Self, InputError> {
        let width = interval
            .end
            .checked_sub(interval.start)
            .ok_or(InputError::Path)?;
        let reference_len = i64::try_from(reference.len()).map_err(|_| InputError::BodyLimit)?;
        let variant_end = position
            .checked_sub(1)
            .and_then(|p| p.checked_add(reference_len));
        if chromosome.is_empty()
            || chromosome
                .len()
                .saturating_add(reference.len())
                .saturating_add(alternate.len())
                > 4096
            || interval.start < 0
            || ![16384, 131072, 524288, 1048576].contains(&width)
            || position <= interval.start
            || variant_end.is_none_or(|end| end > interval.end)
            || reference.is_empty()
            || alternate.is_empty()
            || !reference
                .bytes()
                .chain(alternate.bytes())
                .all(|b| b"ACGTN".contains(&b))
            || uberon <= 0
        {
            return Err(InputError::Path);
        }
        let request = proto::PredictVariantRequest {
            interval: Some(proto::Interval {
                chromosome: chromosome.into(),
                start: interval.start,
                end: interval.end,
                strand: proto::Strand::Unstranded as i32,
            }),
            variant: Some(proto::Variant {
                chromosome: chromosome.into(),
                position,
                reference_bases: reference.into(),
                alternate_bases: alternate.into(),
            }),
            organism: proto::Organism::HomoSapiens as i32,
            requested_outputs: vec![proto::OutputType::Dnase as i32],
            ontology_terms: vec![proto::OntologyTerm {
                ontology_type: proto::OntologyType::Uberon as i32,
                id: uberon,
            }],
            model_version: "ALL_FOLDS".into(),
        };
        let bytes = request.encode_to_vec();
        Ok(Self { request, bytes })
    }
    /// Stable credential-free wire bytes, including all selectors and checkpoint identity.
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
    /// Start an independent bounded validator for this request's response stream.
    pub fn decoder(&self, limits: Limits) -> Result<Envelope, BadReply> {
        Envelope::new(self.request.interval.as_ref().ok_or(BadReply)?, limits)
    }
}
