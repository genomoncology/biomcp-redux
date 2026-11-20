mod id;
mod search_validation;
use super::QueryInvalid;
use biodata::{DiagnosticDeviceId, FdaDrugApplication, GeneSymbol, NcbiGeneId, PartialDate, Text};
pub use id::DiagnosticId;
use search_validation::{validate_condition, validate_device, validate_recall, validate_who_years};
use serde::{Deserialize, Serialize};
mod get;
pub use get::DiagnosticGet;

biodata::wire_enum! {
    /// One explicit diagnostic discovery source.
    DiagnosticSearchSource { #[doc = "Native Gtr spelling."] Gtr => "gtr",
 #[doc = "Native FdaCdx spelling."] FdaCdx => "fda-cdx",
 #[doc = "Native PMA submission records."] FdaPma => "fda-pma",
 #[doc = "Native 510(k) clearance records."] Fda510k => "fda-510k",
 #[doc = "Stored WHO product headers."] WhoIvd => "who-ivd",
 #[doc = "Native recalls for every medical device."] FdaDeviceRecall => "fda-device-recall" }
}
biodata::wire_enum! {
    /// One explicit exact diagnostic record source.
    DiagnosticGetSource { #[doc = "Native Gtr spelling."] Gtr => "gtr",
 #[doc = "Native WhoIvd spelling."] WhoIvd => "who-ivd",
 #[doc = "Native FdaPma spelling."] FdaPma => "fda-pma",
 #[doc = "Native 510(k) clearance spelling."] Fda510k => "fda-510k",
 #[doc = "Exact native medical device recall."] FdaDeviceRecall => "fda-device-recall" }
}
biodata::wire_enum! {
    /// FDA companion-table row order over every matching row of the stored snapshot.
    DiagnosticSort { #[doc = "Newest row date first; undated rows last."] Date => "date" }
}
biodata::wire_enum! {
    described
    /// Source-supported diagnostic sections; no all section.
    DiagnosticSection {
        #[doc = "GTR native gene rows; five per page."] Genes => "genes",
        #[doc = "GTR native condition rows; five per page."] Conditions => "conditions",
        #[doc = "GTR native method entries; five per page."] Methods => "methods",
        #[doc = "FDA SSED PDF text: 25 MiB, 100 pages and 6144 escaped bytes per fragment; scans without text are explicit."] Summary => "summary",
        #[doc = "Selected PMA submission statement; one fragment per page."] Regulatory => "regulatory",
        #[doc = "PMA base application supplement headers; five per page."] Supplements => "supplements"
    }
}
biodata::wire_enum! {
    described
    /// Source-native count of matching 510(k) records.
    DiagnosticAggregate {
        #[doc = "Matching 510(k) records by FDA product code, from openFDA's count facet."] ProductCode => "product-code"
    }
}
fn limit() -> u32 {
    5
}
// Sources that answer this field; it stays offered while any of them is on.
#[cfg(feature = "schema")]
const MANUFACTURER_SOURCES: [crate::Source; 4] = [
    crate::Source::WhoIvd,
    crate::Source::FdaPma,
    crate::Source::Fda510k,
    crate::Source::FdaDeviceRecall,
];
// Sources that answer this field; it stays offered while any of them is on.
#[cfg(feature = "schema")]
const PRODUCT_CODE_SOURCES: [crate::Source; 3] = [
    crate::Source::FdaPma,
    crate::Source::Fda510k,
    crate::Source::FdaDeviceRecall,
];
// Sources that answer this field; it stays offered while any of them is on.
#[cfg(feature = "schema")]
const NAME_SOURCES: [crate::Source; 5] = [
    crate::Source::Gtr,
    crate::Source::FdaPma,
    crate::Source::Fda510k,
    crate::Source::FdaDeviceRecall,
    crate::Source::FdaCdx,
];
/// Search five source records by default. FDA device recalls cover every medical device and show class; totals count recall records, without harm inference.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "SearchFields")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(with = "SearchFields"))]
pub struct DiagnosticSearch(SearchFields);
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
struct SearchFields {
    #[doc = "Native diagnostic source.\nExplicit native GTR, WHO IVD, FDA PMA, 510(k), companion or medical device recall discovery."]
    source: DiagnosticSearchSource,
    #[doc = "Exact NCBI GeneID.\nExact source GeneID, exclusive with gene symbol."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(extend("x-required-component" = crate::Source::Gtr)))]
    gene_id: Option<NcbiGeneId>,
    #[doc = "Exact gene symbol.\nExact source gene_symbol field, without alias expansion."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(extend("x-required-component" = crate::Source::Gtr)))]
    gene: Option<GeneSymbol>,
    #[doc = "Exact MedGen:CUI.\nExact MedGen:CUI identifier only; with a gene, tests that list both."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(regex(pattern = "^MedGen:C[0-9]+$")))]
    #[cfg_attr(feature = "schema", schemars(extend("x-required-component" = crate::Source::Gtr)))]
    condition_id: Option<Text>,
    #[doc = "Condition name text.\nText in GTR condition_identifiers, the test's condition names, matched like the GTR --name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(extend("x-required-component" = crate::Source::Gtr)))]
    condition: Option<Text>,
    #[doc = "Native test_type.\nExact native test_type value."]
    #[serde(default, rename = "type", skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(extend("x-required-component" = crate::Source::Gtr)))]
    test_type: Option<biodata::GtrTestType>,
    #[doc = "PMA or K identifier.\nExact PMA application, application/supplement or native K identity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(extend("x-required-component" = crate::Source::FdaCdx)))]
    device_id: Option<DiagnosticDeviceId>,
    #[doc = "NDA/BLA application.\nCell-owned typed NDA/BLA application."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(extend("x-required-component" = crate::Source::FdaCdx)))]
    drug_application: Option<FdaDrugApplication>,
    #[doc = "Biomarker name in a cell.\nWhole name in the individual table's biomarker cell, any case, \
    surrounding spaces ignored: PIK3CA matches \"PIK3CA, AKT1, and PTEN\", not PIK3CA2. \
    A cell naming it only as \"Not NAME\", such as \"Not MSI-High\", is returned with biomarker_negated: true. \
    The group table is not searched; problems name it as filter.unsupported."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(extend("x-required-component" = crate::Source::FdaCdx)))]
    biomarker: Option<Text>,
    #[doc = "First row date, YYYY-MM-DD.\nInclusive newest date in an FDA companion row's device cell; fda-cdx only. \
    Undated rows are left out and named in problems."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(extend("x-required-component" = crate::Source::FdaCdx)))]
    date_from: Option<PartialDate>,
    #[doc = "Last row date, YYYY-MM-DD.\nInclusive newest date in an FDA companion row's device cell; fda-cdx only."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(extend("x-required-component" = crate::Source::FdaCdx)))]
    date_to: Option<PartialDate>,
    #[doc = "Row order.\nNewest row date first over all matching rows, not one page; fda-cdx only. Without it rows keep FDA's order."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(extend("x-required-component" = crate::Source::FdaCdx)))]
    sort: Option<DiagnosticSort>,
    #[doc = "WHO marker phrase.\nLiteral whole-word phrase in WHO's Pathogen/Disease/Marker cell. WHO rows carry prequalification_year."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(extend("x-required-component" = crate::Source::WhoIvd)))]
    marker: Option<Text>,
    #[doc = "Manufacturer phrase.\nWHO manufacturer words, native FDA applicant (PMA/510(k)) or recalling_firm phrase."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(extend("x-required-component" = MANUFACTURER_SOURCES)))]
    manufacturer: Option<Text>,
    #[doc = "Recall firm phrase.\nThe drug recall search's name for --manufacturer on FDA device recalls: \
    the same recalling_firm phrase; fda-device-recall only."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(extend("x-required-component" = crate::Source::FdaDeviceRecall)))]
    #[cfg_attr(feature = "schema", schemars(extend("x-alias-of" = "manufacturer")))]
    firm: Option<Text>,
    #[doc = "FDA product code.\nExact native FDA product code; PMA, 510(k) and device recalls only."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(extend("x-required-component" = PRODUCT_CODE_SOURCES)))]
    product_code: Option<Text>,
    #[doc = "First date, YYYY-MM-DD.\nInclusive initiation date, YYYY-MM-DD; FDA device recalls only."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(extend("x-required-component" = crate::Source::FdaDeviceRecall)))]
    initiated_from: Option<PartialDate>,
    #[doc = "Last date, YYYY-MM-DD.\nInclusive initiation date, YYYY-MM-DD; FDA device recalls only."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(extend("x-required-component" = crate::Source::FdaDeviceRecall)))]
    initiated_to: Option<PartialDate>,
    #[doc = "FDA recall class.\nNative enforcement classification; FDA device recalls only. Selects enforcement records, which carry no product code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(extend("x-required-component" = crate::Source::FdaDeviceRecall)))]
    classification: Option<crate::DrugRecallClassification>,
    #[doc = "Native test or device name.\nPhrase in PMA trade_name, 510(k) device_name or recall product_description; text in GTR lab_test_name, \
    ignoring letter case; hyphens and spaces in your text are optional. FDA CDx: whole name, any case, in a row's diagnostic or drug cell; \
    name_match says which."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(extend("x-required-component" = NAME_SOURCES)))]
    name: Option<Text>,
    #[doc = "Intended-use phrase.\nPhrase in FDA PMA ao_statement, the approval order's intended use; PMA only, as openFDA 510(k) has no intended-use text."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(extend("x-required-component" = crate::Source::FdaPma)))]
    intended_use: Option<Text>,
    #[doc = "FDA classification name phrase.\nPhrase in openFDA's 510(k) openfda.device_name, FDA's name for the product code's device type, \
    not the submitted device_name --name matches; 510(k) only."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(extend("x-required-component" = crate::Source::Fda510k)))]
    classification_name: Option<Text>,
    #[doc = "First date, YYYY-MM-DD.\nInclusive native decision date, YYYY-MM-DD; PMA and 510(k) only."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(extend("x-required-component" = [crate::Source::FdaPma, crate::Source::Fda510k])))]
    decision_from: Option<PartialDate>,
    #[doc = "Last date, YYYY-MM-DD.\nInclusive native decision date, YYYY-MM-DD; PMA and 510(k) only."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(extend("x-required-component" = [crate::Source::FdaPma, crate::Source::Fda510k])))]
    decision_to: Option<PartialDate>,
    #[doc = "WHO IVD year.\nFirst prequalification year included, YYYY; WHO publishes only the year."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(extend("x-required-component" = crate::Source::WhoIvd)))]
    prequalified_from: Option<PartialDate>,
    #[doc = "WHO IVD year.\nLast prequalification year included, YYYY; WHO publishes only the year."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(extend("x-required-component" = crate::Source::WhoIvd)))]
    prequalified_to: Option<PartialDate>,
    #[doc = "Count by product code.\nAlso counts every matching record by FDA product code, up to openFDA's 1,000 codes, \
    and says whether the counts sum to the total; fda-510k only, first page."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(extend("x-required-component" = crate::Source::Fda510k)))]
    aggregate: Option<DiagnosticAggregate>,
    /// Five results by default; at most fifty source rows or product headers.
    #[serde(default = "limit")]
    #[cfg_attr(feature = "schema", schemars(range(min = 1, max = 50)))]
    limit: u32,
    /// Zero-based source result offset.
    #[serde(default)]
    offset: u32,
    #[doc = "Stored bundle revision.\nImmutable stored bundle revision retained by a continuation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    bundle_revision: Option<Text>,
    /// The caller typed --firm, which folds into manufacturer, so a notice names --firm (0976).
    #[serde(skip)]
    firm_typed: bool,
}
crate::query::validation::checked_query!(DiagnosticSearch, SearchFields);
impl TryFrom<SearchFields> for DiagnosticSearch {
    type Error = QueryInvalid;
    fn try_from(mut v: SearchFields) -> Result<Self, Self::Error> {
        search_validation::fold_firm(&mut v)?;
        let gtr = v.gene_id.is_some()
            || v.gene.is_some()
            || v.condition_id.is_some()
            || v.condition.is_some()
            || v.test_type.is_some();
        let cdx = v.device_id.is_some() || v.drug_application.is_some() || v.biomarker.is_some();
        let who = v.marker.is_some() || v.manufacturer.is_some();
        let dates = v.initiated_from.is_some() || v.initiated_to.is_some();
        validate_device(&v, gtr || cdx || v.marker.is_some() || dates)?;
        validate_recall(&v, gtr || cdx)?;
        if !(1..=50).contains(&v.limit) {
            return Err(QueryInvalid::new(&["limit"]));
        }
        // A gene and a condition together keep tests that list both (0949).
        if v.gene.is_some() && v.gene_id.is_some() {
            return Err(QueryInvalid::new(&["gene", "gene_id"])
                .with_message(crate::ErrorMessage::DiagnosticGeneOrGeneId));
        }
        if v.source == DiagnosticSearchSource::Gtr && (cdx || who) {
            return Err(QueryInvalid::new(&[
                "source",
                "device_id",
                "drug_application",
                "biomarker",
                "marker",
                "manufacturer",
            ]));
        }
        if v.source == DiagnosticSearchSource::FdaCdx && (gtr || who) {
            return Err(QueryInvalid::new(&[
                "source",
                "gene_id",
                "gene",
                "condition_id",
                "condition",
                "type",
                "marker",
                "manufacturer",
            ]));
        }
        if v.source == DiagnosticSearchSource::WhoIvd && (gtr || cdx) {
            return Err(QueryInvalid::new(&[
                "source",
                "gene_id",
                "gene",
                "condition_id",
                "condition",
                "type",
                "device_id",
                "drug_application",
                "biomarker",
            ]));
        }
        if v.source == DiagnosticSearchSource::WhoIvd
            && [v.marker.as_ref(), v.manufacturer.as_ref()]
                .into_iter()
                .flatten()
                .any(|v| !v.as_str().chars().any(char::is_alphanumeric))
        {
            return Err(QueryInvalid::new(&["marker", "manufacturer"]));
        }
        validate_who_years(&v)?;
        search_validation::validate_aggregate(&v)?;
        if v.device_id
            .as_ref()
            .is_some_and(|id| !(id.as_str().starts_with('P') || id.as_str().starts_with('K')))
        {
            return Err(QueryInvalid::new(&["device_id"]));
        }
        validate_condition(&v)?;
        if v.condition_id.as_ref().is_some_and(|id| {
            id.as_str()
                .strip_prefix("MedGen:C")
                .is_none_or(|id| id.is_empty() || !id.bytes().all(|b| b.is_ascii_digit()))
        }) {
            return Err(QueryInvalid::new(&["condition_id"]));
        }
        Ok(Self(v))
    }
}
impl DiagnosticSearch {
    /// Search companion diagnostics for one checked native device identity.
    pub fn companion(device_id: DiagnosticDeviceId) -> Result<Self, QueryInvalid> {
        Self::try_from(SearchFields {
            source: DiagnosticSearchSource::FdaCdx,
            device_id: Some(device_id),
            gene_id: None,
            gene: None,
            condition_id: None,
            condition: None,
            test_type: None,
            drug_application: None,
            biomarker: None,
            date_from: None,
            date_to: None,
            sort: None,
            marker: None,
            manufacturer: None,
            firm: None,
            name: None,
            intended_use: None,
            classification_name: None,
            decision_from: None,
            decision_to: None,
            product_code: None,
            initiated_from: None,
            initiated_to: None,
            classification: None,
            prequalified_from: None,
            prequalified_to: None,
            aggregate: None,
            limit: limit(),
            offset: 0,
            bundle_revision: None,
            firm_typed: false,
        })
    }

    /// Explicit source, without fallback.
    pub fn source(&self) -> DiagnosticSearchSource {
        self.0.source
    }
    /// Exact source GeneID.
    pub fn gene_id(&self) -> Option<NcbiGeneId> {
        self.0.gene_id
    }
    /// Exact source symbol.
    pub fn gene(&self) -> Option<&GeneSymbol> {
        self.0.gene.as_ref()
    }
    /// Exact source MedGen CUI.
    pub fn condition_id(&self) -> Option<&Text> {
        self.0.condition_id.as_ref()
    }
    /// GTR condition name text, matched like the GTR test name.
    pub fn condition(&self) -> Option<&Text> {
        self.0.condition.as_ref()
    }
    /// Exact native test type.
    pub fn test_type(&self) -> Option<biodata::GtrTestType> {
        self.0.test_type
    }
    /// Selected PMA base, supplement or native K identity.
    pub fn device_id(&self) -> Option<&DiagnosticDeviceId> {
        self.0.device_id.as_ref()
    }
    /// Owned NDA/BLA application.
    pub fn drug_application(&self) -> Option<&FdaDrugApplication> {
        self.0.drug_application.as_ref()
    }
    /// Whole exact biomarker cell.
    pub fn biomarker(&self) -> Option<&Text> {
        self.0.biomarker.as_ref()
    }
    /// Inclusive FDA companion row-date lower bound.
    pub fn date_from(&self) -> Option<PartialDate> {
        self.0.date_from
    }
    /// Inclusive FDA companion row-date upper bound.
    pub fn date_to(&self) -> Option<PartialDate> {
        self.0.date_to
    }
    /// Whole-snapshot FDA companion row order, when asked.
    pub fn sort(&self) -> Option<DiagnosticSort> {
        self.0.sort
    }
    /// Literal WHO marker words.
    pub fn marker(&self) -> Option<&Text> {
        self.0.marker.as_ref()
    }
    /// WHO manufacturer words, native FDA applicant (PMA/510(k)) or recalling_firm phrase.
    pub fn manufacturer(&self) -> Option<&Text> {
        self.0.manufacturer.as_ref()
    }
    /// The option the caller typed for the manufacturer: --firm on a device recall, else
    /// --manufacturer.
    pub fn manufacturer_option(&self) -> &'static str {
        if self.0.firm_typed {
            "--firm"
        } else {
            "--manufacturer"
        }
    }
    /// Exact native FDA product code.
    pub fn product_code(&self) -> Option<&Text> {
        self.0.product_code.as_ref()
    }
    /// Inclusive source initiation range.
    pub fn initiated_from(&self) -> Option<PartialDate> {
        self.0.initiated_from
    }
    /// Inclusive source initiation range.
    pub fn initiated_to(&self) -> Option<PartialDate> {
        self.0.initiated_to
    }
    /// Native enforcement classification selecting device recalls.
    pub fn classification(&self) -> Option<crate::DrugRecallClassification> {
        self.0.classification
    }
    /// Native device name phrase, device recall product description phrase or GTR test name text,
    /// ignoring letter case; hyphens and spaces in your text are optional; FDA CDx diagnostic or drug cell name.
    pub fn name(&self) -> Option<&Text> {
        self.0.name.as_ref()
    }
    /// Phrase in the FDA PMA approval-order statement (ao_statement).
    pub fn intended_use(&self) -> Option<&Text> {
        self.0.intended_use.as_ref()
    }
    /// Phrase in FDA's 510(k) classification name (openFDA's harmonized openfda.device_name).
    pub fn classification_name(&self) -> Option<&Text> {
        self.0.classification_name.as_ref()
    }
    /// Inclusive native decision range lower bound.
    pub fn decision_from(&self) -> Option<PartialDate> {
        self.0.decision_from
    }
    /// Inclusive native decision range upper bound.
    pub fn decision_to(&self) -> Option<PartialDate> {
        self.0.decision_to
    }
    /// Inclusive WHO IVD prequalification year lower bound.
    pub fn prequalified_from(&self) -> Option<PartialDate> {
        self.0.prequalified_from
    }
    /// Inclusive WHO IVD prequalification year upper bound.
    pub fn prequalified_to(&self) -> Option<PartialDate> {
        self.0.prequalified_to
    }
    /// Count asked beside the page, when any.
    pub fn aggregate(&self) -> Option<DiagnosticAggregate> {
        self.0.aggregate
    }
    /// The same search without its count, for the next page of records.
    pub fn without_aggregate(&self) -> Self {
        let mut v = self.clone();
        v.0.aggregate = None;
        v
    }
    /// The same first page with its --name sent as the FDA applicant (--manufacturer) (1040).
    pub fn name_as_manufacturer(&self) -> Result<Self, QueryInvalid> {
        let mut v = self.0.clone();
        if v.manufacturer.is_some() {
            return Err(QueryInvalid::new(&["manufacturer"]));
        }
        v.manufacturer = v.name.take();
        v.offset = 0;
        v.try_into()
    }
    /// Native rows per page.
    pub fn limit(&self) -> u32 {
        self.0.limit
    }
    /// Native row offset.
    pub fn offset(&self) -> u32 {
        self.0.offset
    }
    /// Selected immutable bundle revision.
    pub fn bundle_revision(&self) -> Option<&Text> {
        self.0.bundle_revision.as_ref()
    }

    /// Bind a stored query to its immutable source snapshot.
    pub fn with_bundle_revision(&self, revision: Text) -> Self {
        let mut v = self.clone();
        v.0.bundle_revision = Some(revision);
        v
    }
}
impl crate::Paged for DiagnosticSearch {
    type Offset = u32;
    fn set_page_offset(&mut self, offset: Self::Offset) {
        self.0.offset = offset;
    }
}

impl DiagnosticGetSource {
    /// Source identity for this explicit native selector.
    pub const fn source(self) -> crate::Source {
        match self {
            Self::Gtr => crate::Source::Gtr,
            Self::WhoIvd => crate::Source::WhoIvd,
            Self::FdaPma => crate::Source::FdaPma,
            Self::Fda510k => crate::Source::Fda510k,
            Self::FdaDeviceRecall => crate::Source::FdaDeviceRecall,
        }
    }
}

impl DiagnosticSearchSource {
    /// Source identity for this explicit native selector.
    pub const fn source(self) -> crate::Source {
        match self {
            Self::Gtr => crate::Source::Gtr,
            Self::FdaCdx => crate::Source::FdaCdx,
            Self::FdaPma => crate::Source::FdaPma,
            Self::Fda510k => crate::Source::Fda510k,
            Self::WhoIvd => crate::Source::WhoIvd,
            Self::FdaDeviceRecall => crate::Source::FdaDeviceRecall,
        }
    }
}

impl DiagnosticGetSource {
    /// Named disclosures implemented by the exact-record endpoint.
    pub const fn sections(self) -> &'static [DiagnosticSection] {
        use DiagnosticSection::{Conditions, Genes, Methods, Regulatory, Summary, Supplements};
        match self {
            Self::Gtr => &[Genes, Conditions, Methods],
            Self::FdaPma => &[Regulatory, Supplements, Summary],
            Self::WhoIvd | Self::Fda510k | Self::FdaDeviceRecall => &[],
        }
    }
}
