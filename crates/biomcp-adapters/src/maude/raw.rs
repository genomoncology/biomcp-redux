use biodata::{PartialDate, ReportId};
use biomcp_fetch::{BadReply, Reply};
use serde::Deserialize;
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub(super) enum Response {
    Page { meta: Meta, results: Vec<Report> },
    Absent { error: Absence },
}
#[derive(Debug, Deserialize)]
pub(super) struct Meta {
    /// Native last_updated without inferred interpretation.
    pub last_updated: Option<PartialDate>,
    /// Native results without inferred interpretation.
    pub results: Counts,
}
#[derive(Debug, Deserialize)]
pub(super) struct Counts {
    /// Native skip without inferred interpretation.
    pub skip: u32,
    /// Native limit without inferred interpretation.
    pub limit: u32,
    /// Native total without inferred interpretation.
    pub total: u64,
}
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Absence {
    code: String,
    message: String,
}
impl Reply for Response {
    fn check(&self) -> Result<(), BadReply> {
        match self {
            Self::Page { meta, results }
                if meta.results.limit > 0
                    && results.len() as u64
                        == meta
                            .results
                            .total
                            .saturating_sub(meta.results.skip.into())
                            .min(meta.results.limit.into())
                    && results.iter().all(|r| {
                        r.mdr_report_key
                            .as_str()
                            .bytes()
                            .all(|b| b.is_ascii_digit())
                    }) =>
            {
                Ok(())
            }
            Self::Absent { error }
                if error.code == "NOT_FOUND" && error.message == "No matches found!" =>
            {
                Ok(())
            }
            _ => Err(BadReply),
        }
    }
    fn check_status(&self, status: u16) -> Result<(), BadReply> {
        if matches!(
            (self, status),
            (Self::Page { .. }, 200) | (Self::Absent { .. }, 404)
        ) {
            Ok(())
        } else {
            Err(BadReply)
        }
    }
}
#[derive(Debug, Deserialize)]
/// Kept native wire fields without administrative metadata.
pub(super) struct Device {
    /// Native device_sequence_number without inferred interpretation.
    pub device_sequence_number: Option<crate::raw::Field<Box<str>>>,
    /// Native brand_name without inferred interpretation.
    pub brand_name: Option<crate::raw::Field<Box<str>>>,
    /// Native generic_name without inferred interpretation.
    pub generic_name: Option<crate::raw::Field<Box<str>>>,
    /// Native manufacturer_d_name without inferred interpretation.
    pub manufacturer_d_name: Option<crate::raw::Field<Box<str>>>,
    /// Native device_report_product_code without inferred interpretation.
    pub device_report_product_code: Option<crate::raw::Field<Box<str>>>,
    /// Native model_number without inferred interpretation.
    pub model_number: Option<crate::raw::Field<Box<str>>>,
    /// Native catalog_number without inferred interpretation.
    pub catalog_number: Option<crate::raw::Field<Box<str>>>,
    /// Native udi_di without inferred interpretation.
    pub udi_di: Option<crate::raw::Field<Box<str>>>,
    /// Native udi_public without inferred interpretation.
    pub udi_public: Option<crate::raw::Field<Box<str>>>,
    /// Native lot_number without inferred interpretation.
    pub lot_number: Option<crate::raw::Field<Box<str>>>,
    /// Native device_operator without inferred interpretation.
    pub device_operator: Option<crate::raw::Field<Box<str>>>,
    /// Native implant_flag without inferred interpretation.
    pub implant_flag: Option<crate::raw::Field<Box<str>>>,
    /// Native implant_date_year without inferred interpretation.
    pub implant_date_year: Option<crate::raw::Field<Box<str>>>,
    /// Native date_removed_flag without inferred interpretation.
    pub date_removed_flag: Option<crate::raw::Field<Box<str>>>,
    /// Native date_removed_year without inferred interpretation.
    pub date_removed_year: Option<crate::raw::Field<Box<str>>>,
    /// Native device_age_text without inferred interpretation.
    pub device_age_text: Option<crate::raw::Field<Box<str>>>,
    /// Native expiration_date_of_device without inferred interpretation.
    pub expiration_date_of_device: Option<crate::raw::Field<Box<str>>>,
    /// Native device_availability without inferred interpretation.
    pub device_availability: Option<crate::raw::Field<Box<str>>>,
    /// Native device_evaluated_by_manufacturer without inferred interpretation.
    pub device_evaluated_by_manufacturer: Option<crate::raw::Field<Box<str>>>,
    /// Native device_event_key without inferred interpretation.
    pub device_event_key: Option<crate::raw::Field<Box<str>>>,
    /// Native date_received without inferred interpretation.
    pub date_received: Option<crate::raw::Field<Box<str>>>,
    /// Native other_id_number without inferred interpretation.
    pub other_id_number: Option<crate::raw::Field<Box<str>>>,
    /// Native combination_product_flag without inferred interpretation.
    pub combination_product_flag: Option<crate::raw::Field<Box<str>>>,
    /// Native serviced_by_3rd_party_flag without inferred interpretation.
    pub serviced_by_3rd_party_flag: Option<crate::raw::Field<Box<str>>>,
}
#[derive(Debug, Deserialize)]
/// Kept native wire fields without administrative metadata.
pub(super) struct Narrative {
    /// Native mdr_text_key without inferred interpretation.
    pub mdr_text_key: Option<crate::raw::Field<Box<str>>>,
    /// Native patient_sequence_number without inferred interpretation.
    pub patient_sequence_number: Option<crate::raw::Field<Box<str>>>,
    /// Native text_type_code without inferred interpretation.
    pub text_type_code: Option<crate::raw::Field<Box<str>>>,
    /// Native text without inferred interpretation.
    pub text: Option<crate::raw::Field<Box<str>>>,
}
#[derive(Debug, Deserialize)]
/// Kept native wire fields without administrative metadata.
pub(super) struct Report {
    /// Native mdr_report_key without inferred interpretation.
    pub mdr_report_key: ReportId,
    /// Native report_number without inferred interpretation.
    pub report_number: Option<crate::raw::Field<Box<str>>>,
    /// Native event_type without inferred interpretation.
    pub event_type: Option<crate::raw::Field<Box<str>>>,
    /// Native date_of_event without inferred interpretation.
    pub date_of_event: Option<crate::raw::Field<Box<str>>>,
    /// Native date_received without inferred interpretation.
    pub date_received: Option<crate::raw::Field<Box<str>>>,
    /// Native report_date without inferred interpretation.
    pub report_date: Option<crate::raw::Field<Box<str>>>,
    /// Native date_report without inferred interpretation.
    pub date_report: Option<crate::raw::Field<Box<str>>>,
    /// Native date_report_to_fda without inferred interpretation.
    pub date_report_to_fda: Option<crate::raw::Field<Box<str>>>,
    /// Native number_devices_in_event without inferred interpretation.
    pub number_devices_in_event: Option<crate::raw::Field<Box<str>>>,
    /// Native number_patients_in_event without inferred interpretation.
    pub number_patients_in_event: Option<crate::raw::Field<Box<str>>>,
    #[serde(rename = "device")]
    /// Native devices without inferred interpretation.
    pub devices: Option<crate::raw::Field<Vec<crate::raw::Field<Device>>>>,
    #[serde(rename = "mdr_text")]
    /// Native narrative without inferred interpretation.
    pub narrative: Option<crate::raw::Field<Vec<crate::raw::Field<Narrative>>>>,
}
