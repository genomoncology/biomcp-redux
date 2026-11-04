use biodata::DrugId;
use super::DrugSection;
use serde::{Deserialize, Serialize};
/// One explicitly selected drug source; omitted search source means MyChem alone.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "&'static str")]
pub enum DrugSource {
    /// WHO finished pharmaceutical product prequalification.
    WhoPq,
    /// FDA native drug recall records.
    FdaDrugRecall,
    /// MyChem candidate identity.
    Mychem,
    /// ChEMBL native records.
    Chembl,
    /// Open Targets native drug records.
    Opentargets,
    /// CIViC therapy records, sharing the existing canonical source name.
    Civic,
    /// openFDA SPL label records.
    OpenfdaLabel,
    /// Drugs@FDA native applications.
    Drugsfda,
    /// EMA native medicines.
    Ema,
    /// DDInter native bundle records.
    Ddinter,
}
impl DrugSource {
    /// Every retained drug source; registry bindings use these canonical values.
    pub const ALL: [Self; 10] = [
        Self::WhoPq,
        Self::FdaDrugRecall,
        Self::Mychem,
        Self::Chembl,
        Self::Opentargets,
        Self::Civic,
        Self::OpenfdaLabel,
        Self::Drugsfda,
        Self::Ema,
        Self::Ddinter,
    ];
    /// Canonical source spelling, reusing an existing source identity when present.
    pub fn as_str(self) -> &'static str {
        self.source().into()
    }
    /// Shared transport identity, also owning the canonical source spelling.
    pub const fn source(self) -> crate::Source {
        match self {
            Self::FdaDrugRecall => crate::Source::FdaDrugRecall,
            Self::WhoPq => crate::Source::WhoPq,
            Self::Mychem => crate::Source::Mychem,
            Self::Chembl => crate::Source::Chembl,
            Self::Opentargets => crate::Source::Opentargets,
            Self::Civic => crate::Source::Civic,
            Self::OpenfdaLabel => crate::Source::OpenfdaLabel,
            Self::Drugsfda => crate::Source::Drugsfda,
            Self::Ema => crate::Source::Ema,
            Self::Ddinter => crate::Source::Ddinter,
        }
    }
}
impl From<DrugSource> for &'static str {
    fn from(value: DrugSource) -> Self {
        value.as_str()
    }
}
impl std::str::FromStr for DrugSource {
    type Err = biodata::Error;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::ALL
            .into_iter()
            .find(|source| source.as_str() == value)
            .ok_or(biodata::Error::SourceUnknown)
    }
}
impl TryFrom<String> for DrugSource {
    type Error = biodata::Error;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}
impl std::fmt::Display for DrugSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "schema")]
impl schemars::JsonSchema for DrugSource {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "DrugSource".into()
    }
    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        let names = Self::ALL.map(Self::as_str);
        schemars::json_schema!({"description":"A supported drug data source.",
            "type":"string", "enum":names, "x-enum-open":true})
    }
}

impl DrugSource {
    /// Named disclosures actually served by this source's exact-ID endpoint.
    pub fn sections(self) -> &'static [super::DrugSection] {
        use super::DrugSection::{
            Approvals, Civic, Indications, Interactions, Label, Regulatory, Targets, Xrefs,
        };
        match self {
            Self::Mychem => &[Xrefs],
            Self::Chembl => &[Targets, Indications, Xrefs],
            Self::Opentargets => &[Targets, Indications],
            Self::Civic => &[Civic],
            Self::OpenfdaLabel => &[Label],
            Self::Drugsfda => &[Approvals],
            Self::Ema => &[Regulatory],
            Self::Ddinter => &[Interactions],
            Self::WhoPq | Self::FdaDrugRecall => &[],
        }
    }
}

impl DrugSource {
    /// Check only published ID grammar for this endpoint, without inferring identity level or joining records.
    pub fn accepts(self, id: &DrugId) -> bool {
        let id = id.to_string();
        if id.split_once(':').is_some_and(|(prefix, _)| {
            Self::ALL
                .into_iter()
                .any(|source| source.as_str().eq_ignore_ascii_case(prefix))
        }) {
            return false;
        }
        let digits = |s: &str| !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit());
        match self {
            Self::FdaDrugRecall => id.parse::<biodata::DrugRecallNumber>().is_ok(),
            Self::WhoPq => false,
            Self::Mychem => true,
            Self::Chembl | Self::Opentargets => id.strip_prefix("CHEMBL").is_some_and(digits),
            Self::Civic => civic_id(&id).is_some(),
            Self::Ddinter => id.strip_prefix("DDInter").is_some_and(digits),
            Self::Drugsfda => ["NDA", "ANDA", "BLA"].iter().any(|p| {
                id.strip_prefix(p)
                    .is_some_and(|s| s.len() == 6 && digits(s))
            }),
            Self::Ema => id
                .strip_prefix("EMEA/H/C/")
                .is_some_and(|s| s.len() == 6 && digits(s)),
            Self::OpenfdaLabel => {
                let pieces: Vec<_> = id.split('-').collect();
                pieces.iter().map(|s| s.len()).eq([8, 4, 4, 4, 12])
                    && pieces
                        .iter()
                        .all(|s| s.bytes().all(|b| b.is_ascii_hexdigit()))
            }
        }
    }
}

pub(super) fn civic_id(&self) -> Option<std::num::NonZeroU32> {
        match self.native_id {
            NativeDrugId::Civic(id) => Some(id),
            _ => None,
        }
    }
    /// Select one exact native identity, with no sections requested.
    pub fn new(input: DrugId, source: DrugSource) -> Result<Self, QueryInvalid> {
        GetFields {
            input: Text::new(&input.to_string()).map_err(|_| QueryInvalid::new(&["input"]))?,
            source: Some(source),
            section: Box::default(),
            section_limit: None,
            section_offset: None,
            label_section: None,
            product_number: None,
            civic_profile: None,
            civic_kind: None,
            civic_cursor: None,
        }
        .try_into()
    }
    /// Continue one native claim collection while preserving selected drug, source and section paging.
    pub fn with_civic_page(
        &self,
        profile: u64,
        kind: CivicClaimKind,
        cursor: Option<Text>,
    ) -> Result<Self, QueryInvalid> {
        let mut fields = self.fields.clone();
        fields.civic_profile = Some(profile);
        fields.civic_kind = Some(kind);
        fields.civic_cursor = cursor;
        self.with_fields(fields)
    }
    /// Continue a therapy-wide claim stream without selecting a molecular profile.
    pub fn with_civic_stream(
        &self,
        kind: CivicClaimKind,
        cursor: Option<Text>,
    ) -> Result<Self, QueryInvalid> {
        let mut fields = self.fields.clone();
        fields.civic_kind = Some(kind);
        fields.civic_cursor = cursor;
        self.with_fields(fields)
    }
    /// Selected native drug-scoped CIViC molecular profile.
    pub fn civic_profile(&self) -> Option<u64> {
        self.fields.civic_profile
    }
    /// Selected independent native claim collection.
    pub fn civic_kind(&self) -> Option<CivicClaimKind> {
        self.fields.civic_kind
    }
    /// Original native cursor for that selected claim collection.
    pub fn civic_cursor(&self) -> Option<&Text> {
        self.fields.civic_cursor.as_ref()
    }
    /// Unchanged published identifier.
    pub fn input(&self) -> &Text {
        &self.fields.input
    }
    /// Source-native identity after name resolution.
    pub fn native_input(&self) -> Result<DrugId, QueryInvalid> {
        self.fields
            .input
            .as_str()
            .parse()
            .map_err(|_| QueryInvalid::new(&["input"]))
    }
    /// Required selected source.
    pub fn source(&self) -> DrugSource {
        self.fields.source.unwrap_or(DrugSource::Mychem)
    }
    /// Explicit page bound; command handlers apply their existing default when absent.
    pub fn section_limit(&self) -> Option<u32> {
        self.fields.section_limit
    }
    /// Set a checked section page size without changing the selected source or disclosure.
    pub fn with_section_limit(&self, limit: u32) -> Result<Self, QueryInvalid> {
        let mut fields = self.fields.clone();
        fields.section_limit = Some(limit);
        self.with_fields(fields)
    }
    /// Explicit section continuation offset.
    pub fn section_offset(&self) -> Option<u32> {
        self.fields.section_offset
    }
}

fn present_query<'de, D: serde::Deserializer<'de>>(d: D) -> Result<Option<Text>, D::Error> {
    Text::deserialize(d).map(Some)
}

fn selected_name<'de, D: serde::Deserializer<'de>>(
    d: D,
) -> Result<Option<super::DrugNameSelector>, D::Error> {
    super::DrugNameSelector::deserialize(d).map(Some)
}
impl DrugSearch {
    /// Literal source query, without rewriting.
    pub fn query(&self) -> Option<&Text> {
        self.0.query.as_ref()
    }
    /// Selected native recall filter.
    pub fn firm(&self) -> Option<&Text> {
        self.0.firm.as_ref()
    }
    /// Selected native recall filter.
    pub fn classification(&self) -> Option<super::DrugRecallClassification> {
        self.0.classification
    }
    /// Selected native recall filter.
    pub fn status(&self) -> Option<&Text> {
        self.0.status.as_ref()
    }
    /// Selected native recall filter.
    pub fn reason(&self) -> Option<&Text> {
        self.0.reason.as_ref()
    }
    /// Selected native recall filter.
    pub fn initiated_from(&self) -> Option<biodata::PartialDate> {
        self.0.initiated_from
    }
    /// Selected native recall filter.
    pub fn initiated_to(&self) -> Option<biodata::PartialDate> {
        self.0.initiated_to
    }

    /// Single selected native source.
    pub fn source(&self) -> DrugSource {
        self.0.source
    }
    /// Native page bound.
    pub fn limit(&self) -> u32 {
        self.0.limit
    }
    /// Native page offset.
    pub fn offset(&self) -> u32 {
        self.0.offset
    }
}
impl crate::Paged for DrugSearch {
    type Offset = u32;
    fn set_page_offset(&mut self, offset: Self::Offset) {
        self.0.offset = offset;
    }
}

impl DrugGet {
    /// Whether the selected source serves every requested disclosure in one call.
    pub fn supports_sections(&self) -> bool {
        self.sections().len() <= 1
            && self
                .sections()
                .iter()
                .all(|section| self.source().sections().contains(section))
    }
    /// Explicit requested disclosures.
    pub fn sections(&self) -> &[DrugSection] {
        &self.fields.section
    }
}

impl DrugGet {
    /// Continue the same named section without changing source, identity or section selectors.
    pub fn at_section_offset(&self, offset: u32) -> Result<Self, QueryInvalid> {
        let mut fields = self.fields.clone();
        fields.section_offset = Some(offset);
        self.with_fields(fields)
    }
    /// Exact selected native label heading.
    pub fn label_section(&self) -> Option<&Text> {
        self.fields.label_section.as_ref()
    }
    /// Exact application product selector.
    pub fn product_number(&self) -> Option<&Text> {
        self.fields.product_number.as_ref()
    }
}

impl DrugGet {
    /// Select one named disclosure without changing the exact source identity.
    pub fn for_section(&self, section: DrugSection) -> Result<Self, QueryInvalid> {
        let mut fields = self.fields.clone();
        fields.section = vec![section].into();
        self.with_fields(fields)
    }
}

impl DrugGet {
    /// Select an explicit native label heading for the same source identity.
    pub fn for_label(&self, heading: Text) -> Result<Self, QueryInvalid> {
        let mut fields = self.fields.clone();
        fields.section = vec![DrugSection::Label].into();
        fields.label_section = Some(heading);
        self.with_fields(fields)
    }
}

pub(super) fn civic_id(value: &str) -> Option<std::num::NonZeroU32> {
    if !value.bytes().all(|byte| byte.is_ascii_digit()) {
        return None;
    }
    value
        .parse::<std::num::NonZeroU32>()
        .ok()
        .filter(|id| id.get() <= i32::MAX as u32)
}

pub(super) fn inferred_source(sections: &[DrugSection]) -> DrugSource {
    let candidates: Vec<_> = DrugSource::ALL
        .into_iter()
        .filter(|source| {
            !sections.is_empty()
                && sections
                    .iter()
                    .all(|section| source.sections().contains(section))
        })
        .collect();
    match candidates.as_slice() {
        [source] => *source,
        _ => DrugSource::Mychem,
    }
}
impl DrugSource {
    /// Distinguish native MyChem keys from names; other sources use their existing ID grammar.
    pub fn native_id(self, input: &DrugId) -> bool {
        if self != Self::Mychem {
            return self.accepts(input);
        }
        if !self.accepts(input) { return false; }
        let value = input.to_string();
        value.len() == 10
            && value
                .bytes()
                .all(|b| b.is_ascii_uppercase() || b.is_ascii_digit())
            || value.bytes().all(|b| b.is_ascii_digit() || b == b'-')
                && value.bytes().any(|b| b.is_ascii_digit())
            || value.len() == 27
                && value.as_bytes().get(14) == Some(&b'-')
                && value.as_bytes().get(25) == Some(&b'-')
            || value.contains(':')
            || value.starts_with("CHEMBL")
            || value.starts_with("DB") && value[2..].bytes().all(|b| b.is_ascii_digit())
            || value.bytes().all(|b| b.is_ascii_digit())
    }
}
