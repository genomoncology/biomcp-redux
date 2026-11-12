//! A drug get that names several sections or label headings runs each as the call for that
//! section alone and answers once, one part per section, each with its own paging.
//! A part that is not a section reply, such as a refused name or heading, answers alone.
use super::{Encoding, Outcome, ResponseMeta};
use biomcp_types::{
    DrugGet, DrugSection, DrugSectionPart, DrugSections, Envelope, Reply, ReplyBody, ReplyData,
    Request, Status, UsageError,
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
        parts.push(
            body.visit(Part(meta))
                .ok_or_else(|| io::Error::other("drug section reply"))?,
        );
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
        ReplyData::DrugCard { body: None, .. }
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
        variant: fn(Box<Envelope<B>>) -> DrugSectionPart,
    ) -> Option<DrugSectionPart> {
        Some(variant(Box::new(self.0.envelope(query, body))))
    }
}
impl Reply for Part {
    type Output = Option<DrugSectionPart>;
    fn reply<B: ReplyBody>(self, _: B::Query, _: Option<B>) -> Self::Output {
        None
    }
    fn usage(self, _: UsageError) -> Self::Output {
        None
    }
    fn pool(self, _: bool) -> Self::Output {
        None
    }
    /// A record that was not found answers with an identity card and no body; the part
    /// keeps the section it asked for.
    fn drug_card(self, query: DrugGet, body: Option<biomcp_types::DrugCard>) -> Self::Output {
        if body.is_some() {
            return None;
        }
        match query.sections().first()? {
            DrugSection::Xrefs => self.wrap(query, None, DrugSectionPart::Xrefs),
            DrugSection::Targets => self.wrap(query, None, DrugSectionPart::Targets),
            DrugSection::Indications => self.wrap(query, None, DrugSectionPart::Indications),
            DrugSection::Civic => self.wrap(query, None, DrugSectionPart::Civic),
            DrugSection::Label => self.wrap(query, None, DrugSectionPart::Label),
            DrugSection::Approvals => self.wrap(query, None, DrugSectionPart::Approvals),
            DrugSection::Regulatory => self.wrap(query, None, DrugSectionPart::Regulatory),
            DrugSection::Interactions => self.wrap(query, None, DrugSectionPart::Interactions),
        }
    }
    fn drug_xrefs(self, query: DrugGet, body: Option<biomcp_types::DrugXrefs>) -> Self::Output {
        self.wrap(query, body, DrugSectionPart::Xrefs)
    }
    fn drug_targets(self, query: DrugGet, body: Option<biomcp_types::DrugTargets>) -> Self::Output {
        self.wrap(query, body, DrugSectionPart::Targets)
    }
    fn drug_indications(
        self,
        query: DrugGet,
        body: Option<biomcp_types::DrugIndications>,
    ) -> Self::Output {
        self.wrap(query, body, DrugSectionPart::Indications)
    }
    fn drug_civic(self, query: DrugGet, body: Option<biomcp_types::DrugCivic>) -> Self::Output {
        self.wrap(query, body, DrugSectionPart::Civic)
    }
    fn drug_label(self, query: DrugGet, body: Option<biomcp_types::DrugLabel>) -> Self::Output {
        self.wrap(query, body, DrugSectionPart::Label)
    }
    fn drug_approvals(
        self,
        query: DrugGet,
        body: Option<biomcp_types::DrugApprovals>,
    ) -> Self::Output {
        self.wrap(query, body, DrugSectionPart::Approvals)
    }
    fn drug_regulatory(
        self,
        query: DrugGet,
        body: Option<biomcp_types::DrugRegulatory>,
    ) -> Self::Output {
        self.wrap(query, body, DrugSectionPart::Regulatory)
    }
    fn drug_interactions(
        self,
        query: DrugGet,
        body: Option<biomcp_types::DrugInteractions>,
    ) -> Self::Output {
        self.wrap(query, body, DrugSectionPart::Interactions)
    }
}
