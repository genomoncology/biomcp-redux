use super::{DiagnosticGetSource, DiagnosticSection, QueryInvalid, limit};
use biodata::{GtrTestId, PmaNumber, PmaSupplement, Text, WhoProductId};
use serde::{Deserialize, Serialize};
/// Get one exact source record or supported section. A listing is not authorization; a companion row is not a treatment recommendation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "GetFields")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(with = "GetFields"))]
pub struct DiagnosticGet {
    #[serde(flatten)]
    fields: GetFields,
    #[serde(skip)]
    recall_number: Option<biodata::DeviceRecallNumber>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
struct GetFields {
    /// Exact native identity, including version for GTR.
    #[cfg_attr(feature = "schema", schemars(extend("x-positional" = 0)))]
    id: Text,
    /// Explicit source; gtr needs accession/version, who-ivd a known Product ID, fda-pma a P number, fda-510k a K number.
    source: DiagnosticGetSource,
    /// gtr: genes, conditions, methods. fda-pma: regulatory, supplements. WHO and 510(k) have no sections; omit for the card.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(extend("x-positional" = 1)))]
    section: Option<DiagnosticSection>,
    /// Exact native PMA supplement; absent selects the base row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    supplement: Option<PmaSupplement>,
    /// Maximum native rows; omitted means five rows, or one regulatory fragment.
    #[serde(
        default,
        deserialize_with = "super::super::gene::present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(feature = "schema", schemars(with = "u32", range(min = 1, max = 50)))]
    section_limit: Option<u32>,
    /// Zero-based native row or fragment offset.
    #[serde(default)]
    section_offset: u32,
    /// Immutable stored bundle revision retained by a continuation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    bundle_revision: Option<Text>,
}
crate::query::validation::checked_query!(DiagnosticGet, GetFields);
impl TryFrom<GetFields> for DiagnosticGet {
    type Error = QueryInvalid;
    fn try_from(v: GetFields) -> Result<Self, Self::Error> {
        let recall_number = if v.source == DiagnosticGetSource::FdaDeviceRecall {
            Some(v.id.as_str().parse().map_err(|_| QueryInvalid::new(&["id", "source"]))?)
        } else { None };
        validate_source(&v)?;
        if v.section_limit.is_some_and(|v| !(1..=50).contains(&v)) {
            return Err(QueryInvalid::new(&["section_limit"]));
        }
        Ok(Self { fields: v, recall_number })
    }
}
impl DiagnosticGet {
    /// Exact checked source identity.
    pub fn id(&self) -> &Text {
        &self.fields.id
    }
    /// Native recall identity parsed once by the checked input boundary.
    pub fn recall_number(&self) -> Option<&biodata::DeviceRecallNumber> { self.recall_number.as_ref() }
    /// Explicit selected source.
    pub fn source(&self) -> DiagnosticGetSource {
        self.fields.source
    }
    /// One supported named page.
    pub fn section(&self) -> Option<DiagnosticSection> {
        self.fields.section
    }
    /// Explicit selected supplement, absent for the base.
    pub fn supplement(&self) -> Option<&PmaSupplement> {
        self.fields.supplement.as_ref()
    }
    /// Maximum rows or fragments per section page.
    pub fn section_limit(&self) -> u32 {
        self.fields.section_limit.unwrap_or_else(|| {
            if self.fields.section == Some(DiagnosticSection::Regulatory) {
                1
            } else {
                limit()
            }
        })
    }
    /// Native row or text-fragment offset.
    pub fn section_offset(&self) -> u32 {
        self.fields.section_offset
    }
    /// Stored source snapshot.
    pub fn bundle_revision(&self) -> Option<&Text> {
        self.fields.bundle_revision.as_ref()
    }
    /// Continue the exact selected section and record.
    pub fn at_section_offset(&self, offset: u32) -> Self {
        let mut q = self.clone();
        q.fields.section_offset = offset;
        q
    }
    /// Select a supported section; invalid source combinations remain refused.
    pub fn with_section(&self, section: DiagnosticSection) -> Result<Self, QueryInvalid> {
        let mut q = self.fields.clone();
        q.section = Some(section);
        q.section_offset = 0;
        q.try_into()
    }
    /// Bind the selected record to one immutable stored revision.
    pub fn with_bundle_revision(&self, revision: Text) -> Self {
        let mut q = self.clone();
        q.fields.bundle_revision = Some(revision);
        q
    }
}

fn validate_source(v: &GetFields) -> Result<(), QueryInvalid> {
    let valid_id = match v.source {
        DiagnosticGetSource::Gtr => v.id.as_str().parse::<GtrTestId>().is_ok(),
        DiagnosticGetSource::FdaDeviceRecall => true,
        DiagnosticGetSource::WhoIvd => v.id.as_str().parse::<WhoProductId>().is_ok(),
        DiagnosticGetSource::Fda510k => {
            v.id.as_str().parse::<biodata::DiagnosticDeviceId>().is_ok()
                && v.id.as_str().starts_with('K')
        }
        DiagnosticGetSource::FdaPma => v.id.as_str().parse::<PmaNumber>().is_ok(),
    };
    if !valid_id {
        return Err(QueryInvalid::new(&["id", "source"]));
    }
    let valid_section = match v.source {
        DiagnosticGetSource::Gtr => matches!(
            v.section,
            None | Some(
                DiagnosticSection::Genes
                    | DiagnosticSection::Conditions
                    | DiagnosticSection::Methods
            )
        ),
        DiagnosticGetSource::FdaPma => matches!(
            v.section,
            None | Some(DiagnosticSection::Regulatory | DiagnosticSection::Supplements)
        ),
        DiagnosticGetSource::WhoIvd | DiagnosticGetSource::Fda510k | DiagnosticGetSource::FdaDeviceRecall => v.section.is_none(),
    };
    if !valid_section {
        return Err(QueryInvalid::new(&["section", "source"]));
    }
    if v.source != DiagnosticGetSource::FdaPma && v.supplement.is_some() {
        return Err(QueryInvalid::new(&["supplement", "source"]));
    }
    if v.supplement.is_some() && v.section == Some(DiagnosticSection::Supplements) {
        return Err(QueryInvalid::new(&["supplement", "section"]));
    }
    if matches!(
        v.source,
        DiagnosticGetSource::WhoIvd | DiagnosticGetSource::Fda510k | DiagnosticGetSource::FdaDeviceRecall
    ) {
        if v.section_limit.is_some() {
            return Err(QueryInvalid::new(&["section_limit", "source"]));
        }
        if v.section_offset != 0 {
            return Err(QueryInvalid::new(&["section_offset", "source"]));
        }
    }
    if matches!(v.source, DiagnosticGetSource::Fda510k | DiagnosticGetSource::FdaDeviceRecall) && v.bundle_revision.is_some() {
        return Err(QueryInvalid::new(&["bundle_revision", "source"]));
    }
    Ok(())
}
