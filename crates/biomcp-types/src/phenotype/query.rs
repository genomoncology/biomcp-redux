use crate::QueryInvalid;
use biodata::HpId;
use serde::{Deserialize, Serialize};
biodata::wire_enum! { described
    /// One independent term publisher.
    PhenotypeSource {
        /// Direct HPO term metadata.
        Hpo => "hpo",
        /// Monarch term search and associations.
        Monarch => "monarch",
    }
}
biodata::wire_enum! { described
    /// A named source term section.
    PhenotypeSection {
        /// Native synonyms.
        Synonyms => "synonyms",
        /// Native cross references.
        Xrefs => "xrefs",
        /// Native direct parent and child links.
        Hierarchy => "hierarchy",
        /// Native disease association rows.
        Diseases => "diseases",
        /// Native human gene association rows.
        Genes => "genes",
    }
}
fn monarch() -> PhenotypeSource {
    PhenotypeSource::Monarch
}
fn five() -> u32 {
    5
}
/// Literal Monarch HP term search, with no similarity operation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "SearchFields")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(with = "SearchFields"))]
pub struct PhenotypeSearch(SearchFields);
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
struct SearchFields {
    /// Unchanged literal term search text.
    #[cfg_attr(feature = "schema", schemars(length(min=1), extend("x-positional" = 0)))]
    text: Box<str>,
    /// Monarch only; HPO search is unsupported.
    #[serde(default = "monarch")]
    source: PhenotypeSource,
    /// Maximum native headers, default five.
    #[serde(default = "five")]
    #[cfg_attr(feature = "schema", schemars(range(min = 1, max = 50)))]
    limit: u32,
    /// Native source row offset.
    #[serde(default)]
    offset: u32,
}
impl TryFrom<SearchFields> for PhenotypeSearch {
    type Error = QueryInvalid;
    fn try_from(v: SearchFields) -> Result<Self, Self::Error> {
        if v.source != PhenotypeSource::Monarch
            || v.text.trim().is_empty()
            || !(1..=50).contains(&v.limit)
        {
            Err(QueryInvalid)
        } else {
            Ok(Self(v))
        }
    }
}
/// Exact HP term with independently selected source and bounded section.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "GetFields")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(with = "GetFields"))]
pub struct PhenotypeGet(GetFields);
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
struct GetFields {
    /// Exact HP identifier.
    #[cfg_attr(feature = "schema", schemars(extend("x-positional" = 0)))]
    id: HpId,
    /// HPO for card, synonyms and xrefs; Monarch for other sections.
    #[serde(
        default,
        deserialize_with = "crate::query::present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(feature = "schema", schemars(with = "PhenotypeSource"))]
    source: Option<PhenotypeSource>,
    /// Absent means the source identity card.
    #[serde(
        default,
        deserialize_with = "crate::query::present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(feature = "schema", schemars(with = "PhenotypeSection", extend("x-positional" = 1)))]
    section: Option<PhenotypeSection>,
    /// Default five text/link rows or one complete association row.
    #[serde(
        default,
        deserialize_with = "crate::query::present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(feature = "schema", schemars(with = "u32", range(min = 1, max = 50)))]
    section_limit: Option<u32>,
    /// Native section row offset.
    #[serde(default)]
    section_offset: u32,
}
impl TryFrom<GetFields> for PhenotypeGet {
    type Error = QueryInvalid;
    fn try_from(mut v: GetFields) -> Result<Self, Self::Error> {
        let monarch_only = matches!(
            v.section,
            Some(
                PhenotypeSection::Hierarchy | PhenotypeSection::Diseases | PhenotypeSection::Genes
            )
        );
        let association = matches!(
            v.section,
            Some(PhenotypeSection::Diseases | PhenotypeSection::Genes)
        );
        if (monarch_only && v.source == Some(PhenotypeSource::Hpo))
            || v.section_limit.is_some_and(|n| !(1..=50).contains(&n))
        {
            return Err(QueryInvalid);
        }
        v.source.get_or_insert(if monarch_only {
            PhenotypeSource::Monarch
        } else {
            PhenotypeSource::Hpo
        });
        v.section_limit
            .get_or_insert(if association { 1 } else { 5 });
        Ok(Self(v))
    }
}
impl PhenotypeSearch {
    /// Literal caller text without normalization.
    pub fn text(&self) -> &str {
        &self.0.text
    }
    /// Selected source.
    pub fn source(&self) -> PhenotypeSource {
        self.0.source
    }
    /// Bounded native page size.
    pub fn limit(&self) -> u32 {
        self.0.limit
    }
    /// Native offset.
    pub fn offset(&self) -> u32 {
        self.0.offset
    }
    /// Continue with the same selectors.
    pub fn at_offset(&self, offset: u32) -> Self {
        let mut q = self.clone();
        q.0.offset = offset;
        q
    }
}
impl PhenotypeGet {
    /// Construct a default direct HPO card request from a checked identity.
    pub fn new(id: HpId) -> Self {
        Self(GetFields {
            id,
            source: Some(PhenotypeSource::Hpo),
            section: None,
            section_limit: Some(5),
            section_offset: 0,
        })
    }
    /// Checked term identity.
    pub fn id(&self) -> &HpId {
        &self.0.id
    }
    /// Resolved independent publisher.
    pub fn source(&self) -> PhenotypeSource {
        self.0.source.unwrap_or(PhenotypeSource::Hpo)
    }
    /// Selected section or the card.
    pub fn section(&self) -> Option<PhenotypeSection> {
        self.0.section
    }
    /// Checked bounded section size.
    pub fn section_limit(&self) -> u32 {
        self.0.section_limit.unwrap_or(5)
    }
    /// Section offset.
    pub fn section_offset(&self) -> u32 {
        self.0.section_offset
    }
    /// Continue the same section and publisher.
    pub fn at_section_offset(&self, offset: u32) -> Self {
        let mut q = self.clone();
        q.0.section_offset = offset;
        q
    }
}
