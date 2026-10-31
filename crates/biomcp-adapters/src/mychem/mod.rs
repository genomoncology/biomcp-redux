//! Canonical MYCHEM source declaration.
use biomcp_fetch::{KeyUse, SourceSpec, Terms};
use biomcp_types::Source;
use std::time::Duration;
/// Publisher policy shared by health, request admission and response outcomes.
pub static SPEC: SourceSpec = SourceSpec {
    destinations: biomcp_fetch::Destinations::Primary,
    user_agent: None,
    source: Source::Mychem,
    base: "https://mychem.info/v1",
    terms: Terms {
        name: crate::drug_sources::MYCHEM_TERMS.name,
        url: crate::drug_sources::MYCHEM_TERMS.url,
        usage: crate::drug_sources::MYCHEM_TERMS.usage,
    },
    quota: &crate::quotas::BIOTHINGS,
    key: KeyUse::None,
    deadline: Duration::from_secs(15),
};
mod links;
mod map;
mod normalize;
mod raw;
use biomcp_fetch::{Arg, Endpoint, Freshness, Leg, PathSegment, RequestError};
use biomcp_types::{DrugCard, DrugPage};
/// Reviewed identity fields, excluding every unapproved source component.
pub const SEARCH_FIELDS: &str = "drugbank.id,drugbank.name,chembl.molecule_chembl_id,chembl.pref_name,chembl.molecule_type,\
                    gtopdb.ligand_id,gtopdb.name,gtopdb.type,unii.unii,unii.display_name,unii.ingredient_type,ndc.product_id,\
                    ndc.productndc,ndc.producttypename,ndc.nonproprietaryname,ndc.proprietaryname";
const CARD_FIELDS: &str = concat!(
    "drugbank.id,drugbank.name,chembl.molecule_chembl_id,chembl.pref_name,chembl.molecule_type,gtopdb.ligand_id,gtopdb.name,\
                    gtopdb.type,unii.unii,unii.display_name,unii.ingredient_type,ndc.product_id,ndc.productndc,ndc.producttypename,\
                    ndc.nonproprietaryname,ndc.proprietaryname",
    ",drugbank.synonyms,chembl.molecule_hierarchy,chembl.molecule_synonyms,drugcentral.id,drugcentral.synonyms,drugcentral.xrefs,\
                    gtopdb.synonyms,gtopdb.chembl_id,gtopdb.xrefs,unii.rxcui,unii.ncit,ndc.applicationnumber,ndc.dosageformname,\
                    ndc.package,openfda.generic_name,openfda.brand_name,openfda.spl_set_id,openfda.application_number"
);
const SEARCH: Endpoint = SPEC.get("/query").fresh(Freshness::Days(7));
const GET: Endpoint = SPEC.get("/chem").fresh(Freshness::Days(7));
/// Fetch one literal native page with policy applied before cache and transport identity.
pub fn search(
    leg: &Leg<'_>,
    query: &str,
    limit: u32,
    offset: u32,
) -> Result<(crate::Mapped<DrugPage>, u32), RequestError> {
    let fields = crate::drug_sources::components::allowed_fields(leg, SEARCH_FIELDS);
    let reply = leg.json_checked_admission::<raw::Page>(
        &SEARCH,
        &[
            ("q", Arg::Text(query)),
            ("size", Arg::Num(limit as u64)),
            ("from", Arg::Num(offset as u64)),
            ("fields", Arg::Text(&fields)),
        ],
        |_| Ok(()),
        raw::Page::identities_admitted,
    )?;
    let received = reply.hits.len() as u32;
    map::page(reply, leg)
        .map(|page| (page, received))
        .map_err(|_| biomcp_fetch::bad_reply(Source::Mychem, None))
}
/// Retrieve an exact native document and verify the returned identity, never searching by name.
pub fn get(leg: &Leg<'_>, id: &str) -> Result<crate::Mapped<DrugDocument>, RequestError> {
    let segment = PathSegment::new(id).map_err(RequestError::Input)?;
    let fields = crate::drug_sources::components::allowed_fields(leg, CARD_FIELDS);
    let (reply, received) = leg.json_received_checked::<raw::Hit>(
        &GET.clone().with_segments(&[segment]),
        &[("fields", Arg::Text(&fields))],
        |hit| {
            if hit.id == id {
                Ok(())
            } else {
                Err(biomcp_fetch::BadReply)
            }
        },
    )?;
    let mut card =
        map::card(reply, leg).map_err(|_| biomcp_fetch::bad_reply(Source::Mychem, None))?;
    card.value.card.retrieved = crate::drug_sources::received_date(received.fetched);
    Ok(card)
}

/// One exact native document, separating its small identity from complete native links.
#[derive(Debug)]
pub struct DrugDocument {
    /// Identity and native aliases, with no implicit product selection.
    pub card: DrugCard,
    /// All checked native cross-references, products and hierarchy links in stable order.
    pub xrefs: Box<[biodata::DrugIdentityRelation]>,
}

/// Publisher components named by the request projection, before disabled fields are removed.
pub fn requested_components(
    search: bool,
) -> impl Iterator<Item = &'static crate::registry::Component> {
    let fields = if search { SEARCH_FIELDS } else { CARD_FIELDS };
    crate::drug_sources::components::all().filter(move |component| {
        fields
            .split(',')
            .any(|field| field.split('.').next() == Some(component.name))
    })
}

/// Complete source policy and command declaration.
pub(crate) mod decl;
