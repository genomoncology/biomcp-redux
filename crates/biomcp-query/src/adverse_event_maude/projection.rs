use super::{AdverseEventGet, AdverseEventSection, QueryBody, Source, Text, rows, text};
use biomcp_types::{
    AdverseEventDevices, AdverseEventMaudeCard, AdverseEventNarrative, AdverseEventRecords,
};
/// Whole narrative page cap; oversized rows produce a failure and never fragments.
const NARRATIVE_PAGE_BYTES: usize = 32 * 1024;
pub(super) fn project(
    query: &AdverseEventGet,
    report: biodata::MaudeReport,
    snapshot: Text,
) -> Result<(QueryBody, u64, u32), ()> {
    let header = header(&report);
    let row = report;
    let offset = query.section_offset() as usize;
    let limit = query.section_limit() as usize;
    let result = match query.section() {
        Some(AdverseEventSection::Devices) => {
            let total = row.devices.len() as u64;
            let records: Box<[_]> = row
                .devices
                .into_vec()
                .into_iter()
                .enumerate()
                .skip(offset)
                .take(limit)
                .map(|(i, r)| rows::device(i as u32, r))
                .collect();
            let returned = records.len() as u32;
            (
                QueryBody::AdverseEventDevices(Some(AdverseEventDevices {
                    header,
                    snapshot,
                    devices: AdverseEventRecords {
                        records,
                        offset: query.section_offset(),
                        total,
                    },
                    components: biomcp_adapters::adverse_event_sources::components::labels(
                        Source::Maude,
                        Source::Gmdn,
                    )
                    .map_err(|_| ())?,
                })),
                total,
                returned,
            )
        }
        Some(AdverseEventSection::Narrative) => {
            let total = row.narrative.len() as u64;
            let candidates = row
                .narrative
                .into_vec()
                .into_iter()
                .enumerate()
                .skip(offset)
                .take(limit)
                .map(|(i, r)| rows::narrative(i as u32, r));
            let records = narrative_page(candidates)?;
            let returned = records.len() as u32;
            (
                QueryBody::AdverseEventNarrative(Some(AdverseEventNarrative {
                    header,
                    snapshot,
                    narrative: AdverseEventRecords {
                        records,
                        offset: query.section_offset(),
                        total,
                    },
                    components: Box::default(),
                })),
                total,
                returned,
            )
        }
        _ => (
            QueryBody::AdverseEventMaudeCard(Some(card(query, row, header, snapshot)?)),
            0,
            0,
        ),
    };
    Ok(result)
}

fn card(
    query: &AdverseEventGet,
    row: biodata::MaudeReport,
    header: biomcp_types::AdverseEventHeader,
    snapshot: Text,
) -> Result<AdverseEventMaudeCard, ()> {
    let card = AdverseEventMaudeCard {
        header,
        snapshot,
        report_number: row.report_number,
        event_date: row.date_of_event,
        received: row.date_received,
        report_date: row.report_date,
        date_report: row.date_report,
        date_report_to_fda: row.date_report_to_fda,
        number_devices_in_event: row.number_devices_in_event,
        number_patients_in_event: row.number_patients_in_event,
        device_count: row.devices.len() as u32,
        narrative_count: row.narrative.len() as u32,
        source_url: text("https://open.fda.gov/apis/device/event/")?,
        report_url: text(
            &biomcp_adapters::registry::records::record_url(
                biomcp_types::Source::Maude,
                biomcp_adapters::registry::records::RecordKind::AdverseEvent,
                query.id().as_str(),
            )
            .ok_or(())?,
        )?,
        available_sections: vec![AdverseEventSection::Devices, AdverseEventSection::Narrative]
            .into(),
        components: Box::default(),
    };
    Ok(card)
}

fn narrative_page(
    rows: impl Iterator<Item = biomcp_types::AdverseEventNarrativeItem>,
) -> Result<Box<[biomcp_types::AdverseEventNarrativeItem]>, ()> {
    let mut records = Vec::new();
    let mut bytes = 2;
    for row in rows {
        let additional =
            serde_json::to_vec(&row).map_err(|_| ())?.len() + usize::from(!records.is_empty());
        if bytes + additional > NARRATIVE_PAGE_BYTES {
            if records.is_empty() {
                return Err(());
            }
            break;
        }
        bytes += additional;
        records.push(row);
    }
    Ok(records.into())
}

fn header(row: &biodata::MaudeReport) -> biomcp_types::AdverseEventHeader {
    biomcp_types::AdverseEventHeader {
        id: row.mdr_report_key.clone(),
        source: biomcp_types::AdverseEventSource::Maude,
        version: None,
        serious: None,
        event_type: row.event_type.clone(),
        first_received: row.date_received.clone(),
    }
}
