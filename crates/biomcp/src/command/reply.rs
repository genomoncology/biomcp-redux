//! Encode typed replies through the shared envelope visitor.
use super::{Outcome, ResponseMeta};
use biomcp_types::{Reply, ReplyBody, Section, UsageError};
use std::io;

impl Reply for ResponseMeta {
    type Output = io::Result<Outcome>;
    fn reply<B: ReplyBody>(self, query: B::Query, body: Option<B>) -> Self::Output {
        self.encode(query, body)
    }
    fn usage(self, error: UsageError) -> Self::Output {
        if matches!(
            error,
            UsageError::TrialContinuationRejected | UsageError::TrialContinuationUnavailable
        ) {
            let mut envelope = biomcp_types::ErrorEnvelope::from(error);
            envelope.next = self.next;
            return self.encoding.encode(envelope, crate::exit::Status::Usage);
        }
        super::usage_envelope(error, self.encoding)
    }
    fn pool(self, mismatch: bool) -> Self::Output {
        super::pool_error(mismatch, self.next, self.encoding)
    }

    fn organization_page(
        self,
        query: biomcp_types::OrganizationSearch,
        mut body: Option<biomcp_types::OrganizationPage>,
    ) -> Self::Output {
        if let Some(body) = &mut body {
            for row in &mut body.items {
                let Some(name) = row.name.clone() else {
                    continue;
                };
                row.next = Some(crate::spell::step(
                    biomcp_types::StepKind::Pivot,
                    &biomcp_query::Action::Run {
                        request: biomcp_types::Request::SearchTrial(
                            biomcp_types::TrialSearch::for_nci_facility(name),
                        ),
                        fresh: false,
                    },
                )?);
            }
        }
        self.encode(query, body)
    }
    fn cell_line_card(
        self,
        query: biomcp_types::CellLineGet,
        mut body: Option<biomcp_types::CellLineCard>,
    ) -> Self::Output {
        if let Some(card) = &mut body
            && card.candidates.is_empty()
        {
            use biomcp_types::CellLineSection;
            match query.section() {
                Some(CellLineSection::Names) => {
                    return self.encode(query, data(card.sections.names.take()));
                }
                Some(CellLineSection::Identity) => {
                    return self.encode(query, data(card.sections.identity.take()));
                }
                Some(CellLineSection::Xrefs) => {
                    return self.encode(query, data(card.sections.xrefs.take()));
                }
                Some(CellLineSection::Variants) => {
                    return self.encode(query, data(card.sections.variants.take()));
                }
                Some(CellLineSection::All | CellLineSection::Chembl) | None => {}
            }
        }
        self.encode(query, body)
    }
}
fn data<T>(section: Option<Section<T>>) -> Option<T> {
    match section {
        Some(Section::Data { data, .. }) => Some(data),
        _ => None,
    }
}

/// The inputs whose items are absent beside items that were found, named at the top of the
/// reply so a caller need not scan the items. A reply with every input absent says so in its
/// status, so it names none here.
pub(super) fn absent<B: biomcp_types::ResponseBody>(
    envelope: &biomcp_types::Envelope<B>,
) -> Vec<biomcp_types::Text> {
    use serde_json::Value;
    if matches!(envelope.status, biomcp_types::Status::Absent) {
        return Vec::new();
    }
    let result = envelope
        .result
        .as_ref()
        .and_then(|result| serde_json::to_value(result).ok());
    result
        .as_ref()
        .and_then(|result| result.get("items"))
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter(|item| item.get("status").and_then(Value::as_str) == Some("absent"))
        .filter_map(|item| item.get("input")?.as_str())
        .filter_map(|input| biomcp_types::Text::new(input).ok())
        .collect()
}
