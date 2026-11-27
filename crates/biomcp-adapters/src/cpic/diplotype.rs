//! Literal CPIC diplotype reads and a bounded sample of source spellings.
use super::read::{Page, read};
use super::{SPEC, case_insensitive, regex_literal};
use biodata::{GeneSymbol, PgxDiplotypeResult, Text};
use biomcp_fetch::{Arg, Freshness, Input, Leg, Reply, RequestError};
use biomcp_types::PgxGet;
use serde::Deserialize;

const PATH: &str = "/diplotype";
const EXAMPLE_COUNT: u32 = 3;
const EXAMPLE_BYTES: usize = 96;

/// Prefer the exact source row, then try either allele order without regard to case.
pub fn diplotype(leg: &Leg<'_>, query: &PgxGet) -> Result<Page<PgxDiplotypeResult>, RequestError> {
    let expression = query
        .diplotype()
        .ok_or(biomcp_fetch::InputError::Serialization)?
        .as_str();
    let gene = gene_filter(query.gene());
    let exact = read_rows(leg, query, &gene, &format!("eq.{expression}"), |value| {
        value == expression
    })?;
    if exact.total != 0 {
        return Ok(exact);
    }

    let reversed = expression.split_once('/').and_then(|(first, second)| {
        (!first.is_empty() && !second.is_empty() && !second.contains('/'))
            .then(|| format!("{second}/{first}"))
    });
    let filter = match reversed.as_deref() {
        Some(value) if !value.eq_ignore_ascii_case(expression) => {
            format!(
                "imatch.^({}|{})$",
                regex_literal(expression),
                regex_literal(value)
            )
        }
        _ => case_insensitive(expression),
    };
    read_rows(leg, query, &gene, &filter, |value| {
        value.eq_ignore_ascii_case(expression)
            || reversed
                .as_deref()
                .is_some_and(|other| value.eq_ignore_ascii_case(other))
    })
}

fn gene_filter(gene: &GeneSymbol) -> String {
    format!("eq.{}", gene.as_str().to_ascii_uppercase())
}

fn read_rows(
    leg: &Leg<'_>,
    query: &PgxGet,
    gene: &str,
    value: &str,
    matches: impl Fn(&str) -> bool,
) -> Result<Page<PgxDiplotypeResult>, RequestError> {
    read::<super::raw::PgxDiplotypeResult>(
        leg,
        PATH,
        &[
            ("select", Arg::Text("*")),
            ("genesymbol", Arg::Text(gene)),
            ("diplotype", Arg::Text(value)),
            ("limit", Arg::Num(query.section_limit().into())),
            ("offset", Arg::Num(query.section_offset().into())),
        ],
        query.section_offset(),
        query.section_limit(),
        true,
        |rows: &[PgxDiplotypeResult], _range| {
            rows.iter().all(|r| &r.genesymbol == query.gene() && matches(&r.diplotype))
        },
    )
}

#[derive(Deserialize)]
struct Spelling {
    genesymbol: GeneSymbol,
    diplotype: Box<str>,
}

#[derive(Deserialize)]
#[serde(transparent)]
struct Spellings(Vec<Spelling>);

impl Reply for Spellings {
    fn check(&self) -> Result<(), biomcp_fetch::BadReply> {
        Ok(())
    }
}

/// Sample three rows in spelling order, fetching only gene and diplotype.
pub fn diplotype_examples(leg: &Leg<'_>, gene: &GeneSymbol) -> Result<Vec<Box<str>>, RequestError> {
    let filter = gene_filter(gene);
    let endpoint = SPEC.get(PATH).fresh(Freshness::Days(1));
    let rows = leg.call(
        &endpoint,
        Input::Params(&[
            ("select", Arg::Text("genesymbol,diplotype")),
            ("genesymbol", Arg::Text(&filter)),
            ("order", Arg::Text("diplotype.asc")),
            ("limit", Arg::Num(EXAMPLE_COUNT.into())),
            ("offset", Arg::Num(0)),
        ]),
        biomcp_fetch::Read::<Spellings>::json().counted(&|rows, range, status| {
            range.check(0, EXAMPLE_COUNT, rows.0.len(), status)?;
            if rows.0.iter().any(|row| &row.genesymbol != gene) {
                return Err(biomcp_fetch::BadReply);
            }
            Ok(())
        }),
    )?;
    let mut examples = Vec::new();
    for row in rows.value.0 {
        let spelling = row.diplotype;
        if spelling.len() <= EXAMPLE_BYTES
            && Text::new(&spelling).is_ok_and(|text| text.as_str() == spelling.as_ref())
            && !examples.contains(&spelling)
        {
            examples.push(spelling);
        }
    }
    Ok(examples)
}
