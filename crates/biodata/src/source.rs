// Define each canonical spelling once for parsing, display and JSON.
macro_rules! canonical_enum {
    ($(#[$doc:meta])* $name:ident, $error:ident { $($(#[doc = $variant_doc:literal])* $variant:ident => $text:literal),+ $(,)? }) => {
        $(#[$doc])*
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
        #[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
        #[cfg_attr(feature = "schema", schemars(!try_from, !into, with = "str", extend("enum" = [$($text),+], "x-enum-descriptions" = [$(concat!($($variant_doc),*)),+])))]
        #[non_exhaustive]
        #[serde(try_from = "String", into = "&'static str")]
        pub enum $name {
            $($(#[doc = $variant_doc])* $variant),+
        }

        impl From<$name> for &'static str {
            fn from(value: $name) -> Self {
                match value { $($name::$variant => $text),+ }
            }
        }

        impl std::str::FromStr for $name {
            type Err = crate::Error;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                match value {
                    $($text => Ok(Self::$variant),)+
                    _ => Err(crate::Error::$error),
                }
            }
        }

        impl TryFrom<String> for $name {
            type Error = crate::Error;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                value.parse()
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str((*self).into())
            }
        }
    };
}

pub(crate) use canonical_enum;

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
        /// CIViC.
        Civic => "civic",
        /// OncoKB.
        Oncokb => "oncokb",
    }
}

canonical_enum! {
    /// Why a source could not supply data.
    Reason, ReasonUnknown {
        /// The requested record was not found.
        NotFound => "not_found",
        /// The public bucket does not hold an open text record.
        NotOpen => "not_open",
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

crate::macros::wire_enum! {
    /// Reviewed source-use category, without inferring rights in individual records.
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
