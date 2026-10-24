use super::gene::{QueryInvalid, present};
use crate::{Build, Chromosome, GeneSymbol, ProteinChange, macros::text_value};
use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};
crate::macros::wire_enum! {
    /// The source-native query language exposed by variant search.
    VariantSearchSource {
        /// MyVariant.info candidate discovery.
        Myvariant => "myvariant",
    }
}
/// MyVariant's one-based inclusive primary-chromosome region spelling, kept unchanged.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VariantRegion(Box<str>);
impl FromStr for VariantRegion {
    type Err = QueryInvalid;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let (chromosome, bounds) = value.split_once(':').ok_or(QueryInvalid)?;
        chromosome
            .strip_prefix("chr")
            .ok_or(QueryInvalid)?
            .parse::<Chromosome>()
            .map_err(|_| QueryInvalid)?;
        let (start, end) = bounds.split_once('-').ok_or(QueryInvalid)?;
        let position = |v: &str| {
            if v.starts_with('0') || !v.bytes().all(|b| b.is_ascii_digit()) {
                return Err(QueryInvalid);
            }
            v.parse::<u32>().map_err(|_| QueryInvalid)
        };
        if position(start)? > position(end)? {
            return Err(QueryInvalid);
        }
        Ok(Self(value.into()))
    }
}
impl fmt::Display for VariantRegion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
text_value!(
    VariantRegion,
    "VariantRegion",
    r"^chr([1-9]|1[0-9]|2[0-2]|X|Y|MT):[1-9][0-9]*-[1-9][0-9]*$"
);

/// Explicit MyVariant query text, preserving surrounding whitespace and source syntax.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MyVariantQuery(Box<str>);
impl FromStr for MyVariantQuery {
    type Err = QueryInvalid;
    fn from_str(value: &str) -> Result<Self,Self::Err> {
        if value.trim().is_empty() || value.len()>2048 || value.chars().any(char::is_control) { return Err(QueryInvalid); }
        Ok(Self(value.into()))
    }
}
impl fmt::Display for MyVariantQuery {
    fn fmt(&self,f:&mut fmt::Formatter<'_>) -> fmt::Result { f.write_str(&self.0) }
}
text_value!(MyVariantQuery,"MyVariantQuery",r"\S");
/// Discover source candidates using one declared selector, without interpreting free text.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "Fields")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(with = "Fields", extend("anyOf" = [
    {"required":["gene","protein"]},
    {"required":["region","build"]},
    {"required":["query","source"]}
])))]
pub struct VariantSearch(Fields);
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
struct Fields {
    /// Exact checked gene symbol for the fixed MyVariant gene/protein field query.
    #[serde(
        default,
        deserialize_with = "present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(feature = "schema", schemars(with = "GeneSymbol"))]
    gene: Option<GeneSymbol>,
    /// Explicit substitution using checked one-letter or three-letter amino-acid codes.
    #[serde(
        default,
        deserialize_with = "present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(feature = "schema", schemars(with = "ProteinChange"))]
    protein: Option<ProteinChange>,
    /// Exact one-based inclusive MyVariant region, such as chr7:55259500-55259600; requires --build.
    #[serde(
        default,
        deserialize_with = "present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(feature = "schema", schemars(with = "VariantRegion"))]
    region: Option<VariantRegion>,
    /// Raw MyVariant query, passed and echoed unchanged; requires --source myvariant.
    #[serde(
        default,
        deserialize_with = "present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(feature = "schema", schemars(with = "MyVariantQuery"))]
    query: Option<MyVariantQuery>,
    /// Assembly sent to MyVariant. Protein and raw queries default to GRCh37; regions require it explicitly.
    #[serde(
        default,
        deserialize_with = "present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(feature = "schema", schemars(with = "Build"))]
    build: Option<Build>,
    /// Explicit source language; required for a raw query.
    #[serde(
        default,
        deserialize_with = "present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(feature = "schema", schemars(with = "VariantSearchSource"))]
    source: Option<VariantSearchSource>,
    /// Native candidates per page, default 10, maximum 50; all candidates remain candidates.
    #[serde(default = "default_limit")]
    #[cfg_attr(feature = "schema", schemars(range(min = 1, max = 50)))]
    limit: u32,
    /// Native candidate offset, within MyVariant's first 10,000 results.
    #[serde(default)]
    #[cfg_attr(feature = "schema", schemars(range(max = 9999)))]
    offset: u32,
}
fn default_limit() -> u32 {
    10
}
impl TryFrom<Fields> for VariantSearch {
    type Error = QueryInvalid;
    fn try_from(f: Fields) -> Result<Self, Self::Error> {
        let protein =
            f.gene.is_some() && f.protein.is_some() && f.region.is_none() && f.query.is_none();
        let region = f.region.is_some()
            && f.build.is_some()
            && f.gene.is_none()
            && f.protein.is_none()
            && f.query.is_none();
        let raw = f.query.is_some()
            && f.source.is_some()
            && f.gene.is_none()
            && f.protein.is_none()
            && f.region.is_none();
        if !(protein || region || raw)
            || !(1..=50).contains(&f.limit)
            || f.offset >= 10_000
            || f.gene.as_ref().is_some_and(|g| !g.as_str().bytes().all(|b| b.is_ascii_alphanumeric() || matches!(b,b'-'|b'_'|b'.')))

        {
            return Err(QueryInvalid);
        }
        Ok(Self(f))
    }
}
impl VariantSearch {
    /// Exact declared source query; no alternative notation or free-text guessing.
    pub fn source_query(&self) -> String {
        if let Some(q) = &self.0.query {
            return q.to_string();
        }
        if let Some(region) = &self.0.region {
            return region.to_string();
        }
        match (&self.0.gene, &self.0.protein) {
            (Some(g), Some(p)) => format!(
                "dbnsfp.genename:{g} AND dbnsfp.hgvsp:\"p.{}\"",
                p.as_str().strip_prefix("p.").unwrap_or(p.as_str())
            ),
            _ => String::new(),
        }
    }
    /// Assembly declared in the request, including the documented default policy.
    pub fn build(&self) -> Build {
        self.0.build.unwrap_or(Build::Grch37)
    }
    /// Native page size, clipped at the source result-window boundary.
    pub fn limit(&self) -> u32 {
        self.0.limit.min(10_000 - self.0.offset)
    }
    /// Native page offset.
    pub fn offset(&self) -> u32 {
        self.0.offset
    }
    /// Preserve all declared selectors on the next native page.
    pub fn at_offset(&self, offset: u32) -> Self {
        let mut q = self.clone();
        q.0.offset = offset;
        q
    }
}
