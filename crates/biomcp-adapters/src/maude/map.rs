use super::raw;
use biomcp_types::{AdverseEventDevice, AdverseEventNarrativeItem};
/// Preserve the complete reviewed native row without administrative fields.
pub fn device(index: u32, value: raw::Device) -> AdverseEventDevice {
    AdverseEventDevice {
        index,
        sequence: value.device_sequence_number,
        brand_name: value.brand_name,
        generic_name: value.generic_name,
        manufacturer: value.manufacturer_d_name,
        product_code: value.device_report_product_code,
        model: value.model_number,
        catalog: value.catalog_number,
        udi_di: value.udi_di,
        udi_public: value.udi_public,
        lot_number: value.lot_number,
        device_operator: value.device_operator,
        implant_flag: value.implant_flag,
        implant_date_year: value.implant_date_year,
        date_removed_flag: value.date_removed_flag,
        date_removed_year: value.date_removed_year,
        device_age_text: value.device_age_text,
        expiration_date_of_device: value.expiration_date_of_device,
        device_availability: value.device_availability,
        device_evaluated_by_manufacturer: value.device_evaluated_by_manufacturer,
        device_event_key: value.device_event_key,
        date_received: value.date_received,
        other_id_number: value.other_id_number,
        combination_product_flag: value.combination_product_flag,
        serviced_by_3rd_party_flag: value.serviced_by_3rd_party_flag,
    }
}
/// Preserve the complete reviewed native row without administrative fields.
pub fn narrative(index: u32, value: raw::Narrative) -> AdverseEventNarrativeItem {
    AdverseEventNarrativeItem {
        index,
        mdr_text_key: value.mdr_text_key,
        patient_sequence_number: value.patient_sequence_number,
        r#type: value.text_type_code,
        text: value.text,
    }
}
