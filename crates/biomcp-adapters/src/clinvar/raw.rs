use crate::xml_element::Element;
use biomcp_fetch::{BadReply, Reply, xml::XmlReply};
use biomcp_types::{AssertionKind, VariantAssertion};
use serde::Deserialize;
#[derive(Deserialize)]
pub(super) struct Records {
    pub id: Option<String>,
    pub rows: Vec<VariantAssertion>,
    pub problems: Vec<biomcp_types::Problem>,
    pub accession: Option<String>,
    pub version: Option<String>,
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
                problems: Vec::new(),
                accession: None,
                version: None,
            });
        }
        if archives.len() != 1 {
            return Err(BadReply);
        }
        let archive = *archives.first().ok_or(BadReply)?;
        let id = archive.attr("VariationID")?.ok_or(BadReply)?;
        format!("clinvar:{id}")
            .parse::<biodata::VariantId>()
            .map_err(|_| BadReply)?;
        let classified = archive.child("ClassifiedRecord")?.ok_or(BadReply)?;
        let subject =
            biodata::VariantRef::allele(format!("clinvar:{id}").parse().map_err(|_| BadReply)?);
        let mut rows = Vec::new();
        let mut problems = crate::mapped::Problems {
            source: biomcp_types::Source::ClinVar,
            record: archive
                .attr("Accession")?
                .and_then(|s| crate::mapped::text(&s).ok()),
            list: Vec::new(),
        };
        if let Some(class) = classified.child("Classifications")? {
            rows.extend(super::map::aggregate(
                archive,
                class,
                AssertionKind::Vcv,
                subject.clone(),
                Vec::new(),
                &mut problems,
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
                        super::detail::rcv_conditions(rcv, &mut problems)?,
                        &mut problems,
                    )?);
                }
            }
        }
        if let Some(scvs) = classified.child("ClinicalAssertionList")? {
            for scv in scvs.children("ClinicalAssertion")? {
                rows.extend(super::map::submission(scv, subject.clone(), &mut problems)?);
            }
        }
        Ok(Self {
            id: Some(id),
            rows,
            problems: problems.list,
            accession: archive.attr("Accession")?,
            version: archive.attr("Version")?,
        })
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
