//! Preserve request molecule bindings even when a relation identity is malformed.
use crate::raw::Field;
use biomcp_fetch::{RawValue, decode_bundle_document};
use serde::{Deserialize, de::DeserializeOwned};
#[derive(Deserialize)]
struct Molecule {
    molecule_chembl_id: String,
}
#[derive(Debug)]
pub(super) struct Row<T> {
    pub molecule: String,
    pub value: Field<T>,
}
impl<'de, T: DeserializeOwned> Deserialize<'de> for Row<T> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let raw = Box::<RawValue>::deserialize(deserializer)?;
        let molecule: Molecule =
            decode_bundle_document(raw.get().as_bytes()).map_err(serde::de::Error::custom)?;
        let value =
            decode_bundle_document(raw.get().as_bytes()).map_err(serde::de::Error::custom)?;
        Ok(Self {
            molecule: molecule.molecule_chembl_id,
            value,
        })
    }
}
