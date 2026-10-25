use super::{SPEC, map, raw};
use crate::Mapped;
use biomcp_fetch::{Arg, BadReply, Freshness, Leg, RequestError};
use biomcp_types::{DiseaseCard, DiseaseGet};
/// The exact card and complete native section rows, sliced by the query layer.
#[derive(Debug)]
pub struct Record {
    /// Exact identity and native metadata.
    pub card: DiseaseCard,
    /// Every native synonym, with scope absent when the publisher omits it.
    pub synonyms: Mapped<Vec<biodata::DiseaseSynonym>>,
    /// Every cross-reference, without inferred equivalence.
    pub xrefs: Mapped<Vec<biodata::DiseaseRelation>>,
}
/// Request the exact CURIE and refuse wrong returned identities before cache admission.
pub fn get(leg: &Leg<'_>, query: &DiseaseGet) -> Result<Option<Record>, LookupError> {
    let field=std::cell::Cell::new(None);
    checked(leg,query,&field).map_err(|error|LookupError {error,field:field.get()})
}
/// A transport error with an exact identity-field rejection only when established.
#[derive(Clone,Copy,Debug)]
pub struct LookupError {
    /// Shared request failure.
    pub error: RequestError,
    /// Native identity field that actually mismatched.
    pub field: Option<&'static str>,
}
fn checked(leg:&Leg<'_>,query:&DiseaseGet,field:&std::cell::Cell<Option<&'static str>>) -> Result<Option<Record>,RequestError> {
    let ontology = crate::disease_sources::components::ontology(
        leg,
        query
            .ontology()
            .ok_or(biomcp_fetch::InputError::Serialization)?,
    )?;
    let endpoint = SPEC
        .get("/api/ontologies")
        .with_path(format!("/api/ontologies/{}/terms", ontology.as_str()))?
        .fresh(Freshness::Days(1));
    let (reply, _) = leg.json_received_checked::<raw::Exact>(
        &endpoint,
        &[("obo_id", Arg::Text(query.id().as_str()))],
        |reply| {
            for term in &reply.embedded.terms {
                if term.obo_id.to_string()!=query.id().as_str() {field.set(Some("returned_id"));return Err(BadReply);}
                if term.ontology_name.as_str()!=ontology.as_str() {field.set(Some("ontology"));return Err(BadReply);}
            }
            Ok(())
        },
    )?;
    reply
        .embedded
        .terms
        .into_iter()
        .next()
        .map(|term| map::record(term, ontology))
        .transpose()
}
