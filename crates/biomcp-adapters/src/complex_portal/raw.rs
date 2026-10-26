use biodata::{ComplexPortalId,Text};
use biomcp_fetch::{BadReply,Reply};
use serde::Deserialize;
#[derive(Debug,Deserialize)]
#[serde(rename_all="camelCase")]
pub(super) struct Search {pub total_number_of_results:u64,pub elements:Vec<Complex>}
impl Reply for Search {
    fn check(&self)->Result<(),BadReply>{
        if self.elements.len() as u64 > self.total_number_of_results || self.elements.len()>25 {return Err(BadReply);}
        if self.elements.iter().any(|c| taxon(&c.organism_name)!=Some(9606)) {return Err(BadReply);}
        Ok(())
    }
}
#[derive(Debug,Deserialize)]
#[serde(rename_all="camelCase")]
pub(super) struct Complex {
    #[serde(rename="complexAC")]
    pub id:ComplexPortalId,
    #[serde(rename="complexName")]
    pub name:Text,
    pub organism_name:String,
    pub predicted_complex:bool,
    pub interactors:Option<Vec<Member>>,
}
#[derive(Debug,Deserialize)]
#[serde(rename_all="camelCase")]
pub(super) struct Detail {
    #[serde(rename="complexAc")]
    pub id:ComplexPortalId,
    pub name:Text,
    pub species:String,
    pub predicted_complex:bool,
    pub participants:Vec<Member>,
}
impl Reply for Detail {
    fn check(&self)->Result<(),BadReply>{if taxon(&self.species)==Some(9606){Ok(())}else{Err(BadReply)}}
}
#[derive(Debug,Deserialize)]
#[serde(rename_all="camelCase")]
pub(super) struct Member {
    pub identifier:Text,
    pub interactor_type:Text,
    pub organism_name:Option<String>,
    pub name:Option<Text>,
    pub stochiometry:Option<Text>,
    pub xrefs:Option<Vec<Xref>>,
    pub cross_references:Option<Vec<Xref>>,
}
#[derive(Debug,Deserialize)]
pub(super) struct Xref {pub database:Text,pub identifier:Text}
pub(super) fn taxon(s:&str)->Option<u32>{s.rsplit_once(';')?.1.trim().parse().ok()}
