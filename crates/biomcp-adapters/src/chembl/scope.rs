//! Preserve request molecule bindings even when a relation identity is malformed.
use crate::raw::Field;
use biomcp_fetch::{RawValue, decode_bundle_document};
use serde::{Deserialize, de::DeserializeOwned};
#[derive(Clone, Debug, Deserialize)]
pub(super) struct Molecule {
    pub molecule_chembl_id: String,
    parent_molecule_chembl_id: Option<String>,
}
impl Molecule {
    pub(super) fn matches(&self, field: &str, id: &str) -> bool {
        chembl_id(&self.molecule_chembl_id)
            && match field {
                PARENT => self.parent_molecule_chembl_id.as_deref() == Some(id),
                _ => self.molecule_chembl_id == id,
            }
    }
}
pub(super) const PARENT: &str = "parent_molecule_chembl_id";

pub(super) fn chembl_id(id: &str) -> bool {
    id.strip_prefix("CHEMBL")
        .is_some_and(|digits| !digits.is_empty() && digits.bytes().all(|b| b.is_ascii_digit()))
}
#[derive(Debug)]
pub(super) struct Row<T, Scope = Molecule> {
    pub scope: Scope,
    pub value: Field<T>,
}
impl<'de, T: DeserializeOwned, Scope: DeserializeOwned> Deserialize<'de> for Row<T, Scope> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let raw = Box::<RawValue>::deserialize(deserializer)?;
        let scope =
            decode_bundle_document(raw.get().as_bytes()).map_err(serde::de::Error::custom)?;
        let value =
            decode_bundle_document(raw.get().as_bytes()).map_err(serde::de::Error::custom)?;
        Ok(Self { scope, value })
    }
}
