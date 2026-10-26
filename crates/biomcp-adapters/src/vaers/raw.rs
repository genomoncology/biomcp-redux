//! Lossless native cells and applied notices; malformed tables never enter cache.
use biomcp_fetch::{BadReply, Reply, xml::XmlReply};
use biomcp_types::{VaersCell, VaersNotice, VaersRow, VaersScope, VaersSummary};
use quick_xml::{
    Reader,
    events::{BytesStart, Event},
};
use serde::Deserialize;

const MAX_XML: usize = 2 * 1024 * 1024;

#[derive(Deserialize)]
pub(super) struct Native(pub VaersSummary);
impl Reply for Native {
    fn check(&self) -> Result<(), BadReply> {
        Ok(())
    }
}
impl XmlReply for Native {
    fn decode_xml(xml: &str) -> Result<Self, BadReply> {
        if xml.len() > MAX_XML {
            return Err(BadReply);
        }
        map(document(xml)?).map(Self)
    }
}
#[derive(Default)]
struct Node {
    name: String,
    attrs: VaersCell,
    text: String,
    children: Vec<Node>,
}
impl Node {
    fn child(&self, name: &str) -> Result<&Self, BadReply> {
        let mut found = self.children.iter().filter(|n| n.name == name);
        let result = found.next().ok_or(BadReply)?;
        if found.next().is_some() {
            return Err(BadReply);
        }
        Ok(result)
    }
}
fn element(tag: &BytesStart<'_>) -> Result<Node, BadReply> {
    let mut n = Node {
        name: tag.name().as_ref().to_owned(),
        ..Node::default()
    };
    for attr in tag.attributes() {
        let attr = attr.map_err(|_| BadReply)?;
        let key = attr.key.as_ref().to_owned().into_boxed_str();
        let value = attr
            .normalized_value(quick_xml::XmlVersion::Implicit1_0)
            .map_err(|_| BadReply)?;
        if n.attrs
            .insert(key, value.into_owned().into_boxed_str())
            .is_some()
        {
            return Err(BadReply);
        }
    }
    Ok(n)
}
fn document(xml: &str) -> Result<Node, BadReply> {
    let mut reader = Reader::from_str(xml);
    let mut stack = vec![Node::default()];
    loop {
        match reader.read_event().map_err(|_| BadReply)? {
            Event::Start(tag) => stack.push(element(&tag)?),
            Event::Empty(tag) => stack
                .last_mut()
                .ok_or(BadReply)?
                .children
                .push(element(&tag)?),
            Event::End(_) => {
                let node = stack.pop().ok_or(BadReply)?;
                stack.last_mut().ok_or(BadReply)?.children.push(node);
            }
            Event::Text(text) => stack
                .last_mut()
                .ok_or(BadReply)?
                .text
                .push_str(text.as_ref()),
            Event::CData(text) => stack
                .last_mut()
                .ok_or(BadReply)?
                .text
                .push_str(text.as_ref()),
            Event::GeneralRef(value) => {
                let text = if let Some(ch) = value.resolve_char_ref().map_err(|_| BadReply)? {
                    ch.to_string()
                } else {
                    quick_xml::escape::resolve_predefined_entity(value.as_ref())
                        .ok_or(BadReply)?
                        .to_owned()
                };
                stack.last_mut().ok_or(BadReply)?.text.push_str(&text);
            }
            Event::Eof => break,
            _ => (),
        }
        if stack.len() > 64 {
            return Err(BadReply);
        }
    }
    if stack.len() != 1 {
        return Err(BadReply);
    }
    let mut root = stack.pop().ok_or(BadReply)?;
    if root.children.len() != 1 {
        return Err(BadReply);
    }
    let page = root.children.pop().ok_or(BadReply)?;
    if page.name != "page" {
        return Err(BadReply);
    }
    Ok(page)
}
fn map(page: Node) -> Result<VaersSummary, BadReply> {
    let dataset = page.child("dataset")?;
    let response = page.child("response")?;
    let request = response.child("request")?;
    let scope = scope(dataset, request)?;
    let vaccine = request
        .child("variable-selections")?
        .children
        .iter()
        .find(|n| {
            n.name == "variable" && n.attrs.get("code").is_some_and(|v| v.as_ref() == "D8.V14")
        })
        .ok_or(BadReply)?
        .child("value")?
        .attrs
        .clone();
    let grouping = request
        .child("byvariables")?
        .child("variable")?
        .attrs
        .clone();
    let (rows, footer) = table_rows(response.child("data-table")?)?;
    let report_total = page
        .children
        .iter()
        .filter(|n| n.name == "message")
        .find_map(|n| {
            n.text
                .strip_prefix("These results are for ")
                .and_then(|v| v.strip_suffix(" total events."))
        })
        .ok_or(BadReply)?
        .into();
    let measures = dataset
        .children
        .iter()
        .filter(|n| n.name == "measure")
        .filter(|n| {
            n.attrs
                .get("code")
                .is_some_and(|v| matches!(v.as_ref(), "D8.M1" | "D8.M2"))
        })
        .map(|n| n.attrs.clone())
        .collect::<Vec<_>>();
    if measures.len() != 2 {
        return Err(BadReply);
    }
    let mut notices = Vec::new();
    for node in page.children.iter().filter(|n| n.name == "message") {
        notice(node, "message", &mut notices);
    }
    for kind in ["footnotes", "caveats"] {
        for node in &response.child(kind)?.children {
            notice(node, kind, &mut notices);
        }
    }
    Ok(VaersSummary {
        vaccine,
        scope,
        grouping,
        dataset: dataset.attrs.clone(),
        measures: measures.into(),
        measure_metadata: response.child("measure-selections")?.children.iter().map(|n| n.attrs.clone()).collect(),
        report_total,
        footer,
        rows: rows.into(),
        notices: notices.into(),
        interpretation: "Counts are VAERS reports, not people. Reports do not show that a vaccine caused an event and \
            cannot measure risk or compare vaccine safety. A report may mention several symptoms and several \
            vaccines. Each table retains its own source total and percentages."
            .into(),
    })
}
fn row(node: &Node) -> Result<VaersRow, BadReply> {
    let [label, reports, percent] = node.children.as_slice() else {
        return Err(BadReply);
    };
    if [label, reports, percent]
        .iter()
        .any(|n| n.name != "c" || !n.children.is_empty() || !n.text.trim().is_empty())
    {
        return Err(BadReply);
    }
    Ok(VaersRow {
        label: label.attrs.clone(),
        reports: reports.attrs.clone(),
        percent: percent.attrs.clone(),
    })
}
fn notice(node: &Node, kind: &str, notices: &mut Vec<VaersNotice>) {
    if !node.text.trim().is_empty() {
        notices.push(VaersNotice {
            kind: kind.into(),
            text: node.text.clone().into(),
        });
    }
    for child in &node.children {
        notice(child, kind, notices);
    }
}

fn table_rows(table: &Node) -> Result<(Vec<VaersRow>, VaersRow), BadReply> {
    if table.children.iter().any(|r| r.name != "r") {
        return Err(BadReply);
    }
    let mut rows = table
        .children
        .iter()
        .map(row)
        .collect::<Result<Vec<_>, _>>()?;
    let footer = rows.pop().ok_or(BadReply)?;
    if footer.label.get("c").is_none_or(|v| v.as_ref() != "1")
        || !footer.reports.contains_key("dt")
        || !footer.percent.contains_key("dt")
        || rows.iter().any(|r| {
            !r.label.contains_key("l")
                || !r.reports.contains_key("v")
                || !r.percent.contains_key("v")
        })
    {
        return Err(BadReply);
    }
    Ok((rows, footer))
}

fn scope(dataset: &Node, request: &Node) -> Result<VaersScope, BadReply> {
    let variable = dataset
        .children
        .iter()
        .find(|n| {
            n.name == "variable" && n.attrs.get("code").is_some_and(|v| v.as_ref() == "D8.V12")
        })
        .ok_or(BadReply)?;
    let selected = request
        .child("variable-selections")?
        .children
        .iter()
        .find(|n| {
            n.name == "variable" && n.attrs.get("code").is_some_and(|v| v.as_ref() == "D8.V12")
        })
        .ok_or(BadReply)?
        .child("value")?;
    if selected
        .attrs
        .get("code")
        .is_none_or(|v| v.as_ref() != "00")
    {
        return Err(BadReply);
    }
    let value = variable
        .children
        .iter()
        .find(|n| n.name == "value" && n.attrs.get("code").is_some_and(|v| v.as_ref() == "00"))
        .ok_or(BadReply)?;
    if !value.attrs.contains_key("label") {
        return Err(BadReply);
    }
    Ok(VaersScope {
        variable: variable.attrs.clone(),
        value: value.attrs.clone(),
    })
}
