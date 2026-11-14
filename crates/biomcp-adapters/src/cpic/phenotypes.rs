//! CPIC's phenotype text for one gene: the exact filter and the first page's phenotype list.
use super::SPEC;
use biomcp_fetch::{Arg, BadReply, Freshness, Input, Leg, Reply, RequestError};
use biomcp_types::PgxGet;
use serde::Deserialize;
/// Most recommendation rows CPIC holds for one gene and drug is 206 (2026-11-14); a larger
/// table is read as no phenotype list rather than a partial one.
const PHENOTYPE_ROWS: u32 = 1000;
#[derive(Deserialize)]
struct PhenotypeRow {
    phenotype: Option<Box<str>>,
}
#[derive(Deserialize)]
#[serde(transparent)]
struct PhenotypeRows(Vec<PhenotypeRow>);
impl Reply for PhenotypeRows {
    fn check(&self) -> Result<(), BadReply> {
        Ok(())
    }
}
/// Every recommendation row for the gene and drug, ignoring any phenotype filter.
#[derive(Debug)]
pub struct PhenotypeList {
    /// Each row's phenotype for the gene in native row order; rows without one are skipped.
    pub phenotypes: Vec<Box<str>>,
    /// All rows, including those without a phenotype for the gene (HLA rows carry none).
    pub rows: u64,
}
/// Read every row's phenotype for the gene and drug in one complete request.
pub fn drug_phenotypes(leg: &Leg<'_>, query: &PgxGet) -> Result<PhenotypeList, RequestError> {
    let id = query
        .drug_id()
        .ok_or(biomcp_fetch::InputError::Serialization)?;
    let drug = format!("eq.{id}");
    let key = format!("lookupkey->>{}", query.gene());
    let select = format!("phenotype:phenotypes->>{}", query.gene());
    let endpoint = SPEC.get("/recommendation").fresh(Freshness::Days(1));
    let rows = leg
        .call(
            &endpoint,
            Input::Params(&[
                ("select", Arg::Text(&select)),
                (&key, Arg::Text("not.is.null")),
                ("drugid", Arg::Text(&drug)),
                ("order", Arg::Text("id.asc")),
                ("limit", Arg::Num(PHENOTYPE_ROWS.into())),
                ("offset", Arg::Num(0)),
            ]),
            biomcp_fetch::Read::<PhenotypeRows>::json().counted(&|rows, range, status| {
                range.check(0, PHENOTYPE_ROWS, rows.0.len(), status)?;
                if range.total != rows.0.len() as u64 {
                    return Err(BadReply);
                }
                Ok(())
            }),
        )?
        .value;
    Ok(PhenotypeList {
        rows: rows.0.len() as u64,
        phenotypes: rows.0.into_iter().filter_map(|r| r.phenotype).collect(),
    })
}
pub(super) fn filter(query: &PgxGet) -> Option<(String, String)> {
    query.phenotype().map(|p| {
        (
            format!("phenotypes->>{}", query.gene()),
            format!("eq.{}", p.as_str()),
        )
    })
}
pub(super) fn matches(
    query: &PgxGet,
    phenotypes: Option<&std::collections::BTreeMap<biodata::PgxGeneKey, Box<str>>>,
) -> bool {
    let Some(wanted) = query.phenotype() else {
        return true;
    };
    phenotypes.is_some_and(|map| {
        map.iter().any(|(k, v)| {
            k.to_string() == query.gene().to_string() && v.as_ref() == wanted.as_str()
        })
    })
}
