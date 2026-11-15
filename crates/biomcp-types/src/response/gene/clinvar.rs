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
    pub build_updated: Text,
    /// What the counts cover, in plain words.
    pub scope: Text,
    /// ClinVar search term behind `records`; the other counts add classification and review filters.
    pub term: Text,
    /// Variation records that list this gene.
    pub records: u64,
    /// Records by the classification terms in their aggregate germline label.
    pub germline: ClinVarGermlineCounts,
    /// Pathogenic, likely pathogenic and pathogenic/likely pathogenic records together.
    pub pathogenic_or_likely: u64,
    /// Of those, records with a GRCh38 position; the rest are placed only on GRCh37 or not at all.
    pub pathogenic_or_likely_grch38: u64,
    /// Pathogenic or likely pathogenic records by their germline review status; the buckets do not overlap.
    pub pathogenic_or_likely_review: ClinVarReviewCounts,
}
/// Germline classification counts by the terms in each record's aggregate label. ClinVar joins
/// a label's terms with `/` and `;`, as in "Pathogenic/Likely pathogenic; risk factor", and each
/// record counts in the bucket of the classification terms its label lists. "Pathogenic, low
/// penetrance" and "VUS-high" are terms of their own. The buckets do not overlap, except for the
/// rare label that lists terms from two of them, such as "Uncertain significance;
/// Pathogenic/Likely pathogenic".
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ClinVarGermlineCounts {
    /// Records whose label includes Pathogenic and not Likely pathogenic, such as "Pathogenic; risk
    /// factor" and "Pathogenic/Pathogenic, low penetrance".
    pub pathogenic: u64,
    /// Records whose label includes Likely pathogenic and not Pathogenic, such as "Likely
    /// pathogenic/Pathogenic, low penetrance".
    pub likely_pathogenic: u64,
    /// Records whose label includes both Pathogenic and Likely pathogenic, such as
    /// "Pathogenic/Likely pathogenic/Pathogenic, low penetrance; risk factor".
    pub pathogenic_likely_pathogenic: u64,
    /// Records whose label includes Uncertain significance, such as "Uncertain
    /// significance/VUS-high".
    pub uncertain_significance: u64,
    /// Records whose label includes Conflicting classifications of pathogenicity, such as
    /// "Conflicting classifications of pathogenicity; risk factor".
    pub conflicting: u64,
    /// Records whose label includes Likely benign and not Benign.
    pub likely_benign: u64,
    /// Records whose label includes Benign and not Likely benign.
    pub benign: u64,
    /// Records whose label includes both Benign and Likely benign.
    pub benign_likely_benign: u64,
    /// Records whose label includes none of the terms above: no germline classification, not
    /// provided, risk factor alone, "Pathogenic, low penetrance" alone, a bare VUS-high or VUS-mid,
    /// and other terms.
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
