//! Find the eEML medicine by name, then read its JSON-LD medicine and recommendation pages.
use super::raw::{Medicine, Recommendation};
use crate::{Mapped, Refusal, mapped::Problems, quotas};
use biomcp_fetch::{
    Arg, DocumentMedia, Endpoint, Freshness, Input, KeyUse, Leg, PathSegment, Read, RequestError,
    SourceSpec,
};
use biomcp_types::{
    DrugEssentialMedicines, DrugHeader, DrugRecords, DrugSource, Section, Source, Use,
};
use std::time::Duration;

/// The eEML is CC BY 3.0 IGO, commercial use included, but not for advertising or promoting
/// commercial products or suggesting WHO endorsement (https://list.essentialmeds.org/licencing).
pub static SPEC: SourceSpec = SourceSpec {
    source: Source::WhoEml,
    base: "https://list.essentialmeds.org",
    terms: biomcp_fetch::Terms {
        name: "CC BY 3.0 IGO; not for promoting commercial products or implying WHO endorsement",
        url: "https://list.essentialmeds.org/licencing",
        usage: Use::Attribution,
    },
    quota: &quotas::WHO_EML,
    key: KeyUse::None,
    destinations: biomcp_fetch::Destinations::Primary,
    user_agent: None,
    deadline: Duration::from_secs(15),
    command_budget: None,
};
/// The site's own search page; its results name each medicine and link its eEML number.
const SEARCH: Endpoint = SPEC
    .get("/")
    .bytes(2 * 1024 * 1024)
    .fresh(Freshness::Days(7));
const MEDICINE: Endpoint = SPEC.get("/medicines").fresh(Freshness::Days(7));
const RECOMMENDATION: Endpoint = SPEC.get("/recommendations").fresh(Freshness::Days(7));
const HINT: &str = "From the WHO eEML. WHO says the PDF Model Lists (EML and EMLc) are the \
                    binding versions and the eEML may differ from them.";

/// One page of the eEML listings for a medicine named exactly as the eEML names it (letter case
/// ignored), or for its eEML number; `None` when the eEML has no such medicine.
pub fn essential_medicines(
    leg: &Leg<'_>,
    input: &str,
    limit: u32,
    offset: u32,
) -> Result<Mapped<Option<DrugEssentialMedicines>>, RequestError> {
    let number = if input.bytes().all(|b| b.is_ascii_digit()) {
        Some(input.to_owned())
    } else {
        search(leg, input)?
    };
    let mut out = Mapped {
        value: None,
        problems: Vec::new(),
        refusals: Refusal::None,
    };
    let Some(number) = number else {
        return Ok(out);
    };
    let segment = PathSegment::new(&number).map_err(RequestError::Input)?;
    let medicine: Medicine = leg
        .call(
            &MEDICINE.clone().with_segments(&[segment]),
            Input::Params(&[]),
            Read::json(),
        )?
        .value;
    let mut cx = Problems {
        source: Source::WhoEml,
        record: crate::mapped::record(&number),
        list: Vec::new(),
    };
    let (name, listed) = medicine.map(&mut cx);
    let total = listed.len() as u64;
    let mut records = Vec::new();
    let mut previous = None;
    for id in listed.iter().skip(offset as usize).take(limit as usize) {
        let segment = PathSegment::new(&id.to_string()).map_err(RequestError::Input)?;
        let recommendation: Recommendation = leg
            .call(
                &RECOMMENDATION.clone().with_segments(&[segment]),
                Input::Params(&[]),
                Read::json(),
            )?
            .value;
        let url = format!("{}/recommendations/{id}", SPEC.base);
        if let Some(listing) = recommendation.map(*id, &url, name.as_ref(), &mut previous, &mut cx)
        {
            records.push(listing);
        }
    }
    let data = DrugRecords {
        offset,
        total: Some(total),
        truncated: None,
        records: records.into(),
        version: None,
        retrieved: None,
    };
    let listings = if total == 0 {
        Section::Empty {
            source: Source::WhoEml,
            record: None,
        }
    } else {
        Section::Data {
            source: Source::WhoEml,
            data,
        }
    };
    out.value = Some(DrugEssentialMedicines {
        identity: header(&number, name)?,
        listings,
        hint: crate::mapped::record(HINT),
    });
    out.problems = cx.list;
    Ok(out)
}

/// The eEML number of the medicine the site's search lists under exactly this name.
fn search(leg: &Leg<'_>, name: &str) -> Result<Option<String>, RequestError> {
    Ok(medicines(leg, name)?
        .into_iter()
        .find(|header| {
            header
                .name
                .as_ref()
                .is_some_and(|title| title.as_str().eq_ignore_ascii_case(name.trim()))
        })
        .map(|header| header.id.to_string()))
}

/// Every medicine the site's own search lists for this text, in its order: each name and eEML
/// number. The site matches names containing the text, and lists a medicine named only as a
/// therapeutic alternative too.
pub fn medicines(leg: &Leg<'_>, name: &str) -> Result<Vec<DrugHeader>, RequestError> {
    let page = leg
        .call(
            &SEARCH,
            Input::Params(&[("query", Arg::Text(name))]),
            Read::bytes().document_media(DocumentMedia::Html),
        )?
        .value
        .0;
    super::raw::search_hits(&String::from_utf8_lossy(&page))
        .into_iter()
        .map(|(title, number)| header(&number, crate::mapped::record(title)))
        .collect()
}

/// The eEML medicine's identity: its number, name and page.
fn header(number: &str, name: Option<biodata::Text>) -> Result<DrugHeader, RequestError> {
    Ok(DrugHeader {
        name_resolution: None,
        id: number
            .parse()
            .map_err(|_| biomcp_fetch::bad_reply(Source::WhoEml, None))?,
        source: DrugSource::WhoEml,
        name,
        identity_level: biodata::DrugIdentityLevel::Unknown,
        matched_name: None,
        r#type: None,
        version: None,
        effective_date: None,
        application_numbers: Box::default(),
        url: crate::mapped::record(format!("{}/medicines/{number}", SPEC.base)),
    })
}
