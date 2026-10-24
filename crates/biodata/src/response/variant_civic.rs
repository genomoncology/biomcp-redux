use crate::{PageInfo, SourceRecord, Statement, Text, VariantGet, VariantRef};
use serde::{Deserialize, Serialize};

/// CIViC molecular profiles with their original membership and separate source claims.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct VariantCivic {
    /// Source profiles in native order; name candidates carry no attached statements.
    pub profiles: Box<[CivicProfile]>,
    /// Disclosure bounds for this section.
    pub page: PageInfo,
    /// Continue this section without changing the input or source selectors.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<VariantGet>,
}

/// One source molecular profile, preserving compound expressions.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct CivicProfile {
    /// Native profile identity and link.
    pub record: SourceRecord,
    /// Whole source profile label, including its logical expression.
    pub label: Text,
    /// Whether a source member was joined to the requested exact identity.
    pub membership: CivicMembership,
    /// Every source member reference, without collapsing compound profiles.
    pub members: Box<[VariantRef]>,
    /// Native evidence count, independent of the displayed page.
    pub evidence_count: u64,
    /// Native assertion count, independent of the displayed page.
    pub assertion_count: u64,
    /// Source claims; a compound claim retains a profile subject.
    pub statements: Box<[Statement]>,
}

crate::macros::wire_enum! {
    /// Identity strength of a CIViC profile lookup.
    CivicMembership {
        /// Source member identity was checked against the requested variant.
        Exact => "exact",
        /// Only a source name matched; no claim is attached to an allele.
        Candidate => "candidate",
    }
}
