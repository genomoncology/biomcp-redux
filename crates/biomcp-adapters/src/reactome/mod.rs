//! Lowest-level human pathway mappings from Reactome.
use crate::{Mapped, quotas};
use biodata::{PathwayRef, Problem, ProblemCode, Source, Text, UniProtAcc};
use biomcp_fetch::{Arg, BadReply, Freshness, KeyUse, Leg, Reply, RequestError, SourceSpec};
use serde::Deserialize;
use std::time::Duration;
/// Public Reactome mapping endpoint policy.
pub static SPEC: SourceSpec = SourceSpec {
    terms: biomcp_fetch::Terms { name: "CC BY 4.0; CC0 annotation exports", url: "https://reactome.org/license", usage: biodata::Use::Attribution },
    source: Source::Reactome,
    base: "https://reactome.org/ContentService",
    key: KeyUse::None,
    quota: &quotas::REACTOME,
    deadline: Duration::from_secs(8),
};
#[derive(Deserialize)]
struct Rows(Vec<Row>);
impl Reply for Rows {
    fn check(&self) -> Result<(), BadReply> {
        Ok(())
    }
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Row {
    st_id: String,
    display_name: String,
    species_name: String,
    schema_class: String,
}
/// Fetch one accession; a checked 404 is an empty mapping.
pub fn pathways(
    leg: &Leg<'_>,
    accession: &UniProtAcc,
) -> Result<Mapped<Vec<PathwayRef>>, RequestError> {
    let endpoint = SPEC
        .get("/data/mapping/UniProt")
        .with_path(format!("/data/mapping/UniProt/{accession}/pathways"))?
        .fresh(Freshness::Days(7));
    let rows = match leg.json::<Rows>(&endpoint, &[("species", Arg::Text("9606"))]) {
        Ok(rows) => rows,
        Err(RequestError::Source(error)) if error.reason == biodata::Reason::NotFound => {
            return Ok(Mapped {
                value: Vec::new(),
                problems: Vec::new(),
            });
        }
        Err(error) => return Err(error),
    };
    let mut value = Vec::new();
    let mut problems = Vec::new();
    for row in rows.0 {
        let mapped = row
            .st_id
            .parse()
            .ok()
            .zip(Text::new(&row.display_name).ok());
        if row.species_name == "Homo sapiens"
            && row.schema_class == "Pathway"
            && let Some((st_id, display_name)) = mapped
        {
            value.push(PathwayRef {
                id: st_id,
                name: display_name,
            });
        } else {
            problems.push(Problem {
                source: Source::Reactome,
                record: Text::new(accession.as_str()).ok(),
                field: "pathways".into(),
                code: ProblemCode::ValueUnknown,
            });
        }
    }
    Ok(Mapped { value, problems })
}
