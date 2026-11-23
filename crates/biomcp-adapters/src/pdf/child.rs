//! A PDF read in a child process of BioMCP's own binary, when the binary provides one (1075).
//!
//! The pre-load measuring in [`super::scan`] bounds every route found so far, but three reviews
//! each found a new way to make it and lopdf read a file differently. So the binary reads each
//! PDF in a child whose memory the operating system caps and which it kills at the time limit;
//! a hole nobody has found then costs at most the cap. The child gets the PDF's bytes on
//! standard input and writes one header line of JSON, then the text. A child that is killed,
//! crashes or answers in any other form becomes the `refused` state, never a guess.

use super::embedded::{Embedded, Pages, extract};
use biomcp_types::{PdfMissing, PdfTextState, Reason};
use serde_json::{Value, json};
use std::num::NonZeroU32;

/// Most bytes a child's answer may hold: the whole text and its header.
pub const MAX_OUTPUT: usize = super::MAX_EXPANDED_BYTES + 4096;

/// The child's side: extract the requested pages and encode the result for the parent.
pub fn serve(input: &[u8]) -> Vec<u8> {
    let read = request(input).and_then(|(pages, bytes)| extract(bytes, pages, &|| false));
    let (header, text) = match read {
        Ok(read) => {
            let mut header = serde_json::Map::from_iter([
                ("state".into(), json!(read.state)),
                ("pages".into(), json!(read.pages)),
                ("page_starts".into(), json!(read.page_starts)),
                ("missing".into(), json!(read.missing.map(missing_name))),
            ]);
            if read.private_symbol && read.state.readable() {
                header.insert("private_symbol".into(), true.into());
            }
            (Value::Object(header), read.text)
        }
        Err(reason) => (json!({ "error": reason }), String::new()),
    };
    let mut out = header.to_string().into_bytes();
    out.push(b'\n');
    out.extend(text.into_bytes());
    out
}

/// The PDF after the parent's page selection.
pub fn input_pdf(input: &[u8]) -> Result<&[u8], Reason> {
    request(input).map(|(_, pdf)| pdf)
}

fn request(input: &[u8]) -> Result<(Pages, &[u8]), Reason> {
    if let [b'w', a, b, c, d, b'\n', pdf @ ..] = input {
        let first =
            NonZeroU32::new(u32::from_be_bytes([*a, *b, *c, *d])).ok_or(Reason::BadReply)?;
        return Ok((Pages::Window(first), pdf));
    }
    let line = input
        .iter()
        .take(8)
        .position(|byte| *byte == b'\n')
        .ok_or(Reason::BadReply)?;
    let (head, rest) = input.split_at(line);
    let pages = std::str::from_utf8(head)
        .map_err(|_| Reason::BadReply)?
        .parse()
        .map_err(|_| Reason::BadReply)?;
    Ok((Pages::Whole(pages), rest.get(1..).ok_or(Reason::BadReply)?))
}

/// The parent's side: read the selected pages in an admitted worker if configured.
pub(super) fn read(
    isolated: Option<&biomcp_fetch::IsolatedRead<'_>>,
    bytes: &[u8],
    pages: Pages,
    deadline: &biomcp_fetch::LocalDeadline,
) -> Result<Embedded, Reason> {
    let Some(isolated) = isolated else {
        return extract(bytes, pages, &|| deadline.expired());
    };
    let mut input = match pages {
        Pages::Whole(limit) => format!("{limit}\n").into_bytes(),
        Pages::Window(first) => {
            let [a, b, c, d] = first.get().to_be_bytes();
            vec![b'w', a, b, c, d, b'\n']
        }
    };
    input.extend_from_slice(bytes);
    match isolated.run(&input, MAX_OUTPUT, deadline) {
        Ok(answer) => decoded(&answer, pages).unwrap_or_else(|Malformed| Ok(refused())),
        Err(biomcp_fetch::Contained) => Ok(refused()),
    }
}

/// A read stopped at the memory cap or the time limit, or that failed in the child.
fn refused() -> Embedded {
    Embedded {
        pages: None,
        state: PdfTextState::Refused,
        text: String::new(),
        page_starts: Vec::new(),
        missing: None,
        private_symbol: false,
    }
}

/// An answer not in the child's form.
struct Malformed;

/// The child's answer.
fn decoded(answer: &[u8], selection: Pages) -> Result<Result<Embedded, Reason>, Malformed> {
    let line = answer
        .iter()
        .position(|byte| *byte == b'\n')
        .ok_or(Malformed)?;
    let (head, text) = answer.split_at(line);
    let header: Value = serde_json::from_slice(head).map_err(|_| Malformed)?;
    if let Some(reason) = header.get("error") {
        return Ok(Err(
            serde_json::from_value(reason.clone()).map_err(|_| Malformed)?
        ));
    }
    let state: PdfTextState = serde_json::from_value(header.get("state").ok_or(Malformed)?.clone())
        .map_err(|_| Malformed)?;
    let pages = match header.get("pages") {
        Some(Value::Number(pages)) => {
            let pages = pages.as_u64().ok_or(Malformed)?;
            Some(u32::try_from(pages).map_err(|_| Malformed)?)
        }
        _ => None,
    };
    let missing = match header.get("missing") {
        Some(Value::String(name)) => Some(missing_value(name).ok_or(Malformed)?),
        _ => None,
    };
    let text =
        String::from_utf8(text.get(1..).ok_or(Malformed)?.to_vec()).map_err(|_| Malformed)?;
    let page_starts: Vec<usize> =
        serde_json::from_value(header.get("page_starts").ok_or(Malformed)?.clone())
            .map_err(|_| Malformed)?;
    let expected = pages.and_then(|n| selection.range(n as usize));
    if state.readable()
        && (Some(page_starts.len()) != expected.map(|range| range.len())
            || page_starts.first() != Some(&0)
            || !page_starts.is_sorted()
            || !page_starts
                .iter()
                .all(|start| text.is_char_boundary(*start)))
    {
        return Err(Malformed);
    }
    Ok(Ok(Embedded {
        pages,
        state,
        text,
        page_starts,
        missing,
        private_symbol: match header.get("private_symbol") {
            Some(Value::Bool(value)) => *value,
            None => false,
            _ => return Err(Malformed),
        },
    }))
}

const fn missing_name(missing: PdfMissing) -> &'static str {
    match missing {
        PdfMissing::Gibberish => "gibberish",
        PdfMissing::Skipped => "skipped",
        PdfMissing::Both => "both",
    }
}

fn missing_value(name: &str) -> Option<PdfMissing> {
    match name {
        "gibberish" => Some(PdfMissing::Gibberish),
        "skipped" => Some(PdfMissing::Skipped),
        "both" => Some(PdfMissing::Both),
        _ => None,
    }
}
