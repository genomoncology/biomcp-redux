use crate::Text;
use serde::{Deserialize, Serialize};
/// ClinVar variation records (VCV) that list this gene, each counted once whatever its assemblies.
/// Records that also list other genes are included; somatic classifications are not counted.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ClinVarGeneCounts {
    /// ClinVar database build that answered, as E-utilities einfo names it.
    pub build: Text,
    /// ClinVar's last update of that build, in its own spelling.
    pub updated: Text,
    /// What the counts cover, in plain words.
    pub scope: Text,
    /// ClinVar search term behind `records`; the other counts add classification and review filters.
    pub term: Text,
    /// Variation records that list this gene.
    pub records: u64,
    /// Records by aggregate germline classification; the buckets do not overlap.
    pub germline: ClinVarGermlineCounts,
    /// Pathogenic, likely pathogenic and pathogenic/likely pathogenic records together.
    pub pathogenic_or_likely: u64,
    /// Of those, records with a GRCh38 position; the rest are placed only on GRCh37 or not at all.
    pub pathogenic_or_likely_grch38: u64,
    /// Pathogenic or likely pathogenic records by their germline review status; the buckets do not overlap.
    pub pathogenic_or_likely_review: ClinVarReviewCounts,
}
/// Disjoint germline classification counts in ClinVar's own terms.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ClinVarGermlineCounts {
    /// Pathogenic.
    pub pathogenic: u64,
    /// Likely pathogenic.
    pub likely_pathogenic: u64,
    /// Pathogenic/Likely pathogenic.
    pub pathogenic_likely_pathogenic: u64,
    /// Uncertain significance.
    pub uncertain_significance: u64,
    /// Conflicting classifications of pathogenicity.
    pub conflicting: u64,
    /// Likely benign.
    pub likely_benign: u64,
    /// Benign.
    pub benign: u64,
    /// Benign/Likely benign.
    pub benign_likely_benign: u64,
    /// Every other record: no germline classification, not provided, risk factor and other terms.
    pub other: u64,
}
/// Disjoint germline review status counts.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ClinVarReviewCounts {
    /// Practice guideline (four stars).
    pub practice_guideline: u64,
    /// Reviewed by expert panel (three stars).
    pub expert_panel: u64,
    /// Criteria provided, multiple submitters, no conflicts (two stars).
    pub multiple_submitters: u64,
    /// Criteria provided, single submitter (one star).
    pub single_submitter: u64,
    /// No assertion criteria provided (no stars).
    pub no_assertion_criteria: u64,
    /// Records whose review status ClinVar's search did not place in one of the above.
    pub other: u64,
}
