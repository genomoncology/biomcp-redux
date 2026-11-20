//! Choose one label when several share a drug name, and say which and why.
use super::resolve::{NativePage, Rank};
use crate::{Ctx, Outcome, status};
use biodata::{DrugId, Text};
use biomcp_adapters::openfda_label::{self as source, LabelHolder, Record};
use biomcp_fetch::{Leg, RequestError};
use biomcp_types::{DrugHeader, DrugNameMatch, DrugSource, SourceNotice};
use std::cmp::Reverse;

/// The label ordering a name get applies, stated in every label name lookup.
pub(super) const RANKING: &str = "exact brand before generic, unless a matching label carries \
     the name as its generic name; then brand-name NDA/BLA labels, other NDA/BLA labels, then \
     generic or other labels; then the original packager's label before repackaged copies; then \
     most numbered indications; then newest revision";

/// Brand match first, then the label holder, then the original packager's label before a
/// repackager's copy, then more numbered indications; the source's newest-first order breaks
/// ties. A label whose generic name matches ranks as generic even when its brand name matches
/// too; `generic_name` then settles every label's match.
fn rank(matched: DrugNameMatch, row: &Record) -> Rank {
    let choice = row.choice();
    (
        u8::from(matched != DrugNameMatch::Brand),
        choice.holder as u8,
        u8::from(!choice.original),
        Reverse(choice.indications),
        String::new(),
    )
}

/// When some matching label carries the name as its generic name, the name is a generic name,
/// so a label whose brand name repeats it (a generic maker filing as "Metformin") is no exact
/// brand match, and every label is ranked by the tiers alone.
fn generic_name(candidates: &mut [(Rank, DrugHeader)]) {
    if candidates.iter().any(|((matched, ..), _)| *matched != 0) {
        for ((matched, ..), _) in candidates {
            *matched = 1;
        }
    }
}

fn kind(holder: u8) -> &'static str {
    if holder == LabelHolder::Brand as u8 {
        "a brand-name NDA/BLA label"
    } else if holder == LabelHolder::Application as u8 {
        "an NDA/BLA label under the generic name"
    } else {
        "a generic or other label"
    }
}

/// Which label was chosen and why, with how many others of each kind share the name; none
/// when only one label matched.
pub(super) fn reason(candidates: &[(Rank, DrugHeader)]) -> Option<Box<str>> {
    let (((_, holder, copy, Reverse(numbered), _), chosen), others) = candidates.split_first()?;
    if others.is_empty() {
        return None;
    }
    let packager = if *copy == 0 {
        " from its original packager"
    } else {
        " repackaged by another firm"
    };
    let copies = others
        .iter()
        .filter(|((_, _, copy, _, _), _)| *copy != 0)
        .count();
    let listing = if *numbered > 0 {
        format!(" listing {numbered} numbered indications")
    } else {
        String::new()
    };
    let counts: Vec<String> = [
        LabelHolder::Brand,
        LabelHolder::Application,
        LabelHolder::Other,
    ]
    .into_iter()
    .filter_map(|each| {
        let n = others
            .iter()
            .filter(|((_, holder, _, _, _), _)| *holder == each as u8)
            .count();
        let plural = kind(each as u8)
            .split_once(' ')
            .map_or("", |(_, rest)| rest)
            .replace("label", "labels");
        (n > 0).then(|| format!("{n} {plural}"))
    })
    .collect();
    let (s, verb) = if others.len() == 1 {
        ("", "s")
    } else {
        ("s", "")
    };
    let copies = match copies {
        0 => String::new(),
        1 => "; 1 is a repackaged copy".into(),
        n => format!("; {n} are repackaged copies"),
    };
    Some(
        format!(
            "chose {} ({}), {}{packager}{listing}; {} other label{s} share{verb} this name: {}{copies}",
            chosen.name.as_ref().map_or("", |n| n.as_str()),
            chosen.id,
            kind(*holder),
            others.len(),
            counts.join(", "),
        )
        .into(),
    )
}

pub(super) fn candidates(leg: &Leg<'_>, name: &Text) -> Result<NativePage, RequestError> {
    let mut out = NativePage {
        read: None,
        candidates: Vec::new(),
        next_offset: None,
        problems: Vec::new(),
        refusals: biomcp_adapters::Refusal::None,
    };
    for page in source::search_names(leg, name)? {
        out.refusals.include(page.window.refusals());
        out.problems.extend(page.problems);
        for row in &page.window.rows {
            if let Some(matched) = row.exact_name_match(name) {
                out.candidates.push((rank(matched, row), row.header()?));
            }
        }
    }
    generic_name(&mut out.candidates);
    Ok(out)
}

/// A brand found only on repackager copies can use the original packager's label from the
/// same application. Multiple possible applications give no basis for recovering one holder.
pub(super) fn recover(
    cx: &Ctx<'_>,
    candidates: &mut Vec<(Rank, DrugHeader)>,
    discovery: &mut Outcome,
) {
    let Some(application) = repackaged_application(candidates).cloned() else {
        return;
    };
    let command = cx.fetch.command_deadline(cx.budget);
    let leg = cx
        .fetch
        .leg(&source::SPEC, command.deadline())
        .fresh(cx.fresh);
    let found = original_packagers(&leg, &application, &mut discovery.problems);
    let remaining = leg.command_remaining();
    let error = found.as_ref().err().copied();
    let attempt = status::source(DrugSource::OpenfdaLabel.source(), leg.finish(), error);
    command.finish(remaining);
    if let Some(source) = discovery.sources.first_mut() {
        source.notices.extend(attempt.notices);
    }
    match found {
        Ok(mut found) => {
            if !found.is_empty() {
                found.sort_by(|a, b| a.0.cmp(&b.0));
                found.append(candidates);
                *candidates = found;
                if let Some(source) = discovery.sources.first_mut() {
                    source.notices.push(SourceNotice::RequestGuidance {
                        data_date: None,
                        message: format!(
                            "The brand search found repackaged copies; checked application {application} for the original packager's label."
                        )
                        .into(),
                        retry_after_seconds: None,
                    });
                }
            }
        }
        Err(error) => {
            if let Some(source) = discovery.sources.first_mut() {
                source.notices.push(SourceNotice::RequestGuidance {
                    data_date: None,
                    message: format!(
                        "Could not check application {application} for the original packager: {error}."
                    )
                    .into(),
                    retry_after_seconds: None,
                });
            }
        }
    }
}

fn repackaged_application(candidates: &[(Rank, DrugHeader)]) -> Option<&Text> {
    let [application] = candidates.first()?.1.application_numbers.as_ref() else {
        return None;
    };
    if !application
        .as_str()
        .parse::<DrugId>()
        .is_ok_and(|id| DrugSource::Drugsfda.accepts(&id))
    {
        return None;
    }
    candidates
        .iter()
        .all(|((matched, tier, copy, ..), header)| {
            (*matched, *tier, *copy) == (0, LabelHolder::Brand as u8, 1)
                && header.application_numbers.as_ref() == std::slice::from_ref(application)
        })
        .then_some(application)
}

fn original_packagers(
    leg: &Leg<'_>,
    application: &Text,
    problems: &mut Vec<biomcp_types::Problem>,
) -> Result<Vec<(Rank, DrugHeader)>, RequestError> {
    let mut offset = 0;
    let mut originals = Vec::new();
    loop {
        let page = source::search(leg, application, 50, offset).page?;
        problems.extend(page.problems);
        for row in &page.window.rows {
            if row.choice().original {
                let header = row.header()?;
                if header.application_numbers.contains(application) {
                    // The input matched the brand on the copies; the application links the
                    // original, whose own published name may be generic.
                    originals.push((rank(DrugNameMatch::Brand, row), header));
                }
            }
        }
        let Some(next) = page.window.next_offset() else {
            return Ok(originals);
        };
        offset = next;
    }
}
