use biodata::canonical_enum;

canonical_enum! {
    /// A source of BioMCP data, identified by its canonical short name.
    Source, SourceUnknown {
        /// HGNC.
        Hgnc => "hgnc",
        /// MyGene.info.
        MyGene => "mygene",
        /// Reactome.
        Reactome => "reactome",
        /// Human Protein Atlas.
        Hpa => "hpa",
        /// UniProt.
        UniProt => "uniprot",
        /// gnomAD.
        Gnomad => "gnomad",
        /// ClinVar component source policy.
        ClinVar => "clinvar",
        /// GWAS Catalog v2.
        Gwas => "gwas",
        /// ClinGen.
        ClinGen => "clingen",
        /// PubMed.
        PubMed => "pubmed",
        /// Europe PMC.
        EuropePmc => "europepmc",
        /// NCBI LitVar2.
        LitVar => "litvar",
        /// NCBI LitSense2.
        LitSense => "litsense",
        /// Semantic Scholar.
        SemanticScholar => "semantic-scholar",
        /// PMC Open Access bucket.
        PmcOa => "pmc-oa",
        /// NIH iCite.
        Icite => "icite",
        /// PubTator.
        PubTator => "pubtator",
        /// ClinicalTrials.gov.
        ClinicalTrialsGov => "ctgov",
        /// NCI Clinical Trials Search.
        Nci => "nci",
        /// CIViC.
        Civic => "civic",
        /// Cancer Hotspots residue recurrence.
        CancerHotspots => "hotspots",
        /// OncoKB.
        Oncokb => "oncokb",
        /// MyVariant.info aggregation.
        MyVariant => "myvariant",
        /// ClinGen Allele Registry.
        AlleleRegistry => "car",
        /// Mutalyzer.
        Mutalyzer => "mutalyzer",
        /// VariantValidator.
        VariantValidator => "variantvalidator",
        /// Public cBioPortal with study-specific reuse terms.
        Cbioportal => "cbioportal",
    }
}

canonical_enum! {
    /// Why a source could not supply data.
    Reason, ReasonUnknown {
        /// The requested record was not found.
        NotFound => "not_found",
        /// The public bucket does not hold an open text record.
        NotOpen => "not_open",
        /// Publisher response is not a PDF.
        NotPdf => "not_pdf",
        /// The local reader does not support the file or container format.
        Unsupported => "unsupported",
        /// The response exceeds an explicit reader resource bound.
        TooLarge => "too_large",
        /// The source limited the request rate.
        RateLimited => "rate_limited",
        /// The source was unavailable.
        Unavailable => "unavailable",
        /// Platform or configured certificate verification failed.
        Tls => "tls",
        /// Connecting to or negotiating the configured proxy failed.
        Proxy => "proxy",
        /// The source rejected the request.
        Rejected => "rejected",
        /// The reply could not be used.
        BadReply => "bad_reply",
        /// The request exhausted its deadline.
        DeadlineExceeded => "deadline_exceeded",
        /// Network access was disabled.
        Offline => "offline",
        /// A required credential was absent.
        NoKey => "no_key",
        /// The source was disabled.
        Disabled => "disabled",
        /// No recording matched the request.
        ReplayMiss => "replay_miss",
        /// The source data was being synchronized.
        Syncing => "syncing",
        /// Identity lacks the identifier required to join this source.
        NoJoinId => "no_join_id",
    }
}

biodata::wire_enum! {
    /// Reviewed source-use category, without inferring rights in individual records.
    #[derive(PartialOrd, Ord)]
    Use {
        /// Public-domain or unrestricted source data.
        Open => "open",
        /// Attribution is required.
        Attribution => "attribution",
        /// Commercial use requires separate permission.
        Noncommercial => "noncommercial",
        /// Source-specific or mixed rights require checking the publisher's terms.
        Restricted => "restricted",
    }
}

impl Source {
    /// Preserve this consumer's canonical source spelling in neutral provenance.
    #[expect(clippy::expect_used, reason = "The closed source declaration contains only nonempty canonical spellings")]
    pub fn text(self) -> biodata::Text {
        biodata::Text::new(self.into()).expect("canonical source spelling is nonempty")
    }
}
