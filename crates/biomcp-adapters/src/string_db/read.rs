//! Read every STRING partner of one protein at medium confidence or above.
use super::raw::{HUMAN, Partners};
use biodata::{Text, UniProtIsoform};
use biomcp_fetch::{Arg, Freshness, Input, KeyUse, Leg, Read, RequestError, SourceSpec, Terms};
use biomcp_types::{InteractionEvidence, ProteinInteraction, ProteinInteractions, Source, Use};
use std::time::Duration;
/// STRING's versioned address, so replies stay stable until BioMCP moves to a new release.
pub static SPEC: SourceSpec = SourceSpec {
    destinations: biomcp_fetch::Destinations::Primary,
    user_agent: None,
    source: Source::StringDb,
    base: "https://version-12-5.string-db.org",
    terms: Terms {
        name: "CC BY 4.0",
        url: "https://string-db.org/cgi/access",
        usage: Use::Attribution,
    },
    key: KeyUse::None,
    quota: &crate::quotas::STRING,
    deadline: Duration::from_secs(10),
    command_budget: None,
};
/// The STRING release served at `SPEC.base`.
pub const VERSION: &str = "12.5";
/// STRING's medium-confidence threshold, its own default.
pub const REQUIRED_SCORE: u32 = 400;
pub(crate) const PARTNERS: &str = "/api/json/interaction_partners";
/// STRING asks API callers to name their tool.
pub(crate) const CALLER: &str = "biomcp";

fn channel(v: f64) -> Option<f64> {
    (v > 0.0).then_some(v)
}
fn checked(s: &str) -> Result<Text, RequestError> {
    crate::mapped::text(s).map_err(|_| biomcp_fetch::bad_reply(Source::StringDb, Some(200)))
}
/// Every partner STRING lists at medium confidence or above; `None` when STRING does not know the protein.
///
/// STRING keeps one protein per gene, so an isoform asks for its canonical accession.
pub fn partners(
    leg: &Leg<'_>,
    accession: &UniProtIsoform,
) -> Result<Option<ProteinInteractions>, RequestError> {
    let canonical = accession
        .as_str()
        .split_once('-')
        .map_or(accession.as_str(), |(base, _)| base);
    let endpoint = SPEC.get(PARTNERS).fresh(Freshness::Days(7));
    let args = [
        ("identifiers", Arg::Text(canonical)),
        ("species", Arg::Num(u64::from(HUMAN))),
        ("required_score", Arg::Num(u64::from(REQUIRED_SCORE))),
        // limit=0 returned every partner when checked live on 2026-11-17 (WRN 313, TP53 2,396);
        // STRING's API documentation does not state it.
        ("limit", Arg::Num(0)),
        ("caller_identity", Arg::Text(CALLER)),
    ];
    let reply = leg
        .call(&endpoint, Input::Params(&args), Read::<Partners>::json())?
        .value;
    let rows = match reply {
        Partners::Rows(rows) => rows,
        Partners::Missing(_) => return Ok(None),
    };
    let Some(first) = rows.first() else {
        return Ok(None);
    };
    let string_id = checked(&first.string_id_a)?;
    let items = rows
        .iter()
        .map(|r| {
            Ok(ProteinInteraction {
                name: checked(&r.preferred_name_b)?,
                string_id: checked(&r.string_id_b)?,
                score: r.score,
                evidence: InteractionEvidence {
                    experiments: channel(r.escore),
                    databases: channel(r.dscore),
                    textmining: channel(r.tscore),
                    coexpression: channel(r.ascore),
                    neighborhood: channel(r.nscore),
                    fusion: channel(r.fscore),
                    cooccurrence: channel(r.pscore),
                },
            })
        })
        .collect::<Result<Box<[_]>, RequestError>>()?;
    Ok(Some(ProteinInteractions {
        total: items.len() as u64,
        min_score: f64::from(REQUIRED_SCORE) / 1000.0,
        version: checked(VERSION)?,
        string_id,
        items,
    }))
}
