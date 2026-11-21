use super::DrugSource;
use crate::QueryInvalid;
use biodata::Text;
use serde::{Deserialize, Serialize};

fn default_source() -> DrugSource {
    DrugSource::Mychem
}
fn default_limit() -> u32 {
    10
}
/// Drugs@FDA application order; omitted keeps the source's own order.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(with = "str", extend("enum" = ["newest", "original-approval"])))]
pub enum DrugFdaSort {
    /// Latest submission status date first, ties by application number; applications with no
    /// dated submission last.
    Newest,
    /// Original approval date first, ties by application number; applications with no approved
    /// original submission last. BioMCP reads the whole match set and orders it.
    OriginalApproval,
}
/// An openFDA label's FDA application type, read from the prefix of openFDA's application_number.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(with = "str", extend("enum" = ["nda", "anda", "bla"])))]
pub enum LabelApplicationType {
    /// New Drug Application.
    Nda,
    /// Abbreviated New Drug Application, the generics.
    Anda,
    /// Biologics License Application.
    Bla,
}
impl LabelApplicationType {
    /// The application_number prefix openFDA prints for this type.
    pub fn prefix(self) -> &'static str {
        match self {
            Self::Nda => "NDA",
            Self::Anda => "ANDA",
            Self::Bla => "BLA",
        }
    }
}
#[doc = " Search one drug source. FDA drug recalls accept a name or native firm, classification, \
    status, reason and initiation dates; counts are recall records. \
    WHO prequalification accepts a name, area or date range. openFDA labels also accept literal indication or mechanism wording. \
    Drugs@FDA accepts a name or applicant. FDA safety signals accept a drug or report text. \
    Purple Book takes a reference name or BLA and summarizes biosimilars by product."]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "SearchFields")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(with = "SearchFields"))]
pub struct DrugSearch(SearchFields);
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
struct SearchFields {
    /// Drug name phrase. Optional for recalls, safety communications, WHO dates, label indication or mechanism and Drugs@FDA applicant searches.
    /// Drugs@FDA tries exact brand or ingredient names, then words. openFDA labels do likewise for brands and generics, then tries label text.
    /// Recall aliases match without replacing display names.
    /// FDA safety communications match titles, linked names and available page text.
    #[cfg_attr(feature = "schema", schemars(extend("x-positional" = 0)))]
    #[serde(
        default,
        deserialize_with = "present_query",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(feature = "schema", schemars(with = "Text"))]
    query: Option<Text>,
    /// Accepted for one release. Name widening is already automatic where supported.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    expand_drug_names: bool,
    /// Use only the typed name. Default name widening applies to openFDA labels, Drugs@FDA, FDA recalls and safety communications, and EMA.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    exact_term: bool,
    /// Label name page token. Keep the first page's names and step by reusing the token with unchanged filters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    name_snapshot: Option<Text>,
    /// Exact Indications and Usage wording, without disease synonyms. A drug name narrows the search.
    #[serde(
        default,
        deserialize_with = "present_query",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(feature = "schema", schemars(with = "Text"))]
    #[cfg_attr(feature = "schema", schemars(extend("x-required-component" = crate::Source::OpenfdaLabel)))]
    indication: Option<Text>,
    /// Exact Mechanism of Action wording, without synonyms or drug class inference.
    #[serde(
        default,
        deserialize_with = "present_query",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(feature = "schema", schemars(with = "Text"))]
    #[cfg_attr(feature = "schema", schemars(extend("x-required-component" = crate::Source::OpenfdaLabel)))]
    mechanism_of_action: Option<Text>,
    /// Keep label application_number prefixes, e.g. nda,bla to exclude generics (ANDA).
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    #[cfg_attr(feature = "schema", schemars(extend("x-comma-separated" = true)))]
    #[cfg_attr(feature = "schema", schemars(extend("x-required-component" = crate::Source::OpenfdaLabel)))]
    application_type: Box<[LabelApplicationType]>,
    /// sponsor_name prefix. Ignores case, with no company-family or subsidiary inference. A drug name narrows results.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(extend("x-required-component" = crate::Source::Drugsfda)))]
    applicant: Option<Text>,
    /// Application order. newest uses latest submission status date, which may be for a supplement.
    /// original-approval reads all matches and sorts by original_approval. Both put recent dates first,
    /// break ties by application number and put undated rows last. Omit for source order.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(extend("x-required-component" = crate::Source::Drugsfda)))]
    sort: Option<DrugFdaSort>,
    /// Earliest original_approval. Inclusive (YYYY, YYYY-MM or YYYY-MM-DD). Reads all matches and counts those in range.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(extend("x-required-component" = crate::Source::Drugsfda)))]
    approved_from: Option<biodata::PartialDate>,
    /// Latest original_approval. Inclusive (YYYY, YYYY-MM or YYYY-MM-DD).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(extend("x-required-component" = crate::Source::Drugsfda)))]
    approved_to: Option<biodata::PartialDate>,
    /// Exact phrase in the published recall firm name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(extend("x-required-component" = crate::Source::FdaDrugRecall)))]
    firm: Option<Text>,
    /// FDA recall classification.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(extend("x-required-component" = crate::Source::FdaDrugRecall)))]
    classification: Option<super::DrugRecallClassification>,
    /// Literal recall status.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(extend("x-required-component" = crate::Source::FdaDrugRecall)))]
    status: Option<Text>,
    /// Recall reason phrase.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(extend("x-required-component" = crate::Source::FdaDrugRecall)))]
    reason: Option<Text>,
    /// Earliest recall initiation day, inclusive (YYYY-MM-DD).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(extend("x-required-component" = crate::Source::FdaDrugRecall)))]
    initiated_from: Option<biodata::PartialDate>,
    /// Latest recall initiation day, inclusive (YYYY-MM-DD).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(extend("x-required-component" = crate::Source::FdaDrugRecall)))]
    initiated_to: Option<biodata::PartialDate>,

    /// First WHO prequalification date. Inclusive (YYYY, YYYY-MM or YYYY-MM-DD).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(extend("x-required-component" = crate::Source::WhoPq)))]
    prequalified_from: Option<biodata::PartialDate>,
    /// Last WHO prequalification date. Inclusive (YYYY, YYYY-MM or YYYY-MM-DD).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(extend("x-required-component" = crate::Source::WhoPq)))]
    prequalified_to: Option<biodata::PartialDate>,
    /// WHO therapeutic area. Match whole words, ignoring case (e.g. Oncology).
    #[serde(
        default,
        deserialize_with = "present_query",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(feature = "schema", schemars(with = "Text"))]
    #[cfg_attr(feature = "schema", schemars(extend("x-required-component" = crate::Source::WhoPq)))]
    therapeutic_area: Option<Text>,

    /// Return presentation rows, as do --strength and --device.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    #[cfg_attr(feature = "schema", schemars(extend("x-required-component" = crate::Source::PurpleBook)))]
    presentations: bool,
    /// Filter presentations by strength cell, ignoring case and spaces.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(extend("x-required-component" = crate::Source::PurpleBook)))]
    strength: Option<Text>,
    /// Filter presentations by device cell, ignoring case.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(extend("x-required-component" = crate::Source::PurpleBook)))]
    device: Option<Text>,
    /// Drug source. An empty name search may retry names from other sources.
    #[serde(default = "default_source")]
    source: DrugSource,
    /// ChEMBL name to match. Default: preferred name.
    #[serde(
        default,
        deserialize_with = "selected_name",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(feature = "schema", schemars(with = "super::DrugNameSelector"))]
    #[cfg_attr(feature = "schema", schemars(extend("x-required-component" = crate::Source::Chembl)))]
    name_selector: Option<super::DrugNameSelector>,
    /// Rows per page. Purple Book product summaries always return the full roster.
    #[serde(default = "default_limit")]
    #[cfg_attr(feature = "schema", schemars(range(min = 1, max = 50)))]
    limit: u32,
    /// Rows to skip. Purple Book applies this only to presentation rows.
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
        if fields.indication.is_some() && fields.source != DrugSource::OpenfdaLabel {
            return Err(QueryInvalid::new(&["indication", "source"]));
        }
        if fields.mechanism_of_action.is_some() && fields.source != DrugSource::OpenfdaLabel {
            return Err(QueryInvalid::new(&["mechanism_of_action", "source"]));
        }
        if !fields.application_type.is_empty() && fields.source != DrugSource::OpenfdaLabel {
            return Err(QueryInvalid::new(&["application_type", "source"]));
        }
        if fields.name_selector.is_some() && fields.source != DrugSource::Chembl {
            return Err(QueryInvalid::new(&["name_selector", "source"]));
        }
        if fields.applicant.is_some() && fields.source != DrugSource::Drugsfda {
            return Err(QueryInvalid::new(&["applicant", "source"]));
        }
        if fields.sort.is_some() && fields.source != DrugSource::Drugsfda {
            return Err(QueryInvalid::new(&["sort", "source"]));
        }
        validate_purple_book_selection(&fields)?;
        validate_approved(&fields)?;
        let who_dates = fields.prequalified_from.is_some() || fields.prequalified_to.is_some();
        if who_dates && fields.source != DrugSource::WhoPq {
            return Err(QueryInvalid::new(&[
                "prequalified_from",
                "prequalified_to",
                "source",
            ]));
        }
        if fields.therapeutic_area.is_some() && fields.source != DrugSource::WhoPq {
            return Err(QueryInvalid::new(&["therapeutic_area", "source"]));
        }
        let who_filters = who_dates || fields.therapeutic_area.is_some();
        if fields.source == DrugSource::WhoPq && fields.query.is_none() && !who_filters {
            return Err(QueryInvalid::new(&[
                "query",
                "therapeutic_area",
                "prequalified_from",
                "prequalified_to",
            ])
            .with_message(crate::ErrorMessage::WhoSearchFilters));
        }
        if fields
            .prequalified_from
            .zip(fields.prequalified_to)
            .is_some_and(|(a, b)| {
                (a.year(), a.month().unwrap_or(1), a.day().unwrap_or(1))
                    > (b.year(), b.month().unwrap_or(12), b.day().unwrap_or(31))
            })
        {
            return Err(QueryInvalid::new(&["prequalified_from", "prequalified_to"]));
        }
        super::recall::validate_search(
            fields.source,
            fields.query.is_some()
                || fields.indication.is_some()
                || fields.mechanism_of_action.is_some()
                || fields.applicant.is_some()
                || (fields.source == DrugSource::WhoPq && who_filters)
                // FDA's safety communications list newest first with no name.
                || fields.source == DrugSource::FdaDsc,
            fields.firm.is_some()
                || fields.classification.is_some()
                || fields.status.is_some()
                || fields.reason.is_some()
                || fields.initiated_from.is_some()
                || fields.initiated_to.is_some(),
            fields.initiated_from,
            fields.initiated_to,
        )?;
        let query = Self(fields);
        validate_names(&query)?;
        Ok(query)
    }
}
impl DrugSearch {
    /// Retry a source-stated name while preserving every native filter and page selector.
    pub fn with_name(&self, name: Text) -> Self {
        let mut query = self.clone();
        query.0.query = Some(name);
        query
    }
    /// Select the native ChEMBL synonym filter without changing the request's other fields.
    pub fn with_name_selector(
        &self,
        selector: super::DrugNameSelector,
    ) -> Result<Self, QueryInvalid> {
        let mut fields = self.0.clone();
        fields.name_selector = Some(selector);
        fields.try_into()
    }
    /// Whether the caller chose a literal ChEMBL name filter.
    pub fn explicit_name_selector(&self) -> bool {
        self.0.name_selector.is_some()
    }
    /// Inclusive WHO prequalification lower bound.
    pub fn prequalified_from(&self) -> Option<biodata::PartialDate> {
        self.0.prequalified_from
    }
    /// Inclusive WHO prequalification upper bound.
    pub fn prequalified_to(&self) -> Option<biodata::PartialDate> {
        self.0.prequalified_to
    }
    /// Words of WHO's own Therapeutic Area column.
    pub fn therapeutic_area(&self) -> Option<&Text> {
        self.0.therapeutic_area.as_ref()
    }
    /// Construct a literal name search through the same checked fields as every input surface.
    pub fn named(query: Text, source: DrugSource, limit: u32) -> Result<Self, QueryInvalid> {
        SearchFields {
            query: Some(query),
            expand_drug_names: false,
            exact_term: false,
            name_snapshot: None,
            indication: None,
            mechanism_of_action: None,
            application_type: Box::default(),
            source,
            limit,
            offset: 0,
            applicant: None,
            sort: None,
            approved_from: None,
            approved_to: None,
            firm: None,
            classification: None,
            status: None,
            reason: None,
            initiated_from: None,
            initiated_to: None,
            prequalified_from: None,
            prequalified_to: None,
            therapeutic_area: None,
            presentations: false,
            strength: None,
            device: None,
            name_selector: None,
        }
        .try_into()
    }
    /// Checked native selector, with preferred name as the ChEMBL default.
    pub fn name_selector(&self) -> super::DrugNameSelector {
        self.0
            .name_selector
            .unwrap_or(super::DrugNameSelector::Preferred)
    }
}
fn validate_purple_book_selection(fields: &SearchFields) -> Result<(), QueryInvalid> {
    if (fields.presentations || fields.strength.is_some() || fields.device.is_some())
        && fields.source != DrugSource::PurpleBook
    {
        return Err(QueryInvalid::new(&[
            "presentations",
            "strength",
            "device",
            "source",
        ]));
    }
    Ok(())
}
/// The original approval window: Drugs@FDA only, and never ending before it starts.
fn validate_approved(fields: &SearchFields) -> Result<(), QueryInvalid> {
    let window = fields.approved_from.is_some() || fields.approved_to.is_some();
    if window && fields.source != DrugSource::Drugsfda {
        return Err(QueryInvalid::new(&[
            "approved_from",
            "approved_to",
            "source",
        ]));
    }
    if fields
        .approved_from
        .zip(fields.approved_to)
        .is_some_and(|(a, b)| {
            (a.year(), a.month().unwrap_or(1), a.day().unwrap_or(1))
                > (b.year(), b.month().unwrap_or(12), b.day().unwrap_or(31))
        })
    {
        return Err(QueryInvalid::new(&["approved_from", "approved_to"]));
    }
    Ok(())
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
    /// Whether the Purple Book search returns presentation rows.
    pub fn purple_book_presentations(&self) -> bool {
        self.0.presentations || self.0.strength.is_some() || self.0.device.is_some()
    }
    /// Switch a Purple Book summary to the first page of its FDA presentation rows.
    pub fn purple_book_details(&self) -> Result<Self, QueryInvalid> {
        let mut fields = self.0.clone();
        fields.presentations = true;
        fields.offset = 0;
        fields.try_into()
    }
    /// Exact Purple Book strength cell.
    pub fn strength(&self) -> Option<&Text> {
        self.0.strength.as_ref()
    }
    /// Exact Purple Book device or other product presentation.
    pub fn device(&self) -> Option<&Text> {
        self.0.device.as_ref()
    }
    /// Literal source query, without rewriting.
    pub fn query(&self) -> Option<&Text> {
        self.0.query.as_ref()
    }
    /// Literal native Indications and Usage phrase; never an inferred indication.
    pub fn indication(&self) -> Option<&Text> {
        self.0.indication.as_ref()
    }
    /// Literal native Mechanism of Action phrase; never an inferred drug class.
    pub fn mechanism_of_action(&self) -> Option<&Text> {
        self.0.mechanism_of_action.as_ref()
    }
    /// openFDA label application types to keep; empty keeps every label.
    pub fn application_type(&self) -> &[LabelApplicationType] {
        &self.0.application_type
    }
    /// Start of a Drugs@FDA sponsor name, matched by the source without letter case.
    pub fn applicant(&self) -> Option<&Text> {
        self.0.applicant.as_ref()
    }
    /// Requested Drugs@FDA application order; none keeps the source order.
    pub fn sort(&self) -> Option<DrugFdaSort> {
        self.0.sort
    }
    /// Inclusive lower bound on a Drugs@FDA application's original approval date.
    pub fn approved_from(&self) -> Option<biodata::PartialDate> {
        self.0.approved_from
    }
    /// Inclusive upper bound on a Drugs@FDA application's original approval date.
    pub fn approved_to(&self) -> Option<biodata::PartialDate> {
        self.0.approved_to
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
    fn clear_page_context(&mut self) {
        self.0.name_snapshot = None;
    }
}

/// Sources that search a typed drug name together with the drug's other names by default.
const WIDENING: [DrugSource; 5] = [
    DrugSource::OpenfdaLabel,
    DrugSource::Drugsfda,
    DrugSource::FdaDrugRecall,
    DrugSource::Ema,
    DrugSource::FdaDsc,
];

fn widens(fields: &SearchFields) -> bool {
    !fields.exact_term
        && WIDENING.contains(&fields.source)
        && fields.query.as_ref().is_some_and(|name| {
            // A native record identifier names one record, never a drug with other names.
            match name.as_str().parse::<biodata::DrugId>() {
                Ok(id) => !DrugSource::Drugsfda.accepts(&id) && !fields.source.accepts(&id),
                Err(_) => true,
            }
        })
}

fn validate_names(query: &DrugSearch) -> Result<(), QueryInvalid> {
    let fields = &query.0;
    if fields.expand_drug_names && fields.exact_term {
        return Err(QueryInvalid::new(&["expand_drug_names", "exact_term"]));
    }
    if fields.expand_drug_names && !widens(fields) {
        return Err(QueryInvalid::new(&["source", "query", "expand_drug_names"]));
    }
    if fields.name_snapshot.is_some() && !widens(fields) {
        return Err(QueryInvalid::new(&["exact_term", "name_snapshot"]));
    }
    Ok(())
}

impl DrugSearch {
    /// Whether the caller asked for the typed name only.
    pub fn exact_term(&self) -> bool {
        self.0.exact_term
    }
    /// Whether this search takes the typed name together with the drug's other names, as the
    /// widening sources do unless the caller asked for the typed name only.
    pub fn widens_drug_names(&self) -> bool {
        widens(&self.0)
    }
    /// Caller-authorized retained names and native discovery step.
    pub fn name_snapshot(&self) -> Option<&Text> {
        self.0.name_snapshot.as_ref()
    }
    /// Freeze the resolved names and discovery step in subsequent native pages.
    pub fn with_name_snapshot(&self, snapshot: Text) -> Self {
        let mut query = self.clone();
        query.0.name_snapshot = Some(snapshot);
        query
    }
}
