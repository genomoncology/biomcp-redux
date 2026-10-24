//! Checked XML uses the shared request, cache and recording boundaries.
#![expect(
    clippy::disallowed_methods,
    reason = "Local HTTP fixtures and scoped public API callers"
)]
mod support;
use biomcp_fetch::{
    Arg, BadReply, Fetch, FetchConfig, Freshness, KeyUse, Quota, Rate, Reply, SourceKey,
    xml::MixedText,
};
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
        if self.text.as_str().contains("V600E") {
            Ok(())
        } else {
            Err(BadReply)
        }
    }
}
fn spec(server: &Server) -> &'static biomcp_fetch::SourceSpec {
    server.spec(
        Box::leak(Box::new(Quota::new(
            SourceKey::declare("xml"),
            Rate::per_second(1000),
            None,
            4,
        ))),
        KeyUse::None,
    )
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
        (
            spec.get("/answer"),
            "Application/XML; Charset=\"UTF-8\"",
            "\u{feff}<?xml version=\"1.0\" encoding=\"UTF-8\"?><answer>BRAF<sup>V600E</sup></answer>",
        ),
        (
            spec.post_form("/answer"),
            "application/provider+xml; version=1",
            "<!DOCTYPE answer SYSTEM \"http://127.0.0.1:1/never\"><answer>BRAF<sup>V600E</sup></answer>",
        ),
        (
            spec.get("/answer"),
            "text/xml",
            "<answer>BRAF<sup>V600E</sup></answer>",
        ),
    ] {
        thread::scope(|scope| {
            let worker = scope.spawn(|| {
                fetch
                    .leg(spec, fetch.deadline(Duration::from_secs(10)))
                    .xml::<Answer>(&ep, &[("q", Arg::Num(1))])
            });
            let request = server.next();
            assert!(
                request
                    .text
                    .to_ascii_lowercase()
                    .contains("accept: application/xml\r\n")
            );
            if request.text.starts_with("POST") {
                assert_eq!(request.body, b"q=1");
            }
            respond(request, media, body.as_bytes());
            assert_eq!(worker.join().unwrap().unwrap().text.as_str(), "BRAFV600E");
        });
    }
}

fn exchange(
    server: &Server,
    fetch: &Fetch,
    spec: &'static biomcp_fetch::SourceSpec,
    media: &str,
    body: &[u8],
) -> Result<Answer, biomcp_fetch::RequestError> {
    thread::scope(|scope| {
        let worker = scope.spawn(|| invoke(fetch, spec));
        respond(server.next(), media, body);
        worker.join().unwrap()
    })
}
fn invoke(
    fetch: &Fetch,
    spec: &'static biomcp_fetch::SourceSpec,
) -> Result<Answer, biomcp_fetch::RequestError> {
    fetch
        .leg(spec, fetch.deadline(Duration::from_secs(10)))
        .xml(&spec.get("/answer").fresh(Freshness::Days(1)), &[])
}
#[test]
fn invalid_xml_media_encodings_documents_and_reply_checks_are_input_free() {
    use biodata::Reason;
    let server = Server::new();
    let spec = spec(&server);
    let fetch = Fetch::open(FetchConfig::default()).unwrap();
    for (media, body) in [
        ("application/json", "<answer>V600E</answer>"),
        ("text/html+xml", "<answer>V600E</answer>"),
        ("text/xml; charset=iso-8859-1", "<answer>V600E</answer>"),
        (
            "text/xml; charset=utf-8; charset=utf-16",
            "<answer>V600E</answer>",
        ),
        (
            "text/xml",
            "<?xml version=\"1.0\" encoding=\"UTF-16\"?><answer>V600E</answer>",
        ),
        ("text/xml", "<answer>V600E"),
        ("text/xml", "<answer>V600E</wrong>"),
        ("text/xml", "<answer>V600E</answer><extra/>"),
        ("text/xml", "<answer>V600E</answer>private-error"),
        (
            "text/xml",
            "<answer><![CDATA[V600E]]></answer><![CDATA[tail]]>",
        ),
        ("text/xml", "<answer>V600E&custom;</answer>"),
        (
            "text/xml",
            "<!DOCTYPE answer [<!ENTITY custom 'V600E'>]><answer>&custom;</answer>",
        ),
        ("text/xml", "<answer attr=\"&custom;\">V600E</answer>"),
        ("text/xml", "<answer>V600E<!--bad--comment--></answer>"),
        ("text/xml", "<answer>private-error</answer>"),
        ("text/xml", "<answer>V600E</answer><!DOCTYPE answer>"),
        (
            "text/xml",
            "<!DOCTYPE answer><!DOCTYPE answer><answer>V600E</answer>",
        ),
        ("text/xml", " <?xml version=\"1.0\"?><answer>V600E</answer>"),
        (
            "text/xml",
            "<?xml version=\"1.0\"?><?xml version=\"1.0\"?><answer>V600E</answer>",
        ),
        ("text/xml", "&amp;<answer>V600E</answer>"),
        ("text/xml", ""),
    ] {
        let error = exchange(&server, &fetch, spec, media, body.as_bytes()).unwrap_err();
        assert!(
            matches!(error, biomcp_fetch::RequestError::Source(e) if e.reason == Reason::BadReply),
            "{body}: {error}"
        );
        assert!(!format!("{error:?} {error}").contains("private-error"));
    }
    let error = exchange(
        &server,
        &fetch,
        spec,
        "text/xml",
        b"<answer>V600E\xff</answer>",
    )
    .unwrap_err();
    assert!(matches!(error, biomcp_fetch::RequestError::Source(e) if e.reason == Reason::BadReply));
}
#[test]
fn xml_body_and_nested_inline_content_are_bounded() {
    use biodata::Reason;
    let server = Server::new();
    let spec = spec(&server);
    for (max_body, body) in [
        (16, "<answer>V600E</answer>".into()),
        (
            16 * 1024,
            format!(
                "<answer>{}V600E{}</answer>",
                "<i>".repeat(80),
                "</i>".repeat(80)
            ),
        ),
    ] {
        let fetch = Fetch::open(FetchConfig {
            max_body,
            ..FetchConfig::default()
        })
        .unwrap();
        let error = exchange(&server, &fetch, spec, "text/xml", body.as_bytes()).unwrap_err();
        assert!(
            matches!(error, biomcp_fetch::RequestError::Source(e) if e.reason == Reason::BadReply)
        );
    }
    // This remains below the document bound and exercises the actual recursive
    // MixedText visitor's deserializer limit independently of that preflight.
    let body = format!(
        "<answer>{}V600E{}</answer>",
        "<i>".repeat(40),
        "</i>".repeat(40)
    );
    let fetch = Fetch::open(FetchConfig::default()).unwrap();
    assert!(exchange(&server, &fetch, spec, "text/xml", body.as_bytes()).is_err());
}
