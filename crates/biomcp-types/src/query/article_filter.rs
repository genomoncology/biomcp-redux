//! Source capability selectors checked before any article search request.

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
