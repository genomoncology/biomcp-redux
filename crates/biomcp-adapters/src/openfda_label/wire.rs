//! Source label fields admit nonidentity values independently.
use crate::{native_text::NativeText, raw::Field};
use biodata::{DrugId, Text};
use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub(super) struct Record {
    /// Native revision identifier, distinct from its SPL set.
    pub id: DrugId,
    /// Native SPL set identifier.
    pub set_id: DrugId,
    /// Source-stated revision number.
    pub version: Option<Field<Text>>,
    /// Native YYYYMMDD revision date.
    pub effective_time: Option<Field<String>>,
    pub(super) openfda: Option<Field<Names>>,
    pub(super) spl_product_data_elements: Option<Field<Vec<String>>>,
    pub(super) package_label_principal_display_panel: Option<Field<Vec<String>>>,
    pub(super) boxed_warning: Option<Field<Vec<String>>>,
    pub(super) dosage_and_administration: Option<Field<Vec<String>>>,
    pub(super) drug_interactions: Option<Field<Vec<String>>>,
    pub(super) indications_and_usage: Option<Field<Vec<String>>>,
    pub(super) warnings_and_cautions: Option<Field<Vec<String>>>,
}
#[derive(Debug, Deserialize)]
pub(super) struct Names {
    pub brand_name: Option<Field<Vec<NativeText>>>,
    pub generic_name: Option<Field<Vec<NativeText>>>,
    pub spl_set_id: Option<Field<Vec<DrugId>>>,
    pub application_number: Option<Field<Vec<NativeText>>>,
}
