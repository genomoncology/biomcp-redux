//! What a publisher PDF link says about its host, read from the link text alone.
use biomcp_types::ArticleInput;

/// The host of a publisher link, lowercased, when the link is http(s).
pub fn host(url: &str) -> Option<String> {
    let rest = url
        .strip_prefix("https://")
        .or_else(|| url.strip_prefix("http://"))?;
    let host = rest.split(['/', '?', '#']).next()?.to_ascii_lowercase();
    (!host.is_empty()).then_some(host)
}

/// Europe PMC refuses automated PDF requests: its PMC render links answer a
/// Cloudflare browser check (HTTP 403, `cf-mitigated: challenge`), and its
/// preprint `fulltextRepo` links answer 403 `{"error":"PDF link has expired or is
/// invalid"}` even for the link its own REST API lists (pm jobs j41234, j41236).
pub fn europepmc(url: &str) -> bool {
    host(url).is_some_and(|host| host == "europepmc.org" || host.ends_with(".europepmc.org"))
}

/// The PMC or Europe PMC preprint identity a link names, such as
/// `articles/pmc123?pdf=render`, `accid=PMC123` or `pprId=PPR123`.
pub fn named_text(url: &str) -> Option<ArticleInput> {
    url.split(['/', '?', '&', '=', '#'])
        .map(str::to_ascii_uppercase)
        .find(|token| {
            ["PMC", "PPR"].iter().any(|prefix| {
                token.strip_prefix(prefix).is_some_and(|digits| {
                    !digits.is_empty() && digits.bytes().all(|b| b.is_ascii_digit())
                })
            })
        })
        .and_then(|token| token.parse().ok())
}
