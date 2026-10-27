use serde::{Deserialize, Serialize};
/// One explicitly selected drug source; omitted search source means MyChem alone.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "&'static str")]
#[non_exhaustive]
pub enum DrugSource {
    /// WHO finished pharmaceutical product prequalification.
    WhoPq,
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
    pub const ALL: [Self; 9] = [
        Self::WhoPq,
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
        schemars::json_schema!({"description":"A supported drug data source.", "type":"string", "enum":names})
    }
}
