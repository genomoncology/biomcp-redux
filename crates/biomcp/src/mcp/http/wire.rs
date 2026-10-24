//! HTTP syntax belongs to httparse; reads have size and absolute time bounds.
use std::{io::{self, Read, Write}, net::TcpStream, time::{Duration, Instant}};
pub(super) const MAX_HEAD: usize = 16 * 1024;
pub(super) const MAX_BODY: usize = super::super::framing::MAX_FRAME_BYTES;
pub(super) const READ_BUDGET: Duration = Duration::from_secs(2);
pub(super) struct Request {
    pub(super) method: String,
    pub(super) path: String,
    pub(super) headers: Vec<(String, String)>,
    pub(super) body: Vec<u8>,
}
impl Request {
    pub(super) fn one(&self, name: &str) -> Result<Option<&str>, ()> {
        let mut values = self.headers.iter().filter(|(key, _)| key.eq_ignore_ascii_case(name));
        let first = values.next().map(|(_, value)| value.as_str());
        if values.next().is_some() { return Err(()); }
        Ok(first)
    }
}
pub(super) fn read(stream: &mut TcpStream) -> io::Result<Request> {
    let end = Instant::now() + READ_BUDGET;
    let mut bytes = Vec::with_capacity(MAX_HEAD);
    let mut buffer = [0; 4096];
    let (mut request, offset) = loop {
        let mut headers = [httparse::EMPTY_HEADER; 64];
        let mut parsed = httparse::Request::new(&mut headers);
        match parsed.parse(&bytes).map_err(|_| malformed())? {
            httparse::Status::Complete(offset) => {
                if offset > MAX_HEAD || parsed.version != Some(1) { return Err(malformed()); }
                let request = Request {
                    method: parsed.method.ok_or_else(malformed)?.into(),
                    path: parsed.path.ok_or_else(malformed)?.into(),
                    headers: parsed.headers.iter().map(|header| {
                        Ok((header.name.into(), std::str::from_utf8(header.value)
                            .map_err(|_| malformed())?.to_owned()))
                    }).collect::<io::Result<_>>()?,
                    body: Vec::new(),
                };
                break (request, offset);
            }
            httparse::Status::Partial => {
                if bytes.len() >= MAX_HEAD { return Err(malformed()); }
                timeout(stream, end)?;
                let available = buffer.len().min(MAX_HEAD - bytes.len());
                let n = stream.read(buffer.get_mut(..available).ok_or_else(malformed)?)?;
                if n == 0 { return Err(malformed()); }
                bytes.extend_from_slice(buffer.get(..n).ok_or_else(malformed)?);
            }
        }
    };
    let encoding = request.one("transfer-encoding").map_err(|_| malformed())?;
    let length = request.one("content-length").map_err(|_| malformed())?;
    if let Some(encoding) = encoding {
        if length.is_some() || !encoding.eq_ignore_ascii_case("chunked") { return Err(malformed()); }
        request.body = super::chunked::read(stream, bytes.get(offset..).ok_or_else(malformed)?.to_vec(), end)?;
        return Ok(request);
    }
    let length = length
        .map(|n| if n.bytes().all(|c| c.is_ascii_digit()) { n.parse::<usize>().map_err(|_| malformed()) }
            else { Err(malformed()) }).transpose()?.unwrap_or(0);
    if length > MAX_BODY { return Err(malformed()); }
    request.body = bytes.get(offset..).ok_or_else(malformed)?.to_vec();
    if request.body.len() > length { return Err(malformed()); }
    while request.body.len() < length {
        timeout(stream, end)?;
        let available = buffer.len().min(length - request.body.len());
        let n = stream.read(buffer.get_mut(..available).ok_or_else(malformed)?)?;
        if n == 0 { return Err(malformed()); }
        request.body.extend_from_slice(buffer.get(..n).ok_or_else(malformed)?);
    }
    Ok(request)
}
fn timeout(stream: &TcpStream, end: Instant) -> io::Result<()> {
    let left = end.checked_duration_since(Instant::now()).filter(|v| !v.is_zero())
        .ok_or_else(|| io::Error::new(io::ErrorKind::TimedOut, "HTTP read budget"))?;
    stream.set_read_timeout(Some(left))
}
pub(super) fn malformed() -> io::Error { io::Error::new(io::ErrorKind::InvalidData, "Invalid HTTP request") }
pub(super) fn write(stream: &mut TcpStream, status: u16, body: &[u8], revision: Option<&str>) -> io::Result<()> {
    stream.set_write_timeout(Some(READ_BUDGET))?;
    let revision = revision.map(|revision| format!("MCP-Protocol-Version: {revision}\r\n")).unwrap_or_default();
    let head = format!("HTTP/1.1 {status} Response\r\nContent-Type: application/json\r\nConnection: close\r\nCache-Control: no-store\r\n{revision}Content-Length: {}\r\n\r\n", body.len());
    stream.write_all(head.as_bytes())?;
    stream.write_all(body)
}

pub(super) fn read_more(stream: &mut TcpStream, bytes: &mut Vec<u8>, cap: usize, end: Instant) -> io::Result<()> {
    if bytes.len() >= cap { return Err(malformed()); }
    timeout(stream, end)?;
    let mut buffer = [0; 4096];
    let available = buffer.len().min(cap - bytes.len());
    let read = stream.read(buffer.get_mut(..available).ok_or_else(malformed)?)?;
    if read == 0 { return Err(malformed()); }
    bytes.extend_from_slice(buffer.get(..read).ok_or_else(malformed)?);
    Ok(())
}
