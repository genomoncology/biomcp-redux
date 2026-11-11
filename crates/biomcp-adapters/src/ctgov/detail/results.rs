//! Posted native values from the already fetched study; no statistical calculations.
use crate::{mapped::Problems, raw::Field};
use biomcp_types::{
    TrialAdverseEvents, TrialBaseline, TrialBaselineMeasure, TrialEvent, TrialEventGroup,
    TrialFlowPeriod, TrialParticipantFlow, TrialPostedOutcome, TrialResultGroup, TrialResultRecord,
};
use serde::{Deserialize, Deserializer, de::DeserializeOwned};
use serde_json::Value;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Results {
    participant_flow_module: Option<Field<Plain<Flow>>>,
    baseline_characteristics_module: Option<Field<Plain<Baseline>>>,
    outcome_measures_module: Option<Field<Plain<Outcomes>>>,
    adverse_events_module: Option<Field<Plain<Events>>>,
}
/// A module whose every string has lost the source's Markdown escapes before it
/// becomes a public value, so a field added to the public types cannot be missed.
struct Plain<T>(T);
impl<'de, T: DeserializeOwned> Deserialize<'de> for Plain<T> {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let mut value = Value::deserialize(d)?;
        unescape(&mut value);
        T::deserialize(value)
            .map(Self)
            .map_err(serde::de::Error::custom)
    }
}
fn unescape(value: &mut Value) {
    match value {
        Value::String(text) => {
            if let std::borrow::Cow::Owned(plain) = crate::source_text::unescape_markdown(text) {
                *text = plain;
            }
        }
        Value::Array(rows) => rows.iter_mut().for_each(unescape),
        Value::Object(fields) => fields.values_mut().for_each(unescape),
        _ => {}
    }
}
#[derive(Deserialize)]
struct Flow {
    #[serde(flatten)]
    scope: TrialParticipantFlow,
    groups: Option<Field<Vec<Field<TrialResultGroup>>>>,
    periods: Option<Field<Vec<Field<TrialFlowPeriod>>>>,
}
#[derive(Deserialize)]
struct Baseline {
    #[serde(flatten)]
    scope: TrialBaseline,
    groups: Option<Field<Vec<Field<TrialResultGroup>>>>,
    measures: Option<Field<Vec<Field<TrialBaselineMeasure>>>>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Outcomes {
    outcome_measures: Option<Field<Vec<Field<TrialPostedOutcome>>>>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Events {
    #[serde(flatten)]
    scope: TrialAdverseEvents,
    event_groups: Option<Field<Vec<Field<TrialEventGroup>>>>,
    serious_events: Option<Field<Vec<Field<TrialEvent>>>>,
    other_events: Option<Field<Vec<Field<TrialEvent>>>>,
}
impl Results {
    pub(super) fn rows(
        self,
        problems: &mut Problems,
        refusals: &mut crate::Refusal,
    ) -> Box<[TrialResultRecord]> {
        let mut out = Rows {
            problems,
            refusals,
            rows: Vec::new(),
        };
        // Outcomes stay first; participant flow and baseline follow them, then adverse events.
        if let Some(Plain(module)) = out.problems.field(
            "resultsSection.outcomeMeasuresModule",
            self.outcome_measures_module,
        ) {
            out.records(
                "outcomeMeasures",
                module.outcome_measures,
                TrialResultRecord::Outcome,
            );
        }
        if let Some(Plain(module)) = out.problems.field(
            "resultsSection.participantFlowModule",
            self.participant_flow_module,
        ) {
            out.scope(module.scope, TrialResultRecord::ParticipantFlow);
            out.records(
                "participantFlowModule.groups",
                module.groups,
                TrialResultRecord::FlowGroup,
            );
            out.records(
                "participantFlowModule.periods",
                module.periods,
                TrialResultRecord::FlowPeriod,
            );
        }
        if let Some(Plain(module)) = out.problems.field(
            "resultsSection.baselineCharacteristicsModule",
            self.baseline_characteristics_module,
        ) {
            out.scope(module.scope, TrialResultRecord::Baseline);
            out.records(
                "baselineCharacteristicsModule.groups",
                module.groups,
                TrialResultRecord::BaselineGroup,
            );
            out.records(
                "baselineCharacteristicsModule.measures",
                module.measures,
                TrialResultRecord::BaselineMeasure,
            );
        }
        if let Some(Plain(module)) = out.problems.field(
            "resultsSection.adverseEventsModule",
            self.adverse_events_module,
        ) {
            out.rows
                .push(TrialResultRecord::AdverseEvents(module.scope));
            out.records(
                "eventGroups",
                module.event_groups,
                TrialResultRecord::AdverseEventGroup,
            );
            out.records(
                "seriousEvents",
                module.serious_events,
                TrialResultRecord::SeriousEvent,
            );
            out.records(
                "otherEvents",
                module.other_events,
                TrialResultRecord::OtherEvent,
            );
        }
        out.rows.into()
    }
}
/// Appends native collections in order, counting each malformed member as a refused record.
struct Rows<'a> {
    problems: &'a mut Problems,
    refusals: &'a mut crate::Refusal,
    rows: Vec<TrialResultRecord>,
}
impl Rows<'_> {
    /// A module with no notes of its own adds no empty record before its rows.
    fn scope<T: Default + PartialEq>(&mut self, scope: T, record: fn(T) -> TrialResultRecord) {
        if scope != T::default() {
            self.rows.push(record(scope));
        }
    }
    fn records<T>(
        &mut self,
        field: &str,
        values: Option<Field<Vec<Field<T>>>>,
        record: fn(T) -> TrialResultRecord,
    ) {
        self.rows.extend(
            super::super::refusals::records(self.problems, self.refusals, field, values)
                .into_vec()
                .into_iter()
                .map(record),
        );
    }
}
