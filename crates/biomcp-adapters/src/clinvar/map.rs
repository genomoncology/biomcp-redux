use crate::xml_element::Element;
use biodata::{Attribution, Text, VariantRef};
use biomcp_types::{AssertionCondition, AssertionDetail, AssertionDomain, AssertionKind, Source, VariantAssertion};
use biomcp_fetch::BadReply;
const DOMAINS: [(&str, AssertionDomain); 3] = [
    ("GermlineClassification", AssertionDomain::Germline),
    (
        "SomaticClinicalImpact",
        AssertionDomain::SomaticClinicalImpact,
    ),
    ("OncogenicityClassification", AssertionDomain::Oncogenicity),
];
pub(super) fn aggregate(
    record: Element<'_>,
    class: Element<'_>,
    kind: AssertionKind,
    subject: VariantRef,
    conditions: Vec<AssertionCondition>,
) -> Result<Vec<VariantAssertion>, BadReply> {
    let mut rows = Vec::new();
    for (name, domain) in DOMAINS {
        let Some(group) = class.child(name)? else {
            continue;
        };
        let descriptions = group.children("Description")?;
        for description in descriptions {
            let label = text(&description.text()?)?;
            let evaluated = description
                .attr("DateLastEvaluated")?
                .or(group.attr("DateLastEvaluated")?);
            let attribution = attribution(record, evaluated, None)?;
            let review = group
                .value(&["ReviewStatus"])?
                .map(|s| text(&s))
                .transpose()?;
            let detail = AssertionDetail {
                impact_therapies: description
                    .attr("DrugForTherapeuticAssertion")?
                    .as_deref()
                    .map(text)
                    .transpose()?,
                impact_assertion_type: description
                    .attr("ClinicalImpactAssertionType")?
                    .as_deref()
                    .map(text)
                    .transpose()?,
                impact_clinical_significance: description
                    .attr("ClinicalImpactClinicalSignificance")?
                    .as_deref()
                    .map(text)
                    .transpose()?,
                conditions: if let Some(list) = group.child("ConditionList")? {
                    super::detail::conditions(list)?.into()
                } else {
                    conditions.clone().into()
                },
                ..Default::default()
            };
            rows.push(crate::assertion::row(
                kind,
                domain,
                label,
                attribution,
                subject.clone(),
                detail,
                review,
            ));
        }
    }
    Ok(rows)
}
pub(super) fn submission(
    record: Element<'_>,
    subject: VariantRef,
) -> Result<Vec<VariantAssertion>, BadReply> {
    let accession = record.child("ClinVarAccession")?.ok_or(BadReply)?;
    let Some(class) = record.child("Classification")? else {
        return Ok(Vec::new());
    };
    let detail = super::detail::submission(record)?;
    let attribution = attribution(
        accession,
        class.attr("DateLastEvaluated")?,
        accession.attr("SubmitterName")?,
    )?;
    let review = class
        .value(&["ReviewStatus"])?
        .map(|s| text(&s))
        .transpose()?;
    let mut rows = Vec::new();
    for (name, domain) in DOMAINS {
        let Some(value) = class.child(name)? else {
            continue;
        };
        let label = value.value(&["Description"])?.unwrap_or(value.text()?);
        let mut detail = detail.clone();
        detail.impact_assertion_type = value
            .attr("ClinicalImpactAssertionType")?
            .as_deref()
            .map(text)
            .transpose()?;
        detail.impact_clinical_significance = value
            .attr("ClinicalImpactClinicalSignificance")?
            .as_deref()
            .map(text)
            .transpose()?;
        detail.impact_therapies = value
            .attr("DrugForTherapeuticAssertion")?
            .as_deref()
            .map(text)
            .transpose()?;
        detail.comments = class
            .children("Comment")?
            .iter()
            .map(|c| text(&c.text()?))
            .collect::<Result<_, _>>()?;
        rows.push(crate::assertion::row(
            AssertionKind::Scv,
            domain,
            text(&label)?,
            attribution.clone(),
            subject.clone(),
            detail.clone(),
            review.clone(),
        ));
    }
    Ok(rows)
}
fn attribution(
    record: Element<'_>,
    evaluated: Option<String>,
    submitter: Option<String>,
) -> Result<Attribution, BadReply> {
    let id = record.attr("Accession")?.ok_or(BadReply)?;
    let version = record.attr("Version")?;
    let full = version
        .as_ref()
        .map_or_else(|| id.clone(), |v| format!("{id}.{v}"));
    if !(full.parse::<biodata::VcvId>().is_ok()
        || full.parse::<biodata::RcvId>().is_ok()
        || full.parse::<biodata::ScvId>().is_ok())
    {
        return Err(BadReply);
    }
    Ok(Attribution {
        source: Source::ClinVar.text(),
        record: Some(text(&id)?),
        version: version.map(|s| text(&s)).transpose()?,
        release: None,
        submitter: submitter.map(|s| text(&s)).transpose()?,
        evaluated: evaluated
            .map(|s| s.parse().map_err(|_| BadReply))
            .transpose()?,
        url: Some(text(&format!(
            "https://www.ncbi.nlm.nih.gov/clinvar/?term={full}"
        ))?),
    })
}
pub(super) fn text(value: &str) -> Result<Text, BadReply> {
    Text::new(value).map_err(|_| BadReply)
}
