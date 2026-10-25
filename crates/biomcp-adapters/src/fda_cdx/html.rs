//! The approved fixed-column FDA tables; unrelated page markup is never interpreted.
use biodata::{DiagnosticCell, DiagnosticCellLink};
use biomcp_fetch::BadReply;
use quick_xml::{Reader, events::Event};

pub(super) const HEADERS: [&[&str]; 2] = [
    &["Diagnostic Name (Manufacturer)", "Indication - Sample Type", "Drug Trade Name (Generic) NDA / BLA", "Biomarker(s)", "Biomarker(s) (Details)", "PMA / 510(k) /\n513(f)(2) / HDE (Approval / Clearance / Grant Date)"],
    &["Diagnostic Name\n(Manufacturer)", "Indication(s) - Sample Type", "PMA (Approval Date)", "Device Indication for a Specific Group of Oncology Therapeutic Products and Trade Name (Generic) – NDA/BLA"],
];
pub(super) fn decode(body: &[u8]) -> Result<Vec<Vec<Vec<DiagnosticCell>>>, BadReply> {
    if body.len() > 10 * 1024 * 1024 { return Err(BadReply); }
    let text = std::str::from_utf8(body).map_err(|_|BadReply)?;
    let mut rest = text;
    let mut tables = Vec::new();
    while let Some(start) = rest.find("<table") {
        rest = rest.get(start..).ok_or(BadReply)?;
        let end = rest.find("</table>").ok_or(BadReply)? + "</table>".len();
        let fragment = rest.get(..end).ok_or(BadReply)?;
        tables.push(table(fragment)?);
        rest = rest.get(end..).ok_or(BadReply)?;
    }
    if tables.len() != HEADERS.len() { return Err(BadReply); }
    for (rows, expected) in tables.iter_mut().zip(HEADERS) {
        let headers = rows.first().ok_or(BadReply)?;
        if headers.iter().map(|c|c.text.as_ref()).ne(expected.iter().copied()) || rows.iter().skip(1).any(|r|r.len()!=expected.len()) { return Err(BadReply); }
        rows.remove(0);
    }
    Ok(tables)
}
fn table(fragment: &str) -> Result<Vec<Vec<DiagnosticCell>>, BadReply> {
    let mut reader = Reader::from_str(fragment);
    reader.config_mut().check_end_names = false;
    let mut rows = Vec::new();
    let mut row = Vec::new();
    let mut cell: Option<DiagnosticCell> = None;
    let mut link: Option<DiagnosticCellLink> = None;
    loop {
        match reader.read_event().map_err(|_|BadReply)? {
            Event::Start(e) | Event::Empty(e) => {
                let name = e.name();
                for attr in e.attributes() {
                    let attr = attr.map_err(|_|BadReply)?;
                    if matches!(attr.key.as_ref(), b"rowspan"|b"colspan") && attr.value.as_ref()!=b"1" { return Err(BadReply); }
                }
                match name.as_ref() {
                    b"table" if !rows.is_empty() || !row.is_empty() => return Err(BadReply),
                    b"td"|b"th" => {
                        if cell.is_some() { return Err(BadReply); }
                        cell = Some(DiagnosticCell { text: "".into(), links: Box::default() });
                    },
                    b"br" => append(&mut cell, &mut link, "\n"),
                    b"a" if cell.is_some() => {
                        if link.is_some() { return Err(BadReply); }
                        let href = e.try_get_attribute("href").map_err(|_|BadReply)?.ok_or(BadReply)?;
                        link = Some(DiagnosticCellLink { href: href.decode_and_unescape_value(reader.decoder()).map_err(|_|BadReply)?.into_owned().into(), label: Some("".into()) });
                    },
                    _ => {},
                }
            },
            Event::Text(t) => append(&mut cell, &mut link, &t.xml_content().map_err(|_|BadReply)?),
            Event::GeneralRef(r) => {
                let r = r.decode().map_err(|_|BadReply)?;
                let escaped = format!("&{r};");
                let value = quick_xml::escape::unescape(&escaped).map_err(|_|BadReply)?;
                append(&mut cell, &mut link, &value);
            },
            Event::End(e) => match e.name().as_ref() {
                b"a" => if let Some(value) = link.take() { let c = cell.as_mut().ok_or(BadReply)?; let mut links = std::mem::take(&mut c.links).into_vec(); links.push(value); c.links = links.into(); },
                b"td"|b"th" => { if link.is_some() { return Err(BadReply); } row.push(cell.take().ok_or(BadReply)?); },
                b"tr" => { if cell.is_some() || row.is_empty() { return Err(BadReply); } rows.push(std::mem::take(&mut row)); },
                _ => {},
            },
            Event::Eof => break,
            _ => {},
        }
    }
    if cell.is_some() || !row.is_empty() { return Err(BadReply); }
    Ok(rows)
}
fn append(cell: &mut Option<DiagnosticCell>, link: &mut Option<DiagnosticCellLink>, value: &str) {
    if let Some(c) = cell { c.text = format!("{}{value}", c.text).into(); }
    if let Some(l) = link { l.label = Some(format!("{}{value}", l.label.as_deref().unwrap_or_default()).into()); }
}
