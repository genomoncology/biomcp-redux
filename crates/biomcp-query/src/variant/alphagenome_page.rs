//! Page every native tensor independently in publisher row-major order.
use biomcp_fetch::{
    BadReply, RequestError,
    grpc::alphagenome::{ScoreGroup, ScoreReply, ScoreRequest},
};
use biomcp_types::{
    AlphaGenomeDataType, AlphaGenomeScorer, AlphaGenomeScorerPage, AlphaGenomeValue, PageInfo,
    Reason, Source, VariantAlphaGenome, VariantGet, VariantSection,
};

pub(super) fn page(
    query: &VariantGet,
    request: &ScoreRequest,
    reply: ScoreReply,
) -> Result<VariantAlphaGenome, RequestError> {
    mapped(query, request, reply).map_err(|_| {
        biomcp_fetch::SourceError {
            source: Source::AlphaGenome,
            reason: Reason::BadReply,
            status: None,
            retry_after: None,
        }
        .into()
    })
}
fn mapped(
    query: &VariantGet,
    request: &ScoreRequest,
    reply: ScoreReply,
) -> Result<VariantAlphaGenome, BadReply> {
    let scorers = reply
        .groups
        .into_iter()
        .enumerate()
        .map(|(ordinal, group)| group_page(query, ordinal, group))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(VariantAlphaGenome {
        build: request.build(),
        variant: reply.variant,
        requested_interval: request.requested_interval()?,
        requested_model_version: String::new(),
        scorers: scorers.into(),
    })
}
fn group_page(
    query: &VariantGet,
    ordinal: usize,
    group: ScoreGroup,
) -> Result<AlphaGenomeScorerPage, BadReply> {
    let shape = group
        .tensor
        .shape()
        .iter()
        .copied()
        .map(u32::try_from)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| BadReply)?;
    let shape: [u32; 3] = shape.try_into().map_err(|_| BadReply)?;
    let total = group.tensor.bytes().len() / 4;
    let rows = group
        .tensor
        .bytes()
        .chunks_exact(4)
        .enumerate()
        .skip(query.section_offset() as usize)
        .take(query.section_limit() as usize)
        .map(|(index, bytes)| value(&group, shape, index, bytes))
        .collect::<Result<Vec<_>, _>>()?;
    let returned = rows.len() as u32;
    let next_offset = u64::from(query.section_offset()) + u64::from(returned);
    Ok(AlphaGenomeScorerPage {
        request_ordinal: ordinal.try_into().map_err(|_| BadReply)?,
        requested_scorer: match ordinal {
            0 => AlphaGenomeScorer::GeneMaskLfcRnaSeq,
            1 => AlphaGenomeScorer::GeneMaskSplicingSpliceSites,
            2 => AlphaGenomeScorer::CenterMaskDiffMeanDnase501,
            _ => return Err(BadReply),
        },
        data_type: AlphaGenomeDataType::Float32,
        shape,
        rows: rows.into(),
        page: PageInfo {
            offset: query.section_offset(),
            returned,
            total: total as u64,
            pool: None,
            truncated: None,
        },
        next: (next_offset < total as u64).then(|| {
            query
                .at_section_offset(next_offset as u32)
                .with_sections(vec![VariantSection::AlphaGenome].into())
        }),
    })
}
fn value(
    group: &ScoreGroup,
    shape: [u32; 3],
    index: usize,
    bytes: &[u8],
) -> Result<AlphaGenomeValue, BadReply> {
    let tracks = shape[2] as usize;
    let genes = shape[1] as usize;
    if tracks == 0 || genes == 0 {
        return Err(BadReply);
    }
    let plane = index / tracks / genes;
    let row = index / tracks % genes;
    let track = index % tracks;
    Ok(AlphaGenomeValue {
        tensor_index: [plane as u32, row as u32, track as u32],
        value: biomcp_types::AlphaGenomeScalar::from_float32(f32::from_le_bytes(
            bytes.try_into().map_err(|_| BadReply)?,
        ))
        .map_err(|_| BadReply)?,
        track: group.tracks.get(track).ok_or(BadReply)?.clone(),
        gene: if group.genes.is_empty() {
            None
        } else {
            Some(group.genes.get(row).ok_or(BadReply)?.clone())
        },
    })
}
