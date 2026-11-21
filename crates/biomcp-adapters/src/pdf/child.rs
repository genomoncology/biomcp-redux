//! A PDF read in a child process of BioMCP's own binary, when the binary provides one (1075).
//!
//! The pre-load measuring in [`super::scan`] bounds every route found so far, but three reviews
//! each found a new way to make it and lopdf read a file differently. So the binary reads each
//! PDF in a child whose memory the operating system caps and which it kills at the time limit;
//! a hole nobody has found then costs at most the cap. The child gets the PDF's bytes on
//! standard input and writes one header line of JSON, then the text. A child that is killed,
//! crashes or answers in any other form becomes the `refused` state, never a guess.

use super::embedded::Embedded;
use biomcp_types::{PdfMissing, PdfTextState, Reason};
use serde_json::{Value, json};

/// Most bytes a child's answer may hold: the whole text and its header.
pub const MAX_OUTPUT: usize = super::MAX_EXPANDED_BYTES + 4096;

/// The child's side: read the PDF after the parent's first line, which gives the most pages
/// to read, and encode the result for the parent.
pub fn serve(input: &[u8]) -> Vec<u8> {
    let read = pages(input).and_then(|(pages, bytes)| super::embedded(bytes, pages, &|| false));
    let (header, text) = match read {
        Ok(read) => {
            let mut header = serde_json::Map::from_iter([
                ("state".into(), json!(read.state)),
                ("pages".into(), json!(read.pages)),
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

/// The page bound the parent wrote on the input's first line, and the PDF after it.
pub fn pages(input: &[u8]) -> Result<(usize, &[u8]), Reason> {
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
    Ok((pages, rest.get(1..).ok_or(Reason::BadReply)?))
}

/// The parent's side: read `bytes`, at most `pages` pages, in an admitted worker if configured.
pub(super) fn read(
    isolated: Option<&biomcp_fetch::IsolatedRead<'_>>,
    bytes: &[u8],
    pages: usize,
    deadline: &biomcp_fetch::LocalDeadline,
) -> Result<Embedded, Reason> {
    let Some(isolated) = isolated else {
        return super::embedded(bytes, pages, &|| deadline.expired());
    };
    let mut input = format!("{pages}\n").into_bytes();
    input.extend_from_slice(bytes);
    match isolated.run(&input, MAX_OUTPUT, deadline) {
        Ok(answer) => decoded(&answer).unwrap_or_else(|Malformed| Ok(refused())),
        Err(biomcp_fetch::Contained) => Ok(refused()),
    }
}

/// A read stopped at the memory cap or the time limit, or that failed in the child.
fn refused() -> Embedded {
    Embedded {
        pages: None,
        state: PdfTextState::Refused,
        text: String::new(),
        missing: None,
        private_symbol: false,
    }
}

/// An answer not in the child's form.
struct Malformed;

/// The child's answer.
fn decoded(answer: &[u8]) -> Result<Result<Embedded, Reason>, Malformed> {
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
    let state = serde_json::from_value(header.get("state").ok_or(Malformed)?.clone())
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
    Ok(Ok(Embedded {
        pages,
        state,
        text,
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
