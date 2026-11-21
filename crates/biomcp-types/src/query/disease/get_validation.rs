//! Validate disease selectors before admission.
use super::{
    DiseaseGet, DiseaseId, DiseaseOntology, DiseaseSection, DiseaseSource, default_limit,
    get::GetFields, get_source,
};
use crate::{Text, query::gene::QueryInvalid};
impl TryFrom<GetFields> for DiseaseGet {
    type Error = QueryInvalid;
    fn try_from(mut v: GetFields) -> Result<Self, Self::Error> {
        let statistic = matches!(
            v.section,
            Some(DiseaseSection::Incidence | DiseaseSection::Survival)
        );
        if v.section_limit
            .is_some_and(|limit| !(1..=50).contains(&limit))
        {
            return Err(QueryInvalid::new(&["section_limit"]));
        }
        let validity = v.section == Some(DiseaseSection::GeneValidity);
        if !validity && (v.classification.is_some() || v.submitter.is_some()) {
            return Err(QueryInvalid::new(&[
                "section",
                "classification",
                "submitter",
            ]));
        }
        if !validity {
            v.source.get_or_insert_with(get_source);
        }
        let source = v.source.unwrap_or_else(get_source);
        let id = get_id(&v, source, statistic, validity)?;
        // Name the ID, not the section word: prevalence is a valid section, and the message
        // names the one source that serves it.
        if v.section == Some(DiseaseSection::Prevalence) && source != DiseaseSource::Orphadata {
            return Err(QueryInvalid::new(&["id", "section"])
                .with_message(crate::ErrorMessage::DiseasePrevalenceSource));
        }
        if source == DiseaseSource::Orphadata
            && !matches!(&id, DiseaseId::Concept(id) if orphadata_id(id.as_str()))
        {
            return Err(QueryInvalid::new(&["source", "id"])
                .with_message(crate::ErrorMessage::DiseasePrevalenceSource));
        }
        validate_selectors(&v, source)?;
        if source == DiseaseSource::Ols {
            // An OLS read looks the ID up in one ontology, so default to the one the ID names.
            v.ontology
                .get_or_insert_with(|| DiseaseOntology::of(id.as_str()).unwrap_or_default());
        }
        v.section_limit.get_or_insert(match (source, v.section) {
            (DiseaseSource::Monarch, Some(DiseaseSection::Genes)) => 2,
            (DiseaseSource::Monarch, Some(DiseaseSection::Phenotypes)) => 1,
            _ => default_limit(),
        });
        Ok(Self {
            record: v.id,
            fields: GetFields {
                id,
                source: v.source,
                ontology: v.ontology,
                section: v.section,
                section_limit: v.section_limit,
                section_offset: v.section_offset,
                sex: v.sex,
                race: v.race,
                age_range: v.age_range,
                dataset: v.dataset,
                period: v.period,
                interval: v.interval,
                classification: v.classification,
                submitter: v.submitter,
            },
        })
    }
}

fn validate_selectors(v: &GetFields, source: DiseaseSource) -> Result<(), QueryInvalid> {
    if source != DiseaseSource::Seer
        && [
            &v.sex,
            &v.race,
            &v.age_range,
            &v.dataset,
            &v.period,
            &v.interval,
        ]
        .iter()
        .any(|v| v.is_some())
    {
        return Err(QueryInvalid::new(&[
            "source",
            "sex",
            "race",
            "age_range",
            "dataset",
            "period",
            "interval",
        ]));
    }
    if source != DiseaseSource::Ols && v.ontology.is_some() {
        return Err(QueryInvalid::new(&["source", "ontology"]));
    }
    if v.section != Some(DiseaseSection::Incidence) && v.period.is_some() {
        return Err(QueryInvalid::new(&["section", "period"]));
    }
    if v.section != Some(DiseaseSection::Survival) && v.interval.is_some() {
        return Err(QueryInvalid::new(&["section", "interval"]));
    }
    Ok(())
}

/// Orphadata reads an Orphanet code directly, or a MONDO or DOID concept through its Orphanet links.
fn orphadata_id(id: &str) -> bool {
    id.split_once(':').is_some_and(|(prefix, local)| {
        ["MONDO", "DOID", "Orphanet", "ORPHA"]
            .iter()
            .any(|p| p.eq_ignore_ascii_case(prefix))
            && !local.is_empty()
            && local.bytes().all(|b| b.is_ascii_digit())
    })
}
/// A known disease namespace in its published letter case, with an uppercase local part, since sources
/// match IDs exactly.
fn canonical(id: &str) -> String {
    const PREFIXES: [&str; 7] = ["MONDO", "DOID", "NCIT", "OMIM", "ORPHA", "Orphanet", "HP"];
    match id.split_once(':') {
        Some((prefix, local)) => PREFIXES
            .iter()
            .find(|known| known.eq_ignore_ascii_case(prefix))
            .map_or_else(
                || id.to_owned(),
                |known| format!("{known}:{}", local.to_ascii_uppercase()),
            ),
        None => id.to_owned(),
    }
}
/// Bind the record to a SEER site or an ontology concept, naming the real fault.
fn get_id(
    v: &GetFields<Text>,
    source: DiseaseSource,
    statistic: bool,
    validity: bool,
) -> Result<DiseaseId, QueryInvalid> {
    if validity {
        let concept = canonical(v.id.as_str()).parse();
        return Ok(concept.map_or_else(
            |_| DiseaseId::CurationName(v.id.clone()),
            DiseaseId::Concept,
        ));
    }
    Ok(if source == DiseaseSource::Seer {
        if statistic && v.id.as_str().contains(':') {
            // SEER is the right source; the ontology ID is the fault.
            return Err(QueryInvalid::new(&["id", "source", "section"])
                .with_message(crate::ErrorMessage::DiseaseStatistic));
        }
        if !statistic || v.id.as_str().trim().is_empty() || v.id.as_str().contains(':') {
            return Err(QueryInvalid::new(&["source", "id", "section"]));
        }
        DiseaseId::SeerSite(v.id.clone())
    } else {
        if statistic {
            return Err(QueryInvalid::new(&["source", "id", "section"])
                .with_message(crate::ErrorMessage::DiseaseStatistic));
        }
        DiseaseId::Concept(
            canonical(v.id.as_str())
                .parse()
                .map_err(|_| QueryInvalid::new(&["source", "id", "section"]))?,
        )
    })
}
