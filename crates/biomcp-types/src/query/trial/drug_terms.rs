//! Treatment terms the query layer replaces with source-stated drug names.
use super::TrialSearch;
use crate::Text;

impl TrialSearch {
    /// Whether drug-name expansion is disabled.
    pub fn exact_term(&self) -> bool {
        self.0.exact_term
    }
    /// Substitute only registry discovery text and intervention alternatives.
    pub fn with_drug_terms(
        &self,
        text: Option<Text>,
        interventions: Box<[Text]>,
        keywords: Box<[Text]>,
    ) -> Self {
        let mut next = self.clone();
        next.0.query = text;
        next.0.intervention = interventions;
        next.0.keyword = keywords;
        next
    }
    /// Substitute the variant and criteria terms the registry is sent.
    pub fn with_variant_terms(&self, variant: Option<Text>, criteria: Option<Text>) -> Self {
        let mut next = self.clone();
        next.0.variant = variant;
        next.0.criteria = criteria;
        next
    }
    /// The same request, with its terms searched as written.
    pub fn exact(&self) -> Self {
        let mut next = self.clone();
        next.0.exact_term = true;
        next
    }
}
