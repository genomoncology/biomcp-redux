//! Retain complete scoring tensors for bounded native-value paging.
use super::super::{Tensor, proto};
use crate::BadReply;
use biomcp_types::{AlphaGenomeGene, AlphaGenomeTrack, AlphaGenomeVariant};
/// One native group; its ordinal alone associates it with the requested scorer.
#[derive(Debug)]
pub struct ScoreGroup {
    /// Complete checked scalar storage, including every native plane.
    pub tensor: Tensor,
    /// Native track metadata in source order.
    pub tracks: Box<[AlphaGenomeTrack]>,
    /// Native gene metadata in source order, empty for a nongene aggregate.
    pub genes: Box<[AlphaGenomeGene]>,
}
impl ScoreGroup {
    pub(super) fn checked(tensor: Tensor, metadata: Metadata) -> Result<Self, BadReply> {
        let shape = tensor.shape();
        if tensor.data_type().number() != proto::DataType::Float32 as i32
            || shape.len() != 3
            || shape.first().is_none_or(|value| *value < 1 || *value > 3)
            || shape.get(1).is_none_or(|value| *value < 1 || *value > 15)
            || shape.get(2).is_none_or(|value| *value < 1)
            || shape.last().copied().and_then(|v| usize::try_from(v).ok())
                != Some(metadata.tracks.len())
            || (!metadata.genes.is_empty()
                && shape.get(1).copied().and_then(|v| usize::try_from(v).ok())
                    != Some(metadata.genes.len()))
            || tensor.bytes().chunks_exact(4).any(|b| {
                !b.try_into()
                    .map(f32::from_le_bytes)
                    .is_ok_and(f32::is_finite)
            })
        {
            return Err(BadReply);
        }
        Ok(Self {
            tensor,
            tracks: metadata.tracks.into(),
            genes: metadata.genes.into(),
        })
    }
}
/// Complete source reply, published only after successful terminal transport status.
#[derive(Debug)]
pub struct ScoreReply {
    /// Exact source-echoed variant identity.
    pub variant: AlphaGenomeVariant,
    /// Complete native groups in their received order.
    pub groups: Vec<ScoreGroup>,
}
#[derive(Debug, Default)]
pub(super) struct Metadata {
    pub(super) tracks: Vec<AlphaGenomeTrack>,
    pub(super) genes: Vec<AlphaGenomeGene>,
}
