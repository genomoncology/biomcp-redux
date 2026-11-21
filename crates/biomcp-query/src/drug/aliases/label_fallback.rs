//! Verify a label against the brand or the drug named by a source-stated research code.
use super::{
    alias_lookup::Stated,
    name_evidence::Evidence,
    named_get::{answered_alias, notice, retry_name},
};
use crate::drug::{
    label_application,
    resolve::{self, lookup_deadline, within},
};
use crate::{Ctx, Outcome};
use biodata::{DrugId, Text};
use biomcp_types::{
    DrugGet, DrugNameMatch, DrugSearch, DrugSource, Request, Source, SourceState, StepKind,
};

/// Keep the brand lookup and any research-code evidence shared across alias attempts.
#[derive(Default)]
pub(super) struct LabelFallback {
    first_candidate: Option<Text>,
    research_names: Vec<Text>,
}

impl LabelFallback {
    pub(super) fn read(
        &mut self,
        cx: &Ctx<'_>,
        queries: &[DrugGet],
        stated: &Stated,
        name: &Text,
        evidence: &mut Evidence,
    ) -> Option<Vec<Outcome>> {
        let input = queries.first()?.input();
        let deadline = lookup_deadline(cx);
        if self.first_candidate.is_none() {
            self.first_candidate = Some(name.clone());
            let original = applications(cx, input, true, evidence)?;
            match original.as_slice() {
                [application] => {
                    return read_brand(
                        &within(cx, deadline.remaining()),
                        queries,
                        input,
                        application,
                        evidence,
                    );
                }
                [] => {
                    self.research_names =
                        research_names(&within(cx, deadline.remaining()), input, evidence);
                }
                _ => return None,
            }
        }
        if !self
            .research_names
            .iter()
            .any(|proved| DrugNameMatch::same_name(proved.as_str(), name.as_str()))
        {
            return None;
        }
        let apps = applications(&within(cx, deadline.remaining()), name, false, evidence)?;
        if apps.is_empty() {
            return None;
        }
        let found = retry_name(
            &within(cx, deadline.remaining()),
            queries,
            stated,
            name,
            evidence,
        )?;
        observe(&found, evidence);
        let apps: Vec<String> = apps.iter().map(ToString::to_string).collect();
        found
            .iter()
            .all(|out| {
                apps.iter()
                    .any(|app| label_application::verified_label(out, app))
            })
            .then_some(found)
    }

    pub(super) fn offer_candidate(&self, out: &mut Outcome, input: &Text, fresh: bool) {
        let Some(name) = &self.first_candidate else {
            return;
        };
        notice(
            out,
            DrugSource::OpenfdaLabel,
            format!(
                "openFDA did not resolve {input:?} by name. No label could be verified for its application. The name {name:?} may find candidates.",
                input = input.as_str(),
                name = name.as_str(),
            ),
        );
        if let Some(search) = crate::next::checked(
            DrugSearch::named(name.clone(), DrugSource::OpenfdaLabel, 10),
            Source::OpenfdaLabel,
            &mut out.problems,
        ) {
            out.next.push(crate::next::Step::run(
                StepKind::Disambiguate,
                &[DrugSource::OpenfdaLabel.source()],
                Request::SearchDrug(search),
                fresh,
            ));
        }
    }
}

fn applications(
    cx: &Ctx<'_>,
    name: &Text,
    brand_only: bool,
    evidence: &mut Evidence,
) -> Option<Vec<DrugId>> {
    let search = crate::next::checked(
        DrugSearch::named(name.clone(), DrugSource::Drugsfda, 50),
        Source::Drugsfda,
        &mut evidence.problems,
    )?;
    let (lookup, ids) = resolve::applications(cx, name, &search, brand_only);
    evidence.observe(&lookup, DrugSource::Drugsfda, false);
    ids
}

fn research_names(cx: &Ctx<'_>, input: &Text, evidence: &mut Evidence) -> Vec<Text> {
    if cx.fetch.disabled(Source::Chembl) || cx.fetch.disabled(Source::Mychem) {
        return Vec::new();
    }
    let copy = crate::drug::search_names::chembl_copy(cx, input);
    let current = copy
        .evidence
        .iter()
        .any(|source| source.source == Source::Mychem && source.state == SourceState::Ok);
    evidence.observe_sources(copy.evidence, DrugSource::Mychem, copy.molecules.is_none());
    crate::status::append(&mut evidence.problems, copy.problems);
    evidence.local = evidence.local.or(copy.error.and_then(crate::status::local));
    match copy.molecules.as_deref() {
        Some([record]) if current && record.research_code => record.names.clone(),
        _ => Vec::new(),
    }
}

fn read_brand(
    cx: &Ctx<'_>,
    queries: &[DrugGet],
    input: &Text,
    application: &DrugId,
    evidence: &mut Evidence,
) -> Option<Vec<Outcome>> {
    let (mut found, verified) =
        label_application::fallback(cx, queries, application, &mut evidence.problems)?;
    let source = DrugSource::OpenfdaLabel;
    if !verified {
        observe(&found, evidence);
        return None;
    }
    if !answered_alias(&found, source, evidence) {
        return None;
    }
    observe(&found, evidence);
    for out in &mut found {
        notice(
            out,
            source,
            format!(
                "openFDA did not resolve {input:?} by name. Drugs@FDA lists that brand under {application}; looked for its openFDA label by application.",
                input = input.as_str(),
            ),
        );
    }
    Some(found)
}

fn observe(found: &[Outcome], evidence: &mut Evidence) {
    for part in found {
        evidence.observe(part, DrugSource::OpenfdaLabel, false);
    }
}
