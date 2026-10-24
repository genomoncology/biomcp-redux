macro_rules! wire_enum {
    ($(#[$doc:meta])* $name:ident { $($(#[$vdoc:meta])* $variant:ident => $literal:literal),+ $(,)? }) => {
        $(#[$doc])*
        #[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
        #[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
        #[cfg_attr(feature = "schema", schemars(with = "str", extend("enum" = [$($literal),+])))]
        pub enum $name { $($(#[$vdoc])* #[serde(rename = $literal)] $variant,)+ }
    }
}
pub(crate) use wire_enum;
wire_enum! {
    /// Overall command outcome.
    Status {
        /// Every requested source answered.
        Complete => "complete",
        /// Some requested data failed or is stale.
        Partial => "partial",
        /// Every identity was confirmed absent.
        Absent => "absent",
        /// No requested identity could be answered.
        Failed => "failed",
    }
}
/// Outcome for one input.
pub type ItemStatus = Status;
