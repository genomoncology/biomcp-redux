use super::{SPEC, map, raw};
use crate::Mapped;
use biodata::{ReactomeId, Text};
use biomcp_fetch::{Arg, BadReply, Freshness, Leg, RequestError};
use biomcp_types::{PathwayCard, PathwayHeader, PathwaySearch, Source};
/// One native page; rejected rows do not change its native position or total.
#[derive(Debug)]
pub struct Page {
    /// Admitted headers and record-local problems.
    pub rows: Mapped<Vec<PathwayHeader>>,
    /// Source-reported unfiltered count or complete observed relation size.
    pub total: u64,
    /// Native rows consumed, independent of admitted rows.
    pub consumed: u32,
}
/// Search exactly the requested native source window, without refill.
pub fn search(leg: &Leg<'_>, q: &PathwaySearch) -> Result<Page, RequestError> {
    let text = q.query().ok_or(biomcp_fetch::InputError::Serialization)?;
    let endpoint = SPEC
        .get("/search/query")
        .fresh(Freshness::Days(1))
        .json_absence();
    let reply = leg.json_checked_admission::<raw::Search>(
        &endpoint,
        &[
            ("query", Arg::Text(text.as_str())),
            ("rows", Arg::Num(q.limit().into())),
            ("Start row", Arg::Num(q.offset().into())),
            ("species", Arg::Text("Homo sapiens")),
            ("types", Arg::Text("Pathway")),
            ("types", Arg::Text("TopLevelPathway")),
            ("Force filters", Arg::Text("true")),
        ],
        |v| match v {
            raw::Search::Page { count, .. } if *count <= q.limit() => Ok(()),
            raw::Search::Empty(v)
                if v.messages == [format!("No entries found for query: {text}")] =>
            {
                Ok(())
            }
            _ => Err(BadReply),
        },
        |v| match v {
            raw::Search::Page { results, .. } => results
                .iter()
                .flat_map(|g| &g.entries)
                .all(|r| map::hit(r).is_some()),
            raw::Search::Empty(_) => false,
        },
    )?;
    let mut rows = Mapped {
        value: Vec::new(),
        problems: Vec::new(),
    };
    let (total, consumed) = match reply {
        raw::Search::Page {
            results,
            count,
            total,
        } => {
            for row in results.into_iter().flat_map(|g| g.entries) {
                match map::hit(&row) {
                    Some(header) => rows.value.push(header),
                    None => rows.problems.push(crate::mapped::problem(
                        Source::Reactome,
                        biodata::Text::new(&row.st_id).ok(),
                        "identity",
                    )),
                }
            }
            (total, count)
        }
        raw::Search::Empty(_) => (0, 0),
    };
    Ok(Page {
        rows,
        total,
        consumed,
    })
}
/// Read the complete measured human top-level relation once.
pub fn top_level(leg: &Leg<'_>) -> Result<Mapped<Vec<PathwayHeader>>, RequestError> {
    let endpoint = SPEC
        .get("/data/pathways/top/Homo%20sapiens")
        .fresh(Freshness::Days(7));
    let rows = leg.json_checked_cache::<raw::Cards>(&endpoint, &[], |v| {
        v.0.iter().all(|r| map::header(r).is_some())
    })?;
    let mut mapped = Mapped {
        value: Vec::new(),
        problems: Vec::new(),
    };
    for row in rows.0 {
        match map::header(&row) {
            Some(v) => mapped.value.push(v),
            None => mapped.problems.push(crate::mapped::problem(
                Source::Reactome,
                biodata::Text::new(&row.st_id).ok(),
                "identity",
            )),
        }
    }
    Ok(mapped)
}
/// Preserve an exact card mismatch as a problem and never cache that body.
pub fn card(
    leg: &Leg<'_>,
    id: &ReactomeId,
    summary: bool,
) -> Result<Mapped<Option<PathwayCard>>, RequestError> {
    let endpoint = SPEC
        .get("/data/query")
        .with_path(format!("/data/query/{id}"))?
        .fresh(Freshness::Days(7));
    let row = match leg.json_checked_cache::<raw::Card>(&endpoint, &[], |r| {
        r.st_id == id.as_str() && map::card(r.clone(), true).is_some()
    }) {
        Err(RequestError::Source(e)) if e.reason == biomcp_types::Reason::NotFound => {
            return Ok(Mapped {
                value: None,
                problems: Vec::new(),
            });
        }
        result => result?,
    };
    let value = (row.st_id == id.as_str())
        .then(|| map::card(row, summary))
        .flatten();
    let problems = if value.is_none() {
        vec![crate::mapped::problem(
            Source::Reactome,
            biodata::Text::new(id.as_str()).ok(),
            "identity",
        )]
    } else {
        Vec::new()
    };
    Ok(Mapped { value, problems })
}
/// Read the source database release independently from pathway version.
pub fn release(leg: &Leg<'_>) -> Result<Text, RequestError> {
    let endpoint = SPEC.get("/data/database/version").fresh(Freshness::Days(1));
    let value = leg.json::<raw::Version>(&endpoint, &[])?;
    Text::new(&value.0.to_string()).map_err(|_| biomcp_fetch::InputError::Serialization.into())
}
