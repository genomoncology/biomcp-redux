//! Select one offline snapshot and import only validated newer releases.
use super::decode::{FULL_ROSTER_SUFFIX, Release, ReleaseMonth, Stored, decode};
use crate::decl::BundleReply;
use biomcp_fetch::{
    Bundle, CacheError, Fetch, Find, Freshness, KeyKind, KeyUse, Leg, RequestError, RowFormat,
    SourceError, SourceSpec,
};
use biomcp_types::{DrugSearch, PurpleBookPresentation, Reason, Source, SourceNotice};
use std::{marker::PhantomData, path::Path, time::Duration};

const SOURCE: Source = Source::PurpleBook;
pub(super) const NAME: &str = "purple-book";
const PATH: &str = "/downloads";
const MAX_FILE_BYTES: u64 = 4 * 1024 * 1024;
pub(super) const HELP: &str = "FDA does not allow automatic download. Download the monthly CSV from \
    https://purplebooksearch.fda.gov/downloads and run biomcp sync purple-book --file PATH.";

/// Local Purple Book source identity and attribution.
pub static SPEC: SourceSpec = SourceSpec {
    source: Source::PurpleBook,
    base: "https://purplebooksearch.fda.gov",
    terms: crate::fda_cdx::SPEC.terms,
    quota: &crate::quotas::FDA_WEBSITE,
    key: KeyUse::None,
    destinations: biomcp_fetch::Destinations::Primary,
    user_agent: None,
    deadline: Duration::from_secs(15),
    command_budget: None,
};
const BUNDLE: Bundle<Stored> = Bundle {
    spec: &SPEC,
    name: NAME,
    path: PATH,
    format: RowFormat::NativeCsv {
        max_bytes: MAX_FILE_BYTES,
    },
    fresh: Freshness::Days(31),
    row: PhantomData,
};

fn seed() -> Result<Release, RequestError> {
    decode(include_bytes!("seed.csv")).map_err(|_| biomcp_fetch::bad_reply(SOURCE, None))
}

struct Selected {
    rows: Vec<Stored>,
    month: ReleaseMonth,
    full_roster: bool,
}

fn snapshot(leg: &Leg<'_>, key: &str) -> Result<Selected, RequestError> {
    let stored = leg.manual_rows(&BUNDLE, Find::ExactOrUnindexed(KeyKind::Alias, key))?;
    let seed = seed()?;
    if let Some((rows, info)) = stored {
        let date = info.data_date.as_deref().ok_or(CacheError::Encoding)?;
        let full_date = date.strip_suffix(FULL_ROSTER_SUFFIX);
        let month =
            ReleaseMonth::from_key(full_date.unwrap_or(date)).map_err(|_| CacheError::Encoding)?;
        if month > seed.month {
            return Ok(Selected {
                rows,
                month,
                full_roster: full_date.is_some(),
            });
        }
    }
    Ok(Selected {
        rows: seed.rows,
        month: seed.month,
        full_roster: true,
    })
}

pub(super) fn probe(leg: &Leg<'_>) -> Result<(), RequestError> {
    snapshot(leg, "").map(drop)
}

/// Matching presentations and the age of the snapshot searched.
#[derive(Debug)]
pub struct SnapshotSearch {
    /// FDA release month in the publisher's report.
    pub release_month: Box<str>,
    /// Matching licensed presentations in the snapshot.
    pub rows: Vec<PurpleBookPresentation>,
    /// Snapshot age and, for an empty result, its coverage and import instructions.
    pub notice: SourceNotice,
}

/// Search by proprietary name, proper name or BLA, including reference links.
pub fn search(leg: &Leg<'_>, query: &DrugSearch) -> Result<SnapshotSearch, RequestError> {
    let input = query.query().map_or("", biodata::Text::as_str);
    let upper = input.to_ascii_uppercase();
    let number = upper.strip_prefix("BLA").unwrap_or(&upper).trim();
    let key = if number.len() == 6 && number.bytes().all(|byte| byte.is_ascii_digit()) {
        format!("BLA{number}")
    } else {
        upper
    };
    let snapshot = snapshot(leg, &key)?;
    let strength = query.strength().map(|value| strength_key(value.as_str()));
    let device = query.device();
    let matches = snapshot.rows.into_iter().filter(|stored| {
        stored.matches(&key)
            && strength.as_ref().is_none_or(|strength| {
                stored
                    .row
                    .strength
                    .as_deref()
                    .is_some_and(|cell| strength_key(cell) == *strength)
            })
            && device.is_none_or(|device| {
                stored
                    .row
                    .device
                    .as_deref()
                    .is_some_and(|cell| cell.eq_ignore_ascii_case(device.as_str()))
            })
    });
    let rows: Vec<_> = matches.map(|stored| stored.row).collect();
    let release_month = snapshot.month.label();
    let guidance = if !snapshot.full_roster {
        format!(
            "This import contains only biosimilars. Reimport the complete {release_month} CSV to search all products. {HELP}"
        )
    } else if rows.is_empty() {
        format!("Products approved or designated after {release_month} are not included. {HELP}")
    } else {
        String::new()
    };
    Ok(SnapshotSearch {
        release_month,
        rows,
        notice: snapshot_notice(leg, &snapshot.month, &guidance),
    })
}

fn strength_key(value: &str) -> String {
    value
        .split_whitespace()
        .collect::<String>()
        .to_ascii_uppercase()
}

/// Import an FDA release from a local file, or explain manual synchronization.
pub fn sync_file(fetch: &Fetch, path: Option<&Path>) -> BundleReply {
    let leg = fetch.sync_leg(&SPEC);
    let result = if fetch.disabled(SOURCE) {
        Err((source_error(Reason::Disabled), ""))
    } else {
        path.map_or_else(
            || Err((source_error(Reason::Unsupported), HELP)),
            |path| import(&leg, path),
        )
    };
    let (result, message, month) = match result {
        Ok((info, month)) => (
            Ok(crate::mapped::bundle_sync(SOURCE, info)),
            "",
            Some(month),
        ),
        Err((error, message)) => match snapshot(&leg, "") {
            Ok(active) => (Err(error), message, Some(active.month)),
            Err(_) => (Err(error), message, None),
        },
    };
    let notice = month.map_or_else(unavailable_notice, |month| {
        snapshot_notice(&leg, &month, message)
    });
    BundleReply::Manual(result, notice)
}

fn import(
    leg: &Leg<'_>,
    path: &Path,
) -> Result<(biomcp_fetch::BundleInfo, ReleaseMonth), (RequestError, &'static str)> {
    const UNREADABLE: &str = "The requested release file is unavailable or unreadable.";
    const INVALID: &str = "The requested release is unavailable because the file is not a valid complete FDA monthly Purple Book CSV.";
    let body = leg
        .read_manual_file(path, MAX_FILE_BYTES)
        .map_err(|error| {
            if error == RequestError::Cache(CacheError::Limit) {
                (source_error(Reason::BadReply), INVALID)
            } else {
                (error, UNREADABLE)
            }
        })?;
    let release = decode(&body).map_err(|_| (source_error(Reason::BadReply), INVALID))?;
    let seed = seed().map_err(|error| (error, ""))?;
    let bundled_date = format!("{}{FULL_ROSTER_SUFFIX}", seed.date);
    let mut info = leg
        .import_manual_rows(&BUNDLE, &release.rows, body.len() as u64, &bundled_date)
        .map_err(|error| (error, "The requested release could not be stored."))?
        .ok_or((
            source_error(Reason::Rejected),
            "The supplied release is not newer than the active snapshot.",
        ))?;
    info.data_date = Some(release.date.into());
    Ok((info, release.month))
}

fn source_error(reason: Reason) -> RequestError {
    SourceError {
        source: SOURCE,
        reason,
        status: None,
        tls_cause: None,
        retry_after: None,
    }
    .into()
}

fn snapshot_notice(leg: &Leg<'_>, month: &ReleaseMonth, guidance: &str) -> SourceNotice {
    let age = month.age_in_months(leg.utc_month());
    let unit = if age == 1 { "month" } else { "months" };
    let snapshot = format!(
        "This is the {} Purple Book snapshot ({age} {unit} old).",
        month.label()
    );
    notice(if guidance.is_empty() {
        format!(
            "{snapshot} FDA updates the list monthly. FDA downloads: {}{PATH}",
            SPEC.base
        )
    } else {
        format!("{snapshot} {guidance}")
    })
}

/// Explain why a reply cannot state the age of an unread snapshot.
pub fn unavailable_notice() -> SourceNotice {
    notice(format!(
        "The active Purple Book snapshot could not be read. FDA downloads: {}{PATH}",
        SPEC.base
    ))
}

fn notice(message: String) -> SourceNotice {
    SourceNotice::RequestGuidance {
        data_date: None,
        message: message.into(),
        retry_after_seconds: None,
    }
}
