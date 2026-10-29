pub(super) use crate::mapped::text;
use crate::xml_element::Element;
use biodata::{Attribution, VariantRef};

use biomcp_fetch::BadReply;
use biomcp_types::{
    AssertionCondition, AssertionDetail, AssertionDomain, AssertionKind, Source, VariantAssertion,
};
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
    problems: &mut crate::mapped::Problems,
) -> Result<Vec<VariantAssertion>, BadReply> {
    problems.record = record.attr("Accession")?.as_deref().map(text).transpose()?;
    let mut rows = Vec::new();
    for (name, domain) in DOMAINS {
        let Some(group) = class.child(name)? else {
            continue;
        };
        let descriptions = group.children("Description")?;
        for description in descriptions {
            let Some(label) = problems.text("description", Some(&description.text()?)) else {
                continue;
            };
            let evaluated = description
                .attr("DateLastEvaluated")?
                .or(group.attr("DateLastEvaluated")?);
            let attribution = attribution(record, evaluated, None, problems)?;
            let review = problems.text("review_status", group.value(&["ReviewStatus"])?.as_deref());
            let detail = AssertionDetail {
                impact_therapies: problems.text(
                    "impact_therapies",
                    description.attr("DrugForTherapeuticAssertion")?.as_deref(),
                ),
                impact_assertion_type: problems.text(
                    "impact_assertion_type",
                    description.attr("ClinicalImpactAssertionType")?.as_deref(),
                ),
                impact_clinical_significance: problems.text(
                    "impact_clinical_significance",
                    description
                        .attr("ClinicalImpactClinicalSignificance")?
                        .as_deref(),
                ),
                conditions: if let Some(list) = group.child("ConditionList")? {
                    super::detail::conditions(list, problems)?.into()
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
    problems: &mut crate::mapped::Problems,
) -> Result<Vec<VariantAssertion>, BadReply> {
    let accession = record.child("ClinVarAccession")?.ok_or(BadReply)?;
    let Some(class) = record.child("Classification")? else {
        return Ok(Vec::new());
    };
    let attribution = attribution(
        accession,
        class.attr("DateLastEvaluated")?,
        accession.attr("SubmitterName")?,
        problems,
    )?;
    let detail = super::detail::submission(record, problems)?;
    let review = problems.text("review_status", class.value(&["ReviewStatus"])?.as_deref());
    let mut rows = Vec::new();
    for (name, domain) in DOMAINS {
        let Some(value) = class.child(name)? else {
            continue;
        };
        let label = value.value(&["Description"])?.unwrap_or(value.text()?);
        let Some(label) = problems.text("description", Some(&label)) else {
            continue;
        };
        let mut detail = detail.clone();
        detail.impact_assertion_type = problems.text(
            "impact_assertion_type",
            value.attr("ClinicalImpactAssertionType")?.as_deref(),
        );
        detail.impact_clinical_significance = problems.text(
            "impact_clinical_significance",
            value.attr("ClinicalImpactClinicalSignificance")?.as_deref(),
        );
        detail.impact_therapies = problems.text(
            "impact_therapies",
            value.attr("DrugForTherapeuticAssertion")?.as_deref(),
        );
        detail.comments = class
            .children("Comment")?
            .iter()
            .map(|c| c.text())
            .collect::<Result<Vec<_>, _>>()?
            .iter()
            .filter_map(|s| problems.text("comments", Some(s)))
            .collect();
        rows.push(crate::assertion::row(
            AssertionKind::Scv,
            domain,
            label,
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
    problems: &mut crate::mapped::Problems,
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
    problems.record = Some(text(&id)?);
    Ok(Attribution {
        source: Source::ClinVar.text(),
        record: Some(text(&id)?),
        version: problems.checked_text("version", version.as_deref()),
        release: None,
        submitter: problems.text("submitter", submitter.as_deref()),
        evaluated: problems.optional("evaluated", evaluated.map(|s| s.parse())),
        url: Some(text(&format!(
            "https://www.ncbi.nlm.nih.gov/clinvar/?term={full}"
        ))?),
    })
}
