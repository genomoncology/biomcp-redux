//! Find the eEML medicine by name, then read its JSON-LD medicine and recommendation pages.
use super::raw::{Medicine, Recommendation};
use crate::{Mapped, Refusal, mapped::Problems, quotas};
use biomcp_fetch::{
    Arg, DocumentMedia, Endpoint, Freshness, Input, KeyUse, Leg, PathSegment, Read, RequestError,
    SourceSpec,
};
use biomcp_types::{
    DrugEssentialMedicineListing, DrugEssentialMedicines, DrugHeader, DrugNameMatch, DrugRecords,
    DrugSource, Section, Source, Use,
};
use std::time::Duration;

/// The eEML is CC BY 3.0 IGO, commercial use included, but not for advertising or promoting
/// commercial products or suggesting WHO endorsement (<https://list.essentialmeds.org/licencing>).
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
    // Each listing's status is on its own page, read at one a second, so a medicine with many
    // applications (pembrolizumab: 14 pages) needs a longer leg and command than the default.
    deadline: Duration::from_secs(60),
    command_budget: Some(Duration::from_secs(60)),
};
/// The site's own search page; its results name each medicine and link its eEML number.
const SEARCH: Endpoint = SPEC
    .get("/")
    .bytes(2 * 1024 * 1024)
    .fresh(Freshness::Days(7));
const MEDICINE: Endpoint = SPEC.get("/medicines").fresh(Freshness::Days(7));
const RECOMMENDATION: Endpoint = SPEC.get("/recommendations").fresh(Freshness::Days(7));
const HINT: &str = "WHO electronic Essential Medicines List (eEML), World Health Organization, \
                    2020. https://list.essentialmeds.org/ (beta version 1.0). Licence: CC BY 3.0 \
                    IGO. WHO says the PDF Model Lists (EML and EMLc) are the binding versions and \
                    the eEML may differ from them. Listings only: rejected applications are left out.";

/// One page of eEML listings by name or eEML number.
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
    let listed = medicine
        .map(&mut cx)
        .map_err(|_| biomcp_fetch::bad_reply(Source::WhoEml, None))?;
    let all = listings(leg, &listed.numbers, &mut cx)?;
    let total = all.len() as u64;
    let mut records: Vec<DrugEssentialMedicineListing> = all
        .into_iter()
        .skip(offset as usize)
        .take(limit as usize)
        .collect();
    // Listings for related indications often share one Committee text; a page gives it once.
    let mut previous = None;
    for listing in &mut records {
        if listing.committee.is_some() && listing.committee == previous {
            listing.committee = None;
        } else {
            previous.clone_from(&listing.committee);
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
        identity: header(&number, listed.name)?,
        listings,
        hint: crate::mapped::record(HINT),
    });
    out.problems = cx.list;
    Ok(out)
}

/// Every Model List listing among the medicine's recommendations, in order. The status of each
/// is on its own page, so every page is read: the medicine page also links rejected
/// applications, and the total counts listings only.
fn listings(
    leg: &Leg<'_>,
    numbers: &[(u32, bool)],
    cx: &mut Problems,
) -> Result<Vec<DrugEssentialMedicineListing>, RequestError> {
    let mut listings = Vec::new();
    for (id, alternative) in numbers {
        let segment = PathSegment::new(&id.to_string()).map_err(RequestError::Input)?;
        let recommendation: Recommendation = leg
            .call(
                &RECOMMENDATION.clone().with_segments(&[segment]),
                Input::Params(&[]),
                Read::json(),
            )?
            .value;
        let url = format!("{}/recommendations/{id}", SPEC.base);
        listings.extend(recommendation.map(*id, &url, *alternative, cx));
    }
    Ok(listings)
}

/// The eEML number for a complete name or a unique alternate spelling among the search hits.
fn search(leg: &Leg<'_>, name: &str) -> Result<Option<String>, RequestError> {
    let hits = medicines(leg, name)?;
    Ok(matching_medicine(&hits, name).map(|header| header.id.to_string()))
}

fn matching_medicine<'a>(hits: &'a [DrugHeader], name: &str) -> Option<&'a DrugHeader> {
    let name = name.trim();
    let named = hits
        .iter()
        .filter_map(|header| Some((header.name.as_ref()?.as_str(), header)));
    if let Some((_, exact)) = named
        .clone()
        .find(|(title, _)| DrugNameMatch::same_name(title, name))
    {
        return Some(exact);
    }
    let mut aliases = named.filter(|(title, _)| alternate_name(title, name));
    let (_, first) = aliases.next()?;
    aliases
        .all(|(_, header)| header.id == first.id)
        .then_some(first)
}

fn alternate_name(title: &str, name: &str) -> bool {
    if DrugNameMatch::generic_name(title, name) {
        return true;
    }
    let bracketed = title
        .strip_suffix(')')
        .and_then(|title| title.split_once(" ("))
        .or_else(|| {
            title
                .strip_suffix(']')
                .and_then(|title| title.split_once(" ["))
        });
    bracketed.is_some_and(|(base, alternate)| {
        DrugNameMatch::generic_name(base, name)
            || (DrugNameMatch::generic_name(alternate, name) && !qualifier(alternate))
    })
}

fn qualifier(value: &str) -> bool {
    // eEML also brackets formulations, sources and other qualifiers.
    let value = value.to_ascii_lowercase();
    value.contains(',')
        || value.starts_with("as ")
        || matches!(
            value.as_str(),
            "injection"
                | "native"
                | "pathogen-reduced"
                | "seasonal"
                | "high-viscosity"
                | "low-viscosity"
        )
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
    let page = String::from_utf8_lossy(&page);
    let hits = search_hits(&page);
    // A changed page must not read as "no such medicine": the count line must be there, and a
    // count above zero must come with medicine headings.
    if found(&page)? > 0 && hits.is_empty() {
        return Err(biomcp_fetch::bad_reply(Source::WhoEml, None));
    }
    hits.into_iter()
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
        matched_sentence: None,
        r#type: None,
        version: None,
        effective_date: None,
        application_numbers: Box::default(),
        url: crate::mapped::record(format!("{}/medicines/{number}", SPEC.base)),
    })
}

/// The medicines and therapeutic equivalents the search page's count line says it found, from
/// "Found N recommendations for M medicines and E therapeutic equivalents"; a page without that
/// line is a bad reply.
fn found(page: &str) -> Result<u32, RequestError> {
    let bad = || biomcp_fetch::bad_reply(Source::WhoEml, None);
    let line = page
        .split_once("Found ")
        .and_then(|(_, line)| line.split('<').next())
        .ok_or_else(bad)?;
    let words: Vec<&str> = line.split_whitespace().collect();
    let [
        _,
        "recommendation" | "recommendations",
        "for",
        medicines,
        "medicine" | "medicines",
        "and",
        equivalents,
        "therapeutic",
        ..,
    ] = words.as_slice()
    else {
        return Err(bad());
    };
    let medicines = medicines.parse::<u32>().map_err(|_| bad())?;
    Ok(medicines + equivalents.parse::<u32>().map_err(|_| bad())?)
}
/// Each medicine on an eEML search page: the name in its heading's `title` and its eEML number.
fn search_hits(page: &str) -> Vec<(String, String)> {
    const HEADING: &str = "<h5 class=\"medicine-name\" title=\"";
    const LINK: &str = "/medicines/";
    let mut hits = Vec::new();
    for part in page.split(HEADING).skip(1) {
        let Some((title, rest)) = part.split_once('"') else {
            continue;
        };
        let heading = rest.split("</h5>").next().unwrap_or(rest);
        let Some((_, after)) = heading.split_once(LINK) else {
            continue;
        };
        let number: String = after.chars().take_while(char::is_ascii_digit).collect();
        if !number.is_empty() {
            hits.push((unescape(title), number));
        }
    }
    hits
}
fn unescape(value: &str) -> String {
    value
        .replace("&#039;", "'")
        .replace("&#39;", "'")
        .replace("&quot;", "\"")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&amp;", "&")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_who_medicine_name_resolves_to_its_own_entry() {
        let medicines: Vec<_> = include_str!("medicine-names.tsv")
            .lines()
            .map(|line| line.split_once('\t').expect("eEML number and name"))
            .collect();
        let headers: Vec<_> = medicines
            .iter()
            .map(|(number, name)| {
                header(number, crate::mapped::record(*name)).expect("valid eEML number")
            })
            .collect();
        assert_eq!(medicines.len(), 650);
        for (number, name) in medicines {
            let matched = matching_medicine(&headers, name).expect(name);
            assert_eq!(matched.id.to_string(), number, "{name}");
        }
        assert!(matching_medicine(&headers, "insulin").is_none());
        assert!(matching_medicine(&headers, "seasonal").is_none());
    }
}
