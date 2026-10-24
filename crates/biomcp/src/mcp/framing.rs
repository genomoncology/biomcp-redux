//! Bounded newline framing; no input text enters diagnostics.
use std::io::{self, BufRead};

pub(super) const MAX_FRAME_BYTES: usize = 64 * 1024;
#[derive(Debug)]
pub(super) enum Frame {
    Message(Vec<u8>),
    Malformed,
    Oversized,
    Eof,
}

pub(super) fn read(input: &mut impl BufRead) -> io::Result<Frame> {
    let mut frame = Vec::new();
    loop {
        let available = input.fill_buf()?;
        if available.is_empty() {
            return Ok(if frame.is_empty() {
                Frame::Eof
            } else {
                Frame::Malformed
            });
        }
        let newline = available.iter().position(|b| *b == b'\n');
        let length = newline.map_or(available.len(), |n| n + 1);
        if frame.len() + length > MAX_FRAME_BYTES {
            return Ok(Frame::Oversized);
        }
        frame.extend(available.iter().take(length));
        input.consume(length);
        if newline.is_some() {
            return Ok(Frame::Message(frame));
        }
    }
}
