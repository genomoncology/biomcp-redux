use crate::{QueryInvalid, Text};
use biodata::{CbioportalCancerTypeId, StudyId};
use serde::{Deserialize, Serialize};
fn limit() -> u32 {
    5
}

biodata::wire_enum! {
    described
    /// Supported bounded study metadata sections.
    StudySection {
        /// Molecular profiles.
        Profiles => "profiles",
        /// Sample-list metadata.
        SampleLists => "sample-lists",
        /// Mutation rows for one gene, using the all-cases sample list.
        Mutations => "mutations",
        /// Fusions and rearrangements for one gene.
        StructuralVariants => "structural-variants",
        /// Sample IDs and types.
        Samples => "samples",
        /// Discrete copy-number counts for one gene.
        Cna => "cna",
        /// Repository file links.
        Files => "files",
        /// First pages; mutations require a gene.
        All => "all"
    }
}
impl StudySection {
    fn requires_gene(self) -> bool {
        matches!(self, Self::Mutations | Self::StructuralVariants | Self::Cna)
    }
}
#[doc = " Literal stored metadata search; text is never parsed as a query language. No frequency, \
    denominator, survival or comparison is calculated. Source counts keep their native names."]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "SearchFields")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(with = "SearchFields", extend("anyOf" = [{"required":["text"]},{"required":["cancer_type"]}])))]
pub struct StudySearch(SearchFields);
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
struct SearchFields {
    /// Literal study ID, name or description text, without synonyms or typo repair.
    #[serde(
        default,
        deserialize_with = "crate::query::present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(feature = "schema", schemars(with = "Text", extend("x-positional" = 0)))]
    text: Option<Text>,
    /// Exact cBioPortal cancer-type code; no OncoTree conversion.
    #[serde(
        default,
        deserialize_with = "crate::query::present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(feature = "schema", schemars(with = "CbioportalCancerTypeId"))]
    cancer_type: Option<CbioportalCancerTypeId>,
    /// Maximum headers per page.
    #[serde(default = "limit")]
    #[cfg_attr(feature = "schema", schemars(range(min = 1, max = crate::HELD_PAGE_MAX)))]
    limit: u32,
    /// Zero-based offset in the checked snapshot.
    #[serde(default)]
    offset: u32,
    /// Opaque continuation retaining the catalog revision and selectors.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pool: Option<Text>,
}
crate::query::validation::checked_query!(StudySearch, SearchFields);
impl TryFrom<SearchFields> for StudySearch {
    type Error = QueryInvalid;
    fn try_from(v: SearchFields) -> Result<Self, Self::Error> {
        if v.text.is_none() && v.cancer_type.is_none() {
            return Err(QueryInvalid::new(&["text", "cancer_type"]));
        }
        if !(1..=crate::HELD_PAGE_MAX).contains(&v.limit) {
            return Err(QueryInvalid::new(&["limit"]));
        }
        Ok(Self(v))
    }
}
impl StudySearch {
    /// Literal caller text.
    pub fn text(&self) -> Option<&Text> {
        self.0.text.as_ref()
    }
    /// Exact namespace-preserving cancer-type selector.
    pub fn cancer_type(&self) -> Option<&CbioportalCancerTypeId> {
        self.0.cancer_type.as_ref()
    }
    /// Bounded page size.
    pub fn limit(&self) -> u32 {
        self.0.limit
    }
    /// Snapshot offset.
    pub fn offset(&self) -> u32 {
        self.0.offset
    }
    /// Opaque retained snapshot.
    pub fn pool(&self) -> Option<&Text> {
        self.0.pool.as_ref()
    }
    /// Bind this request to an immutable catalog search snapshot.
    pub fn with_pool(&self, pool: Option<Text>) -> Self {
        let mut q = self.clone();
        q.0.pool = pool;
        q
    }
}
impl crate::Paged for StudySearch {
    type Offset = u32;
    fn set_page_offset(&mut self, offset: Self::Offset) {
        self.0.offset = offset;
    }
    fn clear_page_context(&mut self) {
        self.0.pool = None;
    }
}

#[doc = " Exact case-sensitive study card or bounded metadata section. No frequency, denominator, \
    survival or comparison is calculated. Source counts keep their native names."]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "GetFields")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(with = "GetFields"))]
pub struct StudyGet(GetFields);
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
struct GetFields {
    /// Exact study ID.
    #[cfg_attr(feature = "schema", schemars(extend("x-positional" = 0)))]
    id: StudyId,
    /// Omit for the study card.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(extend("x-positional" = 1)))]
    section: Option<StudySection>,
    /// Rows per section (default 5).
    #[serde(default = "limit")]
    #[cfg_attr(feature = "schema", schemars(range(min = 1, max = crate::HELD_PAGE_MAX)))]
    section_limit: u32,
    /// Offset in one section.
    #[serde(default)]
    section_offset: u32,
    /// HUGO symbol or Entrez ID. Required for mutations, structural-variants and cna.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    gene: Option<Text>,
    /// Exact profile ID. Defaults to the first of that section's kind.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    molecular_profile: Option<biodata::StudyProfileId>,
}
crate::query::validation::checked_query!(StudyGet, GetFields);
impl TryFrom<GetFields> for StudyGet {
    type Error = QueryInvalid;
    fn try_from(v: GetFields) -> Result<Self, Self::Error> {
        if !(1..=crate::HELD_PAGE_MAX).contains(&v.section_limit) {
            return Err(QueryInvalid::new(&["section_limit"]));
        }
        if matches!(v.section, None | Some(StudySection::All)) && v.section_offset != 0 {
            return Err(QueryInvalid::new(&["section", "section_offset"]));
        }
        let requires_gene = v.section.is_some_and(StudySection::requires_gene);
        if (requires_gene || v.molecular_profile.is_some()) && v.gene.is_none() {
            return Err(QueryInvalid::new(&["gene"]));
        }
        if (v.gene.is_some() || v.molecular_profile.is_some())
            && !requires_gene
            && v.section != Some(StudySection::All)
        {
            return Err(QueryInvalid::new(&["section", "gene", "molecular_profile"]));
        }
        Ok(Self(v))
    }
}
impl StudyGet {
    /// Check an exact study card with the standard section bounds.
    pub fn new(id: StudyId) -> Result<Self, QueryInvalid> {
        Self::try_from(GetFields {
            id,
            section: None,
            section_limit: limit(),
            section_offset: 0,
            gene: None,
            molecular_profile: None,
        })
    }

    /// Caller gene selector, resolved only by cBioPortal.
    pub fn gene(&self) -> Option<&Text> {
        self.0.gene.as_ref()
    }
    /// Caller-selected profile, otherwise chosen from study metadata.
    pub fn molecular_profile(&self) -> Option<&biodata::StudyProfileId> {
        self.0.molecular_profile.as_ref()
    }
    /// Bind subsequent pages to the selected source profile.
    pub fn with_molecular_profile(&self, profile: biodata::StudyProfileId) -> Self {
        let mut q = self.clone();
        q.0.molecular_profile = Some(profile);
        q
    }
    /// Exact checked study ID.
    pub fn id(&self) -> &StudyId {
        &self.0.id
    }
    /// Selected metadata section.
    pub fn section(&self) -> Option<StudySection> {
        self.0.section
    }
    /// Bounded section size.
    pub fn section_limit(&self) -> u32 {
        self.0.section_limit
    }
    /// Offset within the named collection.
    pub fn section_offset(&self) -> u32 {
        self.0.section_offset
    }
    /// Select a named section and start its bounded first page.
    pub fn with_section(&self, section: StudySection) -> Self {
        let mut q = self.clone();
        q.0.section = Some(section);
        if !section.requires_gene() && section != StudySection::All {
            q.0.gene = None;
            q.0.molecular_profile = None;
        }
        q.0.section_offset = 0;
        q
    }
    /// Continue one named section; card and all have no aggregate offset.
    pub fn at_section_offset(&self, offset: u32) -> Result<Self, QueryInvalid> {
        let mut q = self.0.clone();
        q.section_offset = offset;
        Self::try_from(q)
    }
}
