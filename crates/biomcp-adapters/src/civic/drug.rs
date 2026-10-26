//! Source-local therapy candidates; no chemical identity join is inferred.
use super::{GENES, variant::failure, variant_raw::Connection};
use biodata::{DrugIdentityLevel, Text};
use biomcp_fetch::{BadReply, Leg, Reply, RequestError};
use biomcp_types::{DrugHeader, DrugPage, DrugSource, Reason};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct SearchReply {
    data: SearchData,
    errors: Option<Vec<serde::de::IgnoredAny>>,
}
#[derive(Deserialize)]
struct SearchData {
    therapies: Connection<Therapy>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Therapy {
    pub id: u64,
    pub name: Text,
    pub link: Text,
}
impl Therapy {
    pub(super) fn check(&self) -> Result<(), BadReply> {
        if self.id == 0
            || self.id > i32::MAX as u64
            || self.link.as_str() != format!("/therapies/{}", self.id)
        {
            return Err(BadReply);
        }
        Ok(())
    }
    pub(super) fn header(self) -> Result<DrugHeader, RequestError> {
        Ok(DrugHeader {
            id: self
                .id
                .to_string()
                .parse()
                .map_err(|_| failure(Reason::BadReply))?,
            source: DrugSource::Civic,
            version: None,
            name: Some(self.name),
            identity_level: DrugIdentityLevel::Unknown,
            r#type: Some(Text::new("Therapy").map_err(|_| failure(Reason::BadReply))?),
            url: Some(
                Text::new(&format!("https://civicdb.org{}", self.link.as_str()))
                    .map_err(|_| failure(Reason::BadReply))?,
            ),
        })
    }
}
impl Reply for SearchReply {
    fn check(&self) -> Result<(), BadReply> {
        if self.errors.as_ref().is_some_and(|rows| !rows.is_empty()) {
            return Err(BadReply);
        }
        self.data.therapies.check()?;
        for row in &self.data.therapies.nodes {
            row.check()?;
        }
        Ok(())
    }
}
#[derive(Serialize)]
struct SearchRequest<'a> {
    query: String,
    variables: SearchVariables<'a>,
}
#[derive(Serialize)]
struct SearchVariables<'a> {
    q: &'a Text,
    after: Option<&'a Text>,
}
/// Search literal native therapy names in published order, without selecting a candidate.
/// Offset walking uses bounded source pages and checks cursor progress before requesting more.
pub fn drug_search(
    leg: &Leg<'_>,
    text: &Text,
    limit: u32,
    offset: u32,
) -> Result<DrugPage, RequestError> {
    if !(1..=50).contains(&limit) {
        return Err(failure(Reason::BadReply));
    }
    let mut after = None;
    let mut skipped = 0;
    loop {
        let count = if skipped < offset {
            (offset - skipped).min(50)
        } else {
            limit
        };
        let body = SearchRequest {
            query: format!(
                "query($q:String!,$after:String){{therapies(name:$q,first:{count},after:$after){{totalCount \
                    pageInfo{{hasNextPage endCursor}} nodes{{id name link ncitId therapyAliases deprecated}}}}}}"
            ),
            variables: SearchVariables {
                q: text,
                after: after.as_ref(),
            },
        };
        let reply: SearchReply =
            leg.post_json_checked_cache(&GENES, &body, |reply: &SearchReply| {
                reply.data.therapies.nodes.len() <= count as usize
                    && reply.data.therapies.progresses(after.as_ref())
            })?;
        let rows = reply.data.therapies;
        if rows.nodes.len() > count as usize || !rows.progresses(after.as_ref()) {
            return Err(failure(Reason::BadReply));
        }
        if skipped >= offset || !rows.page_info.has_next_page {
            let items = if skipped < offset {
                Vec::new()
            } else {
                rows.nodes
                    .into_iter()
                    .map(Therapy::header)
                    .collect::<Result<Vec<_>, _>>()?
            };
            return Ok(DrugPage {
                items: items.into(),
                total: Some(rows.total_count),
            });
        }
        skipped = skipped
            .checked_add(rows.nodes.len() as u32)
            .ok_or_else(|| failure(Reason::TooLarge))?;
        after = rows.page_info.end_cursor;
    }
}
