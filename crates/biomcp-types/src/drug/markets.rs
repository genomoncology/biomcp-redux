//! One drug name looked up in each market's own source: one row per market, each from its own
//! record, with no record or date joined across markets.
use super::{DrugSection, DrugSource};
use crate::{QueryInvalid, Reason, ResponseBody};
use biodata::{AuthorizationDate, Text};
use serde::{Deserialize, Serialize};

/// A drug name's registration row in each market BioMCP reads: US FDA (Drugs@FDA), the EU
/// centralised procedure (EMA) and WHO prequalification.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "Fields")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(with = "Fields"))]
pub struct DrugMarketsGet(Fields);
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
struct Fields {
    /// Complete brand or generic name, looked up in each market's source on its own.
    #[cfg_attr(feature = "schema", schemars(extend("x-positional" = 0)))]
    input: Text,
    /// The markets section, alone; it takes no source.
    #[cfg_attr(feature = "schema", schemars(extend("x-positional" = 1)))]
    section: Box<[DrugSection]>,
}
crate::query::validation::checked_query!(DrugMarketsGet, Fields);
impl TryFrom<Fields> for DrugMarketsGet {
    type Error = QueryInvalid;
    fn try_from(fields: Fields) -> Result<Self, Self::Error> {
        if fields.section.as_ref() != [DrugSection::Markets] {
            return Err(QueryInvalid::new(&["section"]));
        }
        Ok(Self(fields))
    }
}
impl DrugMarketsGet {
    /// The name every market looks up.
    pub fn input(&self) -> &Text {
        &self.0.input
    }
    /// Look up one name in every market.
    pub fn new(input: Text) -> Self {
        Self(Fields {
            input,
            section: vec![DrugSection::Markets].into(),
        })
    }
}

/// One row per market, always in the order drugsfda, ema, who-pq.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct DrugMarkets {
    /// Each market's row from its own source; a market without a record says so.
    pub markets: Box<[DrugMarket]>,
}

/// One market's record for the name, as its own source states it.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct DrugMarket {
    /// The market's source: drugsfda (US FDA), ema (EU centralised procedure) or who-pq
    /// (WHO prequalification).
    pub source: DrugSource,
    /// Whether this source has a record for the name.
    pub state: DrugMarketState,
    /// Drugs@FDA application number, EMA product number or WHO reference number.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub record: Option<Text>,
    /// The record's own name: Drugs@FDA brand shared by every product, EMA medicine name, or
    /// WHO's INN, dosage form and strength cell.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<Text>,
    /// Drugs@FDA sponsor, EMA marketing authorisation holder, or WHO applicant.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub holder: Option<Text>,
    /// The source's own status words: each distinct Drugs@FDA product marketing status, or the EMA
    /// medicine status. WHO's list has no status column; every row on it is a prequalified product.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub status: Box<[Text]>,
    /// The record's first approval with the source's own date role: Drugs@FDA status date of the
    /// earliest approved original submission, EMA marketing_authorisation_date, or WHO Date of
    /// Prequalification. Dates are never compared or merged across markets.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_approval: Option<AuthorizationDate>,
    /// The record's public page at the source.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(extend("format" = "uri")))]
    pub url: Option<Text>,
    /// How many records in this source matched the name; this row is one of them.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matches: Option<u64>,
    /// How this record was chosen among the matches.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub choice: Option<Box<str>>,
    /// Why a failed source gave no answer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<Reason>,
}

biodata::wire_enum! {
    /// Whether a market's source has a record for the name.
    DrugMarketState {
        /// The source has a record; the row carries it.
        Found => "found",
        /// The source answered and has no record for the name.
        NoRecord => "no-record",
        /// The source could not be read, so the row says nothing about the market.
        Failed => "failed"
    }
}

impl ResponseBody for DrugMarkets {
    const SCHEMA: &'static str = "biomcp.drug-markets/1";
    type Query = DrugMarketsGet;
}
