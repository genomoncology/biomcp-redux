//! Explicit MAUDE native cards and whole device/narrative pages.
use crate::{Action, Ctx, Outcome, status};
use biodata::Text;
use biomcp_adapters::maude;
use biomcp_types::{
    AdverseEventGet, AdverseEventSection, QueryBody, Request, Source, Status, StepKind,
};
mod projection;
mod rows;
mod snapshot;
/// Read exactly one numeric MAUDE key and project only its requested section.
pub fn get(cx: &Ctx<'_>, query: &AdverseEventGet) -> Outcome {
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let mut out = Outcome {
        status: Status::Failed,
        body: empty(query),
        page: None,
        sources: Vec::new(),
        problems: Vec::new(),
        next: Vec::new(),
        local: None,
    };
    match snapshot::load(cx, query, deadline, &mut out) {
        Ok(Some((report, positions, snapshot))) => {
            let bound = query.with_snapshot(snapshot.clone());
            match projection::project(&bound, report, snapshot, positions) {
                Ok((body, total, returned, offset)) => {
                    out.body = body;
                    navigation(&bound, total, returned, offset, &mut out);
                }
                Err(_) => too_large(query, &mut out),
            }
        }
        Ok(None) => {}
        Err(error) => out.local = status::local(error),
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
fn navigation(query: &AdverseEventGet, total: u64, returned: u32, offset: u32, out: &mut Outcome) {
    if query.section().is_some() {
        out.page = Some(biomcp_types::PageInfo {
            offset: query.section_offset(),
            returned,
            total,
            pool: None,
            truncated: None,
        });
        if offset > query.section_offset() && u64::from(offset) < total {
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
fn text(value: &str) -> Result<Text, biomcp_types::QueryInvalid> {
    Text::new(value).map_err(|_| biomcp_types::QueryInvalid::default())
}

fn too_large(query: &AdverseEventGet, out: &mut Outcome) {
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
