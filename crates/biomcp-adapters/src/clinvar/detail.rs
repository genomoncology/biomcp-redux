use super::map::text;
use crate::xml_element::Element;
use biodata::{AssertionCondition, AssertionDetail, Text};
use biomcp_fetch::BadReply;
pub(super) fn rcv_conditions(record: Element<'_>) -> Result<Vec<AssertionCondition>, BadReply> {
    let Some(list) = record.child("ClassifiedConditionList")? else {
        return Ok(Vec::new());
    };
    list.children("ClassifiedCondition")?
        .into_iter()
        .map(|c| condition(c.attr("DB")?, c.attr("ID")?, Some(c.text()?)))
        .collect()
}
fn condition(
    db: Option<String>,
    id: Option<String>,
    label: Option<String>,
) -> Result<AssertionCondition, BadReply> {
    let id = match (db, id) {
        (Some(db), Some(id)) => Some(format!("{db}:{id}").parse().map_err(|_| BadReply)?),
        _ => None,
    };
    Ok(AssertionCondition {
        id,
        label: label
            .filter(|s| !s.trim().is_empty())
            .map(|s| text(&s))
            .transpose()?,
    })
}
pub(super) fn submission(record: Element<'_>) -> Result<AssertionDetail, BadReply> {
    let mut detail = AssertionDetail::default();
    let mut methods = Vec::new();
    let mut citations = Vec::new();
    for set in record.children("AttributeSet")? {
        for attribute in set.children("Attribute")? {
            if attribute.attr("Type")?.as_deref() == Some("AssertionMethod") {
                methods.push(text(&attribute.text()?)?);
            }
        }
        for citation in set.children("Citation")? {
            for id in citation.children("ID")? {
                if id.attr("Source")?.as_deref() == Some("PubMed") {
                    citations.push(id.text()?.parse().map_err(|_| BadReply)?);
                }
            }
        }
    }
    detail.methods = methods.into();
    detail.citations = citations.into();
    detail.conditions = conditions(record)?.into();
    let mut origins = Vec::<Text>::new();
    if let Some(observations) = record.child("ObservedInList")? {
        for observation in observations.children("ObservedIn")? {
            if let Some(value) = observation.value(&["Sample", "Origin"])? {
                let value = text(&value)?;
                if !origins.contains(&value) {
                    origins.push(value);
                }
            }
        }
    }
    detail.origins = origins.into();
    Ok(detail)
}
fn conditions(record: Element<'_>) -> Result<Vec<AssertionCondition>, BadReply> {
    let Some(set) = record.child("TraitSet")? else {
        return Ok(Vec::new());
    };
    let mut conditions = Vec::new();
    for trait_ in set.children("Trait")? {
        let mut label = None;
        for name in trait_.children("Name")? {
            if let Some(value) = name.value(&["ElementValue"])? {
                label = Some(value);
                break;
            }
        }
        let refs = trait_.children("XRef")?;
        if refs.is_empty() {
            if label.is_some() {
                conditions.push(condition(None, None, label)?);
            }
            continue;
        }
        for reference in refs {
            conditions.push(condition(
                reference.attr("DB")?,
                reference.attr("ID")?,
                label.clone(),
            )?);
        }
    }
    Ok(conditions)
}
