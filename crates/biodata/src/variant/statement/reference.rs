use crate::{GeneSymbol, HgvsText, Text, VariantId};
use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};
/// Checked prefix:local ontology identifier.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Curie(Box<str>);
impl FromStr for Curie {
    type Err = crate::Error;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let (prefix, local) = value.split_once(':').ok_or(crate::Error::CurieInvalid)?;
        if prefix.is_empty()
            || local.is_empty()
            || !prefix
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-' || b == b'.')
            || value.chars().any(|c| c.is_whitespace() || c.is_control())
        {
            return Err(crate::Error::CurieInvalid);
        }
        Ok(Self(value.into()))
    }
}
impl fmt::Display for Curie {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}
crate::macros::text_value!(Curie, "Curie", "^[A-Za-z0-9_.-]+:[^\\s\\x00-\\x1f]+$");
/// A source subject reference, never an embedded variant entity.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "level", rename_all = "snake_case", deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
enum Fields {
    /// Exact allele identifier or reference-qualified notation.
    Allele(AlleleRef),
    /// Protein-level assertion, without genomic conversion.
    Protein(ProteinRef),
    /// Compound source profile, without an inferred allele.
    Profile {
        /// Native profile ID.
        record: Text,
        /// Complete source label.
        label: Text,
        /// Source member references in their native order.
        #[serde(default, skip_serializing_if = "<[ _ ]>::is_empty")]
        #[cfg_attr(feature = "schema", schemars(with = "Box<[LeafRef]>"))]
        members: Box<[VariantRef]>,
    },
}

/// Checked source subject reference; at least one source identity or label is required.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(try_from = "Fields")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(with = "Fields"))]
pub struct VariantRef(Fields);
impl TryFrom<Fields> for VariantRef {
    type Error = crate::Error;
    fn try_from(fields: Fields) -> Result<Self, Self::Error> {
        match &fields {
            Fields::Allele(AlleleRef { id, hgvs }) if id.is_none() && hgvs.is_none() => {
                Err(crate::Error::VariantRefInvalid)
            }
            Fields::Protein(ProteinRef { hgvs, label, .. })
                if hgvs.is_none() && label.is_none() =>
            {
                Err(crate::Error::VariantRefInvalid)
            }
            Fields::Profile { members, .. }
                if members
                    .iter()
                    .any(|m| matches!(&m.0, Fields::Profile { .. })) =>
            {
                Err(crate::Error::VariantRefInvalid)
            }
            _ => Ok(Self(fields)),
        }
    }
}
impl VariantRef {
    /// Reference one checked source allele identifier.
    pub fn allele(id: VariantId) -> Self {
        Self(Fields::Allele(AlleleRef {
            id: Some(id),
            hgvs: None,
        }))
    }
    /// Reference source-qualified notation without computing a genomic allele.
    pub fn allele_hgvs(hgvs: HgvsText) -> Self {
        Self(Fields::Allele(AlleleRef {
            id: None,
            hgvs: Some(hgvs),
        }))
    }
    /// Reference a protein label in its source gene context.
    pub fn protein(gene: GeneSymbol, label: Text) -> Self {
        Self(Fields::Protein(ProteinRef {
            gene,
            hgvs: None,
            label: Some(label),
        }))
    }
    /// Preserve a compound profile and its native member references.
    pub fn profile(record: Text, label: Text, members: Box<[Self]>) -> Result<Self, crate::Error> {
        Fields::Profile {
            record,
            label,
            members,
        }
        .try_into()
    }
}
impl Serialize for VariantRef {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(serializer)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
struct AlleleRef {
    /// Source identifier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    id: Option<VariantId>,
    /// Source-qualified HGVS.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    hgvs: Option<HgvsText>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
struct ProteinRef {
    /// Source gene.
    gene: GeneSymbol,
    /// Source protein HGVS.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    hgvs: Option<HgvsText>,
    /// Source protein label.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    label: Option<Text>,
}
#[cfg(feature = "schema")]
#[derive(schemars::JsonSchema)]
#[schemars(tag = "level", rename_all = "snake_case", deny_unknown_fields)]
#[expect(dead_code, reason = "Schema-only nonrecursive profile member boundary")]
enum LeafRef {
    Allele(AlleleRef),
    Protein(ProteinRef),
}
