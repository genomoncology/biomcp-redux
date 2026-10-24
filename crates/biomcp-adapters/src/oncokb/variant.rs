//! Keyed source requests bind every accepted response to its complete explicit context.
use super::SPEC;
use biodata::Build;
use biomcp_fetch::{Arg, BadReply, Endpoint, Freshness, Leg, Reply, RequestError};
use serde::Deserialize;

const ANNOTATION: Endpoint = SPEC
    .get("/api/v1/annotate/mutations/byHGVSg")
    .fresh(Freshness::Days(1));

/// Native source response; clinical labels have not been mapped or interpreted.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Annotation {
    /// Source echo, required before admission to the shared reply cache.
    pub query: Query,
    /// Provider exact variant curation flag.
    pub variant_exist: bool,
    /// Provider alternate allele curation flag.
    pub allele_exist: Option<bool>,
    /// Native oncogenicity label.
    pub oncogenic: Option<String>,
    /// Native mutation effect.
    pub mutation_effect: Option<MutationEffect>,
    /// Native source text.
    pub gene_summary: Option<String>,
    /// Native source text.
    pub variant_summary: Option<String>,
    /// Native source text.
    pub tumor_type_summary: Option<String>,
    /// Native source text.
    pub diagnostic_summary: Option<String>,
    /// Native source text.
    pub prognostic_summary: Option<String>,
    /// Native source implications in source order.
    #[serde(default)]
    pub treatments: Vec<Treatment>,
    /// Native diagnostic implications in source order.
    #[serde(default)]
    pub diagnostic_implications: Vec<Implication>,
    /// Native prognostic implications in source order.
    #[serde(default)]
    pub prognostic_implications: Vec<Implication>,
    /// Source release.
    pub data_version: Option<String>,
    /// Source release date, without an invented conversion.
    pub last_update: Option<String>,
}
/// Complete source-echoed request identity.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Query {
    /// Source genomic query.
    pub hgvs: String,
    /// Source assembly.
    pub reference_genome: Build,
    /// Source tumor context.
    pub tumor_type: String,
}
/// Native mutation effect and text, without local interpretation.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MutationEffect {
    /// Native label.
    pub known_effect: Option<String>,
    /// Native text.
    pub description: Option<String>,
}
/// Native drug identity.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Drug {
    /// Source label.
    pub drug_name: String,
    /// Native NCIt code, when supplied.
    pub ncit_code: Option<String>,
}
/// Native tumor concept.
#[derive(Deserialize)]
pub struct TumorType {
    /// OncoTree code.
    pub code: Option<String>,
    /// Native display label.
    pub name: Option<String>,
}
/// Native treatment row, including the provider's alteration grouping.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Treatment {
    /// Native source level.
    pub level: String,
    /// Source tumor context.
    pub level_associated_cancer_type: Option<TumorType>,
    /// Source drugs in native order.
    #[serde(default)]
    pub drugs: Vec<Drug>,
    /// Source alteration group, never replaced with a genomic subject.
    #[serde(default)]
    pub alterations: Vec<String>,
    /// Native citations.
    #[serde(default)]
    pub pmids: Vec<String>,
}
/// Native diagnostic or prognostic implication.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Implication {
    /// Native source level.
    pub level_of_evidence: String,
    /// Native source tumor concept.
    pub tumor_type: Option<TumorType>,
    /// Source alteration group.
    #[serde(default)]
    pub alterations: Vec<String>,
    /// Native citations.
    #[serde(default)]
    pub pmids: Vec<String>,
}
impl Reply for Annotation {
    fn check(&self) -> Result<(), BadReply> {
        Ok(())
    }
}
/// Read only the licensed endpoint under the existing required-key source policy.
/// A different source query, assembly or tumor context can never enter the reply cache.
pub fn annotation(
    leg: &Leg<'_>,
    hgvsg: &str,
    build: Build,
    tumor_type: &str,
) -> Result<Annotation, RequestError> {
    let assembly = build.to_string();
    leg.json_received_checked(
        &ANNOTATION,
        &[
            ("hgvsg", Arg::Text(hgvsg)),
            ("referenceGenome", Arg::Text(&assembly)),
            ("tumorType", Arg::Text(tumor_type)),
        ],
        |row: &Annotation| {
            if row.query.hgvs == hgvsg
                && row.query.reference_genome == build
                && row.query.tumor_type == tumor_type
            {
                Ok(())
            } else {
                Err(BadReply)
            }
        },
    )
    .map(|(row, _)| row)
}
