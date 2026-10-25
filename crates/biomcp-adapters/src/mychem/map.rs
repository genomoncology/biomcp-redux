use super::raw::{Component, Hit, Page};
use biodata::{Attribution, DrugIdentity, DrugIdentityLevel, DrugIdentityRelation, Text};
use biomcp_fetch::{BadReply, Leg};
use biomcp_types::{DrugCard, DrugHeader, DrugPage, DrugSource};
fn text(value: &str) -> Result<Text, BadReply> { Text::new(value).map_err(|_| BadReply) }
fn components(hit: Hit, leg: &Leg<'_>) -> (String, Option<u64>, Vec<(&'static str, Component)>) {
    let mut rows = Vec::new();
    for (name, values) in [("drugbank",hit.drugbank), ("chembl",hit.chembl), ("drugcentral",hit.drugcentral), ("gtopdb",hit.gtopdb), ("unii",hit.unii), ("ndc",hit.ndc), ("openfda",hit.openfda)] {
        if crate::drug_sources::components::COMPONENTS.iter().find(|c| c.name == name).is_some_and(|c| !c.disabled_on_leg(leg)) {
            rows.extend(values.into_iter().flat_map(|v|v.values()).map(|v|(name,v)));
        }
    }
    (hit.id, hit.version, rows)
}
fn name(c: &Component) -> Option<&str> {
    c.name.as_deref().or(c.pref_name.as_deref()).or(c.display_name.as_deref()).or(c.proprietaryname.as_deref()).or(c.nonproprietaryname.as_deref())
}
fn header(id: &str, rows: &[(&str, Component)]) -> Result<DrugHeader, BadReply> {
    Ok(DrugHeader {
        id: id.parse().map_err(|_|BadReply)?, source: DrugSource::Mychem,
        name: rows.iter().find_map(|(_,c)|name(c)).map(text).transpose()?,
        identity_level: DrugIdentityLevel::Unknown,
        r#type: rows.iter().find_map(|(_,c)|c.molecule_type.as_deref().or(c.r#type.as_deref()).or(c.ingredient_type.as_deref()).or(c.producttypename.as_deref())).map(text).transpose()?,
        url: Some(text(&format!("https://mychem.info/v1/chem/{id}"))?),
    })
}
pub(super) fn page(page: Page, leg: &Leg<'_>) -> Result<DrugPage, BadReply> {
    let items = page.hits.into_iter().map(|hit| { let (id,_,rows)=components(hit,leg); header(&id,&rows) }).collect::<Result<Vec<_>,_>>()?;
    Ok(DrugPage { items: items.into(), total: Some(page.total) })
}
pub(super) fn card(hit: Hit, leg: &Leg<'_>) -> Result<DrugCard, BadReply> {
    let (id,version,rows)=components(hit,leg);
    let mut card = DrugCard {
        header: header(&id,&rows)?, alternate_names: Box::default(), cross_references: Box::default(), parents: Box::default(), products: Box::default(), formulations: Box::default(), version: version.map(|n|text(&n.to_string())).transpose()?, updated: None, retrieved: None, available_sections: Box::default(), next: Box::default(),
    };
    let mut aliases=Vec::new(); let mut refs=Vec::new(); let mut parents=Vec::new();
    for (component,c) in rows {
        aliases.extend(c.synonyms.as_ref().map(strings).unwrap_or_default().into_iter().map(str::to_owned));
        aliases.extend(c.molecule_synonyms.as_ref().into_iter().flatten().filter_map(|s|s.molecule_synonym.clone()));
        for values in [&c.generic_name,&c.brand_name] { aliases.extend(values.as_ref().map(strings).unwrap_or_default().into_iter().map(str::to_owned)); }
        for (relation, target) in [("drugbank.id",c.id.as_deref().filter(|_|component=="drugbank")), ("chembl.molecule_chembl_id",c.molecule_chembl_id.as_deref()), ("drugcentral.id",c.id.as_deref().filter(|_|component=="drugcentral")), ("gtopdb.ligand_id",c.ligand_id.as_deref()), ("gtopdb.chembl_id",c.chembl_id.as_deref()), ("unii.unii",c.unii.as_deref()), ("unii.rxcui",c.rxcui.as_deref()), ("unii.ncit",c.ncit.as_deref()), ("ndc.product_id",c.product_id.as_deref()), ("ndc.productndc",c.productndc.as_deref()), ("ndc.applicationnumber",c.applicationnumber.as_deref())] {
            if let Some(target)=target { refs.push(reference(&id,component,relation,target,name(&c))?); }
        }
        for (relation,values) in [("openfda.spl_set_id",&c.spl_set_id),("openfda.application_number",&c.application_number)] {
            for target in values.as_ref().map(strings).unwrap_or_default() { refs.push(reference(&id,component,relation,target,name(&c))?); }
        }
        if let Some(h)=&c.molecule_hierarchy {
            for (relation,target) in [("chembl.molecule_hierarchy.parent_chembl_id",&h.parent_chembl_id),("chembl.molecule_hierarchy.active_chembl_id",&h.active_chembl_id)] {
                if let Some(target)=target { parents.push(reference(&id,component,relation,target,None)?); }
            }
        }
    }
    let mut names=Vec::new();
    for alias in aliases { let alias=text(&alias)?; if !names.contains(&alias) { names.push(alias); } }
    card.alternate_names=names.into(); card.cross_references=refs.into(); card.parents=parents.into(); Ok(card)
}
fn strings(values: &crate::raw::OneOrMany<String>) -> Vec<&str> {
    match values { crate::raw::OneOrMany::One(v)=>vec![v], crate::raw::OneOrMany::Many(v)=>v.iter().map(String::as_str).collect() }
}
fn reference(record: &str, component: &str, relation: &str, target: &str, name: Option<&str>) -> Result<DrugIdentityRelation,BadReply> {
    let source=relation.split('.').next().unwrap_or(component);
    let target_source=match relation { "unii.rxcui"=>"rxnorm", "unii.ncit"=>"ncit", "ndc.applicationnumber"|"openfda.application_number"=>"drugsfda", "openfda.spl_set_id"=>"openfda-label", "gtopdb.chembl_id"=>"chembl", _=>source };
    Ok(DrugIdentityRelation {
        identity: DrugIdentity { id:target.parse().map_err(|_|BadReply)?,source:text(target_source)?,name:name.map(text).transpose()?,identity_level:DrugIdentityLevel::Unknown,r#type:None,url:None },
        relation:text(relation)?,
        attribution:Attribution { source:text(component)?,record:Some(text(record)?),version:None,release:None,submitter:None,evaluated:None,url:Some(text(&format!("https://mychem.info/v1/chem/{record}"))?) },
    })
}
