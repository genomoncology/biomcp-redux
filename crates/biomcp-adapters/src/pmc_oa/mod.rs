//! PMC OA bucket listing, checked metadata and confined JATS reads.
mod location;
mod raw;
mod supplement;
use crate::{jats, quotas};
use biodata::License;
use biodata::{Pmcid, Pmid};

use biomcp_fetch::{
    Arg, BadReply, Endpoint, Freshness, KeyUse, Leg, RequestError, SourceError, SourceSpec,
};
use biomcp_types::{Reason, Source};
use std::time::Duration;
pub use supplement::{Download, Supplement, download, download_bytes};
/// Conservative application policy; no numeric provider limit is claimed.
pub static SPEC: SourceSpec = SourceSpec {
    destinations: biomcp_fetch::Destinations::Primary,
    user_agent: None,
    terms: biomcp_fetch::Terms {
        name: "PMC copyright; article-specific licenses",
        url: "https://pmc.ncbi.nlm.nih.gov/about/copyright/",
        usage: biomcp_types::Use::Restricted,
    },
    source: Source::PmcOa,
    base: "https://pmc-oa-opendata.s3.amazonaws.com",
    key: KeyUse::None,
    quota: &quotas::PMC_OA,
    deadline: Duration::from_secs(15),
};
const LIST: Endpoint = SPEC.get("/").fresh(Freshness::Days(1));
const FILE: Endpoint = SPEC.get("/").octet_stream().fresh(Freshness::Days(7));
/// Selected processing version and source text.
#[derive(Debug)]
pub struct Document {
    /// Exact public source URL used to fetch this JATS document.
    pub url: biodata::Text,
    /// License owned by the JSON metadata record.
    pub license: License,
    /// Highest numeric processing version, not publication chronology.
    pub version: Option<u32>,
    /// Author manuscript flag from metadata.
    pub manuscript: Option<bool>,
    /// Parsed source body.
    pub body: jats::Body,
    /// JATS supplements confirmed against the record media list.
    pub supplements: Vec<Supplement>,
    /// Rejected metadata and JATS fields.
    pub problems: Vec<biomcp_types::Problem>,
}
fn bad(_: impl std::fmt::Debug) -> RequestError {
    SourceError {
        source: Source::PmcOa,
        reason: Reason::BadReply,
        status: None,
        retry_after: None,
    }
    .into()
}
/// Return no document only when the bucket has no matching processing record.
pub fn read(
    leg: &Leg<'_>,
    id: Pmcid,
    pmid: Option<Pmid>,
) -> Result<crate::Mapped<Option<Document>>, RequestError> {
    let prefix = format!("{id}.");
    let listing = leg.xml::<raw::Listing>(
        &LIST,
        &[("list-type", Arg::Num(2)), ("prefix", Arg::Text(&prefix))],
    )?;
    let contents = listing.contents.as_deref().unwrap_or(&[]);
    if contents.iter().any(|e| !e.key.starts_with(&prefix)) {
        return Err(bad(BadReply));
    }
    let Some(version) = contents
        .iter()
        .filter_map(|e| location::version(&e.key, id))
        .max()
    else {
        return Ok(crate::Mapped {
            value: None,
            problems: Vec::new(),
        });
    };
    let directory = format!("{id}.{version}/");
    let path = format!("/{directory}{id}.{version}.json");
    let record = leg.json::<raw::Record>(&FILE.clone().with_path(path).map_err(bad)?, &[])?;
    if record.pmcid != id || pmid.is_some_and(|expected| record.pmid != Some(expected.get())) {
        return Err(bad(BadReply));
    }
    let mut problems = crate::mapped::Problems {
        source: Source::PmcOa,
        record: crate::mapped::record(id),
        list: Vec::new(),
    };
    let license_code = problems
        .required_field("license", record.license_code)
        .filter(|v| !v.trim().is_empty());
    let license_code = problems.required("license", license_code);
    let Some(license_code) = license_code else {
        return Ok(crate::Mapped {
            value: None,
            problems: problems.list,
        });
    };
    let loc = location::Location::checked(&record.xml_url, &directory).map_err(bad)?;
    if !loc.path.ends_with(".xml") {
        return Err(bad(BadReply));
    }
    let params: Vec<_> = loc
        .md5
        .as_ref()
        .map(|v| ("md5", Arg::Text(v)))
        .into_iter()
        .collect();
    let url = biodata::Text::new(&format!(
        "{}{}{}",
        SPEC.base,
        loc.path,
        loc.md5
            .as_ref()
            .map(|v| format!("?md5={v}"))
            .unwrap_or_default()
    ))
    .map_err(bad)?;
    let body = leg.xml::<jats::Body>(&FILE.clone().with_path(loc.path).map_err(bad)?, &params)?;
    if body.pmcid.is_some_and(|found| found != id) {
        return Err(bad(BadReply));
    }
    let urls = problems.fields("media_urls", record.media_urls);
    let supplements = supplement::select(&body.supplements, &urls, &directory, &mut problems)?;
    problems.list.extend(body.problems.clone());
    let manuscript = problems.field("manuscript", record.is_manuscript);
    Ok(crate::Mapped {
        value: Some(Document {
            url,
            supplements,
            license: license(&license_code),
            version: Some(version),
            manuscript,
            problems: problems.list.clone(),
            body,
        }),
        problems: Vec::new(),
    })
}
pub(crate) fn license(value: &str) -> License {
    match value.to_ascii_uppercase().replace('-', " ").as_str() {
        "CC BY" => License::CcBy,
        "CC BY NC" => License::CcByNc,
        "CC BY SA" => License::CcBySa,
        "CC BY NC SA" => License::CcByNcSa,
        "CC BY ND" => License::CcByNd,
        "CC BY NC ND" => License::CcByNcNd,
        "CC0" => License::Cc0,
        "PUBLIC DOMAIN" => License::PublicDomain,
        "TDM" => License::Tdm,
        _ => License::Other,
    }
}
