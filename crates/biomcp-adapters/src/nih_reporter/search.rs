//! One NIH RePORTER project search page and its checked reply.
use crate::mapped::Problems;
use crate::quotas;
use biodata::Text;
use biomcp_fetch::{BadReply, Endpoint, Freshness, Leg, Read, Reply, RequestError, SourceSpec};
use biomcp_types::{
    Grant, GrantInstitute, GrantInvestigator, GrantOrganization, GrantSearch, Problem, Source,
};
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Public NIH RePORTER API policy.
pub static SPEC: SourceSpec = SourceSpec {
    destinations: biomcp_fetch::Destinations::Primary,
    user_agent: None,
    terms: biomcp_fetch::Terms {
        name: "NIH RePORTER public data; at most one request per second, large jobs off-hours",
        url: "https://api.reporter.nih.gov/",
        usage: biomcp_types::Use::Open,
    },
    source: Source::NihReporter,
    base: "https://api.reporter.nih.gov",
    key: biomcp_fetch::KeyUse::None,
    quota: &quotas::NIH_REPORTER,
    deadline: Duration::from_secs(30),
    command_budget: Some(Duration::from_secs(35)),
};
const SEARCH: Endpoint = SPEC
    .post_json("/v2/projects/search")
    .idempotent()
    .fresh(Freshness::Days(1));

/// Text fields RePORTER matches: title, abstract and terms, as its own site searches.
const TEXT_FIELDS: &str = "projecttitle,abstracttext,terms";
/// The only fields asked for; abstracts and other grantee text stay at RePORTER.
const FIELDS: &[&str] = &[
    "ApplId",
    "ProjectNum",
    "SubprojectId",
    "CoreProjectNum",
    "FiscalYear",
    "ProjectTitle",
    "ActivityCode",
    "Organization",
    "AgencyIcAdmin",
    "AwardAmount",
    "PrincipalInvestigators",
    "ProjectStartDate",
    "ProjectEndDate",
    "ProjectDetailUrl",
];

#[derive(Serialize)]
struct Body<'a> {
    criteria: Criteria<'a>,
    include_fields: &'static [&'static str],
    offset: u32,
    limit: u32,
}
#[derive(Serialize)]
struct Criteria<'a> {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pi_names: Vec<Name<'a>>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pi_profile_ids: Vec<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    advanced_text_search: Option<TextSearch<'a>>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    org_names: Vec<&'a str>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    fiscal_years: Vec<u32>,
}
#[derive(Serialize)]
struct Name<'a> {
    any_name: &'a str,
}
#[derive(Serialize)]
struct TextSearch<'a> {
    operator: &'static str,
    search_field: &'static str,
    search_text: &'a str,
}

#[derive(Deserialize)]
struct Page {
    meta: Meta,
    results: Vec<Row>,
}
#[derive(Deserialize)]
struct Meta {
    total: u64,
}
#[derive(Deserialize)]
struct Row {
    project_num: Option<String>,
    subproject_id: Option<String>,
    core_project_num: Option<String>,
    fiscal_year: Option<u32>,
    project_title: Option<String>,
    activity_code: Option<String>,
    organization: Option<Organization>,
    agency_ic_admin: Option<Institute>,
    award_amount: Option<u64>,
    principal_investigators: Option<Vec<Investigator>>,
    project_start_date: Option<String>,
    project_end_date: Option<String>,
    project_detail_url: Option<String>,
}
#[derive(Deserialize)]
struct Organization {
    org_name: Option<String>,
    org_city: Option<String>,
    org_state: Option<String>,
    org_country: Option<String>,
}
#[derive(Deserialize)]
struct Institute {
    abbreviation: Option<String>,
    name: Option<String>,
}
#[derive(Deserialize)]
struct Investigator {
    profile_id: Option<u64>,
    full_name: Option<String>,
    is_contact_pi: Option<bool>,
}
impl Reply for Page {
    fn check(&self) -> Result<(), BadReply> {
        let rows = self
            .results
            .iter()
            .all(|row| present(row.project_num.as_ref()).is_some() && row.fiscal_year.is_some());
        if rows && u64::try_from(self.results.len()).is_ok_and(|n| n <= self.meta.total) {
            Ok(())
        } else {
            Err(BadReply)
        }
    }
}

/// One page of RePORTER award rows, RePORTER's total for the same criteria, and the fields
/// RePORTER sent that could not be read.
pub fn search(
    leg: &Leg<'_>,
    query: &GrantSearch,
) -> Result<(u64, Vec<Grant>, Vec<Problem>), RequestError> {
    let body = body(query);
    let received = leg.call(&SEARCH, biomcp_fetch::Input::json(&[], &body), Read::json())?;
    let page: Page = received.value;
    let mut problems = Vec::new();
    let mut rows = Vec::new();
    for row in page.results {
        let mut p = Problems {
            source: Source::NihReporter,
            record: crate::mapped::record(present(row.project_num.as_ref()).unwrap_or_default()),
            list: Vec::new(),
        };
        let grant = grant(&mut p, row);
        for problem in p.list {
            if !problems.contains(&problem) {
                problems.push(problem);
            }
        }
        rows.extend(grant);
    }
    Ok((page.meta.total, rows, problems))
}
fn body(query: &GrantSearch) -> Body<'_> {
    Body {
        criteria: Criteria {
            pi_names: query
                .investigator()
                .map(|name| Name {
                    any_name: name.as_str(),
                })
                .into_iter()
                .collect(),
            pi_profile_ids: query.investigator_id().into_iter().collect(),
            advanced_text_search: query.query().map(|text| TextSearch {
                operator: "and",
                search_field: TEXT_FIELDS,
                search_text: text.as_str(),
            }),
            org_names: query.organization().map(Text::as_str).into_iter().collect(),
            fiscal_years: query.fiscal_year().into_iter().collect(),
        },
        include_fields: FIELDS,
        offset: query.offset(),
        limit: query.limit(),
    }
}
/// Health asks for one known award row.
pub(crate) fn probe(leg: &Leg<'_>) -> Result<(), RequestError> {
    #[derive(Serialize)]
    struct Probe {
        criteria: ProbeCriteria,
        include_fields: &'static [&'static str],
        limit: u32,
    }
    #[derive(Serialize)]
    struct ProbeCriteria {
        project_nums: [&'static str; 1],
    }
    let body = Probe {
        criteria: ProbeCriteria {
            project_nums: ["5P50CA265826-04"],
        },
        include_fields: &["ApplId"],
        limit: 1,
    };
    leg.probe(&SEARCH, biomcp_fetch::Input::json(&[], &body))
}
/// RePORTER sends empty strings for values it does not have.
fn present(value: Option<&String>) -> Option<&str> {
    value.map(String::as_str).filter(|v| !v.trim().is_empty())
}
/// RePORTER dates are midnight timestamps; keep the date.
fn date(value: Option<&String>) -> Option<&str> {
    present(value).map(|v| v.split('T').next().unwrap_or(v))
}
fn grant(p: &mut Problems, row: Row) -> Option<Grant> {
    let organization = row.organization.as_ref().and_then(|org| {
        let organization = GrantOrganization {
            name: p.text("organization.org_name", present(org.org_name.as_ref())),
            city: p.text("organization.org_city", present(org.org_city.as_ref())),
            state: p.text("organization.org_state", present(org.org_state.as_ref())),
            country: p.text(
                "organization.org_country",
                present(org.org_country.as_ref()),
            ),
        };
        (organization != GrantOrganization::default()).then_some(organization)
    });
    let institute = row.agency_ic_admin.as_ref().and_then(|ic| {
        let abbreviation = p.checked_text(
            "agency_ic_admin.abbreviation",
            present(ic.abbreviation.as_ref()),
        )?;
        Some(GrantInstitute {
            abbreviation,
            name: p.text("agency_ic_admin.name", present(ic.name.as_ref())),
        })
    });
    let investigators = row
        .principal_investigators
        .iter()
        .flatten()
        .filter_map(|pi| {
            Some(GrantInvestigator {
                name: p.text(
                    "principal_investigators.full_name",
                    present(pi.full_name.as_ref()),
                )?,
                contact: pi.is_contact_pi == Some(true),
                profile_id: pi.profile_id,
            })
        })
        .collect();
    let project = p.checked_text("project_num", present(row.project_num.as_ref()));
    Some(Grant {
        project: p.required("project_num", project)?,
        subproject: p.checked_text("subproject_id", present(row.subproject_id.as_ref())),
        core_project: p.checked_text("core_project_num", present(row.core_project_num.as_ref())),
        fiscal_year: p.required("fiscal_year", row.fiscal_year)?,
        title: p.text("project_title", present(row.project_title.as_ref())),
        activity: p.checked_text("activity_code", present(row.activity_code.as_ref())),
        institute,
        organization,
        award_amount: row.award_amount,
        start: p.checked_text("project_start_date", date(row.project_start_date.as_ref())),
        end: p.checked_text("project_end_date", date(row.project_end_date.as_ref())),
        investigators,
        url: p.checked_text(
            "project_detail_url",
            present(row.project_detail_url.as_ref()),
        ),
    })
}
