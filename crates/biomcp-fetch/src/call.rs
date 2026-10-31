//! One request entry point shares preparation and checked response policies.
use crate::{Arg, Endpoint, Leg, Read, Reply, ReplyMetadata, RequestError};

/// A bounded JSON body serialized only after request validation.
pub struct JsonBody<'a> {
    encode: Box<dyn Fn(u64) -> Result<Vec<u8>, crate::InputError> + 'a>,
}
impl<'a> JsonBody<'a> {
    /// Borrow a serializable body without changing its wire representation.
    pub fn new<B: serde::Serialize + 'a>(body: &'a B) -> Self {
        Self {
            encode: Box::new(move |limit| crate::prepared::json(body, limit)),
        }
    }
}
/// Request parameters and an optional bounded JSON body.
pub enum Input<'a> {
    /// Canonical query parameters or form fields, as declared by the endpoint.
    Params(&'a [(&'a str, Arg<'a>)]),
    /// Canonical query parameters and a JSON POST body.
    Json {
        /// Query arguments use the same canonicalization as parameter reads.
        params: &'a [(&'a str, Arg<'a>)],
        /// The body is encoded within the transport body bound.
        body: JsonBody<'a>,
    },
}
/// A checked value and the metadata of the response that supplied it.
pub struct Received<T> {
    /// Decoded and checked response.
    pub value: T,
    /// Identity, acquisition time and provider paging metadata.
    pub meta: ReplyMetadata,
}
/// Convert only source not-found errors into an absent record.
pub trait Absent<T> {
    /// Keep every failure other than a source not-found error.
    fn absent(self) -> Result<Option<T>, RequestError>;
}
impl<T> Absent<T> for Result<T, RequestError> {
    fn absent(self) -> Result<Option<T>, RequestError> {
        match self {
            Err(RequestError::Source(error)) if error.reason == biomcp_types::Reason::NotFound => {
                Ok(None)
            }
            result => result.map(Some),
        }
    }
}
impl Leg<'_> {
    /// Read a response through the shared checked cache, replay and transport path.
    pub fn call<T: Reply>(
        &self,
        endpoint: &Endpoint,
        input: Input<'_>,
        mut read: Read<'_, T>,
    ) -> Result<Received<T>, RequestError> {
        if let Some(limit) = read.limit {
            if limit == 0 || limit == u64::MAX {
                return Err(crate::InputError::BodyLimit.into());
            }
            if !read.binary() {
                read.limit = Some(limit.min(self.fetch.config.max_body));
            }
        }
        let endpoint = if read.exact_count() {
            endpoint.clone().exact_count()
        } else {
            endpoint.clone()
        };
        let (value, meta) = match input {
            Input::Params(params) => self.parameters(&endpoint, params, read)?,
            Input::Json { params, body } => {
                self.validate(&endpoint, true)?;
                let body = (body.encode)(self.fetch.config.max_body)?;
                let url = if endpoint.publisher {
                    endpoint.path.to_string()
                } else {
                    format!("{}{}", self.spec.base.trim_end_matches('/'), endpoint.path)
                };
                let url = crate::retry::canonical(&url, params, self.spec.key);
                self.send(
                    &endpoint,
                    crate::prepared::Prepared::new(url, body, endpoint.method, endpoint.idempotent),
                    read,
                )?
            }
        };
        Ok(Received { value, meta })
    }
}
