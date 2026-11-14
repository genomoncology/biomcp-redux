//! Gzip XML parts read once as a stream; only rows the source emits are kept.
use super::{MAX_STREAMED_BYTES, MAX_STREAMED_DECODED_BYTES, Row, parse::ParsedRows};
use crate::BadReply;
use std::io::{BufReader, Read};

/// Counts decoded bytes so a part over its bound fails instead of being read further.
struct Bounded<R> {
    inner: R,
    left: u64,
    over: bool,
}
impl<R: Read> Read for Bounded<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if self.left == 0 {
            // One probe byte tells an exact-size body from an oversized one.
            let n = self.inner.read(&mut [0_u8; 1])?;
            self.over |= n > 0;
            return if n > 0 {
                Err(std::io::Error::other("decoded limit"))
            } else {
                Ok(0)
            };
        }
        let want = buf
            .len()
            .min(usize::try_from(self.left).unwrap_or(usize::MAX));
        let n = self.inner.read(buf.get_mut(..want).unwrap_or_default())?;
        self.left -= n as u64;
        Ok(n)
    }
}

pub(super) fn rows<R: Row>(
    body: &[u8],
    max_bytes: u64,
    max_decoded_bytes: u64,
) -> Result<ParsedRows, BadReply> {
    if max_bytes == 0
        || max_bytes > MAX_STREAMED_BYTES
        || max_decoded_bytes == 0
        || max_decoded_bytes > MAX_STREAMED_DECODED_BYTES
        || body.len() as u64 > max_bytes
    {
        return Err(BadReply);
    }
    let mut xml = BufReader::new(Bounded {
        inner: flate2::read::MultiGzDecoder::new(body),
        left: max_decoded_bytes,
        over: false,
    });
    let mut parsed = ParsedRows::default();
    let mut index = 0_usize;
    let mut failed = false;
    let mut emit = |row: Option<String>| {
        index += 1;
        let Some(row) = row else {
            parsed.reject(index, "cannot decode element");
            return;
        };
        match serde_json::from_str::<R>(&row) {
            Ok(decoded) if decoded.check_bundle(0).is_ok() => {
                parsed.accept(row, decoded.keys(), index);
            }
            Ok(_) => failed = true,
            Err(_) => parsed.reject(index, "cannot decode row"),
        }
    };
    let streamed = R::stream_xml(&mut xml, &mut emit);
    if streamed.is_err() || failed || xml.get_ref().over {
        return Err(BadReply);
    }
    Ok(parsed)
}
