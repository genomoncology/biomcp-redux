use super::raw::{Summary, SummaryRow, SummaryValue};
use crate::{Mapped, raw::Field};
use biodata::{PartialDate, Pmid, Text};

use biomcp_types::{ArticleHeader, PublicationType as P};
use biomcp_types::{Problem, ProblemCode, Source};
/// Preserve selected identities when a summary is absent, invalid or mismatched.
pub(super) fn summaries(reply: Summary, ids: &[Box<str>]) -> Mapped<Vec<ArticleHeader>> {
    let mut problems = Vec::new();
    let mut value = Vec::new();
    for id in ids {
        let mapped = matches!(reply.result.get(id.as_ref()), Some(SummaryValue::Row(_)));
        if !mapped {
            problem(&mut problems, id, "summary");
        }
    }
    for (key, row) in reply.result {
        if key == "uids" {
            continue;
        }
        if !ids.iter().any(|id| id.as_ref() == key) {
            problem(&mut problems, &key, "pmid");
            continue;
        }
        let header = match row {
            SummaryValue::Row(row) => header(*row, &key, &mut problems),
            _ => None,
        };
        if let Some(header) = header {
            value.push(header);
        } else {
            problem(&mut problems, &key, "summary");
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
    let journal = row
        .fulljournalname
        .and_then(Field::value)
        .and_then(|s| Text::new(&s).ok());
    let date = row.pubdate.and_then(Field::value).and_then(|s| date(&s));
    let electronic_date = row
        .epubdate
        .and_then(Field::value)
        .and_then(|s| self::date(&s))
        .filter(|electronic| Some(*electronic) != date);
    let authors = row.authors.and_then(Field::value).unwrap_or_default();
    let author_count = u32::try_from(authors.len()).ok()?;
    let first_author = authors
        .into_iter()
        .next()
        .and_then(Field::value)
        .and_then(|a| Text::new(&a.name).ok());
    let types = row
        .pubtype
        .and_then(Field::value)
        .unwrap_or_default()
        .into_iter()
        .filter_map(Field::value)
        .filter_map(|s| publication_type(&s))
        .collect();
    let mut doi = None;
    let mut pmcid = None;
    for item in row.articleids.and_then(Field::value).unwrap_or_default() {
        let Some(item) = item.value() else {
            problem(problems, key, "articleids");
            continue;
        };
        match item.idtype.as_str() {
            "doi" => {
                doi = item.value.parse().ok();
                if doi.is_none() {
                    problem(problems, key, "doi");
                }
            }
            "pmc" => {
                pmcid = item.value.parse().ok();
                if pmcid.is_none() {
                    problem(problems, key, "pmcid");
                }
            }
            _ => {}
        }
    }
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
fn problem(problems: &mut Vec<Problem>, key: &str, field: &str) {
    let p = Problem {
        source: Source::PubMed,
        record: Text::new(key).ok(),
        field: field.into(),
        code: ProblemCode::ValueUnknown,
    };
    if !problems.contains(&p) {
        problems.push(p);
    }
}
