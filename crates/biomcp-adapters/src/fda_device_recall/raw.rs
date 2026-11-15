use crate::openfda::Envelope;
pub(super) type Response<T> = Envelope<T>;
use crate::raw::Field;
use biodata::{DeviceRecallNumber, PartialDate, Text};
use biomcp_types::{Problem, Source};
use serde::{Deserialize, de::DeserializeOwned};

/// A search page row: a record with a well-formed recall number, or one declined for an empty,
/// `N/A` or otherwise malformed number and named by its native key. Exact lookups read the
/// strict record types, so they never admit a declined row.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub(super) enum Row<T, K> {
    Admitted(T),
    Declined(K),
}
/// The native key of a declined row.
pub(super) trait Key {
    fn declined(&self) -> Problem;
}
#[derive(Debug, Deserialize)]
pub(super) struct RecordKey {
    pub product_res_number: Option<Field<String>>,
    pub cfres_id: Option<Field<String>>,
}
impl Key for RecordKey {
    fn declined(&self) -> Problem {
        crate::mapped::declined(
            Source::FdaDeviceRecall,
            "product_res_number",
            self.product_res_number.as_ref(),
            "cfres_id",
            self.cfres_id.as_ref(),
        )
    }
}
#[derive(Debug, Deserialize)]
pub(super) struct EnforcementKey {
    pub recall_number: Option<Field<String>>,
    pub event_id: Option<Field<String>>,
}
impl Key for EnforcementKey {
    fn declined(&self) -> Problem {
        crate::mapped::declined(
            Source::FdaDeviceRecall,
            "recall_number",
            self.recall_number.as_ref(),
            "event_id",
            self.event_id.as_ref(),
        )
    }
}
impl<T: crate::openfda::Item, K: Key + DeserializeOwned> crate::openfda::Item for Row<T, K> {}

// The recall number must stay the only required field of Record and Enforcement: a search
// page reads any row that fails them as a declined `Row`, whose problem names that field.
#[derive(Debug, Deserialize)]
pub(super) struct Record {
    pub product_res_number: DeviceRecallNumber,
    pub res_event_number: Option<Field<String>>,
    pub recalling_firm: Option<Field<String>>,
    pub product_code: Option<Field<String>>,
    pub recall_status: Option<Field<String>>,
    pub event_date_initiated: Option<Field<PartialDate>>,
    pub reason_for_recall: Option<Field<String>>,
    pub root_cause_description: Option<Field<String>>,
    pub product_description: Option<Field<String>>,
    pub distribution_pattern: Option<Field<String>>,
    pub k_numbers: Option<Field<Vec<Field<Text>>>>,
    pub pma_numbers: Option<Field<Vec<Field<Text>>>>,
}
#[derive(Debug, Deserialize)]
pub(super) struct Enforcement {
    pub recall_number: DeviceRecallNumber,
    pub classification: Option<Field<String>>,
    pub reason_for_recall: Option<Field<String>>,
    pub voluntary_mandated: Option<Field<String>>,
    pub initial_firm_notification: Option<Field<String>>,
    pub product_quantity: Option<Field<String>>,
    pub center_classification_date: Option<Field<String>>,
    pub report_date: Option<Field<String>>,
}
pub(super) trait Identity {
    fn id(&self) -> &DeviceRecallNumber;
}
impl Identity for Record {
    fn id(&self) -> &DeviceRecallNumber {
        &self.product_res_number
    }
}
impl Identity for Enforcement {
    fn id(&self) -> &DeviceRecallNumber {
        &self.recall_number
    }
}

impl crate::openfda::Item for Record {}

impl crate::openfda::Item for Enforcement {}
