//! Read visible cells from a fixed-column FDA HTML table.
use crate::fda_dsc::html::{attribute, visible};
use biomcp_fetch::BadReply;

/// A cell's visible wording and markup for reading its links.
#[derive(Debug, Clone)]
pub(crate) struct Cell<'a> {
    pub text: String,
    pub html: &'a str,
    span: usize,
}

/// The first table with the expected heading and exact column count.
pub(crate) fn rows<const N: usize>(
    body: &str,
    accepts: impl Fn([&str; N]) -> bool,
) -> Result<Vec<[Cell<'_>; N]>, BadReply> {
    for fragment in body.split("<table").skip(1) {
        let (table, _) = fragment.split_once("</table>").ok_or(BadReply)?;
        let mut rows = table.split("<tr").skip(1);
        let found = rows.by_ref().any(|row| {
            cells(row)
                .and_then(as_row::<N>)
                .is_ok_and(|head| accepts(head.each_ref().map(|cell| cell.text.as_str())))
        });
        if found {
            let mut carried: [Option<(usize, Cell<'_>)>; N] = std::array::from_fn(|_| None);
            let rows = rows
                .map(|row| expanded(cells(row)?, &mut carried))
                .collect::<Result<Vec<_>, BadReply>>()?;
            if carried.into_iter().all(|slot| slot.is_none()) {
                return Ok(rows
                    .into_iter()
                    .filter(|row| row.iter().any(|cell| !cell.text.is_empty()))
                    .collect());
            }
            return Err(BadReply);
        }
    }
    Err(BadReply)
}

fn as_row<const N: usize>(cells: Vec<Cell<'_>>) -> Result<[Cell<'_>; N], BadReply> {
    cells.try_into().map_err(|_| BadReply)
}

fn cells(row: &str) -> Result<Vec<Cell<'_>>, BadReply> {
    let (row, _) = row.split_once("</tr>").ok_or(BadReply)?;
    row.match_indices('<')
        .filter_map(|(at, _)| {
            let (tag, content) = row.get(at + 1..)?.split_once('>')?;
            let end = match tag.split_ascii_whitespace().next()? {
                "td" => "</td>",
                "th" => "</th>",
                _ => return None,
            };
            Some(cell(tag, content, end))
        })
        .collect()
}

fn cell<'a>(tag: &str, content: &'a str, end: &str) -> Result<Cell<'a>, BadReply> {
    if attribute(tag, "colspan").is_some_and(|width| width != "1") {
        return Err(BadReply);
    }
    let (html, _) = content.split_once(end).ok_or(BadReply)?;
    let span = attribute(tag, "rowspan")
        .unwrap_or("1")
        .parse()
        .map_err(|_| BadReply)?;
    Ok(Cell {
        text: visible(html),
        html,
        span,
    })
}

fn expanded<'a, const N: usize>(
    cells: Vec<Cell<'a>>,
    carried: &mut [Option<(usize, Cell<'a>)>; N],
) -> Result<[Cell<'a>; N], BadReply> {
    let mut cells = cells.into_iter();
    let row = carried
        .iter_mut()
        .map(|slot| {
            let (remaining, cell) = match slot.take() {
                Some(previous) => previous,
                None => {
                    let cell = cells.next().ok_or(BadReply)?;
                    (cell.span, cell)
                }
            };
            if remaining > 1 {
                *slot = Some((remaining - 1, cell.clone()));
            }
            Ok(cell)
        })
        .collect::<Result<Vec<_>, BadReply>>()?;
    if cells.next().is_some() {
        return Err(BadReply);
    }
    as_row(row)
}

/// Page links paired with their visible labels.
pub(crate) fn anchors(body: &str) -> impl Iterator<Item = (&str, String)> {
    body.split("<a ").skip(1).filter_map(|link| {
        let (tag, content) = link.split_once('>')?;
        let (label, _) = content.split_once("</a>")?;
        let path = attribute(tag, "href").or_else(|| {
            tag.strip_prefix("href=\"")?
                .split_once('"')
                .map(|(value, _)| value)
        })?;
        Some((path, visible(label)))
    })
}

/// FDA drug page paths paired with their visible labels.
pub(crate) fn links(body: &str) -> impl Iterator<Item = (&str, String)> {
    anchors(body).filter_map(|(path, label)| {
        let path = path.strip_prefix(crate::fda_dsc::SITE).unwrap_or(path);
        path.starts_with("/drugs/").then_some((path, label))
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
