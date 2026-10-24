//! Checked XML uses the shared request, cache and recording boundaries.
#![expect(clippy::disallowed_methods, reason = "Local HTTP fixtures and scoped public API callers")]
mod support;
use biomcp_fetch::{Arg, BadReply, Fetch, FetchConfig, Freshness, KeyUse, Quota, Rate, Reply, SourceKey, xml::MixedText};
use serde::Deserialize;
use std::{thread, time::Duration};
type Server = support::Server;
#[derive(Debug, Deserialize)]
struct Answer {
    #[serde(rename = "$value", default)]
    text: MixedText,
}
impl Reply for Answer {
    fn check(&self) -> Result<(), BadReply> {
        if self.text.as_str().contains("V600E") { Ok(()) } else { Err(BadReply) }
    }
}
fn spec(server: &Server) -> &'static biomcp_fetch::SourceSpec {
    server.spec(Box::leak(Box::new(Quota::new(SourceKey::declare("xml"), Rate::per_second(1000), None, 4))), KeyUse::None)
}
fn respond(request: support::Request<()>, media: &str, body: &[u8]) {
    let mut response = format!("HTTP/1.1 200 OK\r\nContent-Type: {media}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n", body.len()).into_bytes();
    response.extend_from_slice(body);
    request.bytes(response);
}
#[test]
fn checked_get_and_form_xml_preserve_text_and_select_media() {
    let server = Server::new();
    let spec = spec(&server);
    let fetch = Fetch::open(FetchConfig::default()).unwrap();
    for (ep, media, body) in [
        (spec.get("/answer"), "Application/XML; Charset=\"UTF-8\"", "\u{feff}<?xml version=\"1.0\" encoding=\"UTF-8\"?><answer>BRAF<sup>V600E</sup></answer>"),
        (spec.post_form("/answer"), "application/provider+xml; version=1", "<!DOCTYPE answer SYSTEM \"http://127.0.0.1:1/never\"><answer>BRAF<sup>V600E</sup></answer>"),
        (spec.get("/answer"), "text/xml", "<answer>BRAF<sup>V600E</sup></answer>"),
    ] {
        thread::scope(|scope| {
            let worker = scope.spawn(|| fetch.leg(spec, fetch.deadline(Duration::from_secs(10))).xml::<Answer>(&ep, &[("q", Arg::Num(1))]));
            let request = server.next();
            assert!(request.text.to_ascii_lowercase().contains("accept: application/xml\r\n"));
            if request.text.starts_with("POST") { assert_eq!(request.body, b"q=1"); }
            respond(request, media, body.as_bytes());
            assert_eq!(worker.join().unwrap().unwrap().text.as_str(), "BRAFV600E");
        });
    }
}
