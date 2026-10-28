//! Normalize optional source text before card and page construction.
use super::raw::{Component, ReceivedComponent, Hierarchy, Synonym};
use crate::{mapped::Problems, raw::{Field, OneOrMany}};
fn field(value: Option<Field<String>>, field: &str, problems: &mut Problems) -> Option<String> {
    crate::mapped::field_text(&mut problems.list, problems.source, problems.record.clone(), field, value).map(|v| v.to_string())
}
fn many(value: Option<OneOrMany<Field<String>>>, field: &str, problems: &mut Problems) -> Option<OneOrMany<String>> {
    value.map(|values| OneOrMany::Many(values.values().into_iter().filter_map(|v| field(Some(v), field, problems)).collect()))
}
pub(super) fn component(raw: ReceivedComponent, name: &str, problems: &mut Problems) -> Component {
    Component {
        xrefs: raw.xrefs.map(|values| values.into_iter().map(|(field, values)| { let values = many(Some(values), &format!("{name}.xrefs.{field}"), problems).unwrap_or_else(|| OneOrMany::Many(Vec::new())); (field, values) }).collect()),
        id: field(raw.id, &format!("{name}.id"), problems),
        name: field(raw.name, &format!("{name}.name"), problems),
        molecule_chembl_id: field(raw.molecule_chembl_id, &format!("{name}.molecule_chembl_id"), problems),
        pref_name: field(raw.pref_name, &format!("{name}.pref_name"), problems),
        molecule_type: field(raw.molecule_type, &format!("{name}.molecule_type"), problems),
        ligand_id: field(raw.ligand_id, &format!("{name}.ligand_id"), problems),
        r#type: field(raw.r#type, &format!("{name}.type"), problems),
        unii: field(raw.unii, &format!("{name}.unii"), problems),
        display_name: field(raw.display_name, &format!("{name}.display_name"), problems),
        ingredient_type: field(raw.ingredient_type, &format!("{name}.ingredient_type"), problems),
        product_id: field(raw.product_id, &format!("{name}.product_id"), problems),
        productndc: field(raw.productndc, &format!("{name}.productndc"), problems),
        producttypename: field(raw.producttypename, &format!("{name}.producttypename"), problems),
        nonproprietaryname: field(raw.nonproprietaryname, &format!("{name}.nonproprietaryname"), problems),
        proprietaryname: field(raw.proprietaryname, &format!("{name}.proprietaryname"), problems),
        synonyms: many(raw.synonyms, &format!("{name}.synonyms"), problems),
        molecule_synonyms: raw.molecule_synonyms.map(|values| values.into_iter().map(|v| Synonym { molecule_synonym: field(v.molecule_synonym, &format!("{name}.molecule_synonyms.molecule_synonym"), problems) }).collect()),
        molecule_hierarchy: raw.molecule_hierarchy.map(|v| Hierarchy { active_chembl_id: field(v.active_chembl_id, &format!("{name}.molecule_hierarchy.active_chembl_id"), problems), parent_chembl_id: field(v.parent_chembl_id, &format!("{name}.molecule_hierarchy.parent_chembl_id"), problems) }),
        chembl_id: field(raw.chembl_id, &format!("{name}.chembl_id"), problems),
        rxcui: field(raw.rxcui, &format!("{name}.rxcui"), problems),
        ncit: field(raw.ncit, &format!("{name}.ncit"), problems),
        applicationnumber: field(raw.applicationnumber, &format!("{name}.applicationnumber"), problems),
        generic_name: many(raw.generic_name, &format!("{name}.generic_name"), problems),
        brand_name: many(raw.brand_name, &format!("{name}.brand_name"), problems),
        spl_set_id: many(raw.spl_set_id, &format!("{name}.spl_set_id"), problems),
        application_number: many(raw.application_number, &format!("{name}.application_number"), problems),
    }
}
