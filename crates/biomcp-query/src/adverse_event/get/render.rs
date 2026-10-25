use crate::{Action,Outcome};
use biodata::{FaersReport,Text};
use biomcp_fetch::RequestError;
use biomcp_types::{AdverseEventGet,AdverseEventHeader,AdverseEventSection as S,AdverseEventSource,QueryBody,Request,StepKind};
mod rows;
mod identifiers;
pub(super) fn render(q:&AdverseEventGet,r:FaersReport,url:Text,snapshot:Text,out:&mut Outcome)->Result<(),RequestError>{
    let q=q.with_snapshot(snapshot.clone());
    let header=AdverseEventHeader {id:r.safetyreportid.clone(),source:AdverseEventSource::Faers,version:r.safetyreportversion.clone(),serious:r.serious.clone(),event_type:None,first_received:r.receivedate.clone()};
    let components=if q.section()==Some(S::Reactions){biomcp_adapters::adverse_event_sources::components::labels(biomcp_types::Source::Faers,biomcp_types::Source::Meddra)?}else{Box::default()};
    match q.section(){
        None=>{
            out.body=QueryBody::AdverseEventFaersCard(Some(biomcp_types::AdverseEventFaersCard{header,snapshot,latest_received:r.receiptdate,latest_received_format:r.receiptdateformat,transmitted:r.transmissiondate,transmitted_format:r.transmissiondateformat,first_received_format:r.receivedateformat,seriousness:r.seriousness,patient:r.patient,reporter:r.reporter,drug_count:r.drugs.len() as u32,reaction_count:r.reactions.len() as u32,source_url:Text::new("https://open.fda.gov/apis/drug/event/").map_err(|_|biomcp_fetch::InputError::Serialization)?,report_url:url,available_sections:vec![S::Drugs,S::Reactions].into(),components}));
            for section in [S::Drugs,S::Reactions]{section_step(&q,section,None,out);}
        }
        Some(S::Drugs)=>{
            let total=r.drugs.len() as u64;
            let records=rows::drugs(&q,&r.drugs);
            page(&q,records.len(),total,out);
            out.body=QueryBody::AdverseEventDrugs(Some(biomcp_types::AdverseEventDrugs{header,snapshot,drugs:biomcp_types::AdverseEventRecords{records,offset:q.section_offset(),total},components}));
            let initial=out.next.len();
            for (index,drug) in r.drugs.iter().enumerate().skip(q.section_offset() as usize).take(q.section_limit() as usize){
                if let Some(ids)=&drug.openfda {
                    for section in identifiers::available(ids){if out.next.len()<initial+3 {section_step(&q,section,Some(index as u32),out);}}
                }
            }
        }
        Some(S::Reactions)=>{
            let total=r.reactions.len() as u64;let records=rows::reactions(&q,&r.reactions);
            page(&q,records.len(),total,out);
            out.body=QueryBody::AdverseEventReactions(Some(biomcp_types::AdverseEventReactions{header,snapshot,reactions:biomcp_types::AdverseEventRecords{records,offset:q.section_offset(),total},components}));
        }
        Some(section)=>identifiers::render(&q,&r,section,header,snapshot,components,out)?,
    }
    Ok(())
}
fn section_step(q:&AdverseEventGet,s:S,index:Option<u32>,out:&mut Outcome){
    if let Ok(q)=q.with_section(s,index){out.next.push((StepKind::Section,Action::Run{request:Request::GetAdverseEvent(q),fresh:false}));}
}
fn page(q:&AdverseEventGet,len:usize,total:u64,out:&mut Outcome){
    let offset=q.section_offset().saturating_add(len as u32);
    if len>0&&u64::from(offset)<total {out.next.push((StepKind::Page,Action::Run{request:Request::GetAdverseEvent(q.at_section_offset(offset)),fresh:false}));}
}
