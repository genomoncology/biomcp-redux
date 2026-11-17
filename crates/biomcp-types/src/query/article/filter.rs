//! Source capability selectors for article search.
use super::{ArticleSearch, ArticleSort};

biodata::wire_enum! {
    /// Source capability selectors checked before any lookup or transport request.
    ArticleFilter {
        /// Free text.
        Text => "TEXT",
        /// Gene selector.
        Gene => "--gene",
        /// Variant selector.
        Variant => "--variant",
        /// Disease selector.
        Disease => "--disease",
        /// Drug selector.
        Drug => "--drug",
        /// Author selector.
        Author => "--author",
        /// Journal selector.
        Journal => "--journal",
        /// Author affiliation selector.
        Affiliation => "--affiliation",
        /// Lower publication date bound.
        Since => "--since",
        /// Upper publication date bound.
        Until => "--until",
        /// Publication type selector.
        Type => "--type",
        /// Native date ordering.
        DateSort => "--sort date",
        /// Native open-access selector.
        OpenAccess => "--open-access",
        /// Native preprint exclusion.
        NoPreprints => "--no-preprints",
        /// Native retraction exclusion.
        ExcludeRetracted => "--exclude-retracted",
    }
}
impl ArticleSearch {
    /// Whether any named unsupported capability is requested.
    pub fn rejects(&self, filters: &[ArticleFilter]) -> Option<ArticleFilter> {
        filters.iter().copied().find(|filter| match filter {
            ArticleFilter::OpenAccess => self.open_access(),
            ArticleFilter::NoPreprints => self.no_preprints(),
            ArticleFilter::ExcludeRetracted => self.exclude_retracted(),
            ArticleFilter::Text => self.0.text.is_some(),
            ArticleFilter::Gene => self.0.gene.is_some(),
            ArticleFilter::Variant => self.0.variant.is_some(),
            ArticleFilter::Disease => self.0.disease.is_some(),
            ArticleFilter::Drug => self.0.drug.is_some(),
            ArticleFilter::Author => self.0.author.is_some(),
            ArticleFilter::Journal => self.0.journal.is_some(),
            ArticleFilter::Affiliation => self.0.affiliation.is_some(),
            ArticleFilter::Since => self.0.since.is_some(),
            ArticleFilter::Until => self.0.until.is_some(),
            ArticleFilter::Type => self.0.kind.is_some(),
            ArticleFilter::DateSort => self.0.sort == ArticleSort::Date,
        })
    }
}
