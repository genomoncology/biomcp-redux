use super::raw::{Object, Relation, XmlLab, XmlTest};
use biodata::{
    GtrConditionReference, GtrConditionRelation, GtrGeneReference, GtrGeneRelation, GtrMethod,
    GtrMethodList, GtrTestRelation, GtrTestVersion, Text,
};
use biomcp_fetch::RequestError;
use biomcp_types::{DiagnosticLaboratory, DiagnosticSection, GtrCard, GtrHeader, GtrNativeStatus};
fn optional(
    value: &str,
    field: &str,
    v: &GtrTestVersion,
    problems: &mut Vec<biomcp_types::Problem>,
) -> Option<Text> {
    // The retained GTR report defines empty native cells as absent (diagnostic slice field review).
    if value.is_empty() {
        return None;
    }
    crate::mapped::field_text(
        problems,
        biomcp_types::Source::Gtr,
        crate::mapped::record(&v.test_accession_ver),
        field,
        Some(crate::raw::Field::Value(value.to_owned())),
    )
}
pub(super) fn header(
    v: &GtrTestVersion,
    xml: Option<&XmlTest>,
    problems: &mut Vec<biomcp_types::Problem>,
) -> Result<GtrHeader, RequestError> {
    // The publisher's web route uses the numeric test base; the typed ID keeps
    // the complete accession/version independently of this current listing link.
    let base = v
        .test_accession_ver
        .as_str()
        .strip_prefix("GTR")
        .and_then(|id| id.split_once('.'))
        .ok_or_else(|| biomcp_fetch::bad_reply(biomcp_types::Source::Gtr, None))?
        .0;
    let base: u32 = base
        .parse()
        .map_err(|_| biomcp_fetch::bad_reply(biomcp_types::Source::Gtr, None))?;
    Ok(GtrHeader {
        id: v.test_accession_ver.clone(),
        name: optional(&v.lab_test_name, "lab_test_name", v, problems),
        laboratory: crate::mapped::field_code_text(
            problems,
            biomcp_types::Source::Gtr,
            crate::mapped::record(&v.test_accession_ver),
            "lab_id",
            (!v.lab_id.is_empty()).then(|| crate::raw::Field::Value(v.lab_id.to_string())),
        )
        .map(|id| DiagnosticLaboratory {
            id,
            name: optional(&v.name_of_laboratory, "name_of_laboratory", v, problems),
            institution: optional(&v.name_of_institution, "name_of_institution", v, problems),
            state: optional(&v.facility_state, "facility_state", v, problems),
            postcode: optional(&v.facility_postcode, "facility_postcode", v, problems),
            country: optional(&v.facility_country, "facility_country", v, problems),
        }),
        gene_count: gene_count(&v.genes),
        method_categories: v
            .method_categories
            .split('|')
            .filter_map(|value| optional(value, "method_categories", v, problems))
            .collect(),
        specimens: xml
            .filter(|x| x.id == v.test_accession_ver)
            .into_iter()
            .flat_map(|x| x.specimens.iter())
            .filter_map(|value| optional(value, "SpecimenType", v, problems))
            .collect(),
        url: Text::new(&format!(
            "https://www.ncbi.nlm.nih.gov/gtr/tests/classic/{base}/"
        ))
        .map_err(|_| biomcp_fetch::bad_reply(biomcp_types::Source::Gtr, None))?,
    })
}
/// Distinct symbols in the pipe-separated native genes cell; absent when the cell is empty.
fn gene_count(cell: &str) -> Option<u64> {
    let genes: std::collections::HashSet<_> = cell.split('|').filter(|g| !g.is_empty()).collect();
    (!genes.is_empty()).then_some(genes.len() as u64)
}
pub(super) fn card(
    v: &GtrTestVersion,
    xml: Option<&XmlTest>,
    lab: Option<&XmlLab>,
    problems: &mut Vec<biomcp_types::Problem>,
) -> Result<GtrCard, RequestError> {
    let header = header(v, xml, problems)?;
    let lab = lab.filter(|lab| *lab.id == *v.lab_id);
    let test = xml.filter(|x| x.id == v.test_accession_ver);
    let laboratory_certifications: Box<[_]> = lab
        .into_iter()
        .flat_map(|lab| lab.certifications.iter())
        .filter_map(|c| {
            Some(biomcp_types::GtrLabCertification {
                name: optional(&c.name, "LabCertification/Name", v, problems)?,
                number: c
                    .number
                    .as_deref()
                    .and_then(|n| optional(n, "LabCertification/Number", v, problems)),
                expiration_date: c
                    .expiration_date
                    .as_deref()
                    .and_then(|d| optional(d, "LabCertification/ExpirationDate", v, problems)),
            })
        })
        .collect();
    let laboratory_address = lab
        .and_then(|lab| lab.address.as_ref())
        .map(|a| biomcp_types::GtrLabAddress {
            lines: a
                .lines
                .iter()
                .filter_map(|l| optional(l, "MailingAddress/Line", v, problems))
                .collect(),
            city: a
                .city
                .as_deref()
                .and_then(|x| optional(x, "MailingAddress/City", v, problems)),
            state: a
                .state
                .as_deref()
                .and_then(|x| optional(x, "MailingAddress/State", v, problems)),
            postcode: a
                .postcode
                .as_deref()
                .and_then(|x| optional(x, "MailingAddress/PostCode", v, problems)),
            country: a
                .country
                .as_deref()
                .and_then(|x| optional(x, "MailingAddress/Country", v, problems)),
        })
        .filter(|a| {
            !a.lines.is_empty()
                || a.city.is_some()
                || a.state.is_some()
                || a.postcode.is_some()
                || a.country.is_some()
        });
    let laboratory_directors: Box<[_]> = lab
        .into_iter()
        .flat_map(|lab| lab.directors.iter())
        .filter_map(|d| optional(d, "Personnel/DisplayName", v, problems))
        .collect();
    let order_url = test
        .and_then(|t| t.order_url.as_deref())
        .and_then(|u| optional(u, "Order/URL", v, problems));
    let test_url = test
        .and_then(|t| t.test_url.as_deref())
        .and_then(|u| optional(u, "TestURL", v, problems));
    let xml_release_url = if header.specimens.is_empty()
        && laboratory_certifications.is_empty()
        && laboratory_address.is_none()
        && laboratory_directors.is_empty()
        && order_url.is_none()
        && test_url.is_none()
    {
        None
    } else {
        Some(
            Text::new(&format!("{}{}", super::SPEC.base, super::bundle::XML_PATH))
                .map_err(|_| biomcp_fetch::bad_reply(biomcp_types::Source::Gtr, None))?,
        )
    };
    Ok(GtrCard {
        header,
        native_status: GtrNativeStatus {
            now_current: v.now_current.clone(),
            test_current: v.test_current.clone(),
            test_public: v.test_public.clone(),
            laboratory_current: v.laboratory_current.clone(),
            laboratory_public: v.laboratory_public.clone(),
        },
        last_touch_date: optional(&v.last_touch_date, "last_touch_date", v, problems),
        test_create_date: optional(&v.test_create_date, "test_create_date", v, problems),
        test_deletion_date: v.test_deletion_date.clone(),
        manufacturer_test_name: v.manufacturer_test_name.clone(),
        test_development: v.test_development.clone(),
        lab_test_id: v.lab_test_id.clone(),
        test_order_code: v.test_order_code.clone(),
        clia_number: v.clia_number.clone(),
        state_licenses: v.state_licenses.clone(),
        state_license_numbers: v.state_license_numbers.clone(),
        laboratory_certifications,
        laboratory_address,
        laboratory_directors,
        order_url,
        test_url,
        xml_release_url,
        sections: vec![
            DiagnosticSection::Genes,
            DiagnosticSection::Conditions,
            DiagnosticSection::Methods,
        ]
        .into(),
    })
}
pub(super) fn relation(
    r: &Relation,
    cx: &mut crate::mapped::Problems,
    refusals: &mut crate::Refusal,
) -> Option<GtrTestRelation> {
    cx.record = crate::mapped::record(&r.id);
    let test_type = cx.optional(
        "test_type",
        r.test_type
            .as_ref()
            .map(|value| value.as_ref().copied().ok_or(())),
    );
    let medgen = cx.optional(
        "GTR_identifier",
        (!r.medgen.is_empty()).then(|| format!("MedGen:{}", r.medgen).parse()),
    );
    if r.object == Object::Gene {
        // A non-GeneID cell such as native -1 remains literal context, not a converted identifier.
        let gene_id = r.object_id.parse::<biodata::NcbiGeneId>().ok();
        let label = cx.checked_text(
            "gene_symbol",
            (!r.gene_symbol.is_empty()).then_some(&r.gene_symbol),
        );
        if gene_id.is_none() && label.is_none() {
            *refusals = crate::Refusal::Refused;
            cx.required::<()>("gene_symbol", None);
            return None;
        }
        Some(GtrTestRelation::Gene(GtrGeneRelation {
            test_id: r.id.clone(),
            test_type,
            gene: GtrGeneReference {
                label,
                ncbi_gene_id: gene_id,
            },
            native_gene_or_snomed_ct_id: (!r.object_id.is_empty() && gene_id.is_none())
                .then(|| r.object_id.clone()),
            native_medgen: medgen,
            mim_number: r.mim.clone(),
            object_name: r.object_name.clone(),
        }))
    } else {
        let Some(id) = cx.required("GTR_identifier", medgen) else {
            *refusals = crate::Refusal::Refused;
            return None;
        };
        let label = cx.text(
            "object_name",
            (!r.object_name.is_empty()).then_some(&r.object_name),
        );
        Some(GtrTestRelation::Condition(GtrConditionRelation {
            test_id: r.id.clone(),
            test_type,
            condition: GtrConditionReference { id, label },
            mim_number: r.mim.clone(),
            snomed_ct_id: r.object_id.clone(),
        }))
    }
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
