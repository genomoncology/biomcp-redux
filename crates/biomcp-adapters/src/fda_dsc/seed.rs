//! FDA communication records and available page text for searches without a full crawl.
//! `scripts/fda_dsc_seed.py` rebuilds the seed with the FDA adapter, 30 seconds between reads.
use biodata::PartialDate;
use biomcp_types::DrugSafetyCommunication;
use serde::Deserialize;

/// The search record and the page text available to match, held outside the reply.
#[derive(Debug, Clone)]
pub struct SearchRow {
    /// FDA's index and page facts.
    pub header: DrugSafetyCommunication,
    /// The page's visible text, when it was collected.
    pub text: Option<String>,
}

#[derive(Deserialize)]
struct SeedRow {
    #[serde(flatten)]
    header: DrugSafetyCommunication,
    #[serde(default)]
    text: Vec<String>,
}

#[derive(Debug)]
pub(super) struct Seed {
    /// The day the seed was read from FDA.
    pub date: PartialDate,
    /// Each communication's row, its page read.
    pub items: Vec<SearchRow>,
}

/// The shipped seed, parsed on each use: about 50 rows.
pub(super) fn seed() -> Result<Seed, serde_json::Error> {
    #[derive(Deserialize)]
    struct StoredSeed {
        date: PartialDate,
        items: Vec<SeedRow>,
    }
    let StoredSeed { date, items } = serde_json::from_str(include_str!("seed.json"))?;
    let items = items
        .into_iter()
        .map(|row| SearchRow {
            header: row.header,
            text: (!row.text.is_empty()).then(|| row.text.join(" ")),
        })
        .collect();
    Ok(Seed { date, items })
}
