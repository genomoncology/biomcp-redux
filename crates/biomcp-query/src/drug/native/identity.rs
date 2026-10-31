use biodata::{DrugIdentity, Error};
use biomcp_adapters::chembl;
use biomcp_types::{DrugCard, DrugHeader, DrugSection};
pub(super) fn chembl_header(
    row: &chembl::Molecule,
    problems: &mut Vec<biomcp_types::Problem>,
) -> Result<DrugHeader, Error> {
    let mapped = chembl::molecule_header(row)?;
    problems.extend(mapped.problems);
    Ok(mapped.value)
}
pub(super) fn neutral(header: &DrugHeader) -> Result<DrugIdentity, Error> {
    Ok(DrugIdentity {
        id: header.id.clone(),
        namespace: header.source.to_string().parse()?,
        name: header.name.clone(),
        identity_level: header.identity_level,
        r#type: header.r#type.clone(),
        url: header.url.clone(),
    })
}
pub(super) fn card(header: DrugHeader, fetched: Option<i64>) -> DrugCard {
    DrugCard {
        header,
        alternate_names: Box::default(),
        cross_references: Box::default(),
        parents: Box::default(),
        products: Box::default(),
        formulations: Box::default(),
        version: None,
        updated: None,
        retrieved: biomcp_adapters::drug_sources::received_date(fetched),
        available_sections: Box::new([DrugSection::Targets, DrugSection::Indications]),
        next: Box::default(),
    }
}
pub(super) fn chembl_card(
    row: chembl::Molecule,
    fetched: Option<i64>,
    dropped: &mut bool,
    problems: &mut Vec<biomcp_types::Problem>,
) -> Result<DrugCard, Error> {
    let mapped = chembl::molecule_card(row, fetched)?;
    *dropped |= mapped.refusals == biomcp_adapters::Refusal::Refused;
    problems.extend(mapped.problems);
    Ok(mapped.value)
}
