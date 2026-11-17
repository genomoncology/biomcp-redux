//! STRING functional association partners of one human protein, in STRING's score order.
use biodata::{Text, UniProtIsoform};
use biomcp_fetch::{
    Arg, BadReply, Freshness, Input, KeyUse, Leg, Read, Reply, RequestError, SourceSpec, Terms,
};
use biomcp_types::{InteractionEvidence, ProteinInteraction, ProteinInteractions, Source, Use};
use serde::Deserialize;
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
const HUMAN: u32 = 9606;

#[derive(Deserialize)]
#[serde(untagged)]
enum Partners {
    Rows(Vec<Row>),
    Missing([Missing; 1]),
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Missing {
    #[serde(rename = "Error")]
    error: String,
    #[serde(rename = "ErrorMessage")]
    _message: String,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Row {
    #[serde(rename = "stringId_A")]
    string_id_a: String,
    #[serde(rename = "stringId_B")]
    string_id_b: String,
    #[serde(rename = "preferredName_B")]
    preferred_name_b: String,
    ncbi_taxon_id: u32,
    score: f64,
    nscore: f64,
    fscore: f64,
    pscore: f64,
    ascore: f64,
    escore: f64,
    dscore: f64,
    tscore: f64,
}
fn unit(v: f64) -> bool {
    v.is_finite() && (0.0..=1.0).contains(&v)
}
impl Row {
    fn valid(&self) -> bool {
        self.ncbi_taxon_id == HUMAN
            && self.string_id_a.starts_with("9606.")
            && self.string_id_b.starts_with("9606.")
            && self.string_id_a != self.string_id_b
            && !self.preferred_name_b.trim().is_empty()
            && [
                self.score,
                self.nscore,
                self.fscore,
                self.pscore,
                self.ascore,
                self.escore,
                self.dscore,
                self.tscore,
            ]
            .into_iter()
            .all(unit)
            && self.score * 1000.0 >= f64::from(REQUIRED_SCORE) - 0.5
    }
}
impl Reply for Partners {
    fn check(&self) -> Result<(), BadReply> {
        match self {
            Self::Rows(rows) => {
                let first = rows.first().map(|r| r.string_id_a.as_str());
                let ordered = rows.windows(2).all(|w| w[0].score >= w[1].score);
                let mut partners = std::collections::BTreeSet::new();
                (rows
                    .iter()
                    .all(|r| r.valid() && Some(r.string_id_a.as_str()) == first)
                    && rows.iter().all(|r| partners.insert(r.string_id_b.as_str()))
                    && ordered)
                    .then_some(())
                    .ok_or(BadReply)
            }
            Self::Missing([m]) if m.error == "not found" => Ok(()),
            Self::Missing(_) => Err(BadReply),
        }
    }
    fn check_status(&self, status: u16) -> Result<(), BadReply> {
        match (self, status) {
            (Self::Rows(_), 200) | (Self::Missing(_), 404) => self.check(),
            _ => Err(BadReply),
        }
    }
}
fn channel(v: f64) -> Option<f64> {
    (v > 0.0).then_some(v)
}
fn text(s: &str) -> Result<Text, RequestError> {
    Text::new(s).map_err(|_| biomcp_fetch::bad_reply(Source::StringDb, Some(200)))
}
/// Every partner STRING lists at medium confidence or above; `None` when STRING does not know the protein.
///
/// STRING scores one protein per gene, so an isoform asks for its canonical accession.
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
        // STRING returns every partner when the limit is zero.
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
    let string_id = text(&first.string_id_a)?;
    let items = rows
        .iter()
        .map(|r| {
            Ok(ProteinInteraction {
                name: text(&r.preferred_name_b)?,
                string_id: text(&r.string_id_b)?,
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
        version: text(VERSION)?,
        string_id,
        items,
    }))
}

/// Complete source policy and command declaration.
pub(crate) mod decl;
