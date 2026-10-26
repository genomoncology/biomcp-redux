//! Bounded disclosure from the same checked UniProt entry; no section fanout.
use biodata::ProteinSequence;
use biomcp_adapters::uniprot::Entry;
use biomcp_types::{ProteinFeatures, ProteinFunctionResult, ProteinGet, ProteinIsoforms,
    ProteinSection, ProteinSections, ProteinSequenceResult, ProteinStructures, Reason, Section, Source};
pub(super) const NAMES: [ProteinSection; 5] = [ProteinSection::Features, ProteinSection::Isoforms,
    ProteinSection::Structures, ProteinSection::Function, ProteinSection::Sequence];
pub(super) fn wants(q: &ProteinGet, section: ProteinSection) -> bool {
    q.sections().contains(&section) || q.sections().contains(&ProteinSection::All)
}
fn rows<T, B>(items: Vec<T>, wrap: impl FnOnce(Box<[T]>) -> B) -> Section<B> {
    if items.is_empty() { Section::Empty { source: Source::UniProt, record: None } }
    else { Section::Data { source: Source::UniProt, data: wrap(items.into()) } }
}
pub(super) fn first(q: &ProteinGet, entry: &Entry) -> ProteinSections {
    let n = q.section_limit() as usize;
    ProteinSections {
        features: wants(q, ProteinSection::Features).then(|| rows(entry.features.iter().take(n).cloned().collect(), |items| ProteinFeatures { items })),
        isoforms: wants(q, ProteinSection::Isoforms).then(|| rows(entry.isoforms.iter().take(n).cloned().collect(), |items| ProteinIsoforms { items })),
        structures: wants(q, ProteinSection::Structures).then(|| rows(entry.structures.iter().take(n).cloned().collect(), |items| ProteinStructures { items })),
        function: wants(q, ProteinSection::Function).then(|| rows(entry.function.clone(), |items| ProteinFunctionResult { items })),
        sequence: wants(q, ProteinSection::Sequence).then(|| sequence(entry.sequence.clone())),
        domains: None, complexes: None,
    }
}
fn sequence(value: Option<ProteinSequence>) -> Section<ProteinSequenceResult> {
    match value {
        Some(sequence) => Section::Data { source: Source::UniProt, data: ProteinSequenceResult { sequence } },
        None => Section::unavailable(Source::UniProt, Reason::BadReply),
    }
}
pub(super) fn failed(q: &ProteinGet, reason: Reason) -> ProteinSections {
    ProteinSections {
        features: wants(q, ProteinSection::Features).then(|| Section::unavailable(Source::UniProt, reason)),
        isoforms: wants(q, ProteinSection::Isoforms).then(|| Section::unavailable(Source::UniProt, reason)),
        structures: wants(q, ProteinSection::Structures).then(|| Section::unavailable(Source::UniProt, reason)),
        function: wants(q, ProteinSection::Function).then(|| Section::unavailable(Source::UniProt, reason)),
        sequence: wants(q, ProteinSection::Sequence).then(|| Section::unavailable(Source::UniProt, reason)),
        domains: None, complexes: None,
    }
}
