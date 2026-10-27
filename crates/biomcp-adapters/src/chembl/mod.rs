//! Canonical CHEMBL source declaration.
use biomcp_fetch::{KeyUse, SourceSpec, Terms};
use biomcp_types::Source;
use std::time::Duration;
/// Publisher policy shared by health, request admission and response outcomes.
pub static SPEC: SourceSpec = SourceSpec {
    destinations: biomcp_fetch::Destinations::Primary,
    user_agent: None,
    source: Source::Chembl,
    base: "https://www.ebi.ac.uk/chembl/api/data",
    terms: Terms {
        name: crate::drug_sources::CHEMBL_TERMS.name,
        url: crate::drug_sources::CHEMBL_TERMS.url,
        usage: crate::drug_sources::CHEMBL_TERMS.usage,
    },
    quota: &crate::quotas::CHEMBL,
    key: KeyUse::None,
    deadline: Duration::from_secs(15),
};
mod raw;
use biomcp_fetch::{Arg, BadReply, Freshness, Leg, PathSegment, RequestError};
pub use raw::{
    CrossReference, Hierarchy, Indication, Indications, Mechanism, Mechanisms, Molecule, Molecules,
    PageMeta, Reference, Synonym,
};
const MOLECULES: biomcp_fetch::Endpoint = SPEC.get("/molecule.json").fresh(Freshness::Days(1));
const MOLECULE: biomcp_fetch::Endpoint = SPEC.get("/molecule").fresh(Freshness::Days(1));
const MECHANISMS: biomcp_fetch::Endpoint = SPEC.get("/mechanism.json").fresh(Freshness::Days(1));
const INDICATIONS: biomcp_fetch::Endpoint =
    SPEC.get("/drug_indication.json").fresh(Freshness::Days(1));
pub use biomcp_types::DrugNameSelector as NameSelector;
/// Read a candidate page with an unchanged literal selector.
pub fn search(
    leg: &Leg<'_>,
    text: &str,
    selector: NameSelector,
    limit: u32,
    offset: u32,
) -> Result<(Molecules, biomcp_fetch::ReplyMetadata), RequestError> {
    let key = match selector {
        NameSelector::Preferred => "pref_name__iexact",
        NameSelector::Synonym => "molecule_synonyms__molecule_synonym__iexact",
        _ => {
            return Err(RequestError::Source(biomcp_fetch::SourceError {
                source: Source::Chembl,
                reason: biomcp_types::Reason::BadReply,
                status: None,
                retry_after: None,
            }));
        }
    };
    leg.json_received_checked(
        &MOLECULES,
        &[
            (key, Arg::Text(text)),
            ("limit", Arg::Num(limit.into())),
            ("offset", Arg::Num(offset.into())),
        ],
        |r: &Molecules| page(&r.page_meta, r.molecules.len(), limit, offset),
    )
}
/// Read exactly the requested molecule and refuse a mismatched source identity before cache admission.
pub fn molecule(
    leg: &Leg<'_>,
    id: &biodata::DrugId,
) -> Result<(Molecule, biomcp_fetch::ReplyMetadata), RequestError> {
    let id = id.to_string();
    let path = PathSegment::new(&format!("{id}.json"))?;
    leg.json_received_checked(
        &MOLECULE.clone().with_segments(&[path]),
        &[],
        |r: &Molecule| {
            if r.molecule_chembl_id == id {
                Ok(())
            } else {
                Err(BadReply)
            }
        },
    )
}
/// Read native mechanisms without resolving a target record by name or substituting a drug ID.
pub fn mechanisms(
    leg: &Leg<'_>,
    id: &biodata::DrugId,
    limit: u32,
    offset: u32,
) -> Result<(Mechanisms, biomcp_fetch::ReplyMetadata), RequestError> {
    let id = id.to_string();
    leg.json_received_checked(
        &MECHANISMS,
        &[
            ("molecule_chembl_id", Arg::Text(&id)),
            ("limit", Arg::Num(limit.into())),
            ("offset", Arg::Num(offset.into())),
        ],
        |r: &Mechanisms| {
            page(&r.page_meta, r.mechanisms.len(), limit, offset)?;
            if r.mechanisms.iter().all(|row| row.molecule_chembl_id == id) {
                Ok(())
            } else {
                Err(BadReply)
            }
        },
    )
}
/// Read native indication rows and clinical phases without interpreting them as authorization.
pub fn indications(
    leg: &Leg<'_>,
    id: &biodata::DrugId,
    limit: u32,
    offset: u32,
) -> Result<(Indications, biomcp_fetch::ReplyMetadata), RequestError> {
    let id = id.to_string();
    leg.json_received_checked(
        &INDICATIONS,
        &[
            ("molecule_chembl_id", Arg::Text(&id)),
            ("limit", Arg::Num(limit.into())),
            ("offset", Arg::Num(offset.into())),
        ],
        |r: &Indications| {
            page(&r.page_meta, r.drug_indications.len(), limit, offset)?;
            if r.drug_indications
                .iter()
                .all(|row| row.molecule_chembl_id == id)
            {
                Ok(())
            } else {
                Err(BadReply)
            }
        },
    )
}
fn page(meta: &PageMeta, count: usize, limit: u32, offset: u32) -> Result<(), BadReply> {
    if (1..=50).contains(&limit)
        && meta.limit == limit
        && meta.offset == offset
        && count <= limit as usize
        && (count == 0 || u64::from(offset) + count as u64 <= meta.total_count)
    {
        Ok(())
    } else {
        Err(BadReply)
    }
}
mod map;
pub use map::{indications as map_indications, targets as map_targets};

mod cell;
pub use cell::{assay_count, cells, release};
