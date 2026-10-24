use super::{ArticleSearch, GeneGet, GeneSearch};
use serde::{Deserialize, Serialize};
/// One checked query. Housekeeping actions are separate.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "command", rename_all = "snake_case")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum Request {
    /// Read source-native variant assertions.
    VariantAssertions(crate::VariantAssertionsGet),
    /// Read source ancestry observations.
    VariantPopulation(crate::VariantPopulationQuery),
    /// Observe an exact allele in an explicitly selected cohort.
    VariantRecurrence(crate::VariantRecurrenceQuery),
    /// Send unchanged HGVS to explicit normalization services.
    VariantNormalize(crate::VariantNormalize),
    /// Read a published variant identifier.
    GetVariant(crate::VariantGet),
    /// Discover source-native variant candidates.
    SearchVariant(crate::VariantSearch),
    /// Save an exact registry document to an explicit destination.
    TrialDocument(crate::TrialDocumentGet),
    /// Retrieve exact trial cards and source protocol sections.
    GetTrial(crate::TrialGet),
    /// Search ClinicalTrials.gov without local screening.
    SearchTrial(crate::TrialSearch),
    /// Save a publisher PDF to an explicit destination.
    ArticlePdf(crate::ArticlePdf),
    /// Read source citation evidence.
    ArticleEvidence(crate::ArticleEvidence),
    /// Source-native paper recommendations.
    ArticleRecommend(crate::ArticleRecommend),
    /// Read a licensed supplementary file.
    ArticleSupplement(crate::ArticleSupplement),
    /// Read licensed PMC body text.
    ArticleText(crate::ArticleText),
    /// Papers citing one PMID.
    ArticleCitations(crate::ArticleEdges),
    /// Papers referenced by one PMID.
    ArticleReferences(crate::ArticleEdges),
    /// Retrieve PubMed article cards.
    GetArticle(crate::ArticleGet),
    /// Search PubMed article identities.
    SearchArticle(ArticleSearch),
    /// Get gene identities and annotations.
    GetGene(GeneGet),
    /// Search gene headers.
    SearchGene(GeneSearch),
}
