//! Native registry advanced expressions.
use biomcp_types::TrialSearch;
use std::collections::BTreeMap;

/// Every search row asks for these; a date's type is its actual or estimated qualifier.
const FIELDS: &str = concat!(
    "NCTId,BriefTitle,OverallStatus,Phase,Condition,InterventionName,LastUpdatePostDate,HasResults",
    ",LeadSponsorName,StartDate,StartDateType,PrimaryCompletionDate,PrimaryCompletionDateType",
    ",WhyStopped,Acronym,OfficialTitle"
);

pub(super) fn base_parameters(query: &TrialSearch) -> BTreeMap<String, String> {
    let mut fields = FIELDS.to_owned();
    if !query.country().is_empty() {
        fields.push_str(",LocationCountry");
    }
    if query.gene().is_some() && !query.count_only() {
        fields.push_str(",EligibilityCriteria,BriefSummary,DetailedDescription,Keyword");
    }
    BTreeMap::from([
        ("countTotal".into(), "true".into()),
        (
            "pageSize".into(),
            if query.count_only() {
                "1".into()
            } else {
                query.limit().to_string()
            },
        ),
        ("fields".into(), fields),
    ])
}

pub(super) fn advanced(query: &TrialSearch, broader_names: bool) -> String {
    let mut advanced = Vec::new();
    if query.exact_condition() {
        advanced.push(format!(
            "({})",
            query
                .condition()
                .iter()
                .map(|condition| format!(
                    "(AREA[Condition]EXPANSION[None]{})",
                    quote(condition.as_str())
                ))
                .collect::<Vec<_>>()
                .join(" OR ")
        ));
    }
    if let Some(id) = query.id() {
        advanced.push(format!("AREA[NCTId]{id}"));
    }
    if !query.country().is_empty() {
        advanced.push(format!(
            "({})",
            query
                .country()
                .iter()
                .map(|country| format!("AREA[LocationCountry]{}", quote(country)))
                .collect::<Vec<_>>()
                .join(" OR ")
        ));
    }
    if let Some(phase) = query.phase() {
        advanced.extend(
            phase
                .phases()
                .iter()
                .map(|phase| format!("AREA[Phase]{}", phase.as_str())),
        );
    }
    for (field, value) in [
        ("Acronym", query.acronym()),
        ("OfficialTitle", query.official_title()),
        ("EligibilityCriteria", query.criteria()),
        (
            "LocationFacility",
            query.facility().filter(|_| !broader_names),
        ),
        (
            sponsor_area(query),
            query.sponsor().filter(|_| !broader_names),
        ),
    ] {
        if let Some(value) = value {
            // A group of quoted spellings BioMCP wrote is already registry syntax.
            if field == "EligibilityCriteria" && biomcp_types::is_criteria_group(value.as_str()) {
                advanced.push(format!("AREA[{field}]{value}"));
            } else {
                advanced.push(format!("AREA[{field}]{}", quote(value.as_str())));
            }
        }
    }
    if let Some(study_type) = query.study_type() {
        advanced.push(format!("AREA[StudyType]{}", study_type.as_str()));
    }
    if let Some(age) = query.age() {
        advanced.push(format!(
            "(AREA[MinimumAge]RANGE[MIN,{age} years] OR AREA[MinimumAge]MISSING) AND \
            (AREA[MaximumAge]RANGE[{age} years,MAX] OR AREA[MaximumAge]MISSING)"
        ));
    }
    advanced.extend(updated_range(query));
    if query.has_results() {
        advanced.push("AREA[ResultsFirstPostDate]RANGE[MIN,MAX]".into());
    }
    advanced.extend(why_stopped(query));
    advanced.join(" AND ")
}

fn updated_range(query: &TrialSearch) -> Option<String> {
    let since = query.updated_since();
    let until = query.updated_until();
    (since.is_some() || until.is_some()).then(|| {
        format!(
            "AREA[LastUpdatePostDate]RANGE[{},{}]",
            since.map_or("MIN".into(), ToString::to_string),
            until.map_or("MAX".into(), ToString::to_string)
        )
    })
}

/// Each stop reason word quoted, so registry operators in the caller's text stay plain words.
fn why_stopped(query: &TrialSearch) -> Option<String> {
    let words: Vec<_> = query.why_stopped_words().into_iter().map(quote).collect();
    (!words.is_empty()).then(|| format!("AREA[WhyStopped]({})", words.join(" AND ")))
}

pub(super) fn name_parameters(
    query: &TrialSearch,
    broader_names: bool,
    params: &mut BTreeMap<String, String>,
) {
    if let Some(value) = query.registry_page_token() {
        params.insert("pageToken".into(), value.into());
    }
    if broader_names {
        for (name, value) in [
            ("query.spons", query.sponsor()),
            ("query.locn", query.facility()),
        ] {
            if let Some(value) = value {
                params.insert(name.into(), value.to_string());
            }
        }
    }
}
/// SponsorSearch is the lead-or-collaborator area that query.spons searches.
fn sponsor_area(query: &TrialSearch) -> &'static str {
    if query.sponsor_role() == Some(&biomcp_types::SponsorRole::Any) {
        "SponsorSearch"
    } else {
        "LeadSponsorName"
    }
}
fn quote(value: &str) -> String {
    format!("\"{}\"", value.replace('\\', "\\\\").replace('"', "\\\""))
}

/// Exact expression sent to the registry's general text search, when used.
pub(super) fn search_term(query: &TrialSearch, gene: Option<&str>) -> Option<String> {
    let mut text = Vec::new();
    if let Some(value) = query.query() {
        text.push(value.as_str().to_owned());
    }
    if let Some(value) = query.variant() {
        text.push(value.as_str().to_owned());
    }
    if let Some(value) = gene {
        text.push(value.to_owned());
    }
    if !query.keyword().is_empty() {
        let keywords = if text.is_empty() {
            super::alternatives(query.keyword())
        } else {
            for value in &mut text {
                *value = format!("({value})");
            }
            format!("({})", super::alternatives(query.keyword()))
        };
        text.push(keywords);
    }
    (!text.is_empty()).then(|| text.join(" AND "))
}
