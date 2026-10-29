use biodata::{Attribution, DrugIdentity, DrugIdentityLevel, DrugIdentityRelation, Error};
use biomcp_adapters::chembl;
use biomcp_types::{DrugCard, DrugHeader, DrugSection, DrugSource};
pub(super) fn chembl_header(row: &chembl::Molecule) -> Result<DrugHeader, Error> {
    Ok(DrugHeader {
        matched_name: None,
        id: row.molecule_chembl_id.parse()?,
        source: DrugSource::Chembl,
        name: row.pref_name.as_deref().map(str::parse).transpose()?,
        version: None,
        identity_level: DrugIdentityLevel::Unknown,
        r#type: Some(row.molecule_type.parse()?),
        url: Some(
            format!(
                "https://www.ebi.ac.uk/chembl/explore/compound/{}",
                row.molecule_chembl_id
            )
            .parse()?,
        ),
    })
}
pub(super) fn neutral(header: &DrugHeader) -> Result<DrugIdentity, Error> {
    Ok(DrugIdentity {
        id: header.id.clone(),
        source: header.source.to_string().parse()?,
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
pub(super) fn chembl_card(row: chembl::Molecule, fetched: Option<i64>) -> Result<DrugCard, Error> {
    let mut card = card(chembl_header(&row)?, fetched);
    let mut names = Vec::new();
    for synonym in row.molecule_synonyms {
        if !names.contains(&synonym.molecule_synonym) {
            names.push(synonym.molecule_synonym);
        }
    }
    card.alternate_names = names
        .into_iter()
        .map(|name| name.parse())
        .collect::<Result<_, _>>()?;
    let attribution = Attribution {
        source: "chembl".parse()?,
        record: Some(row.molecule_chembl_id.parse()?),
        version: None,
        release: None,
        submitter: None,
        evaluated: None,
        url: card.header.url.clone(),
    };
    card.cross_references = row
        .cross_references
        .into_iter()
        .map(|r| {
            Ok(DrugIdentityRelation {
                identity: DrugIdentity {
                    id: r.xref_id.parse()?,
                    source: r.xref_src.parse()?,
                    name: r.xref_name.map(|s| s.parse()).transpose()?,
                    identity_level: DrugIdentityLevel::Unknown,
                    r#type: None,
                    url: None,
                },
                relation: "cross_references".parse()?,
                attribution: attribution.clone(),
            })
        })
        .collect::<Result<_, Error>>()?;
    if !card.cross_references.is_empty() {
        card.available_sections = vec![
            DrugSection::Targets,
            DrugSection::Indications,
            DrugSection::Xrefs,
        ]
        .into();
    }
    if let Some(parent) = row.molecule_hierarchy {
        let mut identity = neutral(&card.header)?;
        identity.id = parent.parent_chembl_id.parse()?;
        identity.name = None;
        identity.url = None;
        card.parents = Box::new([DrugIdentityRelation {
            identity,
            relation: "molecule_hierarchy.parent_chembl_id".parse()?,
            attribution,
        }]);
    }
    Ok(card)
}
