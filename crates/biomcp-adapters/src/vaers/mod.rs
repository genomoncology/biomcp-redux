//! Explicit CDC WONDER summaries; no report retrieval or vaccine-name bridge.
mod encoding;
mod raw;
use biomcp_fetch::{Arg, BadReply, Freshness, KeyUse, Leg, SourceSpec, Terms};
use biomcp_types::{Source, Use, VaersSection, VaersSummary};
use std::time::Duration;
/// Registry-owned restricted statistical use only; no re-identification.
/// This compatibility setting applies only to CDC WONDER.
pub static SPEC: SourceSpec = SourceSpec {
    user_agent: Some("Wget/1.21.4"),
    source: Source::Vaers,
    base: "https://wonder.cdc.gov",
    terms: Terms {
        name: "CDC WONDER data use restrictions",
        url: "https://wonder.cdc.gov/datause.html",
        usage: Use::Restricted,
    },
    quota: &crate::quotas::WONDER,
    key: KeyUse::None,
    deadline: Duration::from_secs(30),
};
const TEMPLATE: &str = include_str!("request.xml");
/// Read exactly one requested table through checked shared cache admission.
pub fn read(
    leg: &Leg<'_>,
    vaccine: &str,
    section: VaersSection,
) -> Result<VaersSummary, biomcp_fetch::RequestError> {
    let xml = TEMPLATE
        .replace(
            "<name>F_D8.V14</name><value>MMR</value>",
            &format!("<name>F_D8.V14</name><value>{vaccine}</value>"),
        )
        .replace(
            "<name>B_1</name><value>D8.V13-level2</value>",
            &format!("<name>B_1</name><value>{}</value>", section.code()),
        );
    let endpoint = SPEC
        .post_form("/controller/datarequest/D8")
        .fresh(Freshness::Days(1));
    let (reply, _) = leg.xml_received_checked::<raw::Native>(
        &endpoint,
        &[
            ("request_xml", Arg::Text(&xml)),
            ("accept_datause_restrictions", Arg::Text("true")),
        ],
        |v| {
            if v.0
                .vaccine
                .get("code")
                .is_some_and(|c| c.as_ref() == vaccine)
                && v.0
                    .grouping
                    .get("code")
                    .is_some_and(|c| c.as_ref() == section.code())
            {
                Ok(())
            } else {
                Err(BadReply)
            }
        },
    )?;
    Ok(reply.0)
}

/// Small read-only publisher probe; never an export or synchronization.
pub(crate) const HEALTH_PATH: &str = "/datause.html";
