use crate::raw::{Field, OneOrMany};
use biodata::{FiniteNumber, Text};
use biomcp_fetch::{BadReply, Reply};
use serde::Deserialize;
pub(super) type Numbers = Option<Field<OneOrMany<FiniteNumber>>>;
pub(super) type Texts = Option<Field<OneOrMany<Text>>>;
#[derive(Deserialize)]
pub(super) struct Record {
    #[serde(rename = "_id")]
    pub id: Text,
    pub cadd: Option<Cadd>,
    pub dbnsfp: Option<Dbnsfp>,
}
impl Reply for Record {
    fn check(&self) -> Result<(), BadReply> {
        if self
            .id
            .as_str()
            .parse::<biomcp_types::VariantInput>()
            .is_ok()
        {
            Ok(())
        } else {
            Err(BadReply)
        }
    }
}
#[derive(Deserialize)]
pub(super) struct Cadd {
    pub phred: Numbers,
    pub rawscore: Numbers,
    pub sift: Option<Sift>,
}
#[derive(Deserialize)]
pub(super) struct Sift {
    pub val: Numbers,
    pub cat: Texts,
}
#[derive(Deserialize)]
pub(super) struct Dbnsfp {
    pub hgvsp: Texts,
    pub ensembl: Option<Ensembl>,
    pub polyphen2: Option<Polyphen>,
    pub revel: Option<Score>,
    pub mutationtaster: Option<Score>,
    #[serde(rename = "gerp++")]
    pub gerp: Option<Gerp>,
    pub phastcons: Option<Groups>,
    pub phylop: Option<Groups>,
    pub siphy_29way: Option<Siphy>,
}
#[derive(Deserialize)]
pub(super) struct Ensembl {
    pub transcriptid: Texts,
}
#[derive(Deserialize)]
pub(super) struct Polyphen {
    pub hdiv: Option<Score>,
    pub hvar: Option<Score>,
}
#[derive(Deserialize)]
pub(super) struct Score {
    pub score: Numbers,
    pub rankscore: Numbers,
    pub pred: Texts,
}
#[derive(Deserialize)]
pub(super) struct Gerp {
    pub nr: Numbers,
    pub rs: Numbers,
    pub rs_rankscore: Numbers,
}
#[derive(Deserialize)]
pub(super) struct Groups {
    #[serde(rename = "100way_vertebrate")]
    pub vertebrate: Option<Score>,
    #[serde(rename = "17way_primate")]
    pub primate: Option<Score>,
    #[serde(rename = "470way_mammalian")]
    pub mammalian: Option<Score>,
}
#[derive(Deserialize)]
pub(super) struct Siphy {
    pub logodds_score: Numbers,
    pub logodds_rankscore: Numbers,
    pub pi: Option<Pi>,
}
#[derive(Deserialize)]
pub(super) struct Pi {
    pub a: Numbers,
    pub c: Numbers,
    pub g: Numbers,
    pub t: Numbers,
}
