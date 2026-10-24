use crate::{Ctx, status};
use biodata::{PageInfo, Problem, ProblemCode, Reason, Section, Source, SourceOutcome, VariantGet, VariantHeader, VariantHotspots};
use biomcp_fetch::{Deadline, RequestError};

pub(super) fn section(cx: &Ctx<'_>, query: &VariantGet, candidates: &[VariantHeader], deadline: Deadline<'_>) -> (Section<VariantHotspots>, Vec<SourceOutcome>, Vec<Problem>, Option<RequestError>) {
    let leg = cx.fetch.leg(&biomcp_adapters::hotspots::SPEC,deadline).fresh(cx.fresh);
    let genes = candidates.iter().flat_map(|c| c.identity.genes.iter().chain(c.myvariant.iter().flat_map(|v|v.genes.iter()))).collect::<std::collections::BTreeSet<_>>();
    let mut rows = Vec::new();
    let mut problems = Vec::new();
    let mut error = None;
    for gene in &genes {
        match biomcp_adapters::hotspots::gene(&leg,gene) {
            Ok(mapped) => {rows.extend(mapped.value);status::append(&mut problems,mapped.problems);},
            Err(e) => {error=Some(e);break;},
        }
    }
    if genes.is_empty() {error=Some(biomcp_fetch::SourceError{source:Source::CancerHotspots,reason:Reason::NoJoinId,status:None,retry_after:None}.into());}
    let source = status::source(Source::CancerHotspots,leg.finish(),error);
    let value = if let Some(error) = error {
        Section::unavailable(Source::CancerHotspots,match error {RequestError::Source(e)=>e.reason,_=>Reason::Unavailable})
    } else if rows.is_empty() {
        Section::Empty{source:Source::CancerHotspots,record:None}
    } else {
        problems.push(Problem{source:Source::CancerHotspots,record:None,field:"transcript_consequence_join".into(),code:ProblemCode::ValueUnknown});
        let total=rows.len() as u64;
        let records=rows.into_iter().skip(query.section_offset() as usize).take(query.section_limit() as usize).collect::<Vec<_>>();
        let returned=records.len() as u32;
        let next=if (query.section_offset() as u64 + returned as u64)<total {Some(query.at_section_offset(query.section_offset()+returned))}else{None};
        Section::Data{source:Source::CancerHotspots,data:VariantHotspots{reason:Some(Reason::NoJoinId),records:records.into(),page:PageInfo{offset:query.section_offset(),returned,total,pool:None,truncated:None},next}}
    };
    (value,vec![source],problems,error.and_then(status::local))
}
