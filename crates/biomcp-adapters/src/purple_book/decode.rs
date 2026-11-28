//! Decode the FDA report's full roster and preserve each product number separately.
use biodata::DrugId;
use biomcp_fetch::{BadReply, KeyKind, Keys, Row};
use biomcp_types::{PurpleBookLicense, PurpleBookPresentation};
use csv::{ReaderBuilder, StringRecord, Trim};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet, HashSet};

const TITLE: &str = "Purple Book Monthly Historical Data Changes Report - ";
pub(super) const MONTHS: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

/// Chronological key and published human-readable release month.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(super) struct ReleaseMonth {
    year: u16,
    month: usize,
    name: &'static str,
}
impl ReleaseMonth {
    fn from_title(title: &str) -> Result<Self, BadReply> {
        let (month, year) = title
            .strip_prefix(TITLE)
            .and_then(|value| value.split_once(' '))
            .ok_or(BadReply)?;
        let month = MONTHS
            .iter()
            .position(|name| *name == month)
            .ok_or(BadReply)?
            + 1;
        Self::new(year, month)
    }
    pub(super) fn from_key(key: &str) -> Result<Self, BadReply> {
        let (year, month) = key.split_once('-').ok_or(BadReply)?;
        Self::new(year, month.parse().map_err(|_| BadReply)?)
    }
    fn new(year: &str, month: usize) -> Result<Self, BadReply> {
        let year: u16 = year.parse().map_err(|_| BadReply)?;
        let name = MONTHS
            .get(month.checked_sub(1).ok_or(BadReply)?)
            .ok_or(BadReply)?;
        if !(1900..=9999).contains(&year) {
            return Err(BadReply);
        }
        Ok(Self { year, month, name })
    }
    fn key(&self) -> String {
        format!("{:04}-{:02}", self.year, self.month)
    }
    pub(super) fn label(&self) -> Box<str> {
        format!("{} {}", self.name, self.year).into()
    }
    pub(super) fn age_in_months(&self, (year, month): (i64, i64)) -> i64 {
        ((year - i64::from(self.year)) * 12 + month - self.month as i64).max(0)
    }
}

#[derive(Debug, Deserialize)]
struct Native {
    #[serde(rename = "BLA Number")]
    bla: Box<str>,
    #[serde(rename = "Product Number")]
    product_number: Box<str>,
    #[serde(rename = "Proprietary Name")]
    name: Box<str>,
    #[serde(rename = "Proper Name")]
    proper_name: Box<str>,
    #[serde(rename = "Applicant")]
    applicant: Option<Box<str>>,
    #[serde(rename = "License Type", alias = "BLA Type")]
    license_type: Box<str>,
    #[serde(rename = "Ref. Product Proper Name")]
    reference_proper_name: Box<str>,
    #[serde(rename = "Ref. Product Proprietary Name")]
    reference_name: Box<str>,
    #[serde(rename = "Strength")]
    strength: Option<Box<str>>,
    #[serde(rename = "Product Presentation")]
    device: Option<Box<str>>,
    #[serde(rename = "Route of Administration")]
    route: Option<Box<str>>,
    #[serde(rename = "Approval Date")]
    approval_date: Option<Box<str>>,
    #[serde(rename = "Inter. Approval Date")]
    interchangeability_date: Option<Box<str>>,
    #[serde(rename = "First Interchangeable Exclusivity Exp. Date")]
    first_interchangeable_exclusivity_expiration: Option<Box<str>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct Stored {
    date: Box<str>,
    pub(super) row: PurpleBookPresentation,
}
impl Stored {
    fn search_keys(&self) -> impl Iterator<Item = &str> {
        let row = &self.row;
        [
            row.name.as_ref(),
            row.proper_name.as_ref(),
            row.bla.as_str(),
            row.reference_name.as_ref(),
            row.reference_proper_name.as_ref(),
        ]
        .into_iter()
        .chain(
            row.reference_bla
                .iter()
                .chain(row.reference_bla_candidates.iter())
                .map(DrugId::as_str),
        )
        .filter(|key| !key.is_empty())
    }
    pub(super) fn matches(&self, query: &str) -> bool {
        self.search_keys()
            .any(|key| key.eq_ignore_ascii_case(query))
    }
}
impl Row for Stored {
    fn keys(&self) -> Keys {
        Keys {
            exact: self
                .search_keys()
                .map(str::to_ascii_lowercase)
                .collect::<BTreeSet<_>>()
                .into_iter()
                .map(|key| (KeyKind::Alias, key.into()))
                .collect(),
            words: Vec::new(),
        }
    }
    fn data_date(&self) -> Option<&str> {
        Some(&self.date)
    }
}

pub(super) struct Release {
    pub(super) month: ReleaseMonth,
    pub(super) date: String,
    pub(super) rows: Vec<Stored>,
}

// Imports written before the full roster kept only biosimilars. A suffix lets
// the same FDA month replace its old projection without allowing older months.
pub(super) const FULL_ROSTER_SUFFIX: &str = "/full";

pub(super) fn decode(bytes: &[u8]) -> Result<Release, BadReply> {
    let mut reader = ReaderBuilder::new()
        .has_headers(false)
        .trim(Trim::All)
        .from_reader(bytes);
    let mut records = reader.records();
    let title = records.next().ok_or(BadReply)?.map_err(csv_error)?;
    let month = ReleaseMonth::from_title(title.get(0).ok_or(BadReply)?)?;
    let date = month.key();
    let mut headers = None;
    let mut full = false;
    let mut rows = Vec::new();
    let mut presentations = HashSet::new();
    for record in records {
        let record = record.map_err(csv_error)?;
        if is_header(&record) {
            match &headers {
                None => headers = Some(record),
                Some(first) if !full && first == &record => full = true,
                Some(_) => return Err(BadReply),
            }
        } else if full && record.iter().any(|cell| !cell.is_empty()) {
            let native: Native = record.deserialize(headers.as_ref()).map_err(csv_error)?;
            let bla = parse_bla(&native.bla)?;
            if native.product_number.is_empty()
                || native.proper_name.is_empty()
                || !presentations.insert((bla.clone(), native.product_number.clone()))
            {
                return Err(BadReply);
            }
            rows.push((bla, native));
        }
    }
    let mut references = BTreeMap::<_, BTreeMap<_, _>>::new();
    for (bla, row) in &rows {
        if row.license_type.as_ref() == "351(a)" {
            references
                .entry(name_key(&row.name, &row.proper_name))
                .or_default()
                .insert(row.bla.clone(), bla.clone());
        }
    }
    if references.is_empty() {
        return Err(BadReply);
    }
    rows.sort_unstable_by(|(_, a), (_, b)| {
        (&a.bla, &a.product_number).cmp(&(&b.bla, &b.product_number))
    });
    if rows
        .iter()
        .all(|(_, row)| row.license_type.as_ref() == "351(a)")
    {
        return Err(BadReply);
    }
    let stored_date = format!("{date}{FULL_ROSTER_SUFFIX}");
    let mut presentations = Vec::with_capacity(rows.len());
    for (bla, row) in rows {
        let candidates = references.get(&name_key(&row.reference_name, &row.reference_proper_name));
        presentations.push(Stored {
            date: stored_date.as_str().into(),
            row: presentation(bla, row, candidates)?,
        });
    }
    Ok(Release {
        month,
        date,
        rows: presentations,
    })
}

fn is_header(record: &StringRecord) -> bool {
    record.get(0) == Some("N/R/U") && record.iter().any(|cell| cell == "BLA Number")
}
fn csv_error(_: csv::Error) -> BadReply {
    BadReply
}
fn name_key(name: &str, proper: &str) -> (String, String) {
    (name.to_ascii_lowercase(), proper.to_ascii_lowercase())
}
fn parse_bla(number: &str) -> Result<DrugId, BadReply> {
    if number.len() != 6 || !number.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(BadReply);
    }
    format!("BLA{number}").parse().map_err(|_| BadReply)
}
fn presentation(
    bla: DrugId,
    row: Native,
    candidates: Option<&BTreeMap<Box<str>, DrugId>>,
) -> Result<PurpleBookPresentation, BadReply> {
    let license_type = row.license_type.parse().map_err(|_| BadReply)?;
    if license_type != PurpleBookLicense::Standalone
        && (row.name.is_empty()
            || row.reference_name.is_empty()
            || row.reference_proper_name.is_empty())
    {
        return Err(BadReply);
    }
    let (reference_bla, reference_bla_candidates) = match candidates {
        Some(bl) if bl.len() == 1 => (
            bl.first_key_value().map(|(_, bla)| bla.clone()),
            Box::default(),
        ),
        Some(bl) => (None, bl.values().cloned().collect()),
        None => (None, Box::default()),
    };
    Ok(PurpleBookPresentation {
        bla,
        product_number: row.product_number,
        name: row.name,
        proper_name: row.proper_name,
        applicant: known(row.applicant),
        reference_name: row.reference_name,
        reference_proper_name: row.reference_proper_name,
        reference_bla,
        reference_bla_candidates,
        strength: row.strength,
        device: row.device,
        route: row.route,
        license_type,
        interchangeable: license_type == PurpleBookLicense::Interchangeable,
        approval_date: known(row.approval_date),
        interchangeability_date: known(row.interchangeability_date),
        first_interchangeable_exclusivity_expiration: known(
            row.first_interchangeable_exclusivity_expiration,
        ),
    })
}
fn known(cell: Option<Box<str>>) -> Option<Box<str>> {
    cell.filter(|value| value.as_ref() != "N/A")
}
