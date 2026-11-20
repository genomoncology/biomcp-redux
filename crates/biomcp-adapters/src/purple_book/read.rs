//! Select one offline snapshot and import only validated newer releases.
use super::decode::{Release, ReleaseMonth, Stored, decode};
use crate::decl::BundleReply;
use biomcp_fetch::{
    BadReply, Bundle, CacheError, Fetch, Find, Freshness, KeyKind, KeyUse, Leg, RequestError,
    RowFormat, SourceError, SourceSpec,
};
use biomcp_types::{DrugSearch, PurpleBookPresentation, Reason, Source, SourceNotice};
use std::{io::Read, marker::PhantomData, path::Path, sync::OnceLock, time::Duration};

const SOURCE: Source = Source::PurpleBook;
const PATH: &str = "/downloads";
const DOWNLOAD: &str = "https://purplebooksearch.fda.gov/downloads";
const MAX_FILE_BYTES: u64 = 4 * 1024 * 1024;
pub(super) const HELP: &str = "FDA does not allow automatic download. Download the monthly CSV from \
    https://purplebooksearch.fda.gov/downloads and run biomcp sync purple-book --file PATH.";

/// Local Purple Book source identity and attribution.
pub static SPEC: SourceSpec = SourceSpec {
    source: SOURCE,
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
    name: SOURCE.as_str(),
    path: PATH,
    format: RowFormat::NativeCsv {
        max_bytes: MAX_FILE_BYTES,
    },
    fresh: Freshness::Days(31),
    row: PhantomData,
};

fn seed() -> Result<&'static Release, RequestError> {
    static SEED: OnceLock<Result<Release, BadReply>> = OnceLock::new();
    SEED.get_or_init(|| decode(include_bytes!("seed.csv")))
        .as_ref()
        .map_err(|_| biomcp_fetch::bad_reply(SOURCE, None))
}

fn snapshot(
    leg: &Leg<'_>,
    key: &str,
) -> Result<(&'static Release, Option<(Vec<Stored>, ReleaseMonth)>), RequestError> {
    let seed = seed()?;
    let stored = leg.manual_rows(&BUNDLE, Find::Exact(KeyKind::Xref, key))?;
    let stored = if let Some((rows, info)) = stored {
        let date = info.data_date.as_deref().ok_or(CacheError::Encoding)?;
        let month = ReleaseMonth::from_key(date).ok_or(CacheError::Encoding)?;
        (month > seed.month).then_some((rows, month))
    } else {
        None
    };
    Ok((seed, stored))
}

pub(super) fn probe(leg: &Leg<'_>) -> Result<(), RequestError> {
    snapshot(leg, "").map(drop)
}

/// Search the full roster by the published reference names or their BLA candidates.
pub fn search(
    leg: &Leg<'_>,
    query: &DrugSearch,
) -> Result<(Box<str>, Vec<PurpleBookPresentation>), RequestError> {
    let input = query.query().map_or("", biodata::Text::as_str);
    let upper = input.to_ascii_uppercase();
    let number = upper.strip_prefix("BLA").unwrap_or(&upper).trim();
    let key = if number.len() == 6 && number.bytes().all(|byte| byte.is_ascii_digit()) {
        format!("BLA{number}")
    } else {
        upper
    };
    let (seed, stored) = snapshot(leg, &key)?;
    let (rows, release): (&[Stored], _) = match &stored {
        Some((rows, month)) => (rows, month),
        None => (&seed.rows, &seed.month),
    };
    let strength = query.strength().map(|value| strength_key(value.as_str()));
    let device = query.device();
    let matches = rows.iter().filter(|stored| {
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
    Ok((
        release.label(),
        matches.map(|stored| stored.row.clone()).collect(),
    ))
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
    let (result, guidance, month) = match result {
        Ok(info) => {
            let month = info.data_date.as_deref().and_then(ReleaseMonth::from_key);
            (
                Ok(crate::mapped::bundle_sync(SOURCE, info)),
                "",
                month.map(|m| m.label()),
            )
        }
        Err((error, message)) => {
            let month = snapshot(&leg, "").ok().map(|(seed, stored)| {
                stored.map_or_else(|| seed.month.label(), |(_, month)| month.label())
            });
            (Err(error), message, month)
        }
    };
    BundleReply::Manual(result, notice(month.as_deref(), guidance))
}

fn import(
    leg: &Leg<'_>,
    path: &Path,
) -> Result<biomcp_fetch::BundleInfo, (RequestError, &'static str)> {
    const UNREADABLE: &str = "The requested release file is unavailable or unreadable.";
    const INVALID: &str = "The requested release is unavailable because the file is not a valid complete FDA monthly Purple Book CSV.";
    let meta = path
        .metadata()
        .map_err(|_| (CacheError::Filesystem.into(), UNREADABLE))?;
    if !meta.is_file() || meta.len() > MAX_FILE_BYTES {
        return Err((source_error(Reason::BadReply), INVALID));
    }
    let mut body = Vec::new();
    std::fs::File::open(path)
        .and_then(|file| file.take(MAX_FILE_BYTES + 1).read_to_end(&mut body))
        .map_err(|_| (CacheError::Filesystem.into(), UNREADABLE))?;
    if body.len() as u64 > MAX_FILE_BYTES {
        return Err((source_error(Reason::BadReply), INVALID));
    }
    let release = decode(&body).map_err(|_| (source_error(Reason::BadReply), INVALID))?;
    let seed = seed().map_err(|error| (error, ""))?;
    leg.import_manual_rows(&BUNDLE, &release.rows, body.len() as u64, &seed.date)
        .map_err(|error| (error, "The requested release could not be stored."))?
        .ok_or((
            source_error(Reason::Rejected),
            "The supplied release is not newer than the active snapshot.",
        ))
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

/// Explain the monthly release cadence and give the publisher's download address.
pub fn notice(month: Option<&str>, guidance: &str) -> SourceNotice {
    let snapshot = month.map_or_else(
        || "FDA updates the Purple Book snapshot monthly.".to_owned(),
        |month| format!("This is the {month} Purple Book snapshot. FDA updates the list monthly."),
    );
    SourceNotice::RequestGuidance {
        data_date: None,
        message: if guidance.is_empty() {
            format!("{snapshot} FDA downloads: {DOWNLOAD}")
        } else {
            format!("{guidance} {snapshot}")
        }
        .into(),
        retry_after_seconds: None,
    }
}
