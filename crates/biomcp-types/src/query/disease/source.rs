//! Disease selectors derive every spelling from the shared source vocabulary.
macro_rules! selector {
    ($name:ident, $description:literal, $default:ident, {$($variant:ident),+}) => {
        #[doc = $description]
        #[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
        #[serde(try_from = "String", into = "&'static str")]
        pub enum $name { $(#[doc = stringify!($variant)] $variant),+ }
        impl Default for $name { fn default() -> Self { Self::$default } }
        impl $name {
            /// Every supported selector.
            pub const ALL: &[Self] = &[$(Self::$variant),+];
            /// Shared source identity owning the canonical spelling.
            pub const fn source(self) -> crate::Source { match self { $(Self::$variant => crate::Source::$variant),+ } }
            /// Canonical source spelling.
            pub fn as_str(self) -> &'static str { self.source().into() }
        }
        impl From<$name> for &'static str { fn from(value: $name) -> Self { value.as_str() } }
        impl TryFrom<String> for $name {
            type Error = biodata::Error;
            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::ALL.iter().copied().find(|s| s.as_str() == value).ok_or(biodata::Error::SourceUnknown)
            }
        }
        #[cfg(feature = "schema")]
        impl schemars::JsonSchema for $name {
            fn schema_name() -> std::borrow::Cow<'static, str> { stringify!($name).into() }
            fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
                let names: Vec<_> = Self::ALL.iter().map(|s| s.as_str()).collect();
                schemars::json_schema!({"description":$description,"type":"string","enum":names})
            }
        }
    }
}
selector!(DiseaseOntology, "One explicit OLS ontology.", Mondo, {Mondo, Doid, Ncit});
selector!(DiseaseSource, "An explicit disease source without fallback.", Ols, {Ols, MyDisease, Monarch, Seer});
