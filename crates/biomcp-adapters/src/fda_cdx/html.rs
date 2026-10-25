//! The approved fixed-column FDA tables; unrelated page markup is never interpreted.
use biodata::{DiagnosticCell, DiagnosticCellLink};
use biomcp_fetch::BadReply;
use quick_xml::{Reader, events::Event};

pub(super) const HEADERS: [&[&str]; 2] = [
    &[
        "Diagnostic Name (Manufacturer)",
        "Indication - Sample Type",
        "Drug Trade Name (Generic) NDA / BLA",
        "Biomarker(s)",
        "Biomarker(s) (Details)",
        "PMA / 510(k) /\n513(f)(2) / HDE (Approval / Clearance / Grant Date)",
    ],
    &[
        "Diagnostic Name\n(Manufacturer)",
        "Indication(s) - Sample Type",
        "PMA (Approval Date)",
        "Device Indication for a Specific Group of Oncology Therapeutic Products and Trade Name (Generic) – NDA/BLA",
    ],
];
pub(super) fn decode(body: &[u8]) -> Result<Vec<Vec<Vec<DiagnosticCell>>>, BadReply> {
    if body.len() > 10 * 1024 * 1024 {
        return Err(BadReply);
    }
    let text = std::str::from_utf8(body).map_err(|_| BadReply)?;
    let mut rest = text;
    let mut tables = Vec::new();
    while let Some(start) = rest.find("<table") {
        rest = rest.get(start..).ok_or(BadReply)?;
        let end = rest.find("</table>").ok_or(BadReply)? + "</table>".len();
        let fragment = rest.get(..end).ok_or(BadReply)?;
        tables.push(table(fragment)?);
        rest = rest.get(end..).ok_or(BadReply)?;
    }
    if tables.len() != HEADERS.len() {
        return Err(BadReply);
    }
    for (rows, expected) in tables.iter_mut().zip(HEADERS) {
        let headers = rows.first().ok_or(BadReply)?;
        if headers
            .iter()
            .map(|c| c.text.as_ref())
            .ne(expected.iter().copied())
            || rows.iter().skip(1).any(|r| r.len() != expected.len())
        {
            return Err(BadReply);
        }
        rows.remove(0);
    }
    Ok(tables)
}
#[derive(Default)]
struct Table {
    rows: Vec<Vec<DiagnosticCell>>,
    row: Vec<DiagnosticCell>,
    cell: Option<DiagnosticCell>,
    link: Option<DiagnosticCellLink>,
}
fn table(fragment: &str) -> Result<Vec<Vec<DiagnosticCell>>, BadReply> {
    let mut reader = Reader::from_str(fragment);
    reader.config_mut().check_end_names = false;
    let mut table = Table::default();
    loop {
        match reader.read_event().map_err(|_| BadReply)? {
            Event::Start(e) | Event::Empty(e) => table.start(&e)?,
            Event::Text(t) => table.append(t.as_ref()),
            Event::CData(t) => table.append(t.as_ref()),
            Event::GeneralRef(r) => {
                let escaped = format!("&{r};", r = r.as_ref());
                table.append(&quick_xml::escape::unescape(&escaped).map_err(|_| BadReply)?);
            }
            Event::End(e) => table.end(e.name().as_ref())?,
            Event::Eof => break,
            _ => {}
        }
    }
    if table.cell.is_some() || !table.row.is_empty() {
        return Err(BadReply);
    }
    Ok(table.rows)
}
impl Table {
    fn start(&mut self, e: &quick_xml::events::BytesStart<'_>) -> Result<(), BadReply> {
        for attr in e.attributes() {
            let attr = attr.map_err(|_| BadReply)?;
            if matches!(attr.key.as_ref(), "rowspan" | "colspan") && attr.value.as_ref() != "1" {
                return Err(BadReply);
            }
        }
        match e.name().as_ref() {
            "table" if !self.rows.is_empty() || !self.row.is_empty() || self.cell.is_some() => {
                return Err(BadReply);
            }
            "td" | "th" => {
                if self.cell.is_some() {
                    return Err(BadReply);
                }
                self.cell = Some(DiagnosticCell {
                    text: "".into(),
                    links: Box::default(),
                });
            }
            "br" => self.append("\n"),
            "a" if self.cell.is_some() => {
                if self.link.is_some() {
                    return Err(BadReply);
                }
                let href = e
                    .try_get_attribute("href")
                    .map_err(|_| BadReply)?
                    .ok_or(BadReply)?;
                self.link = Some(DiagnosticCellLink {
                    href: quick_xml::escape::unescape(href.value.as_ref())
                        .map_err(|_| BadReply)?
                        .into_owned()
                        .into(),
                    label: Some("".into()),
                });
            }
            _ => {}
        }
        Ok(())
    }
    fn end(&mut self, name: &str) -> Result<(), BadReply> {
        match name {
            "a" => {
                if let Some(value) = self.link.take() {
                    let c = self.cell.as_mut().ok_or(BadReply)?;
                    let mut links = std::mem::take(&mut c.links).into_vec();
                    links.push(value);
                    c.links = links.into();
                }
            }
            "td" | "th" => {
                if self.link.is_some() {
                    return Err(BadReply);
                }
                self.row.push(self.cell.take().ok_or(BadReply)?);
            }
            "tr" => {
                if self.cell.is_some() || self.row.is_empty() {
                    return Err(BadReply);
                }
                self.rows.push(std::mem::take(&mut self.row));
            }
            _ => {}
        }
        Ok(())
    }
    fn append(&mut self, value: &str) {
        if let Some(c) = &mut self.cell {
            c.text = format!("{}{value}", c.text).into();
        }
        if let Some(l) = &mut self.link {
            l.label = Some(format!("{}{value}", l.label.as_deref().unwrap_or_default()).into());
        }
    }
}
