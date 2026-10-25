use super::raw;
use biodata::{MaudeDevice, MaudeNarrative, MaudeReport};
pub(super) fn device(row: raw::Device) -> MaudeDevice {
    MaudeDevice {
        device_sequence_number: row.device_sequence_number,
        brand_name: row.brand_name,
        generic_name: row.generic_name,
        manufacturer_d_name: row.manufacturer_d_name,
        device_report_product_code: row.device_report_product_code,
        model_number: row.model_number,
        catalog_number: row.catalog_number,
        udi_di: row.udi_di,
        udi_public: row.udi_public,
        lot_number: row.lot_number,
        device_operator: row.device_operator,
        implant_flag: row.implant_flag,
        implant_date_year: row.implant_date_year,
        date_removed_flag: row.date_removed_flag,
        date_removed_year: row.date_removed_year,
        device_age_text: row.device_age_text,
        expiration_date_of_device: row.expiration_date_of_device,
        device_availability: row.device_availability,
        device_evaluated_by_manufacturer: row.device_evaluated_by_manufacturer,
        device_event_key: row.device_event_key,
        date_received: row.date_received,
        other_id_number: row.other_id_number,
        combination_product_flag: row.combination_product_flag,
        serviced_by_3rd_party_flag: row.serviced_by_3rd_party_flag,
    }
}
pub(super) fn narrative(row: raw::Narrative) -> MaudeNarrative {
    MaudeNarrative {
        mdr_text_key: row.mdr_text_key,
        patient_sequence_number: row.patient_sequence_number,
        text_type_code: row.text_type_code,
        text: row.text,
    }
}
pub(super) fn report(row: raw::Report) -> MaudeReport {
    MaudeReport {
        mdr_report_key: row.mdr_report_key,
        report_number: row.report_number,
        event_type: row.event_type,
        date_of_event: row.date_of_event,
        date_received: row.date_received,
        report_date: row.report_date,
        date_report: row.date_report,
        date_report_to_fda: row.date_report_to_fda,
        number_devices_in_event: row.number_devices_in_event,
        number_patients_in_event: row.number_patients_in_event,
        devices: row.devices.into_vec().into_iter().map(device).collect(),
        narrative: row
            .narrative
            .into_vec()
            .into_iter()
            .map(narrative)
            .collect(),
    }
}
