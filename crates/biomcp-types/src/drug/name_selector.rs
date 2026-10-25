biodata::canonical_enum! {
    /// One explicit ChEMBL name filter without fallback or synonym expansion.
    DrugNameSelector, SourceUnknown {
        /// Match the native preferred name exactly, ignoring case.
        Preferred => "preferred",
        /// Match an explicit native synonym exactly, ignoring case.
        Synonym => "synonym",
    }
}
