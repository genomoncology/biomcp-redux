//! The trusted hosting wrapper checks one header credential on its loopback listener.
use super::{Service, wire::Request};
use crate::key_check::Source;
use biomcp_fetch::Secrets;
use serde::Deserialize;
use serde_json::{Value, json};

pub(super) const PATH: &str = "/internal/key-check";

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Check<'a> {
    source: &'a str,
}

impl Service {
    pub(super) fn key_check(
        &self,
        request: &Request,
    ) -> (u16, Option<Value>, Option<&'static str>) {
        if request.method != "POST" {
            return (405, Some(json!({"error":"Method not allowed"})), None);
        }
        let Some(source) = serde_json::from_slice::<Check<'_>>(&request.body)
            .ok()
            .and_then(|input| Source::parse(input.source))
        else {
            return (400, Some(json!({"error":"Invalid key check source"})), None);
        };
        let mut secrets = Secrets::default();
        if let Source::Keyed(name) = source {
            match request.source_key(name) {
                Ok(Some(key)) => secrets.insert(name, key),
                Ok(None) => {}
                Err(_) => {
                    return (
                        400,
                        Some(json!({"error":"Invalid source key headers"})),
                        None,
                    );
                }
            }
        }
        let fetch = self
            .fetch
            .as_ref()
            .ok()
            .map(|fetch| fetch.with_secrets(secrets));
        let result = source.run(fetch.as_ref(), &self.settings);
        (200, Some(json!(result)), None)
    }
}
