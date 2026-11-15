use crate::openfda::Envelope;
pub(super) type Response<T> = Envelope<T>;
use crate::raw::Field;
use biodata::DrugRecallNumber;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(super) struct Record {
    /// openFDA sends some records with an empty or `N/A` recall number.
    pub recall_number: Option<Field<DrugRecallNumber>>,
    pub classification: Option<Field<String>>,
    pub status: Option<Field<String>>,
    pub recalling_firm: Option<Field<String>>,
    pub product_description: Option<Field<String>>,
    pub recall_initiation_date: Option<Field<String>>,
    pub report_date: Option<Field<String>>,
    pub reason_for_recall: Option<Field<String>>,
    pub distribution_pattern: Option<Field<String>>,
    pub code_info: Option<Field<String>>,
    pub voluntary_mandated: Option<Field<String>>,
}

impl crate::openfda::Item for Record {}
