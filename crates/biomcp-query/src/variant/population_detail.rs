//! Native ancestry pagination reuses the same identity and gnomAD requests.
use crate::{Action, Ctx, Outcome, status};
use biodata::PopulationDataType;
use biomcp_adapters::gnomad;
use biomcp_fetch::RequestError;
use biomcp_types::{
    GnomadCounts, PageInfo, QueryBody, Reason, Request, Section, Source, Status, StepKind,
    VariantPopulationQuery, VariantPopulationResult,
};
pub(crate) fn get(cx: &Ctx<'_>, query: &VariantPopulationQuery) -> Outcome {
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let (population, groups, sources, problems, local) = read(cx, query, deadline);
    let total = groups.len() as u64;
    let groups: Box<[_]> = groups
        .into_iter()
        .skip(query.offset() as usize)
        .take(query.limit() as usize)
        .collect();
    let returned = groups.len() as u32;
    let page = query.detail().then_some(PageInfo {
        offset: query.offset(),
        returned,
        total,
        pool: None,
        truncated: None,
    });
    let next = if query.offset() as u64 + u64::from(returned) < total {
        vec![(
            StepKind::Page,
            Action::Run {
                request: Request::VariantPopulation(query.at_offset(query.offset() + returned)),
                fresh: false,
            },
        )]
    } else {
        Vec::new()
    };
    let status = match population {
        Section::Data { .. } if problems.is_empty() => Status::Complete,
        Section::Data { .. } => Status::Partial,
        Section::Empty { .. } => Status::Absent,
        _ => Status::Failed,
    };
    let out = Outcome {
        status,
        body: QueryBody::VariantPopulation(VariantPopulationResult { population, groups }),
        page,
        sources,
        problems,
        next,
        local,
    };
    command.finish(deadline.remaining());
    out
}
type PopulationRead = (
    Section<biomcp_types::VariantPopulation>,
    Vec<GnomadCounts>,
    Vec<biomcp_types::SourceOutcome>,
    Vec<biomcp_types::Problem>,
    Option<RequestError>,
);
fn read(
    cx: &Ctx<'_>,
    query: &VariantPopulationQuery,
    deadline: biomcp_fetch::Deadline<'_>,
) -> PopulationRead {
    let mut problems = Vec::new();
    let mut sources = Vec::new();
    let mut local = None;
    if cx.fetch.disabled(Source::Gnomad) {
        return (
            Section::Skipped {
                source: Source::Gnomad,
                reason: Reason::Disabled,
            },
            Vec::new(),
            sources,
            problems,
            local,
        );
    }
    let mut rows = Vec::new();
    if let Ok(identity) = query.identity() {
        let spec = super::source(&identity);
        let leg = cx.fetch.leg(spec, deadline).fresh(cx.fresh);
        let found = super::resolve(&identity, &leg, &mut problems);
        let error = found.as_ref().err().copied();
        sources.push(status::source(spec.source, leg.finish(), error));
        local = error.and_then(status::local);
        if let Ok((candidates, None)) = found {
            rows = candidates;
        }
        if let Some(RequestError::Source(error)) = error {
            return (
                Section::unavailable(Source::Gnomad, error.reason),
                Vec::new(),
                sources,
                problems,
                local,
            );
        }
    }
    let Some(id) = super::population::join(&rows) else {
        problems.push(biomcp_types::Problem {
            source: Source::Gnomad,
            record: None,
            field: "GRCh38_allele".into(),
            code: biomcp_types::ProblemCode::ValueUnknown,
        });
        return (
            Section::Unavailable {
                source: Source::Gnomad,
                reason: Reason::NoJoinId,
            },
            Vec::new(),
            sources,
            problems,
            local,
        );
    };
    let (section, groups, source, error) = load(cx, query, &id, &rows, &mut problems, deadline);
    problems.extend(projection_problems(&section, &groups));
    sources.push(source);
    (section, groups, sources, problems, local.or(error))
}
fn load(
    cx: &Ctx<'_>,
    query: &VariantPopulationQuery,
    id: &str,
    candidates: &[biomcp_types::VariantHeader],
    problems: &mut Vec<biomcp_types::Problem>,
    deadline: biomcp_fetch::Deadline<'_>,
) -> (
    Section<biomcp_types::VariantPopulation>,
    Vec<GnomadCounts>,
    biomcp_types::SourceOutcome,
    Option<RequestError>,
) {
    let leg = cx.fetch.leg(&gnomad::SPEC, deadline).fresh(cx.fresh);
    let mut groups = Vec::new();
    let result = gnomad::variant::population(&leg, id).and_then(|raw| {
        raw.map(|raw| {
            let summary = super::population::summary(&raw, candidates, problems)?;
            if query.detail() {
                groups = ancestry(&raw);
            }
            Ok(summary)
        })
        .transpose()
    });
    let error = result.as_ref().err().copied();
    let source = status::source(Source::Gnomad, leg.finish(), error);
    let section = match result {
        Ok(Some(data)) => Section::Data {
            source: Source::Gnomad,
            data,
        },
        Ok(None) => Section::Empty {
            source: Source::Gnomad,
            record: None,
        },
        Err(error) => Section::unavailable(Source::Gnomad, crate::status::reason(error)),
    };
    (section, groups, source, error.and_then(status::local))
}

fn ancestry(raw: &gnomad::variant::Variant) -> Vec<GnomadCounts> {
    let mut groups = Vec::new();
    for (data_type, data) in [
        (PopulationDataType::Exomes, &raw.exome),
        (PopulationDataType::Genomes, &raw.genome),
    ] {
        if let Some(data) = data {
            for group in &data.populations {
                groups.push(super::population::counts(data, data_type, Some(group)));
            }
        }
    }
    groups
}

fn projection_problems(
    section: &Section<biomcp_types::VariantPopulation>,
    groups: &[GnomadCounts],
) -> Vec<biomcp_types::Problem> {
    let mut problems = Vec::new();
    if let Section::Data { .. } = &section {
        for group in groups {
            if group.group_id.is_some() && group.ancestry_group.is_none() && group.sex.is_none() {
                problems.push(biomcp_types::Problem {
                    source: Source::Gnomad,
                    record: group.group_id.clone(),
                    field: "population_group".into(),
                    code: biomcp_types::ProblemCode::ValueUnknown,
                });
            }
        }
    }
    problems
}
