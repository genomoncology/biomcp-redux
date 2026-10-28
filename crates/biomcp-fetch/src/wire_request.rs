//! Outgoing request construction keeps credentials outside prepared and stored identities.
use crate::{KeyPlace, KeyUse, Leg, prepared::Prepared};

impl Leg<'_> {
    pub(crate) fn wire_call(
        &self,
        prepared: &Prepared,
        accept: &str,
        headers: &[(&str, String)],
    ) -> Result<ureq::http::Response<ureq::Body>, ureq::Error> {
        if prepared.publisher {
            return self.publisher_call(&prepared.url, accept);
        }
        let agent = self.fetch.agents.for_url(
            &self.fetch.config.network,
            self.fetch.config.proxy.is_some(),
            &prepared.url,
        );
        let outgoing = self.credential_url(&prepared.url);
        let url = outgoing.as_deref().unwrap_or(&prepared.url);
        let mut request = if prepared.post.is_some() {
            agent.post(url)
        } else if prepared.method == crate::declaration::Method::Head {
            agent.head(url).force_send_body()
        } else {
            agent.get(url).force_send_body()
        }
        .header("Accept", accept);
        if let Some(user_agent) = self.spec.user_agent {
            request = request.header("User-Agent", user_agent);
        }
        if prepared.exact_count {
            request = request.header("Prefer", "count=exact");
        }
        for (name, value) in &prepared.outgoing {
            request = request.query(name, value.value());
        }
        if prepared.identity_encoding {
            request = request.header("Accept-Encoding", "identity");
        }
        for (name, value) in headers {
            request = request.header(*name, value);
        }
        if let KeyUse::Optional(c) | KeyUse::Required(c) = self.spec.key
            && let Some(secret) = self.fetch.secrets.get(c.secret)
        {
            request = match c.place {
                KeyPlace::Query(_) => request,
                KeyPlace::Header(name) => request.header(name, secret),
                KeyPlace::Bearer => request.header("Authorization", format!("Bearer {secret}")),
            };
        }
        if let Some(post) = &prepared.post {
            request = request.header("Content-Type", &post.request_content_type);
        }
        let remaining = self.budget().remaining();
        if remaining.is_zero() {
            return Err(ureq::Error::Timeout(ureq::Timeout::Global));
        }
        request
            .config()
            .timeout_global(Some(remaining))
            .build()
            .send(prepared.body.as_slice())
    }

    // Build the secret URL only at the wire. Prepared URLs and identities stay keyless.
    fn credential_url(&self, url: &str) -> Option<String> {
        let credential = match self.spec.key {
            KeyUse::Optional(credential) | KeyUse::Required(credential) => credential,
            KeyUse::None => return None,
        };
        let KeyPlace::Query(name) = credential.place else {
            return None;
        };
        let secret = self.fetch.secrets.get(credential.secret)?;
        let pair = url::form_urlencoded::Serializer::new(String::new())
            .append_pair(name, secret)
            .finish();
        Some(match url.split_once('?') {
            Some((base, query)) => format!("{base}?{pair}&{query}"),
            None => format!("{url}?{pair}"),
        })
    }
}
