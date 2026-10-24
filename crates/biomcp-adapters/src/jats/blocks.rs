use super::element::Element;
use biomcp_fetch::{BadReply, xml::MixedText};
use quick_xml::{Reader, events::Event};
/// Read source paragraph/title/row boundaries without copying table and figure text into sections.
pub(super) fn rows(xml: &str, omit_floats: bool) -> Result<Vec<String>, BadReply> {
    let mut reader = Reader::from_str(xml);
    let mut rows = Vec::new();
    let mut row = String::new();
    loop {
        let begin = reader.buffer_position() as usize;
        match reader.read_event().map_err(|_| BadReply)? {
            Event::Start(tag) if omit_floats && matches!(tag.name().as_ref(), "table-wrap" | "fig" | "supplementary-material") => {
                reader.read_to_end(tag.name()).map_err(|_| BadReply)?;
            }
            Event::Start(tag) if matches!(tag.name().as_ref(), "p" | "title" | "tr" | "label" | "disp-formula" | "preformat") => {
                flush(&mut row, &mut rows);
                reader.read_to_end(tag.name()).map_err(|_| BadReply)?;
                let element = &xml[begin..reader.buffer_position() as usize];
                if tag.name().as_ref() == "tr" {
                    let cells = Element(element).children("td")?;
                    let cells = if cells.is_empty() { Element(element).children("th")? } else { cells };
                    row = cells.into_iter().map(Element::text).collect::<Result<Vec<_>,_>>()?.join("\t");
                } else {
                    row = MixedText::from_xml(element)?.as_str().to_owned();
                }
                flush(&mut row, &mut rows);
            }
            Event::Text(text) => row.push_str(text.as_ref()),
            Event::CData(text) => row.push_str(text.as_ref()),
            Event::GeneralRef(text) => {
                if let Some(ch) = text.resolve_char_ref().map_err(|_| BadReply)? { row.push(ch); }
                else { row.push_str(quick_xml::escape::resolve_predefined_entity(text.as_ref()).ok_or(BadReply)?); }
            }
            Event::Eof => { flush(&mut row, &mut rows); return Ok(rows); }
            _ => {}
        }
    }
}
fn flush(row: &mut String, rows: &mut Vec<String>) {
    if !row.trim().is_empty() { rows.push(std::mem::take(row)); }
    else { row.clear(); }
}
/// Find all non-nested matching elements in source order.
pub(super) fn descendants<'a>(xml: &'a str, name: &str) -> Result<Vec<Element<'a>>, BadReply> {
    let mut reader = Reader::from_str(xml);
    let mut out = Vec::new();
    loop {
        let begin = reader.buffer_position() as usize;
        match reader.read_event().map_err(|_| BadReply)? {
            Event::Start(tag) if tag.name().as_ref() == name => {
                reader.read_to_end(tag.name()).map_err(|_| BadReply)?;
                out.push(Element(&xml[begin..reader.buffer_position() as usize]));
            }
            Event::Eof => return Ok(out),
            _ => {}
        }
    }
}
