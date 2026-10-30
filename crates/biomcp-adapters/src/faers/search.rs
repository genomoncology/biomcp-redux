use super::{SPEC, raw};
use biodata::PartialDate;
use biomcp_fetch::{Arg, BadReply, Freshness, Leg, ReplyMetadata, RequestError};
use biomcp_types::{
    AdverseEventHeader, AdverseEventSearch, AdverseEventSerious, AdverseEventSource,
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
    /// Native report count, never a population denominator.
    pub total: u64,
    /// Publisher dataset date, independent of retrieval.
    pub updated: Option<PartialDate>,
    /// Original received record metadata.
    pub metadata: ReplyMetadata,
}
fn literal(value: &str) -> String {
    format!("\"{}\"", value.replace('\\', "\\\\").replace('"', "\\\""))
}
/// Exact literal source expression with analyzed reaction phrase-match semantics.
pub fn expression(query: &AdverseEventSearch) -> String {
    let text = literal(query.text().map_or("", biodata::Text::as_str));
    let mut selector = format!(
        "(patient.drug.openfda.generic_name:{text} OR patient.drug.openfda.brand_name:{text} OR patient.drug.medicinalproduct:{text})"
    );
    if let Some(reaction) = query.reaction() {
        selector.push_str(&format!(
            " AND patient.reaction.reactionmeddrapt:{}",
            literal(reaction.as_str())
        ));
    }
    if let Some(flag) = query.serious() {
        let field = match flag {
            AdverseEventSerious::Any => "serious",
            AdverseEventSerious::Death => "seriousnessdeath",
            AdverseEventSerious::Hospitalization => "seriousnesshospitalization",
            AdverseEventSerious::Lifethreatening => "seriousnesslifethreatening",
            AdverseEventSerious::Disabling => "seriousnessdisabling",
            AdverseEventSerious::CongenitalAnomaly => "seriousnesscongenitalanomali",
            AdverseEventSerious::Other => "seriousnessother",
        };
        selector.push_str(&format!(" AND {field}:1"));
    }
    if query.received_from().is_some() || query.received_to().is_some() {
        let from = query
            .received_from()
            .map_or_else(|| "*".into(), |v| v.to_string().replace('-', ""));
        let to = query
            .received_to()
            .map_or_else(|| "*".into(), |v| v.to_string().replace('-', ""));
        selector.push_str(&format!(" AND receivedate:[{from} TO {to}]"));
    }
    selector
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
    let endpoint = SPEC.get("").fresh(Freshness::Days(1)).json_absence();
    let (response, metadata) = leg.json_received_checked::<raw::Response>(
        &endpoint,
        &[
            ("search", Arg::Text(&selector)),
            ("limit", Arg::Num(query.limit().into())),
            ("skip", Arg::Num(query.offset().into())),
            ("sort", Arg::Text("receivedate:desc")),
        ],
        |reply| {
            if let raw::Response::Page { meta, .. } = reply
                && (meta.results.skip != query.offset() || meta.results.limit != query.limit())
            {
                return Err(BadReply);
            }
            Ok(())
        },
    )?;
    let mut problems = crate::mapped::Problems {
        source: biomcp_types::Source::Faers,
        record: None,
        list: Vec::new(),
    };
    match response {
        raw::Response::Page { meta, results } => Ok(Page {
            refusals: crate::Refusal::None,
            components,
            items: results
                .into_iter()
                .map(|v| {
                    problems.record = crate::mapped::record(&v.safetyreportid);
                    AdverseEventHeader {
                        id: v.safetyreportid,
                        source: AdverseEventSource::Faers,
                        version: problems.field("safetyreportversion", v.safetyreportversion),
                        serious: problems.field("serious", v.serious),
                        event_type: None,
                        first_received: problems.field("receivedate", v.receivedate),
                    }
                })
                .collect(),
            total: meta.results.total,
            updated: meta.last_updated,
            problems: problems.list,
            metadata,
        }),
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
