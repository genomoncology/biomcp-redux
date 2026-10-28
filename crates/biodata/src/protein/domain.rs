use crate::{Attribution, InterProId, Text, UncertainSpan0, UniProtIsoform};
use serde::{Deserialize, Serialize};
/// One native integrated InterPro match; not every entry type is a domain.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ProteinDomainMatch {
    /// Integrated entry accession, distinct from member signatures.
    pub id: InterProId,
    /// Native integrated name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<Text>,
    /// Literal source entry type, including family, site and repeat.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entry_type: Option<Text>,
    /// Exact native protein identity.
    pub protein: UniProtIsoform,
    /// Source protein length when supplied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protein_length: Option<u32>,
    /// Source taxon when supplied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub taxon: Option<u32>,
    /// Every native location group in source order.
    #[serde(default, skip_serializing_if = "<[ _ ]>::is_empty")]
    pub locations: Box<[ProteinDomainLocation]>,
    /// Source member signatures remain distinct from integrated entries.
    #[serde(default, skip_serializing_if = "<[ _ ]>::is_empty")]
    pub member_signatures: Box<[ProteinMemberSignature]>,
    /// Source record and release facts.
    pub attribution: Attribution,
}
/// One native InterPro location group retaining all discontinuous fragments.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ProteinDomainLocation {
    /// All native fragments; never reduced to a min/max hull.
    #[serde(default, skip_serializing_if = "<[ _ ]>::is_empty")]
    pub fragments: Box<[ProteinDomainFragment]>,
}
/// One native match fragment after coordinate conversion.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ProteinDomainFragment {
    /// Converted interbase span with uncertainty retained.
    pub span: UncertainSpan0,
    /// Literal discontinuity status supplied by InterPro.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dc_status: Option<Text>,
}
/// A member database signature, not an integrated IPR entry.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ProteinMemberSignature {
    /// Native member database name.
    pub database: Text,
    /// Native signature accession.
    pub accession: Text,
    /// Native member signature name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<Text>,
}
