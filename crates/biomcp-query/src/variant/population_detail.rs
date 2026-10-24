//! Native ancestry pagination reuses the same identity and gnomAD requests.
use crate::{Action, Ctx, Outcome, status};
use biodata::{
    PageInfo, PopulationGroup, PopulationSubset, QueryBody, Reason, Request, Section, Source,
    Status, StepKind, VariantPopulationQuery, VariantPopulationResult,
};
use biomcp_adapters::gnomad;
use biomcp_fetch::RequestError;
pub fn get(cx: &Ctx<'_>, query: &VariantPopulationQuery) -> Outcome {
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let mut problems = Vec::new();
    let mut sources = Vec::new();
    let identity = query.identity();
    let mut local = None;
    let mut rows = Vec::new();
    if let Ok(identity) = &identity {
        let spec = super::source(identity);
        let leg = cx.fetch.leg(spec, deadline).fresh(cx.fresh);
        let found = super::resolve(identity, &leg, &mut problems);
        let error = found.as_ref().err().copied();
        sources.push(status::source(spec.source, leg.finish(), error));
        local = error.and_then(status::local);
        if let Ok((candidates, None)) = found {
            rows = candidates;
        }
    }
    let mut groups = Vec::new();
    let population = if let Some((id, subject)) = super::population::join(&rows) {
        let leg = cx.fetch.leg(&gnomad::SPEC, deadline).fresh(cx.fresh);
        let result = gnomad::variant::population(&leg, &id).and_then(|raw| {
            raw.map(|raw| {
                let summary = super::population::summary(&raw, &subject)?;
                if query.detail() {
                    groups = ancestry(&raw, &subject)?;
                }
                Ok(summary)
            })
            .transpose()
        });
        let error = result.as_ref().err().copied();
        sources.push(status::source(Source::Gnomad, leg.finish(), error));
        local = local.or(error.and_then(status::local));
        match result {
            Ok(Some(data)) => Section::Data {
                source: Source::Gnomad,
                data,
            },
            Ok(None) => Section::Empty {
                source: Source::Gnomad,
                record: None,
            },
            Err(e) => Section::unavailable(
                Source::Gnomad,
                match e {
                    RequestError::Source(e) => e.reason,
                    _ => Reason::Unavailable,
                },
            ),
        }
    } else {
        problems.push(biodata::Problem {
            source: Source::Gnomad,
            record: None,
            field: "GRCh38_allele".into(),
            code: biodata::ProblemCode::ValueUnknown,
        });
        Section::Unavailable {
            source: Source::Gnomad,
            reason: Reason::NoJoinId,
        }
    };
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
        Section::Data { .. } => Status::Complete,
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
fn ancestry(
    raw: &gnomad::variant::Variant,
    subject: &biodata::VariantRef,
) -> Result<Vec<PopulationGroup>, RequestError> {
    let mut groups = Vec::new();
    for (subset, data) in [
        (PopulationSubset::Exome, &raw.exome),
        (PopulationSubset::Genome, &raw.genome),
    ] {
        if let Some(data) = data {
            for group in &data.populations {
                groups.push(PopulationGroup {
                    subset,
                    observation: super::population::observation(
                        &raw.variant_id,
                        subject,
                        Some(group.id.clone()),
                        group.ac,
                        group.an,
                        &data.filters,
                    )?,
                });
            }
        }
    }
    Ok(groups)
}
