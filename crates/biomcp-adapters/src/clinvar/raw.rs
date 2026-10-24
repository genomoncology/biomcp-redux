use crate::xml_element::Element;
use biodata::{AssertionKind, VariantAssertion};
use biomcp_fetch::{BadReply, Reply, xml::XmlReply};
use serde::Deserialize;
#[derive(Deserialize)]
pub(super) struct Records {
    pub id: Option<String>,
    pub rows: Vec<VariantAssertion>,
}
impl Reply for Records {
    fn check(&self) -> Result<(), BadReply> {
        Ok(())
    }
}
impl XmlReply for Records {
    fn decode_xml(xml: &str) -> Result<Self, BadReply> {
        let root = root(xml)?;
        if root.child("ERROR")?.is_some() {
            return Err(BadReply);
        }
        let archives = root.children("VariationArchive")?;
        if archives.is_empty() {
            return Ok(Self {
                id: None,
                rows: Vec::new(),
            });
        }
        if archives.len() != 1 {
            return Err(BadReply);
        }
        let archive = archives[0];
        let id = archive.attr("VariationID")?.ok_or(BadReply)?;
        format!("clinvar:{id}")
            .parse::<biodata::VariantId>()
            .map_err(|_| BadReply)?;
        let classified = archive.child("ClassifiedRecord")?.ok_or(BadReply)?;
        let subject =
            biodata::VariantRef::allele(format!("clinvar:{id}").parse().map_err(|_| BadReply)?);
        let mut rows = Vec::new();
        if let Some(class) = classified.child("Classifications")? {
            rows.extend(super::map::aggregate(
                archive,
                class,
                AssertionKind::Vcv,
                subject.clone(),
                Vec::new(),
            )?);
        }
        if let Some(rcvs) = classified.child("RCVList")? {
            for rcv in rcvs.children("RCVAccession")? {
                if let Some(class) = rcv.child("RCVClassifications")? {
                    rows.extend(super::map::aggregate(
                        rcv,
                        class,
                        AssertionKind::Rcv,
                        subject.clone(),
                        super::detail::rcv_conditions(rcv)?,
                    )?);
                }
            }
        }
        if let Some(scvs) = classified.child("ClinicalAssertionList")? {
            for scv in scvs.children("ClinicalAssertion")? {
                rows.extend(super::map::submission(scv, subject.clone())?);
            }
        }
        Ok(Self { id: Some(id), rows })
    }
}
fn root(xml: &str) -> Result<Element<'_>, BadReply> {
    let mut reader = quick_xml::Reader::from_str(xml);
    loop {
        let begin = reader.buffer_position() as usize;
        match reader.read_event().map_err(|_| BadReply)? {
            quick_xml::events::Event::Start(tag) | quick_xml::events::Event::Empty(tag) => {
                return if tag.name().as_ref() == "ClinVarResult-Set" {
                    Ok(Element(&xml[begin..]))
                } else {
                    Err(BadReply)
                };
            }
            quick_xml::events::Event::Eof => return Err(BadReply),
            _ => {}
        }
    }
}
