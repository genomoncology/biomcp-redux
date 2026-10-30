use super::{SectionResult, SectionValue};
use crate::{Ctx, status};
use biodata::{Gene, HgncId};

use biomcp_adapters::oncokb;
use biomcp_fetch::{Deadline, SecretName};
use biomcp_types::{Reason, Section, Source, SourceState};

pub(super) fn run(cx: &Ctx<'_>, deadline: Deadline<'_>, genes: &[(HgncId, Gene)]) -> SectionResult {
    if cx.fetch.disabled(Source::Oncokb) {
        return skipped(genes, Reason::Disabled);
    }
    if !cx.fetch.has_key(SecretName::OncokbApiKey) {
        return skipped(genes, Reason::NoKey);
    }
    let leg = cx.fetch.leg(&oncokb::SPEC, deadline).fresh(cx.fresh);
    let mut error = None;
    let mut local = None;
    let mut problems = Vec::new();
    let mut values = Vec::new();
    for (id, gene) in genes {
        let result = gene
            .entrez_id
            .map(|entrez| oncokb::gene(&leg, &gene.symbol, entrez));
        let (section, card, partial) = match result {
            Some(Ok(mapped)) => {
                let partial = !mapped.problems.is_empty();
                status::append(&mut problems, mapped.problems);
                mapped_value(mapped.value, partial)
            }
            Some(Err(failure)) => {
                error = Some(failure);
                local = local.or(status::local(failure));
                (
                    Section::unavailable(
                        Source::Oncokb,
                        crate::status::reason(failure).unwrap_or(biomcp_types::Reason::Unavailable),
                    ),
                    None,
                    true,
                )
            }
            None => (Section::NotApplicable, None, false),
        };
        values.push((*id, SectionValue::Oncokb(section, card, partial)));
    }
    if values
        .iter()
        .any(|(_, value)| matches!(value, SectionValue::Oncokb(_, Some(_), _)))
    {
        match oncokb::version(&leg) {
            Ok(version) => {
                let partial_version = !version.problems.is_empty();
                status::append(&mut problems, version.problems);
                apply_version(&mut values, version.value.as_ref(), partial_version);
            }
            Err(failure) => {
                error = Some(failure);
                local = local.or(status::local(failure));
                for (_, value) in &mut values {
                    if let SectionValue::Oncokb(_, Some(_), partial) = value {
                        *partial = true;
                    }
                }
            }
        }
    }
    let source = status::source(Source::Oncokb, leg.finish(), error);
    if source.state == SourceState::Stale {
        for (_, value) in &mut values {
            if let SectionValue::Oncokb(_, Some(_), partial) = value {
                *partial = true;
            }
        }
    }
    SectionResult {
        values,
        source,
        problems,
        local,
    }
}

fn skipped(genes: &[(HgncId, Gene)], reason: Reason) -> SectionResult {
    SectionResult {
        values: genes
            .iter()
            .map(|(id, _)| {
                (
                    *id,
                    SectionValue::Oncokb(
                        Section::Skipped {
                            source: Source::Oncokb,
                            reason,
                        },
                        None,
                        false,
                    ),
                )
            })
            .collect(),
        source: biomcp_adapters::registry::outcome(
            Source::Oncokb,
            SourceState::Skipped,
            Some(reason),
        ),
        problems: Vec::new(),
        local: None,
    }
}
fn mapped_value(
    data: Option<oncokb::GeneData>,
    partial: bool,
) -> (
    Section<biomcp_types::Oncokb>,
    Option<biomcp_types::OncokbCard>,
    bool,
) {
    match data {
        Some(data) => (
            Section::Data {
                source: Source::Oncokb,
                data: data.section,
            },
            Some(data.card),
            partial,
        ),
        None if partial => (
            Section::unavailable(Source::Oncokb, Reason::BadReply),
            None,
            true,
        ),
        None => (
            Section::Empty {
                source: Source::Oncokb,
                record: None,
            },
            None,
            false,
        ),
    }
}

fn apply_version(
    values: &mut [(HgncId, SectionValue)],
    version: Option<&biodata::Text>,
    partial_version: bool,
) {
    for (_, value) in values {
        if let SectionValue::Oncokb(section, Some(card), partial) = value {
            card.version = version.cloned();
            *partial |= partial_version;
            if let Section::Data { data, .. } = section {
                data.version = version.cloned();
            }
        }
    }
}
