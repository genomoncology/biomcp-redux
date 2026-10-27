//! Native fields establishing complete label-name equality.
biodata::canonical_enum! {
    /// Complete native name fields matching the requested label name.
    DrugNameMatch, SourceUnknown {
        /// The complete brand name matches.
        Brand => "brand",
        /// The complete generic name matches.
        Generic => "generic",
        /// Both complete name fields match.
        Both => "both",
    }
}
