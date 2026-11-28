//! Interpret native inactive-entry envelopes for protein cards.
use super::{Entry, endpoint, map, raw};
use crate::Mapped;
use biodata::UniProtIsoform;
use biomcp_fetch::{Absent, BadReply, Input, Leg, Read, Reply, RequestError};
use biomcp_types::Source;
use serde::Deserialize;

/// A provider-confirmed reason that an accession cannot have a protein card.
#[derive(Clone, Copy, Debug)]
pub enum Scope {
    /// The entry describes an organism other than human.
    NonHuman,
    /// UniProt has not reviewed the entry.
    Unreviewed,
    /// UniProt has deleted the entry.
    Deleted,
}

/// The result of looking up a protein card by accession.
#[derive(Debug)]
pub enum Lookup {
    /// A checked card, or a legitimate absence.
    Card(Box<Mapped<Option<Entry>>>),
    /// A known accession outside the reviewed human collection.
    Outside(Scope),
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum Native {
    Inactive(Inactive),
    Active(Box<raw::Entry>),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Inactive {
    #[serde(rename = "entryType")]
    _entry_type: InactiveEntry,
    primary_accession: UniProtIsoform,
    inactive_reason: InactiveReason,
}

#[derive(Debug, Deserialize)]
enum InactiveEntry {
    Inactive,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "inactiveReasonType", rename_all = "UPPERCASE")]
enum InactiveReason {
    Deleted,
    Merged {
        #[serde(rename = "mergeDemergeTo")]
        targets: Vec<UniProtIsoform>,
    },
    #[serde(other)]
    Other,
}

impl Reply for Native {
    fn check(&self) -> Result<(), BadReply> {
        Ok(())
    }

    fn check_status(&self, status: u16) -> Result<(), BadReply> {
        match (status, self) {
            (200, Self::Active(_) | Self::Inactive(_))
            | (
                303,
                Self::Inactive(Inactive {
                    inactive_reason: InactiveReason::Merged { .. },
                    ..
                }),
            ) => Ok(()),
            _ => Err(BadReply),
        }
    }
}

fn read(leg: &Leg<'_>, id: &UniProtIsoform) -> Result<Option<Native>, RequestError> {
    leg.call(
        &endpoint(id)?.json_see_other(),
        Input::Params(&[]),
        Read::<Native>::json()
            .cache_if(&|native| matches!(native, Native::Active(raw) if raw.admitted(id))),
    )
    .absent()
    .map(|received| received.map(|received| received.value))
}

/// Read a card, following a unique native merge target by checked accession.
pub fn lookup(leg: &Leg<'_>, id: &UniProtIsoform) -> Result<Lookup, RequestError> {
    let mut native = read(leg, id)?;
    let mut requested = id.clone();
    if let Some(Native::Inactive(Inactive {
        primary_accession,
        inactive_reason: InactiveReason::Merged { targets },
        ..
    })) = &native
        && primary_accession == id
        && let [target] = targets.as_slice()
        && target != id
        && !id.as_str().contains('-')
        && !target.as_str().contains('-')
    {
        requested = target.clone();
        native = read(leg, &requested)?;
    }

    match native {
        Some(Native::Active(raw)) => {
            if raw.primary_accession == requested {
                if raw.organism.taxon_id != super::super::HUMAN {
                    return Ok(Lookup::Outside(Scope::NonHuman));
                }
                if raw.entry_type.as_str() == super::super::UNREVIEWED {
                    return Ok(Lookup::Outside(Scope::Unreviewed));
                }
            }
            Ok(Lookup::Card(Box::new(map::entry(*raw, &requested))))
        }
        Some(Native::Inactive(Inactive {
            primary_accession,
            inactive_reason: InactiveReason::Deleted,
            ..
        })) if primary_accession == requested => Ok(Lookup::Outside(Scope::Deleted)),
        None if requested == *id => Ok(Lookup::Card(Box::new(Mapped {
            refusals: crate::Refusal::None,
            value: None,
            problems: Vec::new(),
        }))),
        _ => Err(biomcp_fetch::bad_reply(Source::UniProt, None)),
    }
}
