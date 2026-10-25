//! Explicit MAUDE native cards and whole device/narrative pages.
use crate::{Action, Ctx, Outcome, status};
use biodata::Text;
use biomcp_adapters::maude;
use biomcp_types::{
    AdverseEventGet, AdverseEventSection, QueryBody, Request, Source, Status, StepKind,
};
mod projection;
/// Read exactly one numeric MAUDE key and project only its requested section.
pub fn get(cx: &Ctx<'_>, query: &AdverseEventGet) -> Outcome {
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let leg = cx.fetch.leg(&maude::SPEC, deadline).fresh(cx.fresh);
    let reply = maude::get(&leg, query);
    let error = reply.as_ref().err().copied();
    let mut out = Outcome {
        status: Status::Failed,
        body: empty(query),
        page: None,
        sources: vec![status::source(Source::Maude, leg.finish(), error)],
        problems: Vec::new(),
        next: Vec::new(),
        local: error.and_then(status::local),
    };
    if out
        .sources
        .iter()
        .any(|s| s.reason == Some(biomcp_types::Reason::BadReply))
    {
        out.problems.push(biomcp_types::Problem {
            source: Source::Maude,
            record: Text::new(query.id().as_str()).ok(),
            field: "report".into(),
            code: biomcp_types::ProblemCode::ValueUnknown,
        });
    }
    if let Ok(report) = reply {
        out.status = if report.is_none() {
            Status::Absent
        } else if out
            .sources
            .iter()
            .any(|s| s.state == biomcp_types::SourceState::Stale)
        {
            Status::Partial
        } else {
            Status::Complete
        };
        if let Some(report) = report {
            match projection::project(query, report) {
                Ok((body, total, returned)) => {
                    out.body = body;
                    navigation(query, total, returned, &mut out);
                }
                Err(()) => {
                    out.status = Status::Failed;
                    out.sources = vec![biomcp_adapters::registry::outcome(
                        Source::Maude,
                        biomcp_types::SourceState::Failed,
                        Some(biomcp_types::Reason::TooLarge),
                    )];
                    out.problems.push(biomcp_types::Problem {
                        source: Source::Maude,
                        record: Text::new(query.id().as_str()).ok(),
                        field: "narrative".into(),
                        code: biomcp_types::ProblemCode::ValueUnknown,
                    });
                }
            }
        }
    }
    command.finish(deadline.remaining());
    out
}
fn empty(query: &AdverseEventGet) -> QueryBody {
    match query.section() {
        Some(AdverseEventSection::Devices) => QueryBody::AdverseEventDevices(None),
        Some(AdverseEventSection::Narrative) => QueryBody::AdverseEventNarrative(None),
        _ => QueryBody::AdverseEventMaudeCard(None),
    }
}
fn navigation(query: &AdverseEventGet, total: u64, returned: u32, out: &mut Outcome) {
    if query.section().is_some() {
        let offset = query.section_offset().saturating_add(returned);
        out.page = Some(biomcp_types::PageInfo {
            offset: query.section_offset(),
            returned,
            total,
            pool: None,
            truncated: None,
        });
        if returned > 0 && u64::from(offset) < total {
            out.next.push((
                StepKind::Page,
                Action::Run {
                    request: Request::GetAdverseEvent(query.at_section_offset(offset)),
                    fresh: false,
                },
            ));
        }
    } else {
        for section in [AdverseEventSection::Devices, AdverseEventSection::Narrative] {
            if let Ok(next) = query.with_section(section, None) {
                out.next.push((
                    StepKind::Section,
                    Action::Run {
                        request: Request::GetAdverseEvent(next),
                        fresh: false,
                    },
                ));
            }
        }
    }
}
fn text(value: &str) -> Result<Text, ()> {
    Text::new(value).map_err(|_| ())
}
