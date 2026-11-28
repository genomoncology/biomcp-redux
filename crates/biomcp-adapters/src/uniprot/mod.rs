//! Reviewed human UniProt entries and searches.
use biomcp_fetch::{Input, Read};
mod entry;
mod map;
mod raw;
mod search;
mod sequence;
use crate::{Mapped, quotas};
use biodata::UniProtAcc;
use biomcp_fetch::{Arg, Endpoint, Freshness, KeyUse, Leg, RequestError, SourceSpec};
use biomcp_types::{Protein, Source};
pub use entry::{Entry, Lookup, Scope, entry, lookup};
pub use search::{SearchPage, gene_candidates, search};
pub use sequence::sequence;
use std::time::Duration;
/// Source policy for UniProt's accession endpoint.
pub static SPEC: SourceSpec = SourceSpec {
    destinations: biomcp_fetch::Destinations::Primary,
    user_agent: None,
    terms: biomcp_fetch::Terms {
        name: "CC BY 4.0",
        url: "https://www.uniprot.org/help/license",
        usage: biomcp_types::Use::Attribution,
    },
    source: Source::UniProt,
    base: "https://rest.uniprot.org",
    key: KeyUse::None,
    quota: &quotas::UNIPROT,
    deadline: Duration::from_secs(15),
    command_budget: None,
};
/// A normal read ends in under three seconds; a stalled one can pass the deadline while a fresh request, sent beside it, returns.
const STALL_LIMIT: Duration = Duration::from_secs(3);
const PROTEINS: Endpoint = SPEC
    .get("/uniprotkb/accessions")
    .identity_encoding()
    .stall_retry(STALL_LIMIT)
    .fresh(Freshness::Days(7));
const FIELDS: &str = "accession,protein_name,length,cc_function,cc_subcellular_location,reviewed,gene_primary,organism_id";
/// Fetch distinct accessions once, correlating checked replies by identity.
pub fn proteins(leg: &Leg<'_>, ids: &[UniProtAcc]) -> Result<Mapped<Vec<Protein>>, RequestError> {
    if ids.is_empty() {
        return Ok(Mapped {
            value: Vec::new(),
            problems: Vec::new(),
            refusals: crate::Refusal::None,
        });
    }
    let mut ids = ids.to_vec();
    ids.sort_unstable();
    ids.dedup();
    let text: Vec<Box<str>> = ids.iter().map(|id| id.to_string().into()).collect();
    let raw = leg
        .call(
            &PROTEINS,
            Input::Params(&[
                ("accessions", Arg::Ids(&text)),
                ("fields", Arg::Text(FIELDS)),
                ("format", Arg::Text("json")),
            ]),
            Read::<raw::Proteins>::json(),
        )?
        .value;
    Ok(map::proteins(raw, &ids))
}

const HUMAN: u32 = 9606;
const REVIEWED: &str = "UniProtKB reviewed (Swiss-Prot)";
const UNREVIEWED: &str = "UniProtKB unreviewed (TrEMBL)";

fn human_reviewed(taxon: u32, entry_type: &str) -> bool {
    taxon == HUMAN && entry_type == REVIEWED
}

/// Complete source policy and command declaration.
pub(crate) mod decl;
