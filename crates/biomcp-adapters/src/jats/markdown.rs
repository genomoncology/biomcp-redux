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
            Event::Start(tag) if matches!(tag.name().as_ref(), "sup" | "xref" | "list") => {
                let span = reader.read_to_end(tag.name()).map_err(|_| BadReply)?;
                let whole = Element(&xml[begin..reader.buffer_position() as usize]);
                let body = &xml[span.start as usize..span.end as usize];
                if tag.name().as_ref() == "list" {
                    out.push_str(&list(whole)?);
                    out.push_str("\n\n");
                } else if tag.name().as_ref() == "sup" {
                    if citations(body)? {
                        out.push_str(&format!("[{}]", whole.text()?));
                    } else {
                        out.push_str(&format!("<sup>{}</sup>", inline(body)?));
                    }
                } else if matches!(whole.attr("ref-type")?.as_deref(), Some("bibr" | "ref")) {
                    out.push_str(&format!("[{}]", whole.text()?));
                } else {
                    out.push_str(&inline(body)?);
                }
            }
            Event::Start(tag) => out.push_str(marker(tag.name().as_ref(), false)),
            Event::End(tag) => out.push_str(marker(tag.name().as_ref(), true)),
            Event::Empty(tag) if matches!(tag.name().as_ref(), "break" | "hr") => out.push('\n'),
            Event::Text(v) => text(&mut out, v.as_ref()),
            Event::CData(v) => text(&mut out, v.as_ref()),
            Event::GeneralRef(v) => {
                if let Some(ch) = v.resolve_char_ref().map_err(|_| BadReply)? {
                    text(&mut out, &ch.to_string());
                } else {
                    text(
                        &mut out,
                        quick_xml::escape::resolve_predefined_entity(v.as_ref()).ok_or(BadReply)?,
                    );
                }
            }
            Event::Eof => return Ok(out),
            _ => {}
        }
    }
}
fn marker(name: &str, closing: bool) -> &'static str {
    match (name, closing) {
        ("bold", _) => "**",
        ("italic", _) => "*",
        ("sub", false) => "<sub>",
        ("sub", true) => "</sub>",
        ("p", true) => "\n\n",
        _ => "",
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
                if !matches!(node.attr("ref-type")?.as_deref(), Some("bibr" | "ref")) {
                    return Ok(false);
                }
                found = true;
            }
            Event::Text(v)
                if v.as_ref()
                    .chars()
                    .any(|c| !c.is_whitespace() && !",;–-".contains(c)) =>
            {
                return Ok(false);
            }
            Event::Start(_) | Event::Empty(_) | Event::CData(_) | Event::GeneralRef(_) => return Ok(false),
            Event::Eof => return Ok(found),
            _ => {}
        }
    }
}
/// Validate at the same mixed-text boundary before interpreting source markup.
pub(super) fn checked(xml: &str) -> Result<biodata::Markdown, BadReply> {
    MixedText::from_xml(xml)?;
    biodata::Markdown::new(inline(xml)?.trim()).map_err(|_| BadReply)
}

fn list(node: Element<'_>) -> Result<String, BadReply> {
    let mut out = Vec::new();
    for item in node.children("list-item")? {
        let rendered = inline(item.0)?;
        let mut lines = rendered.trim().lines();
        let first = lines.next().unwrap_or("");
        let mut text = format!("- {first}");
        for line in lines {
            text.push_str(&format!("\n  {line}"));
        }
        out.push(text);
    }
    Ok(out.join("\n"))
}

fn text(out: &mut String, value: &str) {
    for ch in value.chars() {
        match ch {
            '\\' | '*' | '_' | '[' | ']' | '`' => {
                out.push('\\');
                out.push(ch);
            }
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            c => out.push(c),
        }
    }
}
