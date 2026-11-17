//! STRING's interaction_partners reply and its checked "not found" form.
use biomcp_fetch::{BadReply, Reply};
use serde::Deserialize;

pub(super) const HUMAN: u32 = 9606;

#[derive(Deserialize)]
#[serde(untagged)]
pub(super) enum Partners {
    Rows(Vec<Row>),
    Missing([Missing; 1]),
}
#[derive(Deserialize)]
pub(super) struct Missing {
    #[serde(rename = "Error")]
    error: String,
    #[serde(rename = "ErrorMessage")]
    _message: String,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Row {
    #[serde(rename = "stringId_A")]
    pub string_id_a: String,
    #[serde(rename = "stringId_B")]
    pub string_id_b: String,
    #[serde(rename = "preferredName_B")]
    pub preferred_name_b: String,
    ncbi_taxon_id: u32,
    pub score: f64,
    pub nscore: f64,
    pub fscore: f64,
    pub pscore: f64,
    pub ascore: f64,
    pub escore: f64,
    pub dscore: f64,
    pub tscore: f64,
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
            && self.score * 1000.0 >= f64::from(super::REQUIRED_SCORE) - 0.5
    }
}
impl Reply for Partners {
    fn check(&self) -> Result<(), BadReply> {
        match self {
            Self::Rows(rows) => {
                let first = rows.first().map(|r| r.string_id_a.as_str());
                let ordered = rows
                    .iter()
                    .zip(rows.iter().skip(1))
                    .all(|(a, b)| a.score >= b.score);
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
