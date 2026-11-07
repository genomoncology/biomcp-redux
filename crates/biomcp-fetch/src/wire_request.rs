//! Outgoing request construction keeps credentials outside prepared and stored identities.
use crate::{KeyPlace, KeyUse, Leg, prepared::Prepared};
use std::{io::Read, sync::mpsc, time::Duration};

type Reply = Result<ureq::http::Response<ureq::Body>, ureq::Error>;

/// Send and read the whole reply, so a stall in the body stays on this worker.
fn read_whole(request: ureq::RequestBuilder<ureq::typestate::WithBody>, body: &[u8], limit: u64) -> Reply {
    let (parts, mut reply) = request.send(body)?.into_parts();
    let mut bytes = Vec::new();
    reply
        .as_reader()
        .take(limit.saturating_add(1))
        .read_to_end(&mut bytes)
        .map_err(ureq::Error::from)?;
    Ok(ureq::http::Response::from_parts(
        parts,
        ureq::Body::builder().data(bytes),
    ))
}

impl Leg<'_> {
    pub(crate) fn wire_call(
        &self,
        prepared: &Prepared,
        accept: &str,
        headers: &[(&str, String)],
    ) -> Reply {
        self.wire_request(prepared, accept, headers)?
            .send(prepared.body.as_slice())
    }

    /// Keep a stalled first read open while one more request runs; the first finished reply wins.
    pub(crate) fn hedged_call(
        &self,
        prepared: &Prepared,
        accept: &str,
        stall: Duration,
        limit: u64,
    ) -> Reply {
        let (sender, replies) = mpsc::channel();
        let start = |sender: mpsc::Sender<Reply>| -> Result<(), ureq::Error> {
            let request = self.wire_request(prepared, accept, &[])?;
            let body = prepared.body.clone();
            std::thread::Builder::new()
                .name("source-read".into())
                .spawn(move || {
                    // A reply nobody waits for any more has nowhere to go.
                    let _ = sender.send(read_whole(request, &body, limit));
                })
                .map(|_| ())
                .map_err(ureq::Error::from)
        };
        start(sender.clone())?;
        if let Ok(reply) = replies.recv_timeout(stall.min(self.budget().remaining())) {
            return reply;
        }
        let permit = self.acquire_quota().ok().flatten();
        let mut pending = 1;
        if permit.is_some() {
            start(sender)?;
            pending = 2;
        }
        let mut last = Err(ureq::Error::Timeout(ureq::Timeout::Global));
        while pending > 0 {
            match replies.recv_timeout(self.budget().remaining()) {
                Ok(Ok(response)) => return Ok(response),
                Ok(Err(error)) => {
                    last = Err(error);
                    pending -= 1;
                }
                Err(_) => break,
            }
        }
        last
    }

    fn wire_request(
        &self,
        prepared: &Prepared,
        accept: &str,
        headers: &[(&str, String)],
    ) -> Result<ureq::RequestBuilder<ureq::typestate::WithBody>, ureq::Error> {
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
        Ok(request.config().timeout_global(Some(remaining)).build())
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
