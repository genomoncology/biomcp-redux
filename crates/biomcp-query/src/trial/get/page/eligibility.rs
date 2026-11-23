//! Preserve registry criteria and arm boundaries within the escaped JSON text budget.
use super::{Outcome, Snapshot, advance, continuation, record};
use biodata::Text;
use biomcp_adapters::ctgov::criteria;
use biomcp_types::{
    Section, Source, TrialEligibilityAgeScope, TrialEligibilityArm, TrialEligibilityPage, TrialGet,
    TrialSection,
};

const NOT_SPLIT: &str = concat!(
    "The registry text lacks an Inclusion Criteria or an Exclusion Criteria heading, ",
    "or has another line ending in \"criteria:\", so it is not split; read criteria."
);
pub(super) fn render(
    query: &TrialGet,
    snapshot: &Snapshot,
    pool: Option<&Text>,
    out: &mut Outcome,
) -> Section<TrialEligibilityPage> {
    let id = snapshot.detail.card.nct_id;
    let Some(e) = snapshot.detail.eligibility.as_ref() else {
        return Section::Empty {
            source: Source::ClinicalTrialsGov,
            record: record(id),
        };
    };
    let text = e.criteria.as_ref().map_or("", |s| s.as_str());
    let total = text.chars().count() as u32;
    let splits = criteria::splits(text);
    let start = text
        .char_indices()
        .nth(query.offset() as usize)
        .map_or(text.len(), |(i, _)| i);
    let (fragment, criterion_continues) = fragment(text, start, splits);
    let split = criteria::split_page(text, start, fragment);
    let not_split = (!splits && !text.is_empty()).then(|| NOT_SPLIT.into());
    let criteria = split.is_none().then(|| fragment.into());
    let criteria::Page { unscoped, arms } = split.unwrap_or_default();
    let criteria::Split {
        preamble,
        inclusion,
        exclusion,
    } = unscoped;
    let arms = arms
        .into_iter()
        .map(|arm| TrialEligibilityArm {
            heading: arm.heading,
            preamble: arm.criteria.preamble,
            inclusion: arm.criteria.inclusion,
            exclusion: arm.criteria.exclusion,
        })
        .collect();
    let end = query.offset() + fragment.chars().count() as u32;
    advance(query, id, TrialSection::Eligibility, pool, end, total, out);
    Section::Data {
        source: Source::ClinicalTrialsGov,
        data: TrialEligibilityPage {
            continuation: continuation(query, id, TrialSection::Eligibility, pool, end, total),
            age_scope: (e.minimum_age.is_some() || e.maximum_age.is_some())
                .then_some(TrialEligibilityAgeScope::StudyWide),
            minimum_age: e.minimum_age.clone(),
            maximum_age: e.maximum_age.clone(),
            sex: e.sex,
            healthy_volunteers: e.healthy_volunteers,
            not_split,
            preamble,
            inclusion,
            exclusion,
            arms,
            criteria,
            criterion_continues,
            offset: query.offset(),
            total_characters: total,
        },
    }
}

const BUDGET: usize = 6000;
/// A split page sends each group as a list item: its quotes and comma.
const ITEM: usize = 3;
/// Cover the arm object's structural JSON around its label and criteria lists.
const ARM: usize = 56;

/// `split` counts each line group as the list item the split page sends in its place.
fn fragment(text: &str, start: usize, split: bool) -> (&str, bool) {
    let item = if split { ITEM } else { 0 };
    let remaining = text.split_at(start).1;
    // Recover groups from the original text even when a continuation starts
    // inside a parent line's oversized group.
    let mut lines = text.split_inclusive('\n').peekable();
    let mut group_start = 0;
    let mut end = start;
    let mut bytes = 0;
    let mut active_arm = None;
    while let Some(line) = lines.next() {
        let arm = split.then(|| criteria::arm_heading(line)).flatten();
        if let Some((title, _)) = arm {
            active_arm = Some(title);
        }
        let mut group = line.len();
        let indent = indentation(line);
        while let Some(next) = lines.peek() {
            if (split && criteria::arm_heading(next).is_some())
                || (!next.trim().is_empty() && (arm.is_some() || indentation(next) <= indent))
            {
                break;
            }
            group += next.len();
            lines.next();
        }
        let group_end = group_start + group;
        let segment_start = start.max(group_start);
        group_start = group_end;
        if group_end <= start {
            continue;
        }
        let candidate = text
            .split_at(segment_start)
            .1
            .split_at(group_end - segment_start)
            .0;
        if end == start
            && arm.is_none()
            && let Some(title) = active_arm
        {
            bytes += ARM + biomcp_adapters::jats::escaped_bytes(title);
        }
        let cost = biomcp_adapters::jats::escaped_bytes(candidate)
            + item
            + if arm.is_some() { ARM } else { 0 };
        if bytes + cost > BUDGET {
            if end > start {
                return (remaining.split_at(end - start).0, false);
            }
            return oversized(candidate, BUDGET.saturating_sub(bytes).max(6));
        }
        bytes += cost;
        end = group_end;
    }
    (remaining, false)
}

fn indentation(line: &str) -> usize {
    line.chars()
        .take_while(|ch| matches!(ch, ' ' | '\t'))
        .count()
}

fn oversized(group: &str, budget: usize) -> (&str, bool) {
    let mut end = 0;
    let mut bytes = 0;
    for line in group.split_inclusive('\n') {
        let cost = biomcp_adapters::jats::escaped_bytes(line);
        if bytes + cost > budget {
            if end > 0 {
                return (group.split_at(end).0, true);
            }
            return split_line(line, budget);
        }
        bytes += cost;
        end += line.len();
    }
    (group, false)
}

fn split_line(line: &str, budget: usize) -> (&str, bool) {
    let mut bytes = 0;
    let mut end = 0;
    for (index, ch) in line.char_indices() {
        let cost = biomcp_adapters::jats::escaped_bytes(ch.encode_utf8(&mut [0; 4]));
        if bytes + cost > budget {
            break;
        }
        bytes += cost;
        end = index + ch.len_utf8();
    }
    (line.split_at(end).0, true)
}
