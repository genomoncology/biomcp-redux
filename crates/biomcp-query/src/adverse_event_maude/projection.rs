use super::*;
use biomcp_types::{AdverseEventDevices,AdverseEventMaudeCard,AdverseEventNarrative,AdverseEventRecords};
/// Whole narrative page cap; oversized rows produce a failure and never fragments.
const NARRATIVE_PAGE_BYTES:usize=32*1024;
pub(super) fn project(query:&AdverseEventGet,report:maude::Report)->Result<(QueryBody,u64,u32),()> {
    let header=maude::header(&report.record);
    let snapshot=text(&format!("maude-{}-{}",report.metadata.identity.iter().map(|b|format!("{b:02x}")).collect::<String>(),report.metadata.fetched))?;
    let row=report.record;
    let offset=query.section_offset() as usize;
    let limit=query.section_limit() as usize;
    let result=match query.section() {
        Some(AdverseEventSection::Devices)=>{
            let total=row.devices.len() as u64;
            let records:Box<[_]>=row.devices.into_vec().into_iter().enumerate().skip(offset).take(limit).map(|(i,r)|maude::device(i as u32,r)).collect();
            let returned=records.len() as u32;
            (QueryBody::AdverseEventDevices(Some(AdverseEventDevices {header,snapshot,devices:AdverseEventRecords {records,offset:query.section_offset(),total},components:biomcp_adapters::adverse_event_sources::components::labels(Source::Maude,Source::Gmdn).map_err(|_|())?})),total,returned)
        }
        Some(AdverseEventSection::Narrative)=>{
            let total=row.narrative.len() as u64;
            let records:Box<[_]>=row.narrative.into_vec().into_iter().enumerate().skip(offset).take(limit).map(|(i,r)|maude::narrative(i as u32,r)).collect();
            if serde_json::to_vec(&records).map_err(|_|())?.len()>NARRATIVE_PAGE_BYTES {return Err(());}
            let returned=records.len() as u32;
            (QueryBody::AdverseEventNarrative(Some(AdverseEventNarrative {header,snapshot,narrative:AdverseEventRecords {records,offset:query.section_offset(),total},components:Box::default()})),total,returned)
        }
        _=>{
            let card=AdverseEventMaudeCard {header,snapshot,report_number:row.report_number,event_date:row.date_of_event,received:row.date_received,report_date:row.report_date,date_report:row.date_report,date_report_to_fda:row.date_report_to_fda,number_devices_in_event:row.number_devices_in_event,number_patients_in_event:row.number_patients_in_event,device_count:row.devices.len() as u32,narrative_count:row.narrative.len() as u32,source_url:text("https://open.fda.gov/apis/device/event/")?,report_url:text(&format!("https://api.fda.gov/device/event.json?search=mdr_report_key%3A%22{}%22&limit=1",query.id().as_str()))?,available_sections:vec![AdverseEventSection::Devices,AdverseEventSection::Narrative].into(),components:Box::default()};
            (QueryBody::AdverseEventMaudeCard(Some(card)),0,0)
        }
    };
    Ok(result)
}
