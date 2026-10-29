use crate::native_text::NativeText;
use biodata::Text;
use biomcp_fetch::{BadReply, Reply};
use serde::Deserialize;
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Study {
    pub study_id: Text,
    #[serde(deserialize_with = "crate::native_text::deserialize")]
    pub name: biodata::Text,
    pub description: Option<NativeText>,
    pub citation: Option<NativeText>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Profile {
    pub molecular_profile_id: Text,
    pub study_id: Text,
    pub molecular_alteration_type: Text,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct SampleList {
    pub sample_list_id: Text,
    pub study_id: Text,
    pub sample_count: Option<u64>,
}
/// Native source mutation identity and sample, without inferred equivalence.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mutation {
    /// Source profile; checked before cache admission.
    pub molecular_profile_id: Text,
    /// Source study; checked before cache admission.
    pub study_id: Text,
    /// Source sample identity, used only for page-local distinct counting.
    pub sample_id: Text,
    /// Source gene, checked against the HGNC-resolved request.
    pub entrez_gene_id: u32,
    /// Source chromosome.
    pub chr: Option<Text>,
    /// Source one-based inclusive start.
    pub start_position: Option<u64>,
    /// Source one-based inclusive end.
    pub end_position: Option<u64>,
    /// Source plus-strand reference.
    pub reference_allele: Option<Text>,
    /// Source plus-strand alternate.
    pub variant_allele: Option<Text>,
    /// Source assembly; missing never means a default.
    pub ncbi_build: Option<Text>,
}
macro_rules! reply {
    ($($t:ty),+ $(,)?) => {$ (impl Reply for $t { fn check(&self) -> Result<(), BadReply> { Ok(()) } })+};
}
#[derive(Deserialize)]
#[serde(transparent)]
pub(super) struct Mutations(pub Vec<Mutation>);
#[derive(Deserialize)]
#[serde(transparent)]
pub(super) struct SampleLists(pub Vec<SampleList>);
reply!(Study, Profile, SampleLists, Mutations);
