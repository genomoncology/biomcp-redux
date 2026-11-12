//! A drug get that names several sections or label headings runs each as the call for that
//! section alone and answers once, one part per section, each with its own paging.
//! A part that is not a section reply, such as a refused name or heading, answers alone.
use super::{Encoding, Outcome, ResponseMeta};
use biomcp_types::{
    DrugGet, DrugSectionPart, DrugSectionReply, DrugSections, Envelope, Reply, ReplyBody,
    ReplyData, Request, Status, UsageError,
};
use std::io;

pub(super) fn run(
    fetch: &biomcp_fetch::Fetch,
    settings: &crate::config::Settings,
    query: &DrugGet,
    fresh: bool,
    globals: &crate::grammar::Globals,
    encoding: Encoding,
) -> io::Result<Outcome> {
    let mut parts = Vec::new();
    let mut statuses = Vec::new();
    let mut local = false;
    for part in query.parts().map_err(io::Error::other)? {
        let request = Request::GetDrug(part.clone());
        let result = super::run_request(fetch, &request, fresh, settings, false)?;
        if !sectioned(&result.body) {
            return super::response(fetch, request, result, fresh, globals, encoding);
        }
        local |= result.local.is_some();
        statuses.push(result.status());
        let (meta, body) = super::finished(fetch, &request, result, fresh, globals, encoding)?;
        let reply = body
            .visit(Part(meta))
            .ok_or_else(|| io::Error::other("drug section reply"))?;
        parts.push(DrugSectionPart {
            section: part
                .sections()
                .first()
                .copied()
                .ok_or_else(|| io::Error::other("drug section"))?,
            label_section: part.label_section().cloned(),
            reply,
        });
    }
    let status = combined(&statuses);
    let exit = if local {
        crate::exit::Status::Local
    } else {
        crate::exit::Status::from(status)
    };
    let body = DrugSections {
        parts: parts.into(),
    };
    encoding.encode(Envelope::new(status, query.clone(), Some(body)), exit)
}

/// Absent or failed only when every part is; partial when any part failed or is partial.
fn combined(statuses: &[Status]) -> Status {
    if statuses.iter().all(|status| *status == Status::Absent) {
        Status::Absent
    } else if statuses.iter().all(|status| *status == Status::Failed) {
        Status::Failed
    } else if statuses
        .iter()
        .any(|status| matches!(status, Status::Failed | Status::Partial))
    {
        Status::Partial
    } else {
        Status::Complete
    }
}

fn sectioned(body: &ReplyData) -> bool {
    matches!(
        body,
        ReplyData::DrugCard { .. }
            | ReplyData::DrugXrefs { .. }
            | ReplyData::DrugTargets { .. }
            | ReplyData::DrugIndications { .. }
            | ReplyData::DrugCivic { .. }
            | ReplyData::DrugLabel { .. }
            | ReplyData::DrugApprovals { .. }
            | ReplyData::DrugRegulatory { .. }
            | ReplyData::DrugInteractions { .. }
    )
}

/// Build one part's envelope from a drug section reply.
struct Part(ResponseMeta);
impl Part {
    fn wrap<B: ReplyBody>(
        self,
        query: B::Query,
        body: Option<B>,
        variant: fn(Box<Envelope<B>>) -> DrugSectionReply,
    ) -> Option<DrugSectionReply> {
        Some(variant(Box::new(self.0.envelope(query, body))))
    }
}
impl Reply for Part {
    type Output = Option<DrugSectionReply>;
    fn reply<B: ReplyBody>(self, _: B::Query, _: Option<B>) -> Self::Output {
        None
    }
    fn usage(self, _: UsageError) -> Self::Output {
        None
    }
    fn pool(self, _: bool) -> Self::Output {
        None
    }
    fn drug_card(self, query: DrugGet, body: Option<biomcp_types::DrugCard>) -> Self::Output {
        self.wrap(query, body, DrugSectionReply::Card)
    }
    fn drug_xrefs(self, query: DrugGet, body: Option<biomcp_types::DrugXrefs>) -> Self::Output {
        self.wrap(query, body, DrugSectionReply::Xrefs)
    }
    fn drug_targets(self, query: DrugGet, body: Option<biomcp_types::DrugTargets>) -> Self::Output {
        self.wrap(query, body, DrugSectionReply::Targets)
    }
    fn drug_indications(
        self,
        query: DrugGet,
        body: Option<biomcp_types::DrugIndications>,
    ) -> Self::Output {
        self.wrap(query, body, DrugSectionReply::Indications)
    }
    fn drug_civic(self, query: DrugGet, body: Option<biomcp_types::DrugCivic>) -> Self::Output {
        self.wrap(query, body, DrugSectionReply::Civic)
    }
    fn drug_label(self, query: DrugGet, body: Option<biomcp_types::DrugLabel>) -> Self::Output {
        self.wrap(query, body, DrugSectionReply::Label)
    }
    fn drug_approvals(
        self,
        query: DrugGet,
        body: Option<biomcp_types::DrugApprovals>,
    ) -> Self::Output {
        self.wrap(query, body, DrugSectionReply::Approvals)
    }
    fn drug_regulatory(
        self,
        query: DrugGet,
        body: Option<biomcp_types::DrugRegulatory>,
    ) -> Self::Output {
        self.wrap(query, body, DrugSectionReply::Regulatory)
    }
    fn drug_interactions(
        self,
        query: DrugGet,
        body: Option<biomcp_types::DrugInteractions>,
    ) -> Self::Output {
        self.wrap(query, body, DrugSectionReply::Interactions)
    }
}
