//! Bounded native UniProt pagination metadata; adapters admit the continuation identity.
use serde::{Deserialize, Serialize};
/// A publisher-given exact total and optional native next URL.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Pagination {
    /// Exact native count, never inferred from the returned page.
    pub total: u64,
    /// Source-supplied next URL; never followed without source-specific checks.
    pub next: Option<String>,
}
impl Pagination {
    pub(crate) fn parse(total: &str, link: Option<&str>) -> Result<Self, crate::BadReply> {
        if total.is_empty() || !total.bytes().all(|c| c.is_ascii_digit()) {
            return Err(crate::BadReply);
        }
        let total = total.parse().map_err(|_| crate::BadReply)?;
        let mut next = None;
        if let Some(link) = link {
            if link.len() > 4096 || !link.bytes().all(|b| b.is_ascii() && !b.is_ascii_control()) {
                return Err(crate::BadReply);
            }
            for part in link.split('<').skip(1) {
                let (target, parameters) = part.split_once('>').ok_or(crate::BadReply)?;
                if parameters
                    .split(';')
                    .any(|v| v.trim().trim_end_matches(',') == "rel=\"next\"")
                {
                    if next.is_some() {
                        return Err(crate::BadReply);
                    }
                    next = Some(target.to_owned());
                }
            }
        }
        let value = Self { total, next };
        if value.valid() {
            Ok(value)
        } else {
            Err(crate::BadReply)
        }
    }
    pub(crate) fn valid(&self) -> bool {
        self.next.as_ref().is_none_or(|v| {
            !v.is_empty()
                && v.len() <= 4096
                && v.bytes().all(|b| b.is_ascii() && !b.is_ascii_control())
        })
    }
}

impl crate::Leg<'_> {
    pub(crate) fn pagination_headers(
        &self,
        prepared: &crate::prepared::Prepared,
        response: &ureq::http::Response<ureq::Body>,
        status: u16,
    ) -> Result<Option<Pagination>, crate::request::Failure> {
        if self.spec.source != biomcp_types::Source::UniProt
            || prepared
                .url
                .split('?')
                .next()
                .and_then(|url| url.strip_prefix(self.spec.base))
                != Some("/uniprotkb/search")
        {
            return Ok(None);
        }
        let total = response
            .headers()
            .get("x-total-results")
            .and_then(|v| v.to_str().ok());
        let link = response.headers().get("link").and_then(|v| v.to_str().ok());
        self.contain_response(prepared, &(total, link), &[], status)
            .map_err(crate::request::Failure::Local)?;
        total
            .map(|total| {
                Pagination::parse(total, link).map_err(|_| {
                    crate::request::Failure::Local(
                        self.failure(biomcp_types::Reason::BadReply, Some(status))
                            .into(),
                    )
                })
            })
            .transpose()
    }
}

impl crate::SourceSpec {
    /// Extract one native cursor only when origin, endpoint and every other selector stay fixed.
    /// The returned cursor remains opaque; this method never follows the supplied link.
    pub fn next_cursor(
        &'static self,
        text: &str,
        path: &str,
        args: &[(&str, crate::Arg<'_>)],
        cursor: &str,
    ) -> Result<String, crate::BadReply> {
        if text.len() > 4096 {
            return Err(crate::BadReply);
        }
        let next = url::Url::parse(text).map_err(|_| crate::BadReply)?;
        let expected = url::Url::parse(&format!("{}{path}", self.base.trim_end_matches('/')))
            .map_err(|_| crate::BadReply)?;
        if next.origin() != expected.origin()
            || next.path() != expected.path()
            || !next.username().is_empty()
            || next.password().is_some()
            || next.fragment().is_some()
        {
            return Err(crate::BadReply);
        }
        let mut pairs = std::collections::BTreeMap::new();
        for (key, value) in next.query_pairs() {
            if pairs.insert(key.into_owned(), value.into_owned()).is_some() {
                return Err(crate::BadReply);
            }
        }
        let value = pairs
            .remove(cursor)
            .filter(|v| !v.is_empty() && v.trim() == v)
            .ok_or(crate::BadReply)?;
        let expected: std::collections::BTreeMap<_, _> = args
            .iter()
            .filter(|(key, _)| *key != cursor)
            .map(|(key, arg)| ((*key).to_owned(), arg.value()))
            .collect();
        if pairs != expected {
            return Err(crate::BadReply);
        }
        Ok(value)
    }
}
