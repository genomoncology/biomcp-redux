//! A finite JATS inline renderer shared by body paragraphs and PubMed abstracts.
use crate::xml_element::Element;
use biomcp_fetch::{BadReply, xml::MixedText};
use quick_xml::{Reader, events::Event};
pub(super) fn inline(xml: &str) -> Result<String, BadReply> {
    let mut reader = Reader::from_str(xml);
    let mut out = String::new();
    loop {
        let begin = reader.buffer_position() as usize;
        match reader.read_event().map_err(|_| BadReply)? {
            Event::Start(tag) if matches!(tag.name().as_ref(), "sup" | "xref") => {
                let span = reader.read_to_end(tag.name()).map_err(|_| BadReply)?;
                let whole = Element(&xml[begin..reader.buffer_position() as usize]);
                let body = &xml[span.start as usize..span.end as usize];
                if tag.name().as_ref() == "sup" {
                    if citations(body)? { out.push_str(&format!("[{}]", whole.text()?)); }
                    else { out.push_str(&format!("<sup>{}</sup>", inline(body)?)); }
                } else if matches!(whole.attr("ref-type")?.as_deref(), Some("bibr" | "ref")) {
                    out.push_str(&format!("[{}]", whole.text()?));
                } else { out.push_str(&inline(body)?); }
            }
            Event::Start(tag) => out.push_str(marker(tag.name().as_ref(), false)),
            Event::End(tag) => out.push_str(marker(tag.name().as_ref(), true)),
            Event::Empty(tag) if matches!(tag.name().as_ref(), "break" | "hr") => out.push('\n'),
            Event::Text(v) => out.push_str(v.as_ref()),
            Event::CData(v) => out.push_str(v.as_ref()),
            Event::GeneralRef(v) => {
                if let Some(ch) = v.resolve_char_ref().map_err(|_| BadReply)? { out.push(ch); }
                else { out.push_str(quick_xml::escape::resolve_predefined_entity(v.as_ref()).ok_or(BadReply)?); }
            }
            Event::Eof => return Ok(out),
            _ => {}
        }
    }
}
fn marker(name: &str, closing: bool) -> &'static str {
    match (name, closing) {
        ("bold", _) => "**", ("italic", _) => "*",
        ("sub", false) => "<sub>", ("sub", true) => "</sub>",
        ("list-item", false) => "\n- ", ("list-item", true) => "\n",
        ("p", true) => "\n\n", _ => "",
    }
}
fn citations(xml: &str) -> Result<bool, BadReply> {
    let mut reader = Reader::from_str(xml);
    let mut found = false;
    loop {
        let begin = reader.buffer_position() as usize;
        match reader.read_event().map_err(|_| BadReply)? {
            Event::Start(tag) if tag.name().as_ref() == "xref" => {
                reader.read_to_end(tag.name()).map_err(|_| BadReply)?;
                let node = Element(&xml[begin..reader.buffer_position() as usize]);
                if !matches!(node.attr("ref-type")?.as_deref(), Some("bibr" | "ref")) { return Ok(false); }
                found = true;
            }
            Event::Text(v) if v.as_ref().chars().any(|c| !c.is_whitespace() && !",;–-".contains(c)) => return Ok(false),
            Event::Start(_) | Event::Empty(_) | Event::CData(_) => return Ok(false),
            Event::Eof => return Ok(found),
            _ => {}
        }
    }
}
/// Validate at the same mixed-text boundary before interpreting source markup.
pub(super) fn checked(xml: &str) -> Result<biodata::Markdown, BadReply> {
    MixedText::from_xml(xml)?;
    biodata::Markdown::new(&inline(xml)?).map_err(|_| BadReply)
}
