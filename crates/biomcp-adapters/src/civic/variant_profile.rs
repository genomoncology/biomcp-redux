//! Resolve an exact CIViC profile name through the source's substring search.
use super::{GENES, variant::failure, variant_raw as raw};
use biodata::Text;
use biomcp_fetch::{BadReply, Input, Leg, Read, Reply, RequestError};
use biomcp_types::Reason;
use serde::Deserialize;

#[derive(Deserialize)]
struct Response {
    data: Data,
    errors: Option<Vec<serde::de::IgnoredAny>>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Data {
    molecular_profiles: raw::Connection<Profile>,
}
#[derive(Deserialize)]
struct Profile {
    id: u64,
    name: String,
}
impl Reply for Response {
    fn check(&self) -> Result<(), BadReply> {
        if self
            .errors
            .as_ref()
            .is_some_and(|errors| !errors.is_empty())
        {
            return Err(BadReply);
        }
        self.data.molecular_profiles.check()
    }
}

pub(super) fn exact_id(leg: &Leg<'_>, name: &Text) -> Result<Option<u64>, RequestError> {
    let mut after = None;
    loop {
        let body = serde_json::json!({
            "query": "query($name:String!,$after:String){molecularProfiles(name:$name,first:50,after:$after){totalCount \
                pageInfo{hasNextPage endCursor} nodes{id name}}}",
            "variables": {"name": name, "after": after}
        });
        let reply: Response = leg
            .call(
                &GENES,
                Input::json(&[], &body),
                Read::json().cache_if(&|reply: &Response| {
                    reply.data.molecular_profiles.progresses(after.as_ref())
                }),
            )?
            .value;
        let profiles = reply.data.molecular_profiles;
        if !profiles.progresses(after.as_ref()) {
            return Err(failure(Reason::BadReply));
        }
        if let Some(profile) = profiles
            .nodes
            .into_iter()
            .find(|profile| profile.name == name.as_str())
        {
            return if raw::id(profile.id) {
                Ok(Some(profile.id))
            } else {
                Err(failure(Reason::BadReply))
            };
        }
        if !profiles.page_info.has_next_page {
            return Ok(None);
        }
        after = profiles.page_info.end_cursor;
    }
}
