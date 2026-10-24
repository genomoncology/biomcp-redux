//! Bounded chunk framing uses the library's size and trailer parsers.
use super::wire::{MAX_BODY, MAX_HEAD, malformed, read_more};
use std::{io, net::TcpStream, time::Instant};
pub(super) fn read(stream: &mut TcpStream, mut bytes: Vec<u8>, end: Instant) -> io::Result<Vec<u8>> {
    let mut offset = 0usize;
    let mut body = Vec::new();
    loop {
        let (header, length) = loop {
            let available = bytes.get(offset..).ok_or_else(malformed)?;
            match httparse::parse_chunk_size(available).map_err(|_| malformed())? {
                httparse::Status::Complete(parsed) => break parsed,
                httparse::Status::Partial => read_more(stream, &mut bytes, MAX_BODY + MAX_HEAD, end)?,
            }
        };
        offset = offset.checked_add(header).ok_or_else(malformed)?;
        let length: usize = length.try_into().map_err(|_| malformed())?;
        if length > MAX_BODY - body.len() { return Err(malformed()); }
        if length == 0 {
            loop {
                let mut headers = [httparse::EMPTY_HEADER; 64];
                match httparse::parse_headers(bytes.get(offset..).ok_or_else(malformed)?, &mut headers)
                    .map_err(|_| malformed())? {
                    httparse::Status::Complete((used, _)) => {
                        if used > MAX_HEAD || offset + used != bytes.len() { return Err(malformed()); }
                        return Ok(body);
                    }
                    httparse::Status::Partial => read_more(stream, &mut bytes, MAX_BODY + MAX_HEAD, end)?,
                }
            }
        }
        let finish = offset.checked_add(length).and_then(|v| v.checked_add(2)).ok_or_else(malformed)?;
        while bytes.len() < finish { read_more(stream, &mut bytes, MAX_BODY + MAX_HEAD, end)?; }
        body.extend_from_slice(bytes.get(offset..finish - 2).ok_or_else(malformed)?);
        if bytes.get(finish - 2..finish) != Some(b"\r\n") { return Err(malformed()); }
        offset = finish;
    }
}
