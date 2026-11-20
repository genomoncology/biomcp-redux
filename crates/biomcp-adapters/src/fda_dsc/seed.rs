//! The list BioMCP ships: what each communication's page linked when the seed was made, so a
//! first search answers at once. `scripts/fda_dsc_seed.py` remakes it with this source's own
//! reader (`get drug ID --source fda-dsc` for each row of FDA's index), 30 seconds apart.
use biodata::PartialDate;
use biomcp_types::DrugSafetyCommunication;
use serde::Deserialize;
use std::sync::OnceLock;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Seed {
    /// The day the seed was read from FDA.
    pub date: PartialDate,
    /// Each communication's row, its page read.
    pub items: Vec<DrugSafetyCommunication>,
}

/// The shipped seed; a seed that does not parse is empty, and a test keeps it parsing.
pub(super) fn seed() -> Option<&'static Seed> {
    static SEED: OnceLock<Option<Seed>> = OnceLock::new();
    SEED.get_or_init(|| serde_json::from_str(include_str!("seed.json")).ok())
        .as_ref()
}

#[cfg(test)]
mod tests {
    #[test]
    fn shipped_seed_parses_and_every_row_was_read() {
        let seed = super::seed();
        assert!(
            seed.is_some_and(|s| !s.items.is_empty() && s.items.iter().all(|row| row.page_read))
        );
    }
}
