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
mod pages;
mod raw;
mod receive;
use biomcp_fetch::{Arg, BadReply, Freshness, Leg, PathSegment, RequestError};
pub use pages::{Indications, Mechanisms, Molecules};
pub use map::{Molecule, Mechanism, Indication};
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
    };
    leg.json_received_checked_admission(
        &MOLECULES,
        &[
            (key, Arg::Text(text)),
            ("limit", Arg::Num(limit.into())),
            ("offset", Arg::Num(offset.into())),
        ],
        |r: &Molecules| page(&r.window, limit, offset),
        |r| r.window.identities_admitted(),
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
        |r: &raw::Molecule| {
            if r.molecule_chembl_id == id {
                Ok(())
            } else {
                Err(BadReply)
            }
        },
    ).map(|(row, meta)| (map::Molecule(row), meta))
}
/// Read native mechanisms without resolving a target record by name or substituting a drug ID.
pub fn mechanisms(
    leg: &Leg<'_>,
    id: &biodata::DrugId,
    limit: u32,
    offset: u32,
) -> Result<(Mechanisms, biomcp_fetch::ReplyMetadata), RequestError> {
    let id = id.to_string();
    leg.json_received_checked_admission(
        &MECHANISMS,
        &[
            ("molecule_chembl_id", Arg::Text(&id)),
            ("limit", Arg::Num(limit.into())),
            ("offset", Arg::Num(offset.into())),
        ],
        |r: &Mechanisms| {
            page(&r.window, limit, offset)?;
            if r.molecules.iter().all(|molecule| molecule == &id) {
                Ok(())
            } else {
                Err(BadReply)
            }
        },
        Mechanisms::identities_admitted,
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
    leg.json_received_checked_admission(
        &INDICATIONS,
        &[
            ("molecule_chembl_id", Arg::Text(&id)),
            ("limit", Arg::Num(limit.into())),
            ("offset", Arg::Num(offset.into())),
        ],
        |r: &Indications| {
            page(&r.window, limit, offset)?;
            if r.molecules.iter().all(|molecule| molecule == &id) {
                Ok(())
            } else {
                Err(BadReply)
            }
        },
        |r| r.window.identities_admitted(),
    )
}
fn page<T>(window: &crate::NativeWindow<T>, limit: u32, offset: u32) -> Result<(), BadReply> {
    if (1..=50).contains(&limit)
        && window.limit() == limit
        && window.offset() == offset
        && window.received() <= limit
        && (window.provider_empty()
            || window
                .total()
                .is_some_and(|total| u64::from(offset) + u64::from(window.received()) <= total))
    {
        Ok(())
    } else {
        Err(BadReply)
    }
}
mod map;
mod molecule;
mod scope;
pub use map::{indications as map_indications, targets as map_targets};
pub use map::{molecule_card, molecule_header};

mod cell;
pub use cell::{CellPage, assay_count, cells, release};
