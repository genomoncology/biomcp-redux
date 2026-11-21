//! Read visible cells from a fixed-column FDA HTML table.
use crate::fda_dsc::html::{attribute, visible};

/// The first table with the expected heading and exact column count.
pub(crate) fn rows<const N: usize>(
    body: &str,
    accepts: impl Fn([&str; N]) -> bool,
) -> Option<Vec<[String; N]>> {
    for fragment in body.split("<table").skip(1) {
        let (table, _) = fragment.split_once("</table>")?;
        let mut rows = table.split("<tr").skip(1).map(cells::<N>);
        let Some(Some(head)) = rows.next() else {
            continue;
        };
        if accepts(head.each_ref().map(String::as_str)) {
            return rows.collect();
        }
    }
    None
}

fn cells<const N: usize>(row: &str) -> Option<[String; N]> {
    let (row, _) = row.split_once("</tr>")?;
    row.match_indices('<')
        .filter_map(|(at, _)| {
            let (tag, content) = row.get(at + 1..)?.split_once('>')?;
            match tag.split_ascii_whitespace().next()? {
                "td" => Some(content.split_once("</td>").map(|(value, _)| visible(value))),
                "th" => Some(content.split_once("</th>").map(|(value, _)| visible(value))),
                _ => None,
            }
        })
        .collect::<Option<Vec<_>>>()?
        .try_into()
        .ok()
}

/// FDA drug page links paired with their visible labels.
pub(crate) fn links(body: &str) -> impl Iterator<Item = (&str, String)> {
    body.split("<a ").skip(1).filter_map(|link| {
        let (tag, content) = link.split_once('>')?;
        let (label, _) = content.split_once("</a>")?;
        let path = attribute(tag, "href").or_else(|| {
            tag.strip_prefix("href=\"")?
                .split_once('"')
                .map(|(value, _)| value)
        })?;
        let path = path.strip_prefix(crate::fda_dsc::SITE).unwrap_or(path);
        path.starts_with("/drugs/").then(|| (path, visible(label)))
    })
}

/// Read HTML over the checked fetch layer, optionally from the local cache alone.
pub(crate) fn read(
    leg: &biomcp_fetch::Leg<'_>,
    endpoint: &biomcp_fetch::Endpoint,
    source: biomcp_types::Source,
    cache_only: bool,
) -> Result<String, biomcp_fetch::RequestError> {
    use biomcp_fetch::{DocumentMedia, Input, Read};
    let read = Read::bytes().document_media(DocumentMedia::Html);
    let read = if cache_only { read.cache_only() } else { read };
    let bytes = leg.call(endpoint, Input::Params(&[]), read)?.value.0;
    String::from_utf8(bytes).map_err(|_| biomcp_fetch::bad_reply(source, None))
}
