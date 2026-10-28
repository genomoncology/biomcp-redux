//! CDC WONDER labels its XML tables as Latin-1 HTML.
use std::borrow::Cow;

pub(super) fn accepts(media: &str) -> bool {
    let mut parts = media.split(';');
    let name = parts.next().unwrap_or("").trim();
    let charset = parts.next().unwrap_or("").trim();
    (name.eq_ignore_ascii_case("text/html")
        && charset.split_once('=').is_some_and(|(key, value)| {
            key.trim().eq_ignore_ascii_case("charset")
                && value
                    .trim()
                    .trim_matches('"')
                    .eq_ignore_ascii_case("ISO-8859-1")
        })
        && parts.next().is_none())
        || biomcp_fetch::xml::accepts_media(media)
}
pub(super) fn normalize<'a>(media: &str, body: &'a [u8]) -> Cow<'a, [u8]> {
    if media
        .split(';')
        .next()
        .is_some_and(|name| name.trim().eq_ignore_ascii_case("text/html"))
    {
        Cow::Owned(
            body.iter()
                .map(|&byte| char::from(byte))
                .collect::<String>()
                .into_bytes(),
        )
    } else {
        Cow::Borrowed(body)
    }
}
