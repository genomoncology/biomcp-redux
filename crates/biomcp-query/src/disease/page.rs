use crate::{Action, Outcome};
use biomcp_types::{DiseaseGet, PageInfo, QueryBody, Request, Status};
pub(super) fn empty(body: QueryBody) -> Outcome {
    Outcome {
        status: Status::Failed,
        body,
        page: None,
        sources: Vec::new(),
        problems: Vec::new(),
        next: Vec::new(),
        local: None,
    }
}
pub(super) fn run(request: Request) -> Action {
    Action::Run {
        request,
        fresh: false,
    }
}
pub(super) fn page_info(offset: u32, returned: u32, total: u64) -> PageInfo {
    PageInfo {
        offset,
        returned,
        total,
        pool: None,
        truncated: None,
    }
}
pub(super) fn slice<T>(rows: Vec<T>, query: &DiseaseGet) -> (Vec<T>, u64) {
    let total = rows.len() as u64;
    (
        rows.into_iter()
            .skip(query.section_offset() as usize)
            .take(query.section_limit() as usize)
            .collect(),
        total,
    )
}
pub(super) fn overall(out: &Outcome, absent: bool) -> Status {
    if !out.problems.is_empty()
        || out
            .sources
            .iter()
            .any(|s| s.state == biomcp_types::SourceState::Stale)
    {
        Status::Partial
    } else if absent {
        Status::Absent
    } else {
        Status::Complete
    }
}
