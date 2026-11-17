//! Choose one label when several share a drug name, and say which and why.
use super::resolve::Rank;
use biomcp_adapters::openfda_label::{LabelHolder, Record};
use biomcp_types::{DrugHeader, DrugNameMatch};
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
pub(super) fn rank(matched: DrugNameMatch, row: &Record) -> Rank {
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
pub(super) fn generic_name(candidates: &mut [(Rank, DrugHeader)]) {
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
