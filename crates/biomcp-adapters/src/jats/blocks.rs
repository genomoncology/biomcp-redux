use crate::xml_element::Element;
use biomcp_fetch::BadReply;
use quick_xml::{Reader, events::Event};
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

/// Preserve direct body elements, including paragraphs outside named sections.
pub(super) fn body_nodes(xml: &str) -> Result<Vec<(String, Element<'_>)>, BadReply> {
    let mut reader = Reader::from_str(xml);
    let mut root = false;
    let mut out = Vec::new();
    loop {
        let begin = reader.buffer_position() as usize;
        match reader.read_event().map_err(|_| BadReply)? {
            Event::Start(_) if !root => root = true,
            Event::Start(tag) => {
                let name = tag.name().as_ref().to_owned();
                reader.read_to_end(tag.name()).map_err(|_| BadReply)?;
                out.push((
                    name,
                    Element(&xml[begin..reader.buffer_position() as usize]),
                ));
            }
            Event::Empty(tag) if root => {
                out.push((
                    tag.name().as_ref().to_owned(),
                    Element(&xml[begin..reader.buffer_position() as usize]),
                ));
            }
            Event::Eof => return Ok(out),
            _ => {}
        }
    }
}
