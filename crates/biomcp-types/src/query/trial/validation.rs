use super::{Fields, TrialSearch, TrialSource};
use crate::{NctId, QueryInvalid};

impl TryFrom<Fields> for TrialSearch {
    type Error = QueryInvalid;
    fn try_from(mut f: Fields) -> Result<Self, Self::Error> {
        if let Some(id) = f
            .query
            .as_ref()
            .and_then(|q| q.as_str().parse::<NctId>().ok())
        {
            if f.id.is_some_and(|explicit| explicit != id) {
                return Err(QueryInvalid::new(&["query", "id"]));
            }
            f.id = Some(id);
            f.query = None;
        }
        if let Some(id) = &f.condition_id {
            if !f.condition.is_empty() {
                return Err(QueryInvalid::new(&["condition_id", "condition"]));
            }
            if !f.keyword.is_empty() {
                return Err(QueryInvalid::new(&["condition_id", "keyword"]));
            }
            if !valid_condition_id(id, f.source) {
                return Err(QueryInvalid::new(&["condition_id", "source"]));
            }
        }
        f.phase = f.phase.map(|phase| phase.for_source(f.source));
        validate_source_filters(&f)?;
        validate_biomarker(&f)?;
        validate_alternatives(&f)?;
        super::country::validate(&f.country)?;
        validate_exact_condition(&f)?;
        super::continuation::validate(&f)?;
        if !(1..=100).contains(&f.limit) {
            return Err(QueryInvalid::new(&["limit"]));
        }
        if f.age.is_some_and(|v| v > 150) {
            return Err(QueryInvalid::new(&["age"]));
        }
        if f.updated_since
            .zip(f.updated_until)
            .is_some_and(|(a, b)| a.to_string() > b.to_string())
        {
            return Err(QueryInvalid::new(&["updated_since", "updated_until"]));
        }
        validate_location(&f)?;
        if f.sponsor_role.is_some() && f.sponsor.is_none() {
            // Name the missing --sponsor, not the role, as the argument to fix.
            return Err(QueryInvalid::new(&["sponsor", "sponsor_role"]));
        }
        validate_counts(&f)?;
        Ok(Self(Box::new(f)))
    }
}

/// A site count reads every page itself, so it takes no page token and returns no total alone.
fn validate_counts(f: &Fields) -> Result<(), QueryInvalid> {
    if f.site_count && f.count_only {
        return Err(QueryInvalid::new(&["site_count", "count_only"]));
    }
    if f.site_count && f.next_page.is_some() {
        return Err(QueryInvalid::new(&["site_count", "next_page"]));
    }
    if f.arms_for.is_some() && (f.site_count || f.count_only) {
        let count = if f.site_count {
            "site_count"
        } else {
            "count_only"
        };
        return Err(QueryInvalid::new(&["arms_for", count]));
    }
    if f.why_stopped
        .as_ref()
        .is_some_and(|text| super::words(text.as_str()).is_empty())
    {
        return Err(QueryInvalid::new(&["why_stopped"]));
    }
    Ok(())
}

fn validate_exact_condition(f: &Fields) -> Result<(), QueryInvalid> {
    if f.exact_condition && f.source != TrialSource::Ctgov {
        return Err(QueryInvalid::new(&["exact_condition", "source"]));
    }
    if f.exact_condition && f.condition.is_empty() {
        return Err(QueryInvalid::new(&["exact_condition", "condition"]));
    }
    Ok(())
}

fn valid_condition_id(id: &biodata::Curie, source: TrialSource) -> bool {
    let value = id.to_string();
    let Some((namespace, local)) = value.split_once(':') else {
        return false;
    };
    match namespace {
        "MONDO" | "DOID" => !local.is_empty() && local.bytes().all(|v| v.is_ascii_digit()),
        "NCIT" => {
            source == TrialSource::Nci
                && local
                    .strip_prefix('C')
                    .is_some_and(|v| !v.is_empty() && v.bytes().all(|v| v.is_ascii_digit()))
        }
        _ => false,
    }
}

fn validate_alternatives(f: &Fields) -> Result<(), QueryInvalid> {
    for (name, values) in [
        ("condition", &f.condition),
        ("intervention", &f.intervention),
        ("keyword", &f.keyword),
    ] {
        if values.len() > super::MAX_ALTERNATIVES {
            return Err(QueryInvalid::new(&[name]));
        }
    }
    Ok(())
}

fn validate_biomarker(f: &Fields) -> Result<(), QueryInvalid> {
    if (f.biomarker_code.is_some() || f.biomarker_use.is_some()) && f.source != TrialSource::Nci {
        return Err(QueryInvalid::new(&[
            "biomarker_code",
            "biomarker_use",
            "source",
        ]));
    }
    if f.biomarker_use.is_some() && f.biomarker_code.is_none() {
        return Err(QueryInvalid::new(&["biomarker_code", "biomarker_use"]));
    }
    Ok(())
}

fn validate_source_filters(f: &Fields) -> Result<(), QueryInvalid> {
    if f.status.as_ref().is_some_and(|v| !v.valid_for(f.source)) {
        return Err(QueryInvalid::new(&["status", "source"]));
    }
    if f.phase.as_ref().is_some_and(|v| !v.valid_for(f.source)) {
        return Err(QueryInvalid::new(&["phase", "source"]));
    }
    Ok(())
}

fn validate_location(f: &Fields) -> Result<(), QueryInvalid> {
    if f.facility.is_some() && (f.lat.is_some() || f.lon.is_some() || f.distance.is_some()) {
        return Err(QueryInvalid::new(&["facility", "lat", "lon", "distance"])
            .with_message(crate::ErrorMessage::TrialFacilityRadius));
    }
    if [f.lat.is_some(), f.lon.is_some(), f.distance.is_some()]
        .iter()
        .filter(|v| **v)
        .count()
        % 3
        != 0
    {
        return Err(QueryInvalid::new(&["lat", "lon", "distance"]));
    }
    if let (Some(lat), Some(lon), Some(distance)) = (&f.lat, &f.lon, &f.distance) {
        super::point(lat, lon)?;
        let distance = distance
            .as_str()
            .strip_suffix("mi")
            .or_else(|| distance.as_str().strip_suffix("km"))
            .ok_or_else(|| QueryInvalid::new(&["distance"]))?
            .parse::<f64>()
            .map_err(|_| QueryInvalid::new(&["distance"]))?;
        if !distance.is_finite() {
            return Err(QueryInvalid::new(&["distance"]));
        }
        if distance <= 0.0 {
            return Err(QueryInvalid::new(&["distance"]));
        }
    }
    Ok(())
}
