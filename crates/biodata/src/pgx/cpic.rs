use crate::{ClinPgxAccession, Curie, EnsemblGeneId, GeneSymbol, HgncId, NcbiGeneId, Pmid};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
/// Source-native PgxGeneIdentity fields; no clinical interpretation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct PgxGeneIdentity {
    /// Native `symbol`; literal source value.
    pub symbol: GeneSymbol,
    /// Native `hgncid`; literal source value.
    pub hgncid: HgncId,
    /// Native `ncbiid`; literal source value.
    pub ncbiid: NcbiGeneId,
    /// Native `ensemblid`; literal source value.
    pub ensemblid: EnsemblGeneId,
    /// Native `clinpgxid`; literal source value.
    pub clinpgxid: ClinPgxAccession,
    /// Native `lookupmethod`; literal source value.
    pub lookupmethod: Box<str>,
    /// Native `version`; literal source value.
    pub version: i64,
}
/// Source-native PgxPair fields; no clinical interpretation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct PgxPair {
    /// Native `pairid`; literal source value.
    pub pairid: i64,
    /// Native `drugid`; literal source value.
    pub drugid: Curie,
    /// Native `drugname`; literal source value.
    pub drugname: Box<str>,
    /// Native `genesymbol`; literal source value.
    pub genesymbol: GeneSymbol,
    /// Native `guidelinename`; literal source value.
    pub guidelinename: Option<Box<str>>,
    /// Native `guidelineurl`; literal source value.
    #[cfg_attr(feature = "schema", schemars(extend("format" = "uri")))]
    pub guidelineurl: Option<Box<str>>,
    /// Native `cpiclevel`; literal source value.
    pub cpiclevel: Box<str>,
    /// Native `clinpgxlevel`; literal source value.
    pub clinpgxlevel: Option<Box<str>>,
    /// Native `pgxtesting`; literal source value.
    pub pgxtesting: Option<Box<str>>,
    /// Native `pmids`; literal source value.
    pub pmids: Option<Box<[Pmid]>>,
    /// Native `usedforrecommendation`; literal source value.
    pub usedforrecommendation: Box<str>,
    /// Native `provisional`; literal source value.
    pub provisional: bool,
}
/// Source-native PgxRecommendation fields; no clinical interpretation.
/// ```compile_fail
/// fn interpret(value: biodata::PgxRecommendation) -> biodata::Statement { value }
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct PgxRecommendation {
    /// Native `recommendationid`; literal source value.
    pub recommendationid: i64,
    /// Native `lookupkey`; literal source value.
    pub lookupkey: BTreeMap<super::PgxGeneKey, Box<str>>,
    /// Native `drugname`; literal source value.
    pub drugname: Box<str>,
    /// Native `guidelinename`; literal source value.
    pub guidelinename: Box<str>,
    /// Native `guidelineurl`; literal source value.
    #[cfg_attr(feature = "schema", schemars(extend("format" = "uri")))]
    pub guidelineurl: Box<str>,
    /// Native `implications`; literal source value.
    pub implications: BTreeMap<super::PgxGeneKey, Box<str>>,
    /// Native `drugrecommendation`; literal source value.
    pub drugrecommendation: Box<str>,
    /// Native `classification`; literal source value.
    pub classification: Box<str>,
    /// Native `phenotypes`; literal source value.
    pub phenotypes: BTreeMap<super::PgxGeneKey, Box<str>>,
    /// Native `activityscore`; literal source value.
    pub activityscore: BTreeMap<super::PgxGeneKey, Box<str>>,
    /// Native `population`; literal source value.
    pub population: Box<str>,
    /// Native `comments`; literal source value.
    pub comments: Option<Box<str>>,
}
/// Source-native PgxDrugRecommendation fields; no clinical interpretation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct PgxDrugRecommendation {
    /// Native `id`; literal source value.
    pub id: i64,
    /// Native `guidelineid`; literal source value.
    pub guidelineid: i64,
    /// Native `drugid`; literal source value.
    pub drugid: Curie,
    /// Native `implications`; literal source value.
    pub implications: BTreeMap<super::PgxGeneKey, Box<str>>,
    /// Native `drugrecommendation`; literal source value.
    pub drugrecommendation: Box<str>,
    /// Native `classification`; literal source value.
    pub classification: Box<str>,
    /// Native `phenotypes`; literal source value.
    pub phenotypes: BTreeMap<super::PgxGeneKey, Box<str>>,
    /// Native `activityscore`; literal source value.
    pub activityscore: BTreeMap<super::PgxGeneKey, Box<str>>,
    /// Native `allelestatus`; literal source value.
    pub allelestatus: BTreeMap<super::PgxGeneKey, Box<str>>,
    /// Native `lookupkey`; literal source value.
    pub lookupkey: BTreeMap<super::PgxGeneKey, Box<str>>,
    /// Native `population`; literal source value.
    pub population: Box<str>,
    /// Native `comments`; literal source value.
    pub comments: Box<str>,
    /// Native `version`; literal source value.
    pub version: i64,
    /// Native `dosinginformation`; literal source value.
    pub dosinginformation: bool,
    /// Native `alternatedrugavailable`; literal source value.
    pub alternatedrugavailable: bool,
    /// Native `otherprescribingguidance`; literal source value.
    pub otherprescribingguidance: bool,
}
/// Source-native PgxGuidelineGene fields; no clinical interpretation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct PgxGuidelineGene {
    /// Native `url`; literal source value.
    #[cfg_attr(feature = "schema", schemars(extend("format" = "uri")))]
    pub url: Box<str>,
    /// Native `symbol`; literal source value.
    pub symbol: GeneSymbol,
}
/// Source-native PgxGuidelineReference fields; no clinical interpretation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct PgxGuidelineReference {
    /// Native `guideline_name`; literal source value.
    pub guideline_name: Box<str>,
    /// Native `guideline_url`; literal source value.
    #[cfg_attr(feature = "schema", schemars(extend("format" = "uri")))]
    pub guideline_url: Box<str>,
    /// Native `drugs`; literal source value.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub drugs: Box<[Box<str>]>,
    /// Native `genes`; literal source value.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub genes: Box<[PgxGuidelineGene]>,
}
/// Source-native PgxDiplotypeResult fields; no clinical interpretation.
/// ```compile_fail
/// fn interpret(value: biodata::PgxDiplotypeResult) -> biodata::Statement { value }
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct PgxDiplotypeResult {
    /// Native `genesymbol`; literal source value.
    pub genesymbol: GeneSymbol,
    /// Native `diplotype`; literal source value.
    pub diplotype: Box<str>,
    /// Native `function1`; literal source value.
    pub function1: Box<str>,
    /// Native `function2`; literal source value.
    pub function2: Box<str>,
    /// Native `activityvalue1`; literal source value.
    pub activityvalue1: Box<str>,
    /// Native `activityvalue2`; literal source value.
    pub activityvalue2: Box<str>,
    /// Native `totalactivityscore`; literal source value.
    pub totalactivityscore: Box<str>,
    /// Native `description`; literal source value.
    pub description: Box<str>,
    /// Native `diplotypekey`; literal source value.
    pub diplotypekey: BTreeMap<super::PgxGeneKey, BTreeMap<Box<str>, u32>>,
    /// Native `generesult`; literal source value.
    pub generesult: Box<str>,
    /// Native `ehrpriority`; literal source value.
    pub ehrpriority: Box<str>,
    /// Native `consultationtext`; literal source value.
    pub consultationtext: Box<str>,
    /// Native `lookupkey`; literal source value.
    pub lookupkey: BTreeMap<super::PgxGeneKey, Box<str>>,
}
