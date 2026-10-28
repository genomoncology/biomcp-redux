use super::raw::{Summary, SummaryRow, SummaryValue};
use crate::mapped::record as record_text;
use crate::mapped::{Problems, field_text, push_problem};
use crate::{Mapped, raw::Field};
use biodata::{PartialDate, Pmid, Text};

use biomcp_types::{ArticleHeader, PublicationType as P};
use biomcp_types::{Problem, Source};
/// Preserve selected identities when a summary is absent, invalid or mismatched.
pub(super) fn summaries(reply: Summary, ids: &[Box<str>]) -> Mapped<Vec<ArticleHeader>> {
    let mut problems = Vec::new();
    let mut value = Vec::new();
    for id in ids {
        let mapped = matches!(reply.result.get(id.as_ref()), Some(SummaryValue::Row(_)));
        if !mapped {
            push_problem(&mut problems, Source::PubMed, Text::new(id).ok(), "summary");
        }
    }
    for (key, row) in reply.result {
        if key == "uids" {
            continue;
        }
        if !ids.iter().any(|id| id.as_ref() == key) {
            push_problem(&mut problems, Source::PubMed, Text::new(&key).ok(), "pmid");
            continue;
        }
        let header = match row {
            SummaryValue::Row(row) => header(*row, &key, &mut problems),
            _ => None,
        };
        if let Some(header) = header {
            value.push(header);
        } else {
            push_problem(
                &mut problems,
                Source::PubMed,
                Text::new(&key).ok(),
                "summary",
            );
        }
    }
    Mapped { value, problems }
}
fn header(row: SummaryRow, key: &str, problems: &mut Vec<Problem>) -> Option<ArticleHeader> {
    if row.error.is_some() {
        return None;
    }
    let id = row.uid.and_then(Field::value)?;
    if key.parse::<Pmid>().ok() != Some(id) {
        return None;
    }
    let title = Text::new(&row.title.and_then(Field::value)?).ok()?;
    let journal = field_text(
        problems,
        Source::PubMed,
        record_text(key),
        "journal",
        row.fulljournalname,
    );
    let mut context = Problems {
        source: Source::PubMed,
        record: record_text(key),
        list: Vec::new(),
    };
    let date = optional_date(row.pubdate, "date", &mut context);
    let electronic_date = optional_date(row.epubdate, "electronic_date", &mut context)
        .filter(|electronic| Some(*electronic) != date);
    let authors = row
        .authors
        .and_then(|v| context.required("authors", v.value()))
        .unwrap_or_default();
    let author_count = u32::try_from(authors.len()).ok()?;
    let first_author = authors
        .into_iter()
        .next()
        .and_then(|v| context.required("first_author", v.value()))
        .and_then(|a| context.text("first_author", Some(&a.name)));
    let types = row
        .pubtype
        .and_then(|v| context.required("types", v.value()))
        .unwrap_or_default()
        .into_iter()
        .filter_map(|v| context.required("types", v.value()))
        .filter_map(|s| publication_type(&s))
        .collect();
    let mut doi = None;
    let mut pmcid = None;
    for item in row
        .articleids
        .and_then(|v| context.required("articleids", v.value()))
        .unwrap_or_default()
    {
        let Some(item) = item.value() else {
            push_problem(problems, Source::PubMed, record_text(key), "articleids");
            continue;
        };
        match item.idtype.as_str() {
            "doi" => {
                doi = context.optional("doi", Some(item.value.parse()));
            }
            "pmc" => {
                pmcid = context.optional("pmcid", Some(item.value.parse()));
            }
            _ => {}
        }
    }
    problems.extend(context.list);
    Some(ArticleHeader {
        pmid: Some(id),
        europepmc_id: None,
        doi,
        pmcid,
        title,
        journal,
        date,
        electronic_date,
        types,
        types_unknown: false,
        first_author,
        first_author_id: None,
        author_count,
    })
}
fn date(s: &str) -> Option<PartialDate> {
    let mut words = s.split_whitespace();
    let year = words.next()?.parse().ok()?;
    let Some(month) = words.next() else {
        return PartialDate::new(year, None, None).ok();
    };
    let month = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ]
    .iter()
    .position(|m| *m == month)
    .and_then(|m| u8::try_from(m + 1).ok());
    let day = words.next().and_then(|d| d.parse().ok());
    PartialDate::new(year, month, if month.is_some() { day } else { None }).ok()
}
pub(crate) fn publication_type(s: &str) -> Option<P> {
    Some(match s {
        "Randomized Controlled Trial" => P::RandomizedControlledTrial,
        "Clinical Trial" => P::ClinicalTrial,
        "Clinical Trial, Phase I" => P::ClinicalTrialPhase1,
        "Clinical Trial, Phase II" => P::ClinicalTrialPhase2,
        "Clinical Trial, Phase III" => P::ClinicalTrialPhase3,
        "Clinical Trial, Phase IV" => P::ClinicalTrialPhase4,
        "Meta-Analysis" => P::MetaAnalysis,
        "Systematic Review" => P::SystematicReview,
        "Review" => P::Review,
        "Practice Guideline" => P::PracticeGuideline,
        "Guideline" => P::Guideline,
        "Case Reports" => P::CaseReport,
        "Observational Study" => P::ObservationalStudy,
        "Multicenter Study" => P::MulticenterStudy,
        "Letter" => P::Letter,
        "Comment" => P::Comment,
        "Editorial" => P::Editorial,
        "Preprint" => P::Preprint,
        "Retracted Publication" => P::RetractedPublication,
        "Retraction of Publication" => P::RetractionNotice,
        "Published Erratum" => P::Erratum,
        _ => return None,
    })
}

fn optional_date(
    value: Option<Field<String>>,
    field: &str,
    problems: &mut Problems,
) -> Option<PartialDate> {
    problems.required(field, value?.value().and_then(|v| date(&v)))
}
