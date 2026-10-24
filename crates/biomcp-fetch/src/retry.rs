use crate::{Arg, Deadline, KeyPlace, KeyUse};
use sha2::{Digest, Sha256};
use std::time::{Duration, Instant, SystemTime};

pub(crate) const ATTEMPTS: u32 = 3;
const BASE_BACKOFF: Duration = Duration::from_millis(250);

pub(crate) fn retry_status(status: u16) -> bool {
    matches!(status, 429 | 500 | 502 | 503 | 504)
}

pub(crate) fn retry_error(error: &ureq::Error) -> bool {
    matches!(error, ureq::Error::Timeout(ureq::Timeout::Connect))
        || matches!(error, ureq::Error::Io(e) if matches!(e.kind(), std::io::ErrorKind::ConnectionReset))
}

/// One parser owns both accepted Retry-After forms. Past dates request no wait.
pub(crate) fn retry_after(value: &str, now: SystemTime) -> Option<Duration> {
    let value = value.trim();
    if !value.is_empty() && value.bytes().all(|v| v.is_ascii_digit()) {
        return value.parse::<u64>().ok().map(Duration::from_secs);
    }
    httpdate::parse_http_date(value)
        .ok()
        .map(|date| date.duration_since(now).unwrap_or_default())
}

pub(crate) fn backoff(identity: &[u8; 32], attempt: u32) -> Duration {
    let seed = identity.iter().fold(0u64, |sum, byte| {
        sum.wrapping_mul(257).wrapping_add(u64::from(*byte))
    });
    BASE_BACKOFF * (1 << attempt) + Duration::from_millis(seed % 100)
}

pub(crate) fn wait(delay: Duration, deadline: Deadline<'_>) -> bool {
    if deadline.virtual_time() { return deadline.advance(delay); }
    let Some(end) = Instant::now().checked_add(delay) else {
        return false;
    };
    loop {
        if deadline.expired() {
            return false;
        }
        let remaining = end.saturating_duration_since(Instant::now());
        if remaining.is_zero() {
            return true;
        }
        std::thread::park_timeout(remaining.min(deadline.remaining()));
    }
}

// The same credential-free URL is both persisted and hashed.
pub(crate) fn canonical(url: &str, params: &[(&str, Arg<'_>)], key: KeyUse) -> String {
    let credential = match key {
        KeyUse::Optional(c) | KeyUse::Required(c) => match c.place {
            KeyPlace::Query(name) => Some(name),
            _ => None,
        },
        KeyUse::None => None,
    };
    let mut canonical = url.to_owned();
    for (name, value) in params.iter().filter(|(name, _)| Some(*name) != credential) {
        canonical.push(if canonical.contains('?') { '&' } else { '?' });
        encode(&mut canonical, name);
        canonical.push('=');
        encode(&mut canonical, &value.value());
    }
    canonical
}
pub(crate) fn identity(canonical: &str) -> [u8; 32] {
    let mut hash = Sha256::new();
    hash.update(b"GET\0");
    hash.update(canonical.as_bytes());
    hash.finalize().into()
}

fn encode(out: &mut String, value: &str) {
    const HEX: &[u8; 16] = b"0123456789ABCDEF";
    for byte in value.bytes() {
        if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'.' | b'_' | b'~') {
            out.push(char::from(byte));
        } else {
            out.push('%');
            // The nibbles are always inside the hexadecimal alphabet.
            out.push(char::from(
                *HEX.get(usize::from(byte >> 4)).unwrap_or(&b'0'),
            ));
            out.push(char::from(
                *HEX.get(usize::from(byte & 15)).unwrap_or(&b'0'),
            ));
        }
    }
}

// These internal contract tables cover classifications that cannot be induced
// portably with a local TCP peer, and an identity that is deliberately not public.
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Credential, SecretName};
    fn identity(url: &str, params: &[(&str, Arg<'_>)], key: KeyUse) -> [u8; 32] {
        super::identity(&canonical(url, params, key))
    }

    #[test]
    fn only_connect_timeout_and_reset_are_retryable_transport_errors() {
        for (error, expected) in [
            (ureq::Error::Timeout(ureq::Timeout::Connect), true),
            (ureq::Error::Timeout(ureq::Timeout::Global), false),
            (ureq::Error::Timeout(ureq::Timeout::RecvResponse), false),
            (
                ureq::Error::Io(std::io::ErrorKind::ConnectionReset.into()),
                true,
            ),
            (ureq::Error::Io(std::io::ErrorKind::TimedOut.into()), false),
            (
                ureq::Error::Io(std::io::ErrorKind::UnexpectedEof.into()),
                false,
            ),
            (ureq::Error::ConnectionFailed, false),
        ] {
            assert_eq!(retry_error(&error), expected);
        }
    }

    #[test]
    fn internal_identity_excludes_credential_query_and_preserves_request_values() {
        let credential = KeyUse::Optional(Credential {
            secret: SecretName::NcbiApiKey,
            place: KeyPlace::Query("api_key"),
        });
        let first = identity(
            "https://example.test/answer",
            &[
                ("q", Arg::Text("a b")),
                ("api_key", Arg::Text("first-placeholder")),
            ],
            credential,
        );
        let second = identity(
            "https://example.test/answer",
            &[
                ("q", Arg::Text("a b")),
                ("api_key", Arg::Text("second-placeholder")),
            ],
            credential,
        );
        assert_eq!(first, second);
        assert_ne!(
            first,
            identity(
                "https://example.test/answer",
                &[("q", Arg::Text("a+b"))],
                credential
            )
        );
        assert_eq!(backoff(&first, 1) - backoff(&first, 0), BASE_BACKOFF);
    }

    #[test]
    fn retry_after_boundaries_accept_only_seconds_and_http_dates() {
        let now = SystemTime::UNIX_EPOCH + Duration::from_secs(784111777);
        for (value, expected) in [
            (" 0 ", Some(Duration::ZERO)),
            ("18446744073709551615", Some(Duration::from_secs(u64::MAX))),
            ("18446744073709551616", None),
            ("+1", None),
            ("-1", None),
            ("1.5", None),
            ("", None),
            ("Sun, 06 Nov 1994 08:49:37 GMT", Some(Duration::ZERO)),
            (
                "Sunday, 06-Nov-94 08:49:38 GMT",
                Some(Duration::from_secs(1)),
            ),
            ("Sun Nov  6 08:49:39 1994", Some(Duration::from_secs(2))),
        ] {
            assert_eq!(retry_after(value, now), expected, "{value}");
        }
    }
}
