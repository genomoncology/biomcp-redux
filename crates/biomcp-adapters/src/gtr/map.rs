use super::{
    bad,
    raw::{Object, Relation},
};
use biodata::{
    Concept, GeneRef, GtrConditionRelation, GtrGeneRelation, GtrMethod, GtrMethodList,
    GtrTestRelation, GtrTestVersion, Text,
};
use biomcp_fetch::RequestError;
use biomcp_types::{DiagnosticLaboratory, DiagnosticSection, GtrCard, GtrHeader, GtrNativeStatus};
fn optional(value: &str) -> Option<Text> {
    Text::from_source(value).ok()
}
pub(super) fn header(v: &GtrTestVersion) -> Result<GtrHeader, RequestError> {
    // The publisher's web route uses the numeric test base; the typed ID keeps
    // the complete accession/version independently of this current listing link.
    let base = v
        .test_accession_ver
        .as_str()
        .strip_prefix("GTR")
        .and_then(|id| id.split_once('.'))
        .ok_or_else(bad)?
        .0;
    let base: u32 = base.parse().map_err(|_| bad())?;
    Ok(GtrHeader {
        id: v.test_accession_ver.clone(),
        name: optional(&v.lab_test_name),
        laboratory: DiagnosticLaboratory {
            id: Text::new(&v.lab_id).map_err(|_| bad())?,
            name: optional(&v.name_of_laboratory),
            institution: optional(&v.name_of_institution),
            country: optional(&v.facility_country),
        },
        url: Text::new(&format!(
            "https://www.ncbi.nlm.nih.gov/gtr/tests/classic/{base}/"
        ))
        .map_err(|_| bad())?,
    })
}
pub(super) fn card(v: &GtrTestVersion) -> Result<GtrCard, RequestError> {
    Ok(GtrCard {
        header: header(v)?,
        native_status: GtrNativeStatus {
            now_current: v.now_current.clone(),
            test_current: v.test_current.clone(),
            test_public: v.test_public.clone(),
            laboratory_current: v.laboratory_current.clone(),
            laboratory_public: v.laboratory_public.clone(),
        },
        last_touch_date: Text::new(&v.last_touch_date).map_err(|_| bad())?,
        test_create_date: Text::new(&v.test_create_date).map_err(|_| bad())?,
        test_deletion_date: v.test_deletion_date.clone(),
        manufacturer_test_name: v.manufacturer_test_name.clone(),
        test_development: v.test_development.clone(),
        lab_test_id: v.lab_test_id.clone(),
        test_order_code: v.test_order_code.clone(),
        clia_number: v.clia_number.clone(),
        state_licenses: v.state_licenses.clone(),
        state_license_numbers: v.state_license_numbers.clone(),
        sections: vec![
            DiagnosticSection::Genes,
            DiagnosticSection::Conditions,
            DiagnosticSection::Methods,
        ]
        .into(),
    })
}
pub(super) fn relation(r: &Relation) -> Result<GtrTestRelation, RequestError> {
    let medgen = format!("MedGen:{}", r.medgen).parse().map_err(|_| bad())?;
    let text = Text::from_source(&r.object_name).map_err(|_| bad())?;
    Ok(if r.object == Object::Gene {
        let gene_id = r.object_id.parse::<biodata::NcbiGeneId>().ok();
        GtrTestRelation::Gene(GtrGeneRelation {
            test_id: r.id.clone(),
            test_type: r.test_type,
            gene: GeneRef::new(
                Text::new(&r.gene_symbol).map_err(|_| bad())?,
                None,
                gene_id,
                None,
            )
            .map_err(|_| bad())?,
            native_gene_or_snomed_ct_id: (!r.object_id.is_empty() && gene_id.is_none())
                .then(|| r.object_id.clone()),
            native_medgen: medgen,
            mim_number: r.mim.clone(),
            object_name: r.object_name.clone(),
        })
    } else {
        GtrTestRelation::Condition(GtrConditionRelation {
            test_id: r.id.clone(),
            test_type: r.test_type,
            condition: Concept {
                id: Some(medgen),
                label: text,
            },
            mim_number: r.mim.clone(),
            snomed_ct_id: r.object_id.clone(),
        })
    })
}
pub(super) fn methods(v: &GtrTestVersion) -> Vec<GtrMethod> {
    [
        (GtrMethodList::Categories, &v.method_categories),
        (GtrMethodList::Methods, &v.methods),
        (GtrMethodList::Platforms, &v.platforms),
    ]
    .into_iter()
    .flat_map(|(list, cell)| {
        cell.split('|')
            .enumerate()
            .filter(|(_, value)| !value.is_empty())
            .map(move |(index, value)| GtrMethod {
                test_id: v.test_accession_ver.clone(),
                list,
                index: index as u32,
                value: value.into(),
            })
    })
    .collect()
}
