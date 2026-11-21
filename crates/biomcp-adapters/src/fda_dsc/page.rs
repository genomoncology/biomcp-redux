//! FDA's index table and one communication page, read only where FDA marks them.
use super::html::{Date, attribute, date, visible};
use biodata::Text;
use biomcp_types::ProblemCode;

/// One row of FDA's index table.
#[derive(Debug, Clone)]
pub(super) struct Listed {
    /// FDA's page identifier (`data-entity-uuid`).
    pub id: Text,
    pub title: Text,
    /// The Date cell is absent, a valid date, or a problem to report for this row.
    pub date: Date,
    /// The page's path on www.fda.gov.
    pub path: String,
    /// The page's address.
    pub url: Text,
}

/// What one communication page states.
#[derive(Debug, Default)]
pub(super) struct Page {
    pub issued: Date,
    pub updated: Date,
    pub applications: Vec<Text>,
    pub drugs: Vec<Text>,
    pub text: String,
}

const TABLE: &str = "summary=\"Drug Safety Communications\"";
const MAIN: &str = "<div class=\"col-md-8\" role=\"main\">";
const CURRENT: &str = "node-current-date";
const DRUGS_AT_FDA: &str = "accessdata.fda.gov/scripts/cder/daf/";

/// Every row of the index table, in FDA's order; `None` when the table is not there.
pub(super) fn index(body: &str) -> Option<Vec<Listed>> {
    let start = body.find(TABLE)?;
    let table = body.get(start..)?;
    let table = table.get(..table.find("</table>")?)?;
    let mut rows = Vec::new();
    for row in table.split("<tr").skip(1) {
        let cells: Vec<&str> = row.split("<td").skip(1).collect();
        let [when, what, ..] = cells.as_slice() else {
            continue;
        };
        let Some(open) = what.find("<a ") else {
            continue;
        };
        let anchor = what.get(open..).unwrap_or_default();
        let Some(tag) = anchor.get(..anchor.find('>').unwrap_or(0) + 1) else {
            continue;
        };
        let (Some(path), Some(id)) = (attribute(tag, "href"), attribute(tag, "data-entity-uuid"))
        else {
            continue;
        };
        let inner = anchor.get(tag.len()..).unwrap_or_default();
        let title = visible(
            inner
                .get(..inner.find("</a>").unwrap_or(inner.len()))
                .unwrap_or_default(),
        );
        let date_text = visible(&format!("<td{when}"));
        let (Ok(id), Ok(title), Ok(url)) = (
            Text::from_source(id),
            Text::from_source(&title),
            Text::from_source(&format!("{}{path}", super::SITE)),
        ) else {
            continue;
        };
        rows.push(Listed {
            id,
            date: date(&date_text)
                .or((!date_text.is_empty()).then_some(Err(ProblemCode::ValueUnknown))),
            title,
            path: path.to_owned(),
            url,
        });
    }
    Some(rows)
}

/// The page's own facts; `None` when its main content is not where FDA puts it.
pub(super) fn page(body: &str) -> Option<Page> {
    let start = body.find(MAIN)? + MAIN.len();
    let main = body.get(start..)?;
    let main = main.get(..main.find(CURRENT).unwrap_or(main.len()))?;
    let words = visible(main);
    let mut out = Page {
        issued: issued(&words),
        updated: updated(body),
        ..Page::default()
    };
    for anchor in main.split("<a ").skip(1) {
        let tag = anchor
            .get(..anchor.find('>').unwrap_or(0))
            .unwrap_or_default();
        let spaced = format!(" {tag}");
        let Some(href) = attribute(&spaced, "href").filter(|h| h.contains(DRUGS_AT_FDA)) else {
            continue;
        };
        let Some((_, number)) = href.split_once("ApplNo=") else {
            continue;
        };
        let number: String = number.chars().take_while(char::is_ascii_digit).collect();
        if number.is_empty() {
            continue;
        }
        let inner = anchor.get(tag.len() + 1..).unwrap_or_default();
        let name = visible(
            inner
                .get(..inner.find("</a>").unwrap_or(0))
                .unwrap_or_default(),
        );
        // An empty link text names no drug; the application number still counts.
        for (value, list) in [(number, &mut out.applications), (name, &mut out.drugs)] {
            match Text::from_source(&value) {
                Ok(value) if !list.contains(&value) => list.push(value),
                Ok(_) | Err(_) => continue,
            }
        }
    }
    out.text = words;
    Some(out)
}

/// The date written just before the words "FDA Drug Safety Communication" at the page's start.
fn issued(text: &str) -> Date {
    let mut invalid = None;
    for (at, _) in text.match_indices("FDA Drug Safety Communication") {
        let candidate = text
            .get(..at)
            .and_then(|before| before.trim_end().rsplit(' ').next());
        match candidate.and_then(|word| date(word.trim_matches(['[', ']', '(', ')']))) {
            Some(Ok(day)) => return Some(Ok(day)),
            Some(Err(problem)) => invalid = Some(Err(problem)),
            None => {}
        }
    }
    invalid
}

/// The "Content current as of" date from its machine-readable `<time datetime>`.
fn updated(body: &str) -> Date {
    let block = body.get(body.find(CURRENT)?..)?;
    let tag = block.get(block.find("<time")?..)?;
    let value = attribute(tag.get(..tag.find('>')?)?, "datetime")?;
    updated_date(value).or(Some(Err(ProblemCode::ValueUnknown)))
}

fn updated_date(value: &str) -> Date {
    let day = value.get(..10)?;
    let mut parts = day.split('-');
    let (year, month, day) = (parts.next()?, parts.next()?, parts.next()?);
    date(&format!("{month}-{day}-{year}"))
}
