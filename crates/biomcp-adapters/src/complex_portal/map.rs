use super::{bad,raw,Complex};
use biodata::{ProteinComplexHeader,ProteinComplexMember,Text,UniProtIsoform};
use biomcp_fetch::RequestError;
pub(super) fn members(rows:Vec<raw::Member>)->Result<Vec<ProteinComplexMember>,RequestError>{
    rows.into_iter().map(|r| {
        let protein=if r.interactor_type.as_str()=="protein" {UniProtIsoform::new(r.identifier.as_str()).ok().filter(|p|p.as_str()==r.identifier.as_str())}else{None};
        let xrefs=r.xrefs.or(r.cross_references).into_iter().flatten().map(|x|format!("{}:{}",x.database,x.identifier).parse().map_err(|_|bad())).collect::<Result<Box<[_]>,RequestError>>()?;
        Ok(ProteinComplexMember{id:r.identifier,participant_type:r.interactor_type,taxon:r.organism_name.as_deref().and_then(raw::taxon),name:r.name,stoichiometry:r.stochiometry,xrefs,protein})
    }).collect()
}
pub(super) fn complex(raw:raw::Complex,anchor:&UniProtIsoform)->Result<Option<Complex>,RequestError>{
    let rows=match raw.interactors {Some(rows)=>rows,None=>return Ok(None)};
    let members=members(rows)?;
    if !members.iter().any(|m|m.protein.as_ref()==Some(anchor)) {return Ok(None);}
    let header=ProteinComplexHeader{id:raw.id.clone(),name:raw.name,taxon:9606,predicted:raw.predicted_complex,participant_total:members.len() as u64,matched_participant:text(anchor.as_str())?,url:text(&format!("https://www.ebi.ac.uk/complexportal/complex/{}",raw.id))?};
    Ok(Some(Complex{header,members}))
}
pub(super) fn text(s:&str)->Result<Text,RequestError>{Text::new(s).map_err(|_|bad())}
