// JSON strings use the public parser, so serde cannot bypass validation.
/// Implement validated string serialization and schema.
#[macro_export]
macro_rules! text_value {
    ($ty:ty, $schema_name:expr, $pattern:expr) => {
        $crate::text_value!($ty, $schema_name, $pattern,
            concat!("A validated canonical ", $schema_name, " string."));
    };
    ($ty:ty, $schema_name:expr, $pattern:expr, $description:expr) => {
        impl serde::Serialize for $ty {
            fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                serializer.collect_str(self)
            }
        }

        impl<'de> serde::Deserialize<'de> for $ty {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                struct Visitor;
                impl serde::de::Visitor<'_> for Visitor {
                    type Value = $ty;
                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        formatter.write_str(concat!("canonical ", $schema_name, " string"))
                    }
                    fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<$ty, E> {
                        value.parse().map_err(E::custom)
                    }
                }
                deserializer.deserialize_str(Visitor)
            }
        }

        #[cfg(feature = "schema")]
        impl schemars::JsonSchema for $ty {
            fn schema_name() -> std::borrow::Cow<'static, str> {
                $schema_name.into()
            }
            fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
                schemars::json_schema!({"description":$description, "type":"string", "pattern":$pattern})
            }
        }
    };
}

/// Define one closed wire vocabulary.
#[macro_export]
macro_rules! wire_enum {
    (described $(#[$doc:meta])* $name:ident { $($(#[doc = $vdoc:literal])* $variant:ident => $literal:literal),+ $(,)? }) => {
        $crate::wire_enum! {
            $(#[$doc])*
            #[cfg_attr(feature = "schema", schemars(extend("x-enum-descriptions" = [$(concat!($($vdoc),*)),+])))]
            $name { $($(#[doc = $vdoc])* $variant => $literal),+ }
        }
    };

    ($(#[$doc:meta])* $name:ident { $($(#[$vdoc:meta])* $variant:ident => $literal:literal),+ $(,)? }) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
        #[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
        $(#[$doc])*
        #[cfg_attr(feature = "schema", schemars(with = "str", extend("enum" = [$($literal),+])))]
        pub enum $name { $($(#[$vdoc])* #[serde(rename = $literal)] $variant,)+ }
        impl $name {
            /// Canonical wire spelling from the enum declaration.
            pub const fn as_str(self) -> &'static str { match self { $(Self::$variant => $literal,)+ } }
        }
    }
}

// Define each canonical spelling once for parsing, display and JSON.
/// Define one canonical string vocabulary.
#[macro_export]
macro_rules! canonical_enum {
    ($(#[$doc:meta])* $name:ident, $error:ident { $($(#[doc = $variant_doc:literal])* $variant:ident => $text:literal),+ $(,)? }) => {
        $(#[$doc])*
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
        #[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
        #[cfg_attr(feature = "schema",
 schemars(!try_from,
 !into,
 with = "str",
 extend("enum" = [$($text),
+],
 "x-enum-descriptions" = [$(concat!($($variant_doc),
*)),
+])))]
        #[non_exhaustive]
        #[serde(try_from = "String", into = "&'static str")]
        pub enum $name {
            $($(#[doc = $variant_doc])* $variant),+
        }

        impl From<$name> for &'static str {
            fn from(value: $name) -> Self {
                match value { $($name::$variant => $text),+ }
            }
        }

        impl std::str::FromStr for $name {
            type Err = $crate::Error;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                match value {
                    $($text => Ok(Self::$variant),)+
                    _ => Err($crate::Error::$error),
                }
            }
        }

        impl TryFrom<String> for $name {
            type Error = $crate::Error;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                value.parse()
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str((*self).into())
            }
        }
    };
}

/// Define a publisher vocabulary that retains new source values unchanged.
#[macro_export]
macro_rules! native_enum {
    ($(#[$doc:meta])* $name:ident { $($(#[$vdoc:meta])* $variant:ident => $literal:literal),+ $(,)? }) => {
        $(#[$doc])*
        #[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize)]
        #[serde(from = "String")]
        pub enum $name {
            $($(#[$vdoc])* $variant,)+
            /// A new publisher value, retained without interpretation.
            Unknown($crate::SourceText),
        }
        impl $name {
            /// Borrow the exact publisher spelling.
            pub fn as_str(&self) -> &str {
                match self { $(Self::$variant => $literal,)+ Self::Unknown(value) => value.as_str() }
            }
            /// Whether this value falls outside the published vocabulary.
            pub fn is_unknown(&self) -> bool { matches!(self, Self::Unknown(_)) }
        }
        impl From<String> for $name {
            fn from(value: String) -> Self {
                match value.as_str() { $($literal => Self::$variant,)+ _ => Self::Unknown(value.into()) }
            }
        }
        impl serde::Serialize for $name {
            fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                serializer.serialize_str(self.as_str())
            }
        }
        #[cfg(feature = "schema")]
        impl schemars::JsonSchema for $name {
            fn schema_name() -> std::borrow::Cow<'static, str> { stringify!($name).into() }
            fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
                schemars::json_schema!({"type":"string", "description":concat!(
                    "Native publisher vocabulary; new values are retained unchanged. Known values: ",
                    $($literal, "; ",)+
                )})
            }
        }
    }
}
