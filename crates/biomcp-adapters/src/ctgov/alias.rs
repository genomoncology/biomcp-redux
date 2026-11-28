//! Confirm an obsolete ID using the registry's alias index and returned identification fields.
use biodata::NctId;
use biomcp_fetch::{Arg, BadReply, Input, Leg, Read, Reply, RequestError};
use biomcp_types::SourceNotice;
use serde::Deserialize;

pub(super) fn resolve(leg: &Leg<'_>, alias: NctId) -> Result<Option<NctId>, RequestError> {
    let filter = format!("AREA[NCTIdAlias]{alias}");
    let params = [
        ("countTotal", Arg::Text("true")),
        ("fields", Arg::Text("NCTId,NCTIdAlias")),
        ("filter.advanced", Arg::Text(&filter)),
        ("pageSize", Arg::Text("1")),
    ];
    let reply = leg.call(
        &super::SEARCH,
        Input::Params(&params),
        Read::<Page>::json().check(&|page| page.canonical(alias).map(|_| ())),
    )?;
    reply
        .value
        .canonical(alias)
        .map_err(|_| biomcp_fetch::bad_reply(super::SPEC.source, Some(200)))
}

/// Explain the registry-confirmed mapping in a get or search reply.
pub fn alias_notice(alias: NctId, canonical: NctId) -> SourceNotice {
    SourceNotice::RequestGuidance {
        data_date: None,
        message: format!("ClinicalTrials.gov lists {alias} as an obsolete alias for {canonical}.")
            .into(),
        retry_after_seconds: None,
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Page {
    total_count: u64,
    studies: Option<Vec<Study>>,
}

impl Reply for Page {
    fn check(&self) -> Result<(), BadReply> {
        Ok(())
    }
}

impl Page {
    fn canonical(&self, alias: NctId) -> Result<Option<NctId>, BadReply> {
        match (
            self.total_count,
            self.studies.as_deref().unwrap_or_default(),
        ) {
            (0, []) => Ok(None),
            (1, [study]) => {
                let id = &study.protocol_section.identification_module;
                (id.nct_id != alias && id.nct_id_aliases.contains(&alias))
                    .then_some(Some(id.nct_id))
                    .ok_or(BadReply)
            }
            _ => Err(BadReply),
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Study {
    protocol_section: Protocol,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Protocol {
    identification_module: Identification,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Identification {
    nct_id: NctId,
    nct_id_aliases: Vec<NctId>,
}
