//! Preserve ClinVar classification domains and submission provenance.
use crate::mapped::text;
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
pub(super) fn has_classification(class: Option<Element<'_>>) -> Result<bool, BadReply> {
    let Some(class) = class else {
        return Ok(false);
    };
    for (name, _) in DOMAINS {
        if class.child(name)?.is_some() {
            return Ok(true);
        }
    }
    Ok(false)
}
pub(super) fn aggregate(
    record: Element<'_>,
    class: Element<'_>,
    kind: AssertionKind,
    subject: VariantRef,
    conditions: Vec<AssertionCondition>,
    problems: &mut crate::mapped::Problems,
    refusals: &mut crate::Refusal,
) -> Result<Vec<VariantAssertion>, BadReply> {
    problems.record = record.attr("Accession")?.and_then(crate::mapped::record);
    let mut rows = Vec::new();
    for (name, domain) in DOMAINS {
        let Some(group) = class.child(name)? else {
            continue;
        };
        let descriptions = group.children("Description")?;
        for description in descriptions {
            let Some(label) = problems.text("description", Some(&description.text()?)) else {
                *refusals = crate::Refusal::Refused;
                continue;
            };
            let evaluated = description
                .attr("DateLastEvaluated")?
                .or(group.attr("DateLastEvaluated")?);
            let Some(attribution) = attribution(record, evaluated, None, kind, problems)? else {
                *refusals = crate::Refusal::Refused;
                continue;
            };
            let review = problems.text("review_status", group.value(&["ReviewStatus"])?.as_deref());
            let mut row_conditions = conditions.clone();
            if let Some(list) = group.child("ConditionList")? {
                for condition in super::detail::conditions(list, problems)? {
                    if !row_conditions.contains(&condition) {
                        row_conditions.push(condition);
                    }
                }
            }
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
                conditions_missing: kind == AssertionKind::Rcv && row_conditions.is_empty(),
                conditions: row_conditions.into(),
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
    refusals: &mut crate::Refusal,
    evidence_only: bool,
) -> Result<Vec<VariantAssertion>, BadReply> {
    problems.record = None;
    let Some(accession) = record.child("ClinVarAccession")? else {
        problems.required::<()>("ClinVarAccession", None);
        *refusals = crate::Refusal::Refused;
        return Ok(Vec::new());
    };
    let class = record.child("Classification")?;
    let unclassified_record = evidence_only && !has_classification(class)?;
    if class.is_none() && !unclassified_record {
        return Ok(Vec::new());
    }
    let evaluated = class
        .map(|class| class.attr("DateLastEvaluated"))
        .transpose()?
        .flatten();
    let submitter = accession.attr("SubmitterName")?;
    let Some(attribution) = attribution(
        accession,
        evaluated,
        submitter,
        AssertionKind::Scv,
        problems,
    )?
    else {
        *refusals = crate::Refusal::Refused;
        return Ok(Vec::new());
    };
    let detail = super::detail::submission(record, problems)?;
    if unclassified_record {
        return Ok(vec![unclassified(
            AssertionKind::Scv,
            attribution,
            class,
            subject,
            detail,
            problems,
        )?]);
    }
    let class = class.ok_or(BadReply)?;
    let review = problems.text("review_status", class.value(&["ReviewStatus"])?.as_deref());
    classified_submission(
        class,
        attribution,
        subject,
        detail,
        review,
        problems,
        refusals,
    )
}

fn classified_submission(
    class: Element<'_>,
    attribution: Attribution,
    subject: VariantRef,
    detail: AssertionDetail,
    review: Option<biodata::Text>,
    problems: &mut crate::mapped::Problems,
    refusals: &mut crate::Refusal,
) -> Result<Vec<VariantAssertion>, BadReply> {
    let mut rows = Vec::new();
    for (name, domain) in DOMAINS {
        let Some(value) = class.child(name)? else {
            continue;
        };
        let label = value.value(&["Description"])?.unwrap_or(value.text()?);
        let Some(label) = problems.text("description", Some(&label)) else {
            *refusals = crate::Refusal::Refused;
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
pub(super) fn unclassified_rcv(
    record: Element<'_>,
    group: Option<Element<'_>>,
    subject: VariantRef,
    conditions: Vec<AssertionCondition>,
    problems: &mut crate::mapped::Problems,
    refusals: &mut crate::Refusal,
) -> Result<Option<VariantAssertion>, BadReply> {
    let Some(attribution) = attribution(record, None, None, AssertionKind::Rcv, problems)? else {
        *refusals = crate::Refusal::Refused;
        return Ok(None);
    };
    let detail = AssertionDetail {
        conditions_missing: conditions.is_empty(),
        conditions: conditions.into(),
        ..Default::default()
    };
    unclassified(
        AssertionKind::Rcv,
        attribution,
        group,
        subject,
        detail,
        problems,
    )
    .map(Some)
}

fn unclassified(
    kind: AssertionKind,
    attribution: Attribution,
    group: Option<Element<'_>>,
    subject: VariantRef,
    detail: AssertionDetail,
    problems: &mut crate::mapped::Problems,
) -> Result<VariantAssertion, BadReply> {
    let review = group
        .map(|group| group.value(&["ReviewStatus"]))
        .transpose()?
        .flatten();
    Ok(crate::assertion::row(
        kind,
        AssertionDomain::Unclassified,
        text("no classification provided")?,
        attribution,
        subject,
        detail,
        problems.text("review_status", review.as_deref()),
    ))
}

fn attribution(
    record: Element<'_>,
    evaluated: Option<String>,
    submitter: Option<String>,
    kind: AssertionKind,
    problems: &mut crate::mapped::Problems,
) -> Result<Option<Attribution>, BadReply> {
    problems.record = None;
    let Some(id) = record.attr("Accession")? else {
        problems.required::<()>("Accession", None);
        return Ok(None);
    };
    problems.record = crate::mapped::record(&id);
    let version = record.attr("Version")?;
    let full = version
        .as_ref()
        .map_or_else(|| id.clone(), |v| format!("{id}.{v}"));
    let valid = |value: &str| match kind {
        AssertionKind::Vcv => value.parse::<biodata::VcvId>().is_ok(),
        AssertionKind::Rcv => value.parse::<biodata::RcvId>().is_ok(),
        AssertionKind::Scv => value.parse::<biodata::ScvId>().is_ok(),
        AssertionKind::Erepo => false,
    };
    if !valid(&full) {
        let field = if valid(&id) { "Version" } else { "Accession" };
        problems.required::<()>(field, None);
        return Ok(None);
    }
    Ok(Some(Attribution {
        approved: None,
        published: None,
        source: Source::ClinVar.text(),
        record: Some(text(&id)?),
        version: problems.checked_text("version", version.as_deref()),
        release: None,
        submitter: problems.text("submitter", submitter.as_deref()),
        evaluated: problems.optional("evaluated", evaluated.map(|s| s.parse())),
        url: Some(text(&format!(
            "https://www.ncbi.nlm.nih.gov/clinvar/?term={full}"
        ))?),
    }))
}
