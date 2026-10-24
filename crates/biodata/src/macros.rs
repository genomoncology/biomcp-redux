// JSON strings use the public parser, so serde cannot bypass validation.
macro_rules! text_value {
    ($ty:ty, $schema_name:literal, $pattern:expr) => {
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
                schemars::json_schema!({"type":"string", "pattern":$pattern})
            }
        }
    };
}

pub(crate) use text_value;

macro_rules! wire_enum {
    (described $(#[$doc:meta])* $name:ident { $($(#[doc = $vdoc:literal])* $variant:ident => $literal:literal),+ $(,)? }) => {
        $crate::macros::wire_enum! {
            $(#[$doc])*
            #[cfg_attr(feature = "schema", schemars(extend("x-enum-descriptions" = [$(concat!($($vdoc),*)),+])))]
            $name { $($(#[doc = $vdoc])* $variant => $literal),+ }
        }
    };

    ($(#[$doc:meta])* $name:ident { $($(#[$vdoc:meta])* $variant:ident => $literal:literal),+ $(,)? }) => {
        $(#[$doc])*
        #[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
        #[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
        #[cfg_attr(feature = "schema", schemars(with = "str", extend("enum" = [$($literal),+])))]
        pub enum $name { $($(#[$vdoc])* #[serde(rename = $literal)] $variant,)+ }
        impl $name {
            /// Canonical wire spelling from the enum declaration.
            pub const fn as_str(self) -> &'static str { match self { $(Self::$variant => $literal,)+ } }
        }
    }
}
pub(crate) use wire_enum;
