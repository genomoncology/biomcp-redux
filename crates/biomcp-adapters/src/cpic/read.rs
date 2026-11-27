//! Complete native CPIC rows with provider counts checked before cache admission.
use super::SPEC;
use crate::{mapped::Problems, pgx_sources::Native, raw::Field};
use biodata::{
    PgxDrugRecommendation, PgxGeneIdentity, PgxGuidelineReference, PgxPair, PgxRecommendation,
};
use biomcp_fetch::Input;
use biomcp_fetch::{Arg, BadReply, Freshness, Leg, Reply, RequestError};
use biomcp_types::{PgxGet, PgxSearch};
use serde::Deserialize;
/// One native array and its exact collection size.
#[derive(Debug)]
pub struct Page<T> {
    /// Complete rows in native order.
    pub items: Vec<T>,
    /// Native exact collection size.
    pub total: u64,
    /// Native rows consumed before identity refusals.
    pub consumed: u32,
    /// Actual refused native records, independent of optional fields.
    pub refusals: crate::Refusal,
    /// Rejected native fields and records.
    pub problems: Vec<biomcp_types::Problem>,
}
#[derive(Deserialize)]
#[serde(transparent, bound(deserialize = "T: Deserialize<'de>"))]
struct Rows<T>(Vec<Field<T>>);
impl<T: serde::de::DeserializeOwned> Reply for Rows<T> {
    fn check(&self) -> Result<(), BadReply> {
        Ok(())
    }
}
pub(super) fn read<T: serde::de::DeserializeOwned + Native>(
    leg: &Leg<'_>,
    path: &'static str,
    args: &[(&str, Arg<'_>)],
    offset: u32,
    limit: u32,
    complete: bool,
    check: impl Fn(&[T::Value], biomcp_fetch::ContentRange) -> bool,
) -> Result<Page<T::Value>, RequestError> {
    let endpoint = SPEC.get(path).fresh(Freshness::Days(1));
    let (rows, metadata) = leg
        .call(
            &endpoint,
            Input::Params(args),
            biomcp_fetch::Read::<Rows<T>>::json().counted(&|rows, range, status| {
                range.check(offset, limit, rows.0.len(), status)?;
                if complete && range.total != rows.0.len() as u64 {
                    return Err(BadReply);
                }
                Ok(())
            }),
        )
        .map(|received| (received.value, received.meta))?;
    let range = metadata
        .content_range
        .ok_or(biomcp_fetch::InputError::Serialization)?;
    let consumed = rows.0.len() as u32;
    let mut problems = Problems {
        source: biomcp_types::Source::Cpic,
        record: None,
        list: Vec::new(),
    };
    let mut items = Vec::new();
    let mut refusals = crate::Refusal::None;
    for row in rows.0 {
        problems.record = None;
        if let Some(raw) = problems.record("record", row, &mut refusals) {
            problems.record = raw.record();
            let before = problems.list.len();
            if let Some(mapped) = raw.map(&mut problems, &mut refusals) {
                if check(std::slice::from_ref(&mapped), range) {
                    items.push(mapped);
                } else {
                    refusals = crate::Refusal::Refused;
                    if problems.list.len() == before {
                        problems.required::<()>("record", None);
                    }
                }
            }
        }
    }
    if !check(&items, range) {
        return Err(biomcp_fetch::SourceError {
            source: biomcp_types::Source::Cpic,
            reason: biomcp_types::Reason::BadReply,
            status: None,
            tls_cause: None,
            retry_after: None,
        }
        .into());
    }
    Ok(Page {
        refusals,
        items,
        total: range.total,
        consumed,
        problems: problems.list,
    })
}
/// Search gene pairs at `offset`, preserving native level namespaces and row order.
pub(super) fn pairs(
    leg: &Leg<'_>,
    query: &PgxSearch,
    offset: u32,
    limit: u32,
) -> Result<Page<PgxPair>, RequestError> {
    let symbol = query.gene().as_str();
    let page = pairs_for_gene(leg, query, offset, limit, &format!("eq.{symbol}"));
    if page.as_ref().is_ok_and(|p| p.total == 0) {
        return pairs_for_gene(leg, query, offset, limit, &super::case_insensitive(symbol));
    }
    page
}
pub(super) fn pairs_for_gene(
    leg: &Leg<'_>,
    query: &PgxSearch,
    offset: u32,
    limit: u32,
    gene: &str,
) -> Result<Page<PgxPair>, RequestError> {
    let level = query
        .cpic_level()
        .map(|v| format!("eq.{}", v.as_str().to_ascii_uppercase()));
    let mut args = vec![
        ("select", Arg::Text("*")),
        ("genesymbol", Arg::Text(gene)),
        ("order", Arg::Text("pairid.asc")),
        ("limit", Arg::Num(limit.into())),
        ("offset", Arg::Num(offset.into())),
    ];
    if let Some(level) = &level {
        args.push(("cpiclevel", Arg::Text(level)));
    }
    read::<super::raw::PgxPair>(
        leg,
        "/pair_view",
        &args,
        offset,
        limit,
        false,
        |rows: &[PgxPair], _range| {
            rows.iter().all(|r| {
                &r.genesymbol == query.gene()
                    && query.cpic_level().is_none_or(|v| {
                        r.cpiclevel
                            .as_deref()
                            .is_some_and(|level| level.eq_ignore_ascii_case(v.as_str()))
                    })
            }) && ordered(rows.iter().map(|r| r.pairid.id))
        },
    )
}
/// Select only the seven reviewed native identity fields.
pub fn card(leg: &Leg<'_>, query: &PgxGet) -> Result<Page<PgxGeneIdentity>, RequestError> {
    let symbol = query.gene().as_str();
    let page = card_for_gene(leg, query, &format!("eq.{symbol}"))?;
    if page.total == 0 {
        return card_for_gene(leg, query, &super::case_insensitive(symbol));
    }
    Ok(page)
}
fn card_for_gene(
    leg: &Leg<'_>,
    query: &PgxGet,
    gene: &str,
) -> Result<Page<PgxGeneIdentity>, RequestError> {
    read::<super::raw::PgxGeneIdentity>(
        leg,
        super::GENE_PATH,
        &[
            (
                "select",
                Arg::Text("symbol,hgncid,ncbiid,ensemblid,clinpgxid,lookupmethod,version"),
            ),
            ("symbol", Arg::Text(gene)),
            ("limit", Arg::Num(5)),
            ("offset", Arg::Num(0)),
        ],
        0,
        5,
        true,
        |rows: &[PgxGeneIdentity], _range| {
            rows.len() <= 1 && rows.iter().all(|r| &r.symbol == query.gene())
        },
    )
}
/// Read gene-keyed recommendation view rows without inventing drug IDs.
pub fn recommendations(
    leg: &Leg<'_>,
    query: &PgxGet,
) -> Result<Page<PgxRecommendation>, RequestError> {
    let key = format!("lookupkey->>{}", query.gene());
    read::<super::raw::PgxRecommendation>(
        leg,
        "/recommendation_view",
        &[
            ("select", Arg::Text("*")),
            (&key, Arg::Text("not.is.null")),
            ("order", Arg::Text("recommendationid.asc")),
            ("limit", Arg::Num(query.section_limit().into())),
            ("offset", Arg::Num(query.section_offset().into())),
        ],
        query.section_offset(),
        query.section_limit(),
        false,
        |rows: &[PgxRecommendation], _range| {
            rows.iter().all(|r| {
                r.lookupkey
                    .keys()
                    .any(|k| k.to_string() == query.gene().to_string())
            }) && ordered(rows.iter().map(|r| r.recommendationid.id))
        },
    )
}
/// Exact native drug filter selects the distinct table contract, with the declared gene.
pub fn drug_recommendations(
    leg: &Leg<'_>,
    query: &PgxGet,
) -> Result<Page<PgxDrugRecommendation>, RequestError> {
    let id = query
        .drug_id()
        .ok_or(biomcp_fetch::InputError::Serialization)?;
    let drug = format!("eq.{id}");
    let key = format!("lookupkey->>{}", query.gene());
    let filter = super::phenotypes::filter(query);
    let mut args = vec![
        ("select", Arg::Text("*")),
        (key.as_str(), Arg::Text("not.is.null")),
        ("drugid", Arg::Text(&drug)),
        ("order", Arg::Text("id.asc")),
        ("limit", Arg::Num(query.section_limit().into())),
        ("offset", Arg::Num(query.section_offset().into())),
    ];
    if let Some((field, value)) = &filter {
        args.push((field.as_str(), Arg::Text(value)));
    }
    read::<super::raw::PgxDrugRecommendation>(
        leg,
        "/recommendation",
        &args,
        query.section_offset(),
        query.section_limit(),
        false,
        |rows: &[PgxDrugRecommendation], _range| {
            rows.iter().all(|r| {
                &r.drugid == id
                    && r.lookupkey
                        .keys()
                        .any(|k| k.to_string() == query.gene().to_string())
                    && super::phenotypes::matches(query, r.phenotypes.as_ref())
            }) && ordered(rows.iter().map(|r| r.id.id))
        },
    )
}
/// Read the native summary references, with no publication or date enrichment.
pub fn guidelines(
    leg: &Leg<'_>,
    query: &PgxGet,
) -> Result<Page<PgxGuidelineReference>, RequestError> {
    let symbol = query
        .gene()
        .to_string()
        .replace('\\', "\\\\")
        .replace('"', "\\\"");
    let genes = format!("cs.[{{\"symbol\":\"{symbol}\"}}]");
    read::<super::raw::PgxGuidelineReference>(
        leg,
        "/guideline_summary_view",
        &[
            ("select", Arg::Text("*")),
            ("genes", Arg::Text(&genes)),
            ("limit", Arg::Num(query.section_limit().into())),
            ("offset", Arg::Num(query.section_offset().into())),
        ],
        query.section_offset(),
        query.section_limit(),
        false,
        |rows: &[PgxGuidelineReference], _range| {
            rows.iter()
                .all(|r| r.genes.iter().any(|g| &g.symbol == query.gene()))
        },
    )
}
fn ordered(values: impl Iterator<Item = i64>) -> bool {
    let mut previous = None;
    for value in values {
        if previous.is_some_and(|p| value <= p) {
            return false;
        }
        previous = Some(value);
    }
    true
}
