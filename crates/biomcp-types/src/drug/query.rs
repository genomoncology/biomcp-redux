//! Checked drug reads and their selectors.
use super::query_validation::{validate_civic, validate_sections};
use super::source::{civic_id, inferred_source};
use super::{DrugSection, DrugSource, label_find};
use crate::{CivicClaimKind, QueryInvalid};
use biodata::{DrugId, Text};
use serde::{Deserialize, Serialize};
/// Retrieve a native ID or a complete source name.
/// Brands rank first; Drugs@FDA then prefers original applications and earliest approval, otherwise source search order.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "GetFields")]
pub struct DrugGet {
    #[serde(flatten)]
    fields: GetFields,
    #[serde(skip)]
    native_id: NativeDrugId,
}
#[derive(Clone, Debug, PartialEq, Eq)]
enum NativeDrugId {
    Recall(biodata::DrugRecallNumber),
    Civic(std::num::NonZeroU32),
    Other,
    Name,
    LabelBatch(Box<[DrugId]>),
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub(super) struct GetFields {
    /// Published source-native identifier or complete brand or generic name.
    #[cfg_attr(feature = "schema", schemars(extend("x-positional" = 0)))]
    input: Text,
    /// Source inferred from an unambiguous requested section; otherwise MyChem.
    #[serde(default, deserialize_with = "selected_source")]
    #[cfg_attr(feature = "schema", schemars(with = "DrugSource", extend("default" = "mychem")))]
    pub(super) source: Option<DrugSource>,
    /// Named disclosures of the selected source, each its own part of one reply; empty means identity only.
    #[serde(default, skip_serializing_if = "<[ _ ]>::is_empty")]
    #[cfg_attr(feature = "schema", schemars(title = "Source sections.", extend("x-positional" = 1)))]
    pub(super) section: Box<[DrugSection]>,
    /// Page size. Requires a section. Max 200 for approvals, interactions, label, regulatory, xrefs and essential-medicines, 50 elsewhere.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(range(min = 1, max = crate::HELD_PAGE_MAX)))]
    pub(super) section_limit: Option<u32>,
    /// Section offset. With label-history --every-version, start at this version. Requires a section.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) section_offset: Option<u32>,
    /// Exact openFDA SPL fields.
    /// Comma-separated. Omit to list fields.
    /// Current subsections take :NUMBER or :TITLE, e.g. dosage_and_administration:2.1.
    /// warnings_and_precautions reads warnings_and_cautions.
    /// For Drugs@FDA history: boxed_warning, indications_and_usage, contraindications,
    /// warnings_and_cautions, adverse_reactions, drug_interactions or use_in_specific_populations.
    /// Only with section label or label-history.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(extend("x-comma-separated" = true)))]
    pub(super) label_section: Option<Text>,
    /// Application product number. Only with --source drugsfda. Only with section approvals.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) product_number: Option<Text>,
    /// Earliest status date, inclusive (YYYY-MM-DD). Only with section approvals.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(extend("x-required-component" = crate::Source::Drugsfda)))]
    pub(super) decision_from: Option<biodata::PartialDate>,
    /// Latest status date, inclusive (YYYY-MM-DD). Only with section approvals.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(extend("x-required-component" = crate::Source::Drugsfda)))]
    pub(super) decision_to: Option<biodata::PartialDate>,
    /// Exact approval letter ID. Limits: 25 MiB, 100 pages and 6144 escaped bytes per fragment. Reports scans without text.
    /// Only with --source drugsfda. Only with section approvals.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) document_id: Option<Text>,
    /// CIViC profile. Selects a molecular profile for this drug. Only with --source civic.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(range(min = 1, max = 2147483647)))]
    pub(super) civic_profile: Option<u64>,
    /// CIViC collection. Evidence or assertions in this profile. Only with --source civic.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) civic_kind: Option<CivicClaimKind>,
    /// CIViC cursor. Continue this drug, profile and collection. Only with --source civic.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) civic_cursor: Option<Text>,
    #[serde(flatten)]
    pub(super) evidence_filter: crate::CivicEvidenceFilter,
    /// Exact label text. Repeat for OR. Ignore case and whitespace breaks. History also ignores hyphenation.
    /// Current labels: two excerpts per section, up to five sections and five comma-separated SPL set IDs.
    /// Only with section label or label-history.
    #[serde(default, skip_serializing_if = "<[ _ ]>::is_empty")]
    #[cfg_attr(feature = "schema", schemars(length(max = 5)))]
    pub(super) find: Box<[Text]>,
    /// Read every version. In order up to the limit, instead of halving. Only with section label-history.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub(super) every_version: bool,
    /// Interaction partner. Keep pairs matching a DDInter ID or full name (any case). Retry missing names with aliases.
    /// Only with section interactions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(extend("x-required-component" = crate::Source::Ddinter)))]
    pub(super) partner: Option<Text>,
}
crate::query::validation::checked_query!(DrugGet, GetFields);
impl TryFrom<GetFields> for DrugGet {
    type Error = QueryInvalid;
    fn try_from(mut fields: GetFields) -> Result<Self, Self::Error> {
        let source = fields
            .source
            .unwrap_or_else(|| inferred_source(&fields.section));
        (fields.source, fields.input) = (Some(source), source.listed_case(fields.input));
        validate_sections(&fields)?;
        if source == DrugSource::OpenfdaLabel
            && fields.section.contains(&DrugSection::Label)
            && let Some(sections) = fields.label_section.as_mut()
        {
            super::normalize_label_numbers(sections, fields.find.is_empty())?;
        }
        let native_id =
            if let Some(ids) = label_find::batch(&fields.input, &fields.find, &fields.section)? {
                NativeDrugId::LabelBatch(ids)
            } else if !fields
                .input
                .as_str()
                .parse::<DrugId>()
                .is_ok_and(|id| source.native_id(&id))
            {
                NativeDrugId::Name
            } else {
                match source {
                    DrugSource::FdaDrugRecall => NativeDrugId::Recall(
                        fields
                            .input
                            .to_string()
                            .parse()
                            .map_err(|_| QueryInvalid::new(&["source", "input"]))?,
                    ),
                    DrugSource::Civic => NativeDrugId::Civic(
                        civic_id(&fields.input.to_string())
                            .ok_or_else(|| QueryInvalid::new(&["source", "input"]))?,
                    ),
                    source => {
                        if !fields
                            .input
                            .as_str()
                            .parse::<DrugId>()
                            .is_ok_and(|id| source.accepts(&id))
                        {
                            return Err(QueryInvalid::new(&["source", "input"]));
                        }
                        NativeDrugId::Other
                    }
                }
            };
        if source == DrugSource::FdaDsc && native_id == NativeDrugId::Name {
            return Err(QueryInvalid::new(&["input"]));
        }
        validate_civic(&fields)?;
        Ok(Self { fields, native_id })
    }
}
impl DrugGet {
    fn with_fields(&self, fields: GetFields) -> Result<Self, QueryInvalid> {
        validate_sections(&fields)?;
        validate_civic(&fields)?;
        Ok(Self {
            fields,
            native_id: self.native_id.clone(),
        })
    }
    /// Whether this input requires exact source name resolution.
    pub fn is_name(&self) -> bool {
        self.native_id == NativeDrugId::Name
    }
    /// Whether an openFDA label get names an FDA application number, as Drugs@FDA and label
    /// replies print it, rather than a drug name.
    pub fn is_label_application(&self) -> bool {
        self.is_name()
            && self.source() == DrugSource::OpenfdaLabel
            && self
                .fields
                .input
                .as_str()
                .parse::<DrugId>()
                .is_ok_and(|id| DrugSource::Drugsfda.accepts(&id))
    }
    /// Preserve section selectors while replacing a resolved name with its native ID.
    pub fn with_native_id(&self, input: DrugId) -> Result<Self, QueryInvalid> {
        let mut fields = self.fields.clone();
        fields.input = Text::new(&input.to_string()).map_err(|_| QueryInvalid::new(&["input"]))?;
        fields.try_into()
    }
    /// FDA recall identity retained by the checked boundary.
    pub fn recall_number(&self) -> Option<&biodata::DrugRecallNumber> {
        match &self.native_id {
            NativeDrugId::Recall(id) => Some(id),
            _ => None,
        }
    }
    /// CIViC therapy number retained by the checked boundary.
    pub fn civic_id(&self) -> Option<std::num::NonZeroU32> {
        match self.native_id {
            NativeDrugId::Civic(id) => Some(id),
            _ => None,
        }
    }
    /// Select one exact native identity, with no sections requested.
    pub fn new(input: DrugId, source: DrugSource) -> Result<Self, QueryInvalid> {
        Self::named(
            Text::new(&input.to_string()).map_err(|_| QueryInvalid::new(&["input"]))?,
            source,
        )
    }
    /// Resolve a complete source name through the same checked boundary as CLI input.
    pub fn named(input: Text, source: DrugSource) -> Result<Self, QueryInvalid> {
        GetFields {
            input,
            source: Some(source),
            section: Box::default(),
            section_limit: None,
            section_offset: None,
            label_section: None,
            product_number: None,
            decision_from: None,
            decision_to: None,
            document_id: None,
            civic_profile: None,
            civic_kind: None,
            civic_cursor: None,
            evidence_filter: Default::default(),
            find: Box::default(),
            every_version: false,
            partner: None,
        }
        .try_into()
    }
    /// Retry a source-stated alias while preserving the requested sections and filters.
    pub fn with_name(&self, input: Text) -> Result<Self, QueryInvalid> {
        let mut fields = self.fields.clone();
        fields.input = input;
        fields.try_into()
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
    /// Native evidence selectors retained across pages.
    pub fn evidence_filter(&self) -> &crate::CivicEvidenceFilter {
        &self.fields.evidence_filter
    }
    /// Original native cursor for that selected claim collection.
    pub fn civic_cursor(&self) -> Option<&Text> {
        self.fields.civic_cursor.as_ref()
    }
    /// Selected native approval document ID.
    pub fn document_id(&self) -> Option<&Text> {
        self.fields.document_id.as_ref()
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
    /// Literal texts to find in the selected current-label headings.
    pub fn label_find(&self) -> &[Text] {
        &self.fields.find
    }
    /// Known openFDA SPL sets to read together, in caller order.
    pub fn label_batch(&self) -> &[DrugId] {
        match &self.native_id {
            NativeDrugId::LabelBatch(ids) => ids,
            _ => &[],
        }
    }
    /// A label history's texts, whether it reads every version, and its section.
    pub fn label_history(&self) -> (&[Text], bool, Option<super::LabelHistorySection>) {
        let section = super::history::section(self.fields.label_section.as_ref());
        (&self.fields.find, self.fields.every_version, section)
    }
    /// The DDInter drug whose pairs with this drug the interactions section keeps.
    pub fn partner(&self) -> Option<&Text> {
        self.fields.partner.as_ref()
    }
}

impl DrugGet {
    /// Whether the selected source serves every requested disclosure.
    pub fn supports_sections(&self) -> bool {
        self.sections()
            .iter()
            .all(|section| self.source().sections().contains(section))
    }
    /// One request per named section and label heading, in caller order, each keeping this
    /// request's source, identity and section paging; one part for a single section.
    pub fn parts(&self) -> Result<Vec<Self>, QueryInvalid> {
        let mut parts = Vec::new();
        for section in self.sections() {
            let headings = match (section, &self.fields.label_section) {
                (DrugSection::Label, Some(headings)) => headings
                    .as_str()
                    .split(',')
                    .map(|heading| Text::new(heading.trim()).map(Some))
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(|_| QueryInvalid::new(&["label_section"]))?,
                _ => vec![None],
            };
            for heading in headings {
                let mut fields = self.fields.clone();
                fields.section = vec![*section].into();
                fields.label_section = heading;
                parts.push(self.with_fields(fields)?);
            }
        }
        if parts.len() <= 1 {
            return Ok(vec![self.clone()]);
        }
        Ok(parts)
    }
    /// Explicit requested disclosures.
    pub fn sections(&self) -> &[DrugSection] {
        &self.fields.section
    }

    /// Continue the same named section without changing source, identity or section selectors.
    pub fn at_section_offset(&self, offset: u32) -> Result<Self, QueryInvalid> {
        let mut fields = self.fields.clone();
        fields.section_offset = Some(offset);
        self.with_fields(fields)
    }
    /// Label history read in order from version `offset`, continuing a call that stopped.
    pub fn every_version_from(&self, offset: u32) -> Result<Self, QueryInvalid> {
        let mut fields = self.fields.clone();
        (fields.every_version, fields.section_offset) = (true, Some(offset));
        self.with_fields(fields)
    }
    /// Exact selected native label heading.
    pub fn label_section(&self) -> Option<&Text> {
        self.fields.label_section.as_ref()
    }
    /// Inclusive Drugs@FDA submission status date window.
    pub fn decision_window(&self) -> (Option<biodata::PartialDate>, Option<biodata::PartialDate>) {
        (self.fields.decision_from, self.fields.decision_to)
    }
    /// Exact application product selector.
    pub fn product_number(&self) -> Option<&Text> {
        self.fields.product_number.as_ref()
    }

    /// Select one named disclosure without changing the exact source identity.
    pub fn for_section(&self, section: DrugSection) -> Result<Self, QueryInvalid> {
        let mut fields = self.fields.clone();
        fields.section = vec![section].into();
        self.with_fields(fields)
    }

    /// Select an explicit native label heading for the same source identity.
    pub fn for_label(&self, heading: Text) -> Result<Self, QueryInvalid> {
        let mut fields = self.fields.clone();
        fields.section = vec![DrugSection::Label].into();
        fields.label_section = Some(heading);
        self.with_fields(fields)
    }
}

fn selected_source<'de, D: serde::Deserializer<'de>>(d: D) -> Result<Option<DrugSource>, D::Error> {
    DrugSource::deserialize(d).map(Some)
}

#[cfg(feature = "schema")]
impl schemars::JsonSchema for DrugGet {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "DrugGet".into()
    }
    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        let mut schema = GetFields::json_schema(generator);
        schema.insert(
            "description".into(),
            "A source-selected drug request echoed with a native ID after name resolution, otherwise with the name as given.".into(),
        );
        // A name can stay in the echo, so the echo admits every input the parser admits.
        let text = <Text as schemars::JsonSchema>::json_schema(generator);
        if let Some(properties) = schema.get_mut("properties").and_then(|p| p.as_object_mut()) {
            properties.insert(
                "input".into(),
                schemars::json_schema!({
                    "description": "Native source ID after name resolution, otherwise the requested name as given, spaces included.",
                    "pattern": text.get("pattern"),
                    "type": "string",
                })
                .to_value(),
            );
            properties.insert(
                "source".into(),
                generator.subschema_for::<DrugSource>().to_value(),
            );
        }
        if let Some(required) = schema.get_mut("required").and_then(|r| r.as_array_mut()) {
            required.push("source".into());
        }
        schema
    }
}
