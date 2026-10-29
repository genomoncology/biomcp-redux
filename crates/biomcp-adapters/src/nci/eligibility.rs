//! Map publisher age numbers with their explicitly supplied units.
use crate::{mapped::Problems, raw::Field};
use biodata::{FiniteNumber, Text, WithUnit};
use biomcp_types::{NciEligibility, NciEligibilityStructured};
use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub(super) struct Eligibility {
    structured: Option<Structured>,
    #[serde(default)]
    unstructured: Box<[biomcp_types::NciCriterion]>,
}
#[derive(Debug, Deserialize)]
struct Structured {
    min_age: Option<Field<Text>>,
    max_age: Option<Field<Text>>,
    min_age_number: Option<Field<FiniteNumber>>,
    max_age_number: Option<Field<FiniteNumber>>,
    min_age_unit: Option<Field<Text>>,
    max_age_unit: Option<Field<Text>>,
    min_age_in_years: Option<Field<FiniteNumber>>,
    max_age_in_years: Option<Field<FiniteNumber>>,
    sex: Option<Text>,
    accepts_healthy_volunteers: Option<bool>,
}
impl Eligibility {
    pub(super) fn map(self, p: &mut Problems) -> NciEligibility {
        NciEligibility {
            structured: self.structured.map(|s| s.map(p)),
            unstructured: self.unstructured,
        }
    }
}
impl Structured {
    fn map(self, p: &mut Problems) -> NciEligibilityStructured {
        NciEligibilityStructured {
            min_age: p.field("eligibility.structured.min_age", self.min_age),
            max_age: p.field("eligibility.structured.max_age", self.max_age),
            min_age_number: age(p, "min", self.min_age_number, self.min_age_unit),
            max_age_number: age(p, "max", self.max_age_number, self.max_age_unit),
            min_age_in_years: years(p, "min", self.min_age_in_years),
            max_age_in_years: years(p, "max", self.max_age_in_years),
            sex: self.sex,
            accepts_healthy_volunteers: self.accepts_healthy_volunteers,
        }
    }
}
fn age(
    p: &mut Problems,
    bound: &str,
    number: Option<Field<FiniteNumber>>,
    unit: Option<Field<Text>>,
) -> Option<WithUnit<FiniteNumber>> {
    let supplied = unit.is_some();
    let unit = p.field(&format!("eligibility.structured.{bound}_age_unit"), unit);
    let value = p.field(
        &format!("eligibility.structured.{bound}_age_number"),
        number,
    )?;
    if supplied && unit.is_none() {
        return None;
    }
    Some(WithUnit { value, unit })
}
fn years(
    p: &mut Problems,
    bound: &str,
    number: Option<Field<FiniteNumber>>,
) -> Option<WithUnit<FiniteNumber>> {
    let value = p.field(
        &format!("eligibility.structured.{bound}_age_in_years"),
        number,
    )?;
    let unit = match "Years".parse() {
        Ok(unit) => unit,
        Err(_) => {
            p.required::<Text>("eligibility.structured.age_in_years_unit", None);
            return None;
        }
    };
    Some(WithUnit {
        value,
        unit: Some(unit),
    })
}
