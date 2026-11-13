//! A drug get that names several sections or label headings runs each as the call for that
//! section alone and answers once: the drug's identity once, then one part per section, each
//! with its own status, paging and next calls.
//! A part that is not a section reply, such as a refused name or heading, answers alone.
use super::{Encoding, Outcome, ResponseMeta};
use biomcp_types::{
    DrugApprovals, DrugCivic, DrugGet, DrugHeader, DrugIndications, DrugInteractions, DrugLabel,
    DrugRegulatory, DrugSection, DrugSectionBody, DrugSectionPart, DrugSectionReply, DrugSections,
    DrugTargets, DrugXrefs, Envelope, Reply, ReplyBody, ReplyData, Status, UsageError,
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
    let mut identity = None;
    let mut statuses = Vec::new();
    let mut local = false;
    let requested = query.parts().map_err(io::Error::other)?;
    for (request, result) in super::run_parts(fetch, &requested, fresh, settings)? {
        if !sectioned(&result.body) {
            return super::response(fetch, request, result, fresh, globals, encoding);
        }
        local |= result.local.is_some();
        statuses.push(result.status());
        let (meta, body) = super::finished(fetch, &request, result, fresh, globals, encoding)?;
        parts.push(
            body.visit(Part {
                meta,
                identity: &mut identity,
            })
            .ok_or_else(|| io::Error::other("drug section reply"))?,
        );
    }
    let status = combined(&statuses);
    let exit = if local {
        crate::exit::Status::Local
    } else {
        crate::exit::Status::from(status)
    };
    // A name echoes as the native ID it resolved to, as a single section's reply does.
    let echoed = identity
        .as_ref()
        .filter(|_| query.is_name())
        .and_then(|identity| query.with_native_id(identity.id.clone()).ok())
        .unwrap_or_else(|| query.clone());
    let body = DrugSections {
        identity,
        parts: parts.into(),
    };
    encoding.encode(Envelope::new(status, echoed, Some(body)), exit)
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

/// Build one part from a drug section reply. The first identity found becomes the reply's;
/// a part states its own identity only when it differs.
struct Part<'a> {
    meta: ResponseMeta,
    identity: &'a mut Option<DrugHeader>,
}
impl Part<'_> {
    fn wrap<B, P>(
        self,
        query: DrugGet,
        body: Option<B>,
        variant: fn(Box<DrugSectionReply<P>>) -> DrugSectionPart,
    ) -> Option<DrugSectionPart>
    where
        B: ReplyBody<Query = DrugGet> + DrugSectionBody<Part = P>,
    {
        let envelope = self.meta.envelope(query.clone(), body);
        let (identity, result) = match envelope.result.map(|body| body.split(&query)) {
            Some((identity, part)) => (Some(identity), Some(part)),
            None => (None, None),
        };
        let identity = match (identity, self.identity.as_ref()) {
            (Some(identity), None) => {
                *self.identity = Some(identity);
                None
            }
            (Some(identity), Some(top)) if identity == *top => None,
            (identity, _) => identity,
        };
        Some(variant(Box::new(DrugSectionReply {
            status: envelope.status,
            local_error: envelope.local_error,
            identity,
            result,
            page: envelope.page,
            sources: envelope.sources,
            problems: envelope.problems,
            next: envelope.next,
        })))
    }
}
impl Reply for Part<'_> {
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
            DrugSection::Xrefs => self.wrap::<DrugXrefs, _>(query, None, DrugSectionPart::Xrefs),
            DrugSection::Targets => {
                self.wrap::<DrugTargets, _>(query, None, DrugSectionPart::Targets)
            }
            DrugSection::Indications => {
                self.wrap::<DrugIndications, _>(query, None, DrugSectionPart::Indications)
            }
            DrugSection::Civic => self.wrap::<DrugCivic, _>(query, None, DrugSectionPart::Civic),
            DrugSection::Label => self.wrap::<DrugLabel, _>(query, None, DrugSectionPart::Label),
            DrugSection::Approvals => {
                self.wrap::<DrugApprovals, _>(query, None, DrugSectionPart::Approvals)
            }
            DrugSection::Regulatory => {
                self.wrap::<DrugRegulatory, _>(query, None, DrugSectionPart::Regulatory)
            }
            DrugSection::Interactions => {
                self.wrap::<DrugInteractions, _>(query, None, DrugSectionPart::Interactions)
            }
            // The markets section is its own request; a drug get never carries it.
            DrugSection::Markets => None,
        }
    }
    fn drug_xrefs(self, query: DrugGet, body: Option<DrugXrefs>) -> Self::Output {
        self.wrap(query, body, DrugSectionPart::Xrefs)
    }
    fn drug_targets(self, query: DrugGet, body: Option<DrugTargets>) -> Self::Output {
        self.wrap(query, body, DrugSectionPart::Targets)
    }
    fn drug_indications(self, query: DrugGet, body: Option<DrugIndications>) -> Self::Output {
        self.wrap(query, body, DrugSectionPart::Indications)
    }
    fn drug_civic(self, query: DrugGet, body: Option<DrugCivic>) -> Self::Output {
        self.wrap(query, body, DrugSectionPart::Civic)
    }
    fn drug_label(self, query: DrugGet, body: Option<DrugLabel>) -> Self::Output {
        self.wrap(query, body, DrugSectionPart::Label)
    }
    fn drug_approvals(self, query: DrugGet, body: Option<DrugApprovals>) -> Self::Output {
        self.wrap(query, body, DrugSectionPart::Approvals)
    }
    fn drug_regulatory(self, query: DrugGet, body: Option<DrugRegulatory>) -> Self::Output {
        self.wrap(query, body, DrugSectionPart::Regulatory)
    }
    fn drug_interactions(self, query: DrugGet, body: Option<DrugInteractions>) -> Self::Output {
        self.wrap(query, body, DrugSectionPart::Interactions)
    }
}
