use super::DrugSource;
use super::source::{civic_id, inferred_source};
use crate::{CivicClaimKind, QueryInvalid};
use biodata::{DrugId, Text};
use serde::{Deserialize, Serialize};
biodata::canonical_enum! {
    /// Explicit drug disclosures; identity cards never expand into these sections.
    DrugSection, SourceUnknown {
        /// All native identity cross-references and product links, mechanically paged.
        Xrefs => "xrefs",
        /// Native target and mechanism relations.
        Targets => "targets",
        /// Native indication wording.
        Indications => "indications",
        /// Native CIViC therapy evidence and profile context.
        Civic => "civic",
        /// One exact native label heading.
        Label => "label",
        /// Native application and product history rows.
        Approvals => "approvals",
        /// Source-scoped regulator events.
        Regulatory => "regulatory",
        /// Native DDInter partner rows and levels; five rows per default page, with explicit section limits preserved.
        Interactions => "interactions",
    }
}
fn default_source() -> DrugSource {
    DrugSource::Mychem
}
fn default_limit() -> u32 {
    10
}
#[doc = " Search one drug source. FDA drug recalls accept a name or native firm, classification, \
    status, reason and initiation dates; counts are recall records. Other sources require a name."]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "SearchFields")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(with = "SearchFields"))]
pub struct DrugSearch(SearchFields);
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
struct SearchFields {
    /// Literal name phrase; optional for FDA drug recalls. Drugs@FDA searches brands and ingredients,
    /// exact names before word matches. Harmonized recall names match but never become display names.
    #[cfg_attr(feature = "schema", schemars(extend("x-positional" = 0)))]
    #[serde(
        default,
        deserialize_with = "present_query",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(feature = "schema", schemars(with = "Text"))]
    query: Option<Text>,
    /// Literal published recalling firm phrase; drug recalls only.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    firm: Option<Text>,
    /// Native FDA classification; drug recalls only.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    classification: Option<super::DrugRecallClassification>,
    /// Literal native recall status; drug recalls only.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    status: Option<Text>,
    /// Phrase in the published recall reason; drug recalls only.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    reason: Option<Text>,
    /// Inclusive initiation day, YYYY-MM-DD; drug recalls only.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    initiated_from: Option<biodata::PartialDate>,
    /// Inclusive initiation day, YYYY-MM-DD; drug recalls only.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    initiated_to: Option<biodata::PartialDate>,

    /// Exactly one source; default MyChem never fans out.
    #[serde(default = "default_source")]
    source: DrugSource,
    /// Explicit ChEMBL native name selector; omitted means preferred name.
    #[serde(
        default,
        deserialize_with = "selected_name",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(feature = "schema", schemars(with = "super::DrugNameSelector"))]
    name_selector: Option<super::DrugNameSelector>,
    /// Native candidate rows per page.
    #[serde(default = "default_limit")]
    #[cfg_attr(feature = "schema", schemars(range(min = 1, max = 50)))]
    limit: u32,
    /// Native rows to skip.
    #[serde(default)]
    offset: u32,
}
crate::query::validation::checked_query!(DrugSearch, SearchFields);
impl TryFrom<SearchFields> for DrugSearch {
    type Error = QueryInvalid;
    fn try_from(fields: SearchFields) -> Result<Self, Self::Error> {
        if !(1..=50).contains(&fields.limit) {
            return Err(QueryInvalid::new(&["limit"]));
        }
        if fields.name_selector.is_some() && fields.source != DrugSource::Chembl {
            return Err(QueryInvalid::new(&["name_selector", "source"]));
        }
        super::recall::validate_search(
            fields.source,
            fields.query.is_some(),
            fields.firm.is_some()
                || fields.classification.is_some()
                || fields.status.is_some()
                || fields.reason.is_some()
                || fields.initiated_from.is_some()
                || fields.initiated_to.is_some(),
            fields.initiated_from,
            fields.initiated_to,
        )?;
        Ok(Self(fields))
    }
}
impl DrugSearch {
    /// Checked native selector, with preferred name as the ChEMBL default.
    pub fn name_selector(&self) -> super::DrugNameSelector {
        self.0
            .name_selector
            .unwrap_or(super::DrugNameSelector::Preferred)
    }
}
/// Retrieve a native ID or an exact brand or generic name. Brand matches rank first; Drugs@FDA then ranks original applications before ANDAs and earliest approval first; other sources retain search order.
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
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
struct GetFields {
    /// Published source-native identifier or complete brand or generic name.
    #[cfg_attr(feature = "schema", schemars(extend("x-positional" = 0)))]
    input: Text,
    /// Source inferred from an unambiguous requested section; otherwise MyChem.
    #[serde(default, deserialize_with = "selected_source")]
    #[cfg_attr(feature = "schema", schemars(with = "DrugSource", extend("default" = "mychem")))]
    source: Option<DrugSource>,
    /// Explicit named disclosures; empty means identity only.
    #[serde(default, skip_serializing_if = "<[ _ ]>::is_empty")]
    #[cfg_attr(feature = "schema", schemars(extend("x-positional" = 1)))]
    section: Box<[DrugSection]>,
    /// Named section page size, refused on an identity-only card.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(range(min = 1, max = 50)))]
    section_limit: Option<u32>,
    /// Named section continuation, refused on an identity-only card.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    section_offset: Option<u32>,
    /// Exact native label heading, required only for the label section.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    label_section: Option<Text>,
    /// Explicit native application product number, only for approvals.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    product_number: Option<Text>,
    /// Selected native CIViC molecular profile for a drug-scoped claim stream.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(range(min = 1, max = 2147483647)))]
    civic_profile: Option<u64>,
    /// Independent native evidence or assertion collection within the selected profile.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    civic_kind: Option<CivicClaimKind>,
    /// Native continuation cursor for that exact drug, profile and claim collection.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    civic_cursor: Option<Text>,
}
crate::query::validation::checked_query!(DrugGet, GetFields);
impl TryFrom<GetFields> for DrugGet {
    type Error = QueryInvalid;
    fn try_from(mut fields: GetFields) -> Result<Self, Self::Error> {
        let source = fields
            .source
            .unwrap_or_else(|| inferred_source(&fields.section));
        fields.source = Some(source);
        validate_sections(&fields)?;
        let native_id = if !fields
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
        validate_civic(&fields)?;
        Ok(Self { fields, native_id })
    }
}
fn validate_sections(fields: &GetFields) -> Result<(), QueryInvalid> {
    if fields.section_limit.is_some_and(|n| !(1..=50).contains(&n)) {
        return Err(QueryInvalid::new(&["section_limit"]));
    }
    if fields.section.is_empty()
        && (fields.section_limit.is_some() || fields.section_offset.is_some())
    {
        return Err(QueryInvalid::new(&[
            "section",
            "section_limit",
            "section_offset",
        ]));
    }
    if fields.section.contains(&DrugSection::Label) != fields.label_section.is_some() {
        return Err(QueryInvalid::new(&["label_section", "section"]));
    }
    if fields.product_number.is_some() && !fields.section.contains(&DrugSection::Approvals) {
        return Err(QueryInvalid::new(&["product_number", "section"]));
    }
    if fields.source == Some(DrugSource::FdaDrugRecall) && !fields.section.is_empty() {
        return Err(QueryInvalid::new(&["source", "section"]));
    }
    Ok(())
}
fn validate_civic(fields: &GetFields) -> Result<(), QueryInvalid> {
    if fields
        .civic_profile
        .is_some_and(|id| id == 0 || id > i32::MAX as u64)
    {
        return Err(QueryInvalid::new(&["civic_profile"]));
    }
    if fields.civic_profile.is_some() && fields.civic_kind.is_none() {
        return Err(QueryInvalid::new(&["civic_profile", "civic_kind"]));
    }
    if fields.civic_cursor.is_some() && fields.civic_kind.is_none() {
        return Err(QueryInvalid::new(&["civic_cursor", "civic_kind"]));
    }
    if fields.civic_kind.is_some()
        && (fields.source != Some(DrugSource::Civic)
            || !fields.section.contains(&DrugSection::Civic))
    {
        return Err(QueryInvalid::new(&["civic_kind", "source", "section"]));
    }
    Ok(())
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
    pub 
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
        if let Some(properties) = schema.get_mut("properties").and_then(|p| p.as_object_mut()) {
            properties.insert(
                "input".into(),
                generator.subschema_for::<DrugId>().to_value(),
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
