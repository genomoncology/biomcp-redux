use super::{SPEC, raw};
use biodata::PartialDate;
use biomcp_fetch::{Arg, BadReply, Freshness, Leg, Received, ReplyMetadata, RequestError};
use biomcp_types::{
    AdverseEventDrugRole, AdverseEventHeader, AdverseEventSearch, AdverseEventSource,
};
/// One native ordered report page; no drug/reaction-array join.
#[derive(Debug)]
pub struct Page {
    /// Actual refused native records, independent of optional fields.
    pub refusals: crate::Refusal,
    /// Rejected optional source metadata.
    pub problems: Vec<biomcp_types::Problem>,
    /// Applicable filtered terminology terms, independent of service terms.
    pub components: Box<[biomcp_types::ComponentLabel]>,
    /// Whole report headers in source order.
    pub items: Vec<AdverseEventHeader>,
    /// Matching reports, never a population denominator.
    pub total: u64,
    /// Publisher dataset date, independent of retrieval.
    pub updated: Option<PartialDate>,
    /// Original received record metadata.
    pub metadata: ReplyMetadata,
}
/// Exact literal source expression with analyzed reaction phrase-match semantics.
pub fn expression(query: &AdverseEventSearch) -> String {
    query.faers_expression()
}
/// Read one checked native page; component policy applies before cache, replay or HTTP.
pub fn search(leg: &Leg<'_>, query: &AdverseEventSearch) -> Result<Page, RequestError> {
    if query.source() != AdverseEventSource::Faers {
        return Err(biomcp_fetch::InputError::Serialization.into());
    }
    let selector = expression(query);
    if query.reaction().is_some() {
        crate::adverse_event_sources::components::meddra(leg)?;
    }
    let components = if query.reaction().is_some() {
        crate::adverse_event_sources::components::labels(
            biomcp_types::Source::Faers,
            biomcp_types::Source::Meddra,
        )?
    } else {
        Box::default()
    };
    let Received {
        value,
        meta: metadata,
    } = if query.drug_role() == AdverseEventDrugRole::Suspect {
        super::suspect::search(leg, query, &selector)?
    } else {
        fetch(leg, &selector, query.offset(), query.limit())?
    };
    let mut problems = crate::mapped::Problems {
        source: biomcp_types::Source::Faers,
        record: None,
        list: Vec::new(),
    };
    match value {
        raw::Response::Page { meta, results } => {
            let items = results
                .into_iter()
                .map(|v| header(v, &mut problems))
                .collect();
            Ok(Page {
                refusals: crate::Refusal::None,
                components,
                items,
                total: meta.results.total,
                updated: meta.last_updated,
                problems: problems.list,
                metadata,
            })
        }
        raw::Response::Absent { .. } => Ok(Page {
            refusals: crate::Refusal::None,
            components,
            items: Vec::new(),
            total: 0,
            updated: None,
            problems: Vec::new(),
            metadata,
        }),
    }
}
pub(super) fn fetch<T: crate::openfda::Item>(
    leg: &Leg<'_>,
    selector: &str,
    offset: u32,
    limit: u32,
) -> Result<Received<raw::Response<T>>, RequestError> {
    let endpoint = SPEC.get("").fresh(Freshness::Days(1)).json_absence();
    crate::openfda::page(
        biomcp_types::Source::Faers,
        leg,
        &endpoint,
        &[
            ("search", Arg::Text(selector)),
            ("limit", Arg::Num(limit.into())),
            ("skip", Arg::Num(offset.into())),
            ("sort", Arg::Text("receivedate:desc")),
        ],
        offset,
        limit,
        &|reply| {
            if let raw::Response::Page { meta, .. } = reply
                && (meta.results.skip != offset || meta.results.limit != limit)
            {
                return Err(BadReply);
            }
            Ok(())
        },
    )
}

/// One report row, with openFDA's words beside its `serious` code.
fn header(v: raw::Header, problems: &mut crate::mapped::Problems) -> AdverseEventHeader {
    problems.record = crate::mapped::record(&v.safetyreportid);
    let serious = problems.field("serious", v.serious);
    AdverseEventHeader {
        id: v.safetyreportid,
        source: AdverseEventSource::Faers,
        version: problems.field("safetyreportversion", v.safetyreportversion),
        serious_label: super::words::label(super::words::SERIOUS_SHORT, serious.as_deref()),
        serious,
        event_type: None,
        first_received: problems.field("receivedate", v.receivedate),
    }
}
