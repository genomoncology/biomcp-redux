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

impl DrugNameMatch {
    /// Case-insensitive equality of complete source names, preserving spacing and punctuation.
    pub fn same_name(value: &str, input: &str) -> bool {
        value.to_lowercase() == input.to_lowercase()
    }
    /// Retain the source fields establishing equality.
    pub fn from_fields(brand: bool, generic: bool) -> Option<Self> {
        match (brand, generic) {
            (true, true) => Some(Self::Both),
            (true, false) => Some(Self::Brand),
            (false, true) => Some(Self::Generic),
            _ => None,
        }
    }
}
