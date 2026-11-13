//! One exact CPIC gene and drug pair, read only to say why that drug's recommendations are empty.
use super::read::{Page, read};
use biodata::PgxPair;
use biomcp_fetch::{Arg, Leg, RequestError};
use biomcp_types::PgxGet;
/// The pair row for the query's gene and drug, at most one.
pub fn pair(leg: &Leg<'_>, query: &PgxGet) -> Result<Page<PgxPair>, RequestError> {
    let id = query
        .drug_id()
        .ok_or(biomcp_fetch::InputError::Serialization)?;
    let gene = format!("eq.{}", query.gene());
    let drug = format!("eq.{id}");
    read::<super::raw::PgxPair>(
        leg,
        "/pair_view",
        &[
            ("select", Arg::Text("*")),
            ("genesymbol", Arg::Text(&gene)),
            ("drugid", Arg::Text(&drug)),
            ("order", Arg::Text("pairid.asc")),
            ("limit", Arg::Num(1)),
            ("offset", Arg::Num(0)),
        ],
        0,
        1,
        false,
        |rows: &[PgxPair], _range| {
            rows.iter()
                .all(|r| &r.genesymbol == query.gene() && r.drugid.as_ref() == Some(id))
        },
    )
}
