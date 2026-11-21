//! Verify a brand's application before trying its label under another name.
use super::{
    name_evidence::Evidence,
    named_get::{answered_alias, notice},
};
use crate::drug::{
    label_application,
    resolve::{self, lookup_deadline, within},
};
use crate::{Ctx, Outcome};
use biodata::Text;
use biomcp_types::{DrugGet, DrugSearch, DrugSource, Request, Source, StepKind};

pub(super) fn label_fallback(
    cx: &Ctx<'_>,
    queries: &[DrugGet],
    input: &Text,
    evidence: &mut Evidence,
) -> Option<Vec<Outcome>> {
    let search = crate::next::checked(
        DrugSearch::named(input.clone(), DrugSource::Drugsfda, 50),
        Source::Drugsfda,
        &mut evidence.problems,
    )?;
    let deadline = lookup_deadline(cx);
    let (lookup, application) = resolve::brand_application(cx, input, &search);
    evidence.observe(&lookup, DrugSource::Drugsfda, false);
    let application = application?;
    let source = DrugSource::OpenfdaLabel;
    let (mut found, verified) = label_application::fallback(
        &within(cx, deadline.remaining()),
        queries,
        &application,
        &mut evidence.problems,
    )?;
    if !verified {
        for part in &found {
            evidence.observe(part, source, false);
        }
        return None;
    }
    if !answered_alias(&found, source, evidence) {
        return None;
    }
    for out in &mut found {
        evidence.observe(out, source, false);
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

pub(super) fn label_candidates(out: &mut Outcome, input: &Text, name: &Text, fresh: bool) {
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
