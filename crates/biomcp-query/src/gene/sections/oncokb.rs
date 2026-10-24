use super::{SectionResult, SectionValue};
use crate::{Ctx, status};
use biodata::{Gene, GeneSection, HgncId, Reason, Section, Source, SourceOutcome, SourceState};
use biomcp_adapters::oncokb;
use biomcp_fetch::{Deadline, RequestError, SecretName};

pub(super) fn run(cx: &Ctx<'_>, deadline: Deadline<'_>, genes: &[(HgncId, Gene)]) -> SectionResult {
    if !cx.fetch.has_key(SecretName::OncokbApiKey) {
        return SectionResult {
            section: GeneSection::Oncokb,
            values: genes.iter().map(|(id, _)| (*id, SectionValue::Oncokb(Section::Skipped { source: Source::Oncokb, reason: Reason::NoKey }, None, false))).collect(),
            source: SourceOutcome { source: Source::Oncokb, state: SourceState::Skipped, reason: Some(Reason::NoKey) },
            problems: Vec::new(), local: None,
        };
    }
    let leg = cx.fetch.leg(&oncokb::SPEC, deadline).fresh(cx.fresh);
    let mut error = None;
    let mut local = None;
    let mut problems = Vec::new();
    let mut values = Vec::new();
    for (id, gene) in genes {
        let result = gene.entrez_id.map(|entrez| oncokb::gene(&leg, &gene.symbol, entrez));
        let (section, card, partial) = match result {
            Some(Ok(mapped)) => {
                let partial = !mapped.problems.is_empty();
                status::append(&mut problems, mapped.problems);
                match mapped.value {
                    Some(data) => (Section::Data { source: Source::Oncokb, data: data.section }, Some(data.card), partial),
                    None if partial => (Section::Unavailable { source: Source::Oncokb, reason: Reason::BadReply }, None, true),
                    None => (Section::Empty { source: Source::Oncokb, record: None }, None, false),
                }
            }
            Some(Err(failure)) => {
                error = Some(failure);
                local = local.or(status::local(failure));
                (Section::Unavailable { source: Source::Oncokb, reason: reason(failure) }, None, true)
            }
            None => (Section::NotApplicable, None, false),
        };
        values.push((*id, SectionValue::Oncokb(section, card, partial)));
    }
    if values.iter().any(|(_, value)| matches!(value, SectionValue::Oncokb(_, Some(_), _))) {
        match oncokb::version(&leg) {
            Ok(version) => for (_, value) in &mut values {
                if let SectionValue::Oncokb(_, Some(card), _) = value { card.version = Some(version.clone()); }
            },
            Err(failure) => {
                error = Some(failure);
                local = local.or(status::local(failure));
                for (_, value) in &mut values {
                    if let SectionValue::Oncokb(_, Some(_), partial) = value { *partial = true; }
                }
            }
        }
    }
    let source = status::source(Source::Oncokb, leg.finish(), error);
    if source.state == SourceState::Stale {
        for (_, value) in &mut values { if let SectionValue::Oncokb(_, Some(_), partial) = value { *partial = true; } }
    }
    SectionResult { section: GeneSection::Oncokb, values, source, problems, local }
}
fn reason(error: RequestError) -> Reason {
    match error { RequestError::Source(error) => error.reason, _ => Reason::Unavailable }
}
