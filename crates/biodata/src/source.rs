// Define each canonical spelling once for parsing, display and JSON.
macro_rules! canonical_enum {
    ($(#[$doc:meta])* $name:ident, $error:ident { $($(#[$variant_doc:meta])* $variant:ident => $text:literal),+ $(,)? }) => {
        $(#[$doc])*
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
        #[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
        #[cfg_attr(feature = "schema", schemars(!try_from, !into, with = "str", extend("enum" = [$($text),+])))]
        #[non_exhaustive]
        #[serde(try_from = "String", into = "&'static str")]
        pub enum $name {
            $($(#[$variant_doc])* $variant),+
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
        /// PubTator.
        PubTator => "pubtator",
        /// ClinicalTrials.gov.
        ClinicalTrialsGov => "ctgov",
        /// CIViC.
        Civic => "civic",
    }
}

canonical_enum! {
    /// Why a source could not supply data.
    Reason, ReasonUnknown {
        /// The requested record was not found.
        NotFound => "not_found",
        /// The source limited the request rate.
        RateLimited => "rate_limited",
        /// The source was unavailable.
        Unavailable => "unavailable",
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
