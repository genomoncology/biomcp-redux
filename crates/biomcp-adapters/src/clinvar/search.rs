//! Native identity discovery follows NCBI's ClinVar E-utilities search contract.
//! https://www.ncbi.nlm.nih.gov/clinvar/docs/programmatic_access/
use super::SPEC;
use crate::Mapped;
use biodata::{ClinVarVariationId, Variant, VariantId};
use biomcp_fetch::{Arg, BadReply, Endpoint, Freshness, Leg, Reply, RequestError};
use biomcp_types::{Source, SourceRecord, VariantHeader, VariantSearch};
use serde::Deserialize;
const SEARCH: Endpoint = SPEC.get("/esearch.fcgi").fresh(Freshness::Days(1));
/// One native ClinVar candidate page, without inferred transcripts or alleles.
pub struct SearchPage {
    /// Native total before paging.
    pub total: u64,
    /// Source identities in their original order.
    pub rows: Mapped<Vec<VariantHeader>>,
}
#[derive(Deserialize)]
struct Search {
    esearchresult: Page,
}
#[derive(Deserialize)]
struct Page {
    count: String,
    retstart: String,
    idlist: Vec<Id>,
    #[serde(rename = "ERROR")]
    error: Option<serde::de::IgnoredAny>,
    errorlist: Option<serde::de::IgnoredAny>,
}
#[derive(Deserialize)]
#[serde(try_from = "String")]
struct Id(ClinVarVariationId);
impl TryFrom<String> for Id {
    type Error = biodata::Error;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        format!("clinvar:{value}").parse().map(Self)
    }
}
impl Reply for Search {
    fn check(&self) -> Result<(), BadReply> {
        let page = &self.esearchresult;
        let total = page.count.parse::<u64>().map_err(|_| BadReply)?;
        let offset = page.retstart.parse::<u32>().map_err(|_| BadReply)?;
        let ids: std::collections::BTreeSet<_> = page.idlist.iter().map(|id| id.0.get()).collect();
        if page.error.is_some()
            || page.errorlist.is_some()
            || page.idlist.len() > 50
            || ids.len() != page.idlist.len()
            || page.idlist.len() as u64 > total.saturating_sub(u64::from(offset))
        {
            return Err(BadReply);
        }
        Ok(())
    }
    fn discovery(&self) -> Option<biomcp_fetch::Identities> {
        biomcp_fetch::Identities::bounded(
            "clinvar",
            self.esearchresult
                .idlist
                .iter()
                .map(|id| id.0.to_string())
                .collect(),
        )
    }
}
/// Search the supplied gene and coding notation without choosing a reference sequence.
pub fn search(leg: &Leg<'_>, query: &VariantSearch) -> Result<SearchPage, RequestError> {
    let term = query.source_query();
    let args = [
        ("db", Arg::Text("clinvar")),
        ("term", Arg::Text(&term)),
        ("retmode", Arg::Text("json")),
        ("retmax", Arg::Num(u64::from(query.limit()))),
        ("retstart", Arg::Num(u64::from(query.offset()))),
        ("tool", Arg::Text("biomcp")),
    ];
    let (reply, _) = leg.json_received_checked::<Search>(&SEARCH, &args, |reply| {
        let page = &reply.esearchresult;
        let total = page.count.parse::<u64>().map_err(|_| BadReply)?;
        if page.retstart != query.offset().to_string()
            || page.idlist.len() as u64
                != total
                    .saturating_sub(u64::from(query.offset()))
                    .min(u64::from(query.limit()))
        {
            return Err(BadReply);
        }
        Ok(())
    })?;
    let page = reply.esearchresult;
    let rows = page
        .idlist
        .into_iter()
        .map(|id| {
            let id = VariantId::ClinVarVariation(id.0);
            let record = SourceRecord {
                id: crate::mapped::text(&id.to_string())?,
                url: None,
            };
            Ok(VariantHeader {
                identity: Variant {
                    ids: Box::new([id]),
                    representations: Box::default(),
                    genes: Box::default(),
                },
                requested: None,
                transcripts: Box::default(),
                source: Source::ClinVar,
                record,
                myvariant: None,
                sections: None,
                component_terms: Box::default(),
            })
        })
        .collect::<Result<_, BadReply>>()?;
    Ok(SearchPage {
        total: page.count.parse().map_err(|_| BadReply)?,
        rows: Mapped {
            value: rows,
            problems: Vec::new(),
        },
    })
}
