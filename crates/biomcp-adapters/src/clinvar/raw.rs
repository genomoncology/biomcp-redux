use crate::xml_element::Element;
use biodata::VcvId;
use biomcp_fetch::{BadReply, Reply, xml::XmlReply};
use biomcp_types::{AssertionKind, ClinvarIncluded, VariantAssertion};
use serde::Deserialize;
#[derive(Deserialize)]
pub(super) struct Records {
    pub id: Option<String>,
    pub rows: Vec<VariantAssertion>,
    pub included_only: Option<ClinvarIncluded>,
    pub problems: Vec<biomcp_types::Problem>,
    pub refusals: crate::Refusal,
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
        let root = Element::root(xml, "ClinVarResult-Set")?;
        if root.child("ERROR")?.is_some() {
            return Err(BadReply);
        }
        let archives = root.children("VariationArchive")?;
        if archives.is_empty() {
            return Ok(Self {
                id: None,
                rows: Vec::new(),
                included_only: None,
                problems: Vec::new(),
                refusals: crate::Refusal::None,
                accession: None,
                version: None,
            });
        }
        if archives.len() != 1 {
            return Err(BadReply);
        }
        let archive = *archives.first().ok_or(BadReply)?;
        let mut reply = Self {
            id: archive.attr("VariationID")?,
            accession: archive.attr("Accession")?,
            version: archive.attr("Version")?,
            rows: Vec::new(),
            included_only: None,
            problems: Vec::new(),
            refusals: crate::Refusal::None,
        };
        let mut problems = crate::mapped::Problems {
            source: biomcp_types::Source::ClinVar,
            record: reply.accession.as_ref().and_then(crate::mapped::record),
            list: Vec::new(),
        };
        let id = problems.optional(
            "VariationID",
            Some(reply.id.as_ref().ok_or(BadReply).and_then(|id| {
                format!("clinvar:{id}")
                    .parse::<biodata::VariantId>()
                    .map_err(|_| BadReply)
            })),
        );
        if let Some(classified) = archive.child("ClassifiedRecord")? {
            let subject = match id {
                Some(id) => super::subject::map(archive, classified, id, &mut problems)?,
                None => None,
            };
            if let Some(subject) = subject {
                reply.rows = assertions(
                    archive,
                    classified,
                    subject,
                    &mut problems,
                    &mut reply.refusals,
                )?;
            } else {
                reply.refusals = crate::Refusal::Refused;
            }
        } else {
            let included = archive.child("IncludedRecord")?.ok_or(BadReply)?;
            reply.included_only = Some(included_only(
                included,
                reply.accession.as_deref(),
                reply.version.as_deref(),
            )?);
        }
        reply.problems = problems.list;
        Ok(reply)
    }
}

impl Records {
    pub(super) fn archive(&self) -> Result<Option<VcvId>, BadReply> {
        let Some(id) = self.id.as_deref() else {
            return Ok(None);
        };
        let accession: VcvId = self
            .accession
            .as_deref()
            .ok_or(BadReply)?
            .parse()
            .map_err(|_| BadReply)?;
        let version = self
            .version
            .as_deref()
            .ok_or(BadReply)?
            .parse()
            .map_err(|_| BadReply)?;
        if id.parse::<u32>().map_err(|_| BadReply)? != accession.get() {
            return Err(BadReply);
        }
        VcvId::new(accession.get(), Some(version))
            .map(Some)
            .map_err(|_| BadReply)
    }

    pub(super) fn identity_problem(
        &self,
        id: &str,
        selected: Option<(&str, Option<&str>)>,
    ) -> Option<&'static str> {
        if self
            .problems
            .iter()
            .any(|problem| problem.field.as_ref() == "VariationID")
            || self.id.as_deref().is_some_and(|given| given != id)
        {
            return Some("VariationID");
        }
        if let Some((accession, version)) = selected.filter(|_| self.id.is_some()) {
            if self.accession.as_deref() != Some(accession) {
                return Some("Accession");
            }
            if version.is_some_and(|version| self.version.as_deref() != Some(version)) {
                return Some("Version");
            }
        }
        None
    }
}

fn included_only(
    record: Element<'_>,
    accession: Option<&str>,
    version: Option<&str>,
) -> Result<ClinvarIncluded, BadReply> {
    let list = record.child("ClassifiedVariationList")?.ok_or(BadReply)?;
    let mut included_in = Vec::new();
    for parent in list.children("ClassifiedVariation")? {
        let mut accession = parent.attr("Accession")?.ok_or(BadReply)?;
        if let Some(version) = parent.attr("Version")? {
            accession.push('.');
            accession.push_str(&version);
        }
        included_in.push(accession.parse().map_err(|_| BadReply)?);
    }
    if included_in.is_empty() {
        return Err(BadReply);
    }
    Ok(ClinvarIncluded {
        record: accession.ok_or(BadReply)?.parse().map_err(|_| BadReply)?,
        version: version.map(crate::mapped::text).transpose()?,
        included_in: included_in.into(),
    })
}

fn assertions(
    archive: Element<'_>,
    classified: Element<'_>,
    subject: biodata::VariantRef,
    problems: &mut crate::mapped::Problems,
    refusals: &mut crate::Refusal,
) -> Result<Vec<VariantAssertion>, BadReply> {
    let mut rows = Vec::new();
    let classifications = classified.child("Classifications")?;
    // Mixed archives keep the rows from their existing classification domains.
    let evidence_only = !super::map::has_classification(classifications)?;
    if let Some(class) = classifications {
        rows.extend(super::map::aggregate(
            archive,
            class,
            AssertionKind::Vcv,
            subject.clone(),
            Vec::new(),
            problems,
            refusals,
        )?);
    }
    if let Some(list) = classified.child("RCVList")? {
        rows.extend(rcvs(list, &subject, evidence_only, problems, refusals)?);
    }
    if let Some(scvs) = classified.child("ClinicalAssertionList")? {
        for scv in scvs.children("ClinicalAssertion")? {
            rows.extend(super::map::submission(
                scv,
                subject.clone(),
                problems,
                refusals,
                evidence_only,
            )?);
        }
    }
    Ok(rows)
}

fn rcvs(
    list: Element<'_>,
    subject: &biodata::VariantRef,
    evidence_only: bool,
    problems: &mut crate::mapped::Problems,
    refusals: &mut crate::Refusal,
) -> Result<Vec<VariantAssertion>, BadReply> {
    let mut rows = Vec::new();
    for rcv in list.children("RCVAccession")? {
        let class = rcv.child("RCVClassifications")?;
        if class.is_none() && !evidence_only {
            continue;
        }
        problems.record = rcv.attr("Accession")?.and_then(crate::mapped::record);
        let conditions = super::detail::rcv_conditions(rcv, problems)?;
        if evidence_only && !super::map::has_classification(class)? {
            let group = class
                .map(|class| class.child("NoClassification"))
                .transpose()?
                .flatten();
            rows.extend(super::map::unclassified_rcv(
                rcv,
                group,
                subject.clone(),
                conditions,
                problems,
                refusals,
            )?);
        } else if let Some(class) = class {
            rows.extend(super::map::aggregate(
                rcv,
                class,
                AssertionKind::Rcv,
                subject.clone(),
                conditions,
                problems,
                refusals,
            )?);
        }
    }
    Ok(rows)
}
