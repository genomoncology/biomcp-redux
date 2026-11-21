//! Disease pages and their publishers.
biodata::wire_enum! { described
    /// One named disease page.
    DiseaseSection {
        /// Publisher scoped synonyms.
        Synonyms => "synonyms",
        /// Native cross-references. Use --source ols for publisher-supplied target URLs.
        Xrefs => "xrefs",
        /// Native parent relations.
        Hierarchy => "hierarchy",
        /// Native gene association rows.
        Genes => "genes",
        /// Native phenotype association rows.
        Phenotypes => "phenotypes",
        /// Native stage incidence rows.
        Incidence => "incidence",
        /// Annual survival rows.
        Survival => "survival",
        /// Orphanet prevalence, age of onset and inheritance.
        Prevalence => "prevalence",
        /// ClinGen and GenCC curations by exact native disease name or ID.
        GeneValidity => "gene-validity",
    }
}
impl DiseaseSection {
    /// Publishers implementing this native section.
    pub const fn sources(self) -> &'static [crate::Source] {
        use crate::Source;
        match self {
            Self::Synonyms | Self::Xrefs => &[Source::Ols, Source::MyDisease],
            Self::Hierarchy => &[Source::MyDisease],
            Self::Genes | Self::Phenotypes => &[Source::Monarch],
            Self::Incidence | Self::Survival => &[Source::Seer],
            Self::Prevalence => &[Source::Orphadata],
            Self::GeneValidity => &[Source::ClinGen, Source::Gencc],
        }
    }
}
