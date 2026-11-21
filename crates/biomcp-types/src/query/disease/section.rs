//! Disease pages and their publishers.
biodata::wire_enum! { described
    /// One named disease page.
    DiseaseSection {
        /// Publisher synonyms.
        Synonyms => "synonyms",
        /// Native cross-references. Use --source ols for publisher-supplied target URLs.
        Xrefs => "xrefs",
        /// Parent terms.
        Hierarchy => "hierarchy",
        /// Gene associations.
        Genes => "genes",
        /// Phenotype associations.
        Phenotypes => "phenotypes",
        /// Incidence by stage.
        Incidence => "incidence",
        /// Annual survival.
        Survival => "survival",
        /// Orphanet prevalence, onset and inheritance.
        Prevalence => "prevalence",
        /// ClinGen and GenCC curations. Filters ignore case.
        GeneValidity => "gene-validity",
    }
}
impl DiseaseSection {
    /// Publishers implementing this native section.
    pub const fn sources(self) -> &'static [crate::Source] {
        use crate::Source;
        match self {
            Self::Synonyms => &[Source::Ols, Source::MyDisease],
            Self::Xrefs => &[Source::Ols, Source::MyDisease, Source::Orphadata],
            Self::Hierarchy => &[Source::MyDisease],
            Self::Genes | Self::Phenotypes => &[Source::Monarch],
            Self::Incidence | Self::Survival => &[Source::Seer],
            Self::Prevalence => &[Source::Orphadata],
            Self::GeneValidity => &[Source::ClinGen, Source::Gencc],
        }
    }
}
