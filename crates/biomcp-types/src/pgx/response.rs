use super::{PgxAnnotationGet, PgxGeneSearch, PgxGet, PgxSearch, PgxSource, PgxTotalKind};
use crate::ResponseBody;
use biodata::{
    ClinPgxAccession, GeneSymbol, PgxAnnotationDetail, PgxAnnotationHeader, PgxAnnotationReference,
    PgxDiplotypeResult, PgxDrugRecommendation, PgxGeneIdentity, PgxGuidelineReference, PgxPair,
    PgxRecommendation,
};
use serde::{Deserialize, Serialize};
/// Source and declared gene context; no chosen pair or patient result.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct PgxHeader {
    /// Explicit source.
    pub source: PgxSource,
    /// Declared source symbol.
    pub gene: GeneSymbol,
}
/// Source and selector for a PGx search page.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum PgxSearchHeader {
    /// Drug-keyed pairs or clinical annotations.
    Drug {
        /// Explicit source.
        source: PgxSource,
        /// Selected drug name.
        drug: biodata::Text,
    },
    /// Variant-keyed clinical annotations.
    Variant {
        /// Explicit source.
        source: PgxSource,
        /// Selected dbSNP rsID.
        variant: biodata::RsId,
    },
}
/// Independent collection scope; observed counts never imply native source totals.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct PgxPageInfo {
    /// Offset in this collection.
    pub offset: u32,
    /// Number of whole returned rows.
    pub returned: u32,
    /// Exact source total or received bounded collection length.
    pub total: u64,
    /// Provenance of total.
    pub total_kind: PgxTotalKind,
}
/// One phenotype CPIC lists for the gene and drug, with its count of recommendation rows.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct PgxPhenotypeRows {
    /// Native phenotype text for the gene; filter with --phenotype.
    pub phenotype: Box<str>,
    /// Recommendation rows with this phenotype, across all pages.
    pub rows: u32,
}
macro_rules! collection {
    ($name:ident,$row:ty,$header:ty,$query:ty,$schema:literal) => {
        /// Independent source-ordered page of complete native records.
        #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
        #[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
        pub struct $name {
            /// Source and selected context.
            pub header: $header,
            /// Meaning and bounds of this collection.
            pub page: PgxPageInfo,
            /// Complete native rows without duplicate summaries.
            #[serde(default)]
            pub items: Box<[$row]>,
        }
        impl ResponseBody for $name {
            const SCHEMA: &'static str = $schema;
            type Query = $query;
        }
    };
}
collection!(PgxPage, PgxPair, PgxHeader, PgxGeneSearch, "biomcp.pgx-page/7");
/// Drug or variant evidence from the selected source.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "representation", rename_all = "kebab-case")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum PgxDiscovery {
    /// CPIC pair rows, with native totals.
    Pairs {
        /// Source and selected drug.
        header: PgxSearchHeader,
        /// Collection size and paging.
        page: PgxPageInfo,
        /// Native pair records.
        items: Box<[PgxPair]>,
    },
    /// ClinPGx clinical annotation rows, with observed totals.
    Clinical {
        /// Source and selected drug or variant.
        header: PgxSearchHeader,
        /// Collection size and paging.
        page: PgxPageInfo,
        /// Native clinical annotation headers.
        items: Box<[PgxAnnotationHeader]>,
    },
}
impl ResponseBody for PgxDiscovery {
    const SCHEMA: &'static str = "biomcp.pgx-discovery/1";
    type Query = PgxSearch;
}
collection!(
    PgxGuidelines,
    PgxGuidelineReference,
    PgxHeader,
    PgxGet,
    "biomcp.pgx-guidelines/5"
);
/// Independent recommendation contracts preserve native table and view shapes.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "representation", rename_all = "kebab-case")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum PgxRecommendations {
    /// Gene-keyed recommendation_view with native names and no inferred drug ID.
    View {
        /// Selected gene and source.
        header: PgxHeader,
        /// Native source collection scope.
        page: PgxPageInfo,
        /// Complete view rows in source order.
        #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
        items: Box<[PgxRecommendation]>,
    },
    /// Exact drug selector uses the distinct native recommendation table.
    Table {
        /// Selected gene and source.
        header: PgxHeader,
        /// Native source collection scope.
        page: PgxPageInfo,
        /// First page only: every phenotype CPIC lists for this gene and drug, in row order, with its row count.
        #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
        phenotypes: Box<[PgxPhenotypeRows]>,
        /// Complete table rows without view-name enrichment.
        #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
        items: Box<[PgxDrugRecommendation]>,
    },
}
impl ResponseBody for PgxRecommendations {
    const SCHEMA: &'static str = "biomcp.pgx-recommendations/6";
    type Query = PgxGet;
}
/// Compact exact source gene identity, not a selected gene-drug pair.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct PgxCard {
    /// Selected gene and source.
    pub header: PgxHeader,
    /// Native record version is not a dataset release.
    pub identity: PgxGeneIdentity,
}
impl ResponseBody for PgxCard {
    const SCHEMA: &'static str = "biomcp.pgx-card/5";
    type Query = PgxGet;
}
/// Every source-returned mapping for the exact declared expression.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct PgxDiplotype {
    /// Explicit gene and source.
    pub header: PgxHeader,
    /// Zero, one or multiple native mappings; never a selected patient phenotype.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub diplotypes: Box<[PgxDiplotypeResult]>,
}
impl ResponseBody for PgxDiplotype {
    const SCHEMA: &'static str = "biomcp.pgx-diplotype/5";
    type Query = PgxGet;
}
/// One independent ClinPGx collection, preserving its native min-tier shape.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "annotation_kind", rename_all = "kebab-case")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum PgxAnnotations {
    /// Complete clinical min rows including location and relationship arrays.
    Clinical {
        /// Declared gene and source.
        header: PgxHeader,
        /// Observed bounded collection scope.
        page: PgxPageInfo,
        /// Complete clinical min records.
        #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
        items: Box<[PgxAnnotationHeader]>,
    },
    /// Guideline references without invented full guidance.
    Guideline {
        /// Declared gene and source.
        header: PgxHeader,
        /// Observed bounded collection scope.
        page: PgxPageInfo,
        /// Complete guideline min references.
        #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
        items: Box<[PgxAnnotationReference]>,
    },
    /// Label references without invented full guidance.
    Label {
        /// Declared gene and source.
        header: PgxHeader,
        /// Observed bounded collection scope.
        page: PgxPageInfo,
        /// Complete label min references.
        #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
        items: Box<[PgxAnnotationReference]>,
    },
}
impl ResponseBody for PgxAnnotations {
    const SCHEMA: &'static str = "biomcp.pgx-annotations/6";
    type Query = PgxGet;
}
/// Exact clinical annotation identity, distinct from any genomic allele identity.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct PgxAnnotationIdentity {
    /// Explicit ClinPGx source.
    pub source: PgxSource,
    /// Selected exact accession.
    pub id: ClinPgxAccession,
}
/// Complete proved base-tier annotation.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct PgxAnnotation {
    /// Exact selected source identity.
    pub header: PgxAnnotationIdentity,
    /// Complete native base record, including history and genotype text.
    pub annotation: PgxAnnotationDetail,
}
impl ResponseBody for PgxAnnotation {
    const SCHEMA: &'static str = "biomcp.pgx-annotation/6";
    type Query = PgxAnnotationGet;
}
