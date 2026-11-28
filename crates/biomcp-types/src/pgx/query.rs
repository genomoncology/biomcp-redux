use super::{PgxAnnotationKind, PgxSection, PgxSource};
use crate::QueryInvalid;
use biodata::{ClinPgxAccession, Curie, GeneSymbol, PgxDiplotypeExpression, RsId, Text};
use serde::{Deserialize, Serialize};
fn default_limit() -> u32 {
    5
}
fn section_default(source: PgxSource, kind: Option<PgxAnnotationKind>) -> u32 {
    if source == PgxSource::Clinpgx
        && kind.unwrap_or(PgxAnnotationKind::Clinical) == PgxAnnotationKind::Clinical
    {
        3
    } else {
        default_limit()
    }
}
fn explicit_section_limit<'de, D: serde::Deserializer<'de>>(
    decoder: D,
) -> Result<Option<u32>, D::Error> {
    u32::deserialize(decoder).map(Some)
}
#[cfg(feature = "schema")]
fn conditional_limit(schema: &mut schemars::Schema) {
    schema.remove("default");
}
/// CPIC pairs by gene or drug, or ClinPGx clinical annotations by variant or drug.
/// Give one selector and its source. By default, returns five rows per page.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "SearchFields")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(with = "SearchFields"))]
pub struct PgxSearch(SearchFields);
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(extend("x-reply-query-titles" = ["PgxGeneSearch"])))]
struct SearchFields {
    /// Gene symbol for CPIC pairs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(with = "GeneSymbol", extend("x-required-component" = crate::Source::Cpic)))]
    gene: Option<GeneSymbol>,
    /// dbSNP rsID for ClinPGx.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(extend("x-required-component" = crate::Source::ClinPgx)))]
    variant: Option<RsId>,
    /// Drug name for CPIC or ClinPGx.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    drug: Option<Text>,
    /// Source of pairs or clinical annotations.
    source: PgxSource,
    /// Literal CPIC level, including B/C.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(extend("x-required-component" = crate::Source::Cpic)))]
    cpic_level: Option<Text>,
    /// Rows per native page.
    #[serde(default = "default_limit")]
    #[cfg_attr(feature = "schema", schemars(range(min = 1, max = crate::SEARCH_PAGE_MAX)))]
    limit: u32,
    /// Source offset.
    #[serde(default)]
    offset: u32,
}
crate::query::validation::checked_query!(PgxSearch, SearchFields);
impl TryFrom<SearchFields> for PgxSearch {
    type Error = QueryInvalid;
    fn try_from(v: SearchFields) -> Result<Self, Self::Error> {
        let count = [v.gene.is_some(), v.variant.is_some(), v.drug.is_some()]
            .into_iter()
            .filter(|given| *given)
            .count();
        if count != 1 {
            return Err(QueryInvalid::new(&["gene", "variant", "drug"]));
        }
        match v.source {
            PgxSource::Cpic if v.variant.is_some() => {
                return Err(QueryInvalid::new(&["variant", "source"]));
            }
            PgxSource::Clinpgx if v.gene.is_some() || v.cpic_level.is_some() => {
                return Err(QueryInvalid::new(&[if v.gene.is_some() { "gene" } else { "cpic_level" }, "source"]));
            }
            _ => {}
        }
        if !(1..=crate::SEARCH_PAGE_MAX).contains(&v.limit) {
            return Err(QueryInvalid::new(&["limit"]));
        }
        Ok(Self(v))
    }
}
impl PgxSearch {
    /// Checked gene selector.
    pub fn gene(&self) -> Option<&GeneSymbol> {
        self.0.gene.as_ref()
    }
    /// Checked dbSNP selector.
    pub fn variant(&self) -> Option<&RsId> {
        self.0.variant.as_ref()
    }
    /// Exact drug name.
    pub fn drug(&self) -> Option<&Text> {
        self.0.drug.as_ref()
    }
    /// Checked source selector.
    pub fn source(&self) -> PgxSource {
        self.0.source
    }
    /// Checked cpic_level selector.
    pub fn cpic_level(&self) -> Option<&Text> {
        self.0.cpic_level.as_ref()
    }
    /// Checked limit selector.
    pub fn limit(&self) -> u32 {
        self.0.limit
    }
    /// Checked offset selector.
    pub fn offset(&self) -> u32 {
        self.0.offset
    }
}
impl crate::Paged for PgxSearch {
    type Offset = u32;
    fn set_page_offset(&mut self, offset: Self::Offset) {
        self.0.offset = offset;
    }
}

/// Query echoed by the gene-keyed CPIC pair reply.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct PgxGeneSearch {
    gene: GeneSymbol,
    #[cfg_attr(feature = "schema", schemars(with = "str", extend("enum" = ["cpic"])))]
    source: PgxSource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    cpic_level: Option<Text>,
    #[serde(default = "default_limit")]
    #[cfg_attr(feature = "schema", schemars(range(min = 1, max = crate::SEARCH_PAGE_MAX)))]
    limit: u32,
    #[serde(default)]
    offset: u32,
}

impl PgxSearch {
    /// The existing gene-keyed reply keeps its required gene context.
    pub fn gene_context(&self) -> Option<PgxGeneSearch> {
        Some(PgxGeneSearch {
            gene: self.gene()?.clone(),
            source: self.source(),
            cpic_level: self.cpic_level().cloned(),
            limit: self.limit(),
            offset: self.offset(),
        })
    }
}

/// PGx card or section. No phenotype, phase or dose is calculated.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "GetFields")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(with = "GetFields"))]
pub struct PgxGet(GetFields);
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
struct GetFields {
    /// Explicit source gene symbol.
    gene: GeneSymbol,
    /// Required source, checked against the section.
    source: PgxSource,
    /// One named section; omitted means the CPIC gene card.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(extend("x-positional" = 0)))]
    section: Option<PgxSection>,
    /// Unchanged declared expression required for the diplotype section only.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(extend("x-required-component" = crate::Source::Cpic)))]
    diplotype: Option<PgxDiplotypeExpression>,
    /// Exact published CPIC drug key; recommendations only.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(extend("x-required-component" = crate::Source::Cpic)))]
    drug_id: Option<Curie>,
    /// Exact CPIC phenotype for the gene, such as Poor Metabolizer; recommendations with --drug-id only.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(extend("x-required-component" = crate::Source::Cpic)))]
    phenotype: Option<Text>,
    /// One ClinPGx collection; clinical is the default only on annotations.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(extend("x-required-component" = crate::Source::ClinPgx)))]
    kind: Option<PgxAnnotationKind>,
    /// Evidence level for ClinPGx clinical annotations.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(extend("x-required-component" = crate::Source::ClinPgx)))]
    level: Option<super::PgxEvidenceLevel>,
    /// Drug name for ClinPGx clinical annotations.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(extend("x-required-component" = crate::Source::ClinPgx)))]
    drug: Option<Text>,
    /// Rows per page: three clinical, five elsewhere.
    #[serde(
        default,
        deserialize_with = "explicit_section_limit",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(feature = "schema", schemars(with="u32",range(min = 1, max = 50),transform=conditional_limit))]
    section_limit: Option<u32>,
    /// Offset within the selected section.
    #[serde(default)]
    section_offset: u32,
    /// Retained collection for local continuation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(extend("x-required-component" = crate::Source::ClinPgx)))]
    snapshot: Option<Text>,
}
impl crate::CheckedQuery for PgxGet {
    /// A drug name or bare ClinPGx accession is not a CPIC drug key; show the key's form.
    fn invalid_field(name: &str) -> QueryInvalid {
        let cause = QueryInvalid::new(&[name]);
        if name == "drug_id" {
            cause.with_message(crate::ErrorMessage::PgxDrugId)
        } else {
            cause
        }
    }
    fn from_fields<'de, D: serde::Deserializer<'de, Error = QueryInvalid>>(
        deserializer: D,
    ) -> Result<Self, QueryInvalid> {
        Self::try_from(GetFields::deserialize(deserializer)?)
    }
}
impl TryFrom<GetFields> for PgxGet {
    type Error = QueryInvalid;
    fn try_from(mut v: GetFields) -> Result<Self, Self::Error> {
        let annotations = v.section == Some(PgxSection::Annotations);
        let limit = v
            .section_limit
            .unwrap_or_else(|| section_default(v.source, v.kind));
        if v.section.map_or(v.source == PgxSource::Clinpgx, |section| {
            section.source() != v.source.source()
        }) {
            return Err(QueryInvalid::new(&["source", "section"]));
        }
        if !(1..=50).contains(&limit) {
            return Err(QueryInvalid::new(&["section_limit"]));
        }
        if (v.section == Some(PgxSection::Diplotype)) != v.diplotype.is_some() {
            return Err(QueryInvalid::new(&["section", "diplotype"]));
        }
        if v.drug_id.is_some() && v.section != Some(PgxSection::Recommendations) {
            return Err(QueryInvalid::new(&["drug_id", "section"]));
        }
        if v.phenotype.is_some() && v.section != Some(PgxSection::Recommendations) {
            return Err(QueryInvalid::new(&["phenotype", "section"]));
        }
        if v.phenotype.is_some() && v.drug_id.is_none() {
            return Err(QueryInvalid::new(&["phenotype", "drug_id"])
                .with_message(crate::ErrorMessage::PgxPhenotype));
        }
        if v.kind.is_some() && !annotations {
            return Err(QueryInvalid::new(&["kind", "section"]));
        }
        if (!annotations || v.kind.is_some_and(|kind| kind != PgxAnnotationKind::Clinical))
            && (v.level.is_some() || v.drug.is_some())
        {
            return Err(QueryInvalid::new(&[if v.level.is_some() { "level" } else { "drug" }, "section", "kind"]));
        }
        if v.snapshot.is_some() && !annotations {
            return Err(QueryInvalid::new(&["snapshot", "section"]));
        }
        if v.section.is_none() && v.section_offset != 0 {
            return Err(QueryInvalid::new(&["section", "section_offset"]));
        }
        if annotations && v.kind.is_none() {
            v.kind = Some(PgxAnnotationKind::Clinical);
        }
        v.section_limit = Some(limit);
        Ok(Self(v))
    }
}
impl PgxGet {
    /// Construct a checked CPIC gene card with the standard section defaults.
    pub fn new(gene: GeneSymbol) -> Result<Self, QueryInvalid> {
        Self::try_from(GetFields {
            gene,
            source: PgxSource::Cpic,
            section: None,
            diplotype: None,
            drug_id: None,
            phenotype: None,
            kind: None,
            level: None,
            drug: None,
            section_limit: None,
            section_offset: 0,
            snapshot: None,
        })
    }

    /// Checked gene selector.
    pub fn gene(&self) -> &GeneSymbol {
        &self.0.gene
    }
    /// Checked source selector.
    pub fn source(&self) -> PgxSource {
        self.0.source
    }
    /// Checked section selector.
    pub fn section(&self) -> Option<PgxSection> {
        self.0.section
    }
    /// Checked diplotype selector.
    pub fn diplotype(&self) -> Option<&PgxDiplotypeExpression> {
        self.0.diplotype.as_ref()
    }
    /// Checked drug_id selector.
    pub fn drug_id(&self) -> Option<&Curie> {
        self.0.drug_id.as_ref()
    }
    /// Exact CPIC phenotype filter.
    pub fn phenotype(&self) -> Option<&Text> {
        self.0.phenotype.as_ref()
    }
    /// Checked kind selector.
    pub fn kind(&self) -> Option<PgxAnnotationKind> {
        self.0.kind
    }
    /// Exact clinical evidence level.
    pub fn level(&self) -> Option<super::PgxEvidenceLevel> {
        self.0.level
    }
    /// Exact annotation drug name.
    pub fn drug(&self) -> Option<&Text> {
        self.0.drug.as_ref()
    }
    /// Checked section_limit selector.
    pub fn section_limit(&self) -> u32 {
        self.0
            .section_limit
            .unwrap_or_else(|| self.default_section_limit())
    }
    /// Default complete row count for the checked source and tier.
    pub fn default_section_limit(&self) -> u32 {
        section_default(self.source(), self.kind())
    }
    /// Checked section_offset selector.
    pub fn section_offset(&self) -> u32 {
        self.0.section_offset
    }
    /// Retained ClinPGx collection selector.
    pub fn snapshot(&self) -> Option<&Text> {
        self.0.snapshot.as_ref()
    }
    /// Bind later local pages to the complete received collection.
    pub fn with_snapshot(&self, snapshot: Text) -> Result<Self, QueryInvalid> {
        let mut q = self.0.clone();
        q.snapshot = Some(snapshot);
        Self::try_from(q)
    }

    /// Continue the same independent section.
    pub fn at_section_offset(&self, offset: u32) -> Result<Self, QueryInvalid> {
        let mut q = self.0.clone();
        q.section_offset = offset;
        Self::try_from(q)
    }
    /// Select a section with an explicit source and check its required context.
    pub fn with_section(
        &self,
        section: PgxSection,
        source: PgxSource,
    ) -> Result<Self, QueryInvalid> {
        let mut q = self.0.clone();
        q.section = Some(section);
        if q.source != source {
            q.section_limit = None;
        }
        q.source = source;
        q.section_offset = 0;
        Self::try_from(q)
    }
}
impl crate::Paged for PgxGet {
    type Offset = ();
    fn set_page_offset(&mut self, _offset: Self::Offset) {
        self.0.section_offset = 0;
    }
    fn clear_page_context(&mut self) {
        self.0.snapshot = None;
    }
}

/// Exact single clinical annotation get, with no inferred source or annotation kind.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "AnnotationFields")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(with = "AnnotationFields"))]
pub struct PgxAnnotationGet(AnnotationFields);
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
struct AnnotationFields {
    /// Checked ClinPGx accession, never a shared PaId.
    id: ClinPgxAccession,
    /// Required ClinPGx source.
    source: PgxSource,
}
crate::query::validation::checked_query!(PgxAnnotationGet, AnnotationFields);
impl TryFrom<AnnotationFields> for PgxAnnotationGet {
    type Error = QueryInvalid;
    fn try_from(v: AnnotationFields) -> Result<Self, Self::Error> {
        if v.source != PgxSource::Clinpgx {
            return Err(QueryInvalid::new(&["source"]));
        }
        Ok(Self(v))
    }
}
impl PgxAnnotationGet {
    /// Select a checked native ClinPGx accession.
    pub fn new(id: ClinPgxAccession) -> Result<Self, QueryInvalid> {
        Self::try_from(AnnotationFields {
            id,
            source: PgxSource::Clinpgx,
        })
    }

    /// Exact selected accession.
    pub fn id(&self) -> &ClinPgxAccession {
        &self.0.id
    }
    /// Explicit source.
    pub fn source(&self) -> PgxSource {
        self.0.source
    }
}
