//! The list BioMCP ships: what each communication's page linked when the seed was made, so a
//! first search answers at once. `scripts/fda_dsc_seed.py` remakes it with this source's own
//! reader (`get drug ID --source fda-dsc` for each row of FDA's index), 30 seconds apart.
use biodata::PartialDate;
use biomcp_types::DrugSafetyCommunication;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(super) struct Seed {
    /// The day the seed was read from FDA.
    pub date: PartialDate,
    /// Each communication's row, its page read.
    pub items: Vec<DrugSafetyCommunication>,
}

/// The shipped seed, parsed on each use: about 50 rows.
pub(super) fn seed() -> Result<Seed, serde_json::Error> {
    serde_json::from_str(include_str!("seed.json"))
}
