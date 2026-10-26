//! Human source-owned pathway discovery and exact identity retrieval.
use crate::{Ctx, Outcome};
use biomcp_types::{PathwayGet, PathwaySearch, QueryBody, Status};
fn empty(body: QueryBody) -> Outcome {
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
/// Discover source-native headers.
pub fn search(_cx: &Ctx<'_>, query: &PathwaySearch) -> Outcome {
    empty(QueryBody::PathwayPage(biomcp_types::PathwayPage {
        items: Box::default(),
        page: biomcp_types::PathwayPageInfo {
            offset: query.offset(),
            returned: 0,
            total: 0,
            total_kind: biomcp_types::PathwayTotalKind::Source,
            next_offset: None,
            pool: None,
        },
    }))
}
/// Read one exact card.
pub fn get(_cx: &Ctx<'_>, _query: &PathwayGet) -> Outcome {
    empty(QueryBody::PathwayCard(None))
}
