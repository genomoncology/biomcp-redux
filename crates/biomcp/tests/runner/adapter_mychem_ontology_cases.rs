//! Source field edge-case tables for the MyChem and ontology command cases.
use serde_json::{Value, json};
type Case = (&'static str, Value, &'static str);
pub(super) fn cases(family: &str) -> Vec<Case> {
    match family {
        "mychem" => mychem(),
        "ols" => ols(),
        "mydisease-doid" => doid(),
        _ => mondo(),
    }
}
fn mychem_scalars() -> Vec<Case> {
    [
        "drugbank.name",
        "drugbank.id",
        "drugcentral.id",
        "chembl.molecule_chembl_id",
        "chembl.pref_name",
        "chembl.molecule_type",
        "gtopdb.ligand_id",
        "gtopdb.name",
        "gtopdb.type",
        "gtopdb.chembl_id",
        "unii.unii",
        "unii.display_name",
        "unii.ingredient_type",
        "unii.rxcui",
        "unii.ncit",
        "ndc.product_id",
        "ndc.productndc",
        "ndc.producttypename",
        "ndc.nonproprietaryname",
        "ndc.proprietaryname",
        "ndc.applicationnumber",
        "chembl.molecule_hierarchy.parent_chembl_id",
        "chembl.molecule_hierarchy.active_chembl_id",
    ]
    .into_iter()
    .map(|field| (field, json!(""), field))
    .collect()
}
fn mychem() -> Vec<Case> {
    let mut cases = mychem_scalars();
    cases.extend([
        ("drugbank.name", json!(42), "drugbank.name"),
        (
            "drugbank.synonyms",
            json!(["valid", "", ""]),
            "drugbank.synonyms",
        ),
        (
            "drugcentral.synonyms",
            json!(["valid", "", ""]),
            "drugcentral.synonyms",
        ),
        (
            "gtopdb.synonyms",
            json!(["valid", "", ""]),
            "gtopdb.synonyms",
        ),
        (
            "chembl.molecule_synonyms",
            json!([{"molecule_synonym":""}]),
            "chembl.molecule_synonyms.molecule_synonym",
        ),
        (
            "drugcentral.xrefs.chebi",
            json!([""]),
            "drugcentral.xrefs.chebi",
        ),
        (
            "gtopdb.xrefs.pubchem",
            json!(["invalid:"]),
            "gtopdb.xrefs.pubchem",
        ),
        ("drugbank.id", json!("invalid:"), "drugbank.id"),
        (
            "drugcentral.xrefs.pdb_chem_id",
            json!(" STI"),
            "drugcentral.xrefs.pdb_chem_id",
        ),
        (
            "openfda.generic_name",
            json!(["valid", "", ""]),
            "openfda.generic_name",
        ),
        (
            "openfda.brand_name",
            json!(["valid", "", ""]),
            "openfda.brand_name",
        ),
        ("openfda.spl_set_id", json!([""]), "openfda.spl_set_id"),
        (
            "openfda.application_number",
            json!([""]),
            "openfda.application_number",
        ),
    ]);
    cases
}
fn ols() -> Vec<Case> {
    vec![
        ("label", json!(""), "label"),
        ("iri", json!(""), "iri"),
        ("description", json!(["valid", "", ""]), "description"),
        ("description", json!(42), "description"),
        ("synonyms", json!(["valid", "", ""]), "synonyms"),
        ("term_replaced_by", json!(["invalid:"]), "term_replaced_by"),
        (
            "term_replaced_by",
            json!([" MONDO:0005105"]),
            "term_replaced_by",
        ),
        (
            "annotation",
            json!({"consider":["invalid:"]}),
            "annotation.consider",
        ),
        (
            "obo_synonym",
            json!([{"name":"valid", "scope":""}]),
            "synonym.scope",
        ),
        ("obo_synonym", json!([{"name":""}]), "synonym.name"),
        ("obo_xref", json!([{"database":"", "id":"12"}]), "xref.id"),
        ("obo_xref", json!([{"database":"DOID", "id":""}]), "xref.id"),
        (
            "obo_xref",
            json!([{"database":" DOID", "id":"12"}]),
            "xref.id",
        ),
        (
            "obo_synonym",
            json!([{"name":"valid", "scope":" EXACT"}]),
            "synonym.scope",
        ),
        (
            "obo_xref",
            json!([{"database":"DOID", "id":"12", "description":""}]),
            "xref.description",
        ),
    ]
}
fn doid() -> Vec<Case> {
    vec![
        ("name", json!(""), "disease_ontology.name"),
        ("def", json!(""), "disease_ontology.def"),
        (
            "synonyms",
            json!({"exact":["valid", ""]}),
            "disease_ontology.synonyms.exact",
        ),
        (
            "xrefs",
            json!({"mondo":[""]}),
            "disease_ontology.xrefs.mondo",
        ),
        ("parents", json!(["invalid:"]), "disease_ontology.parents"),
        ("consider", json!(["invalid:"]), "disease_ontology.consider"),
        (
            "replaced_by",
            json!(["invalid:"]),
            "disease_ontology.replaced_by",
        ),
    ]
}
fn mondo() -> Vec<Case> {
    vec![
        ("label", json!(""), "mondo.label"),
        ("definition", json!(""), "mondo.definition"),
        ("definition", json!(42), "mondo.definition"),
        (
            "synonym",
            json!({"exact":["valid", ""]}),
            "mondo.synonym.exact",
        ),
        (
            "synonym",
            json!({"bad scope":["valid"]}),
            "mondo.synonym.bad scope",
        ),
        ("xrefs", json!({"doid":[""]}), "mondo.xrefs.doid"),
        ("parents", json!(["invalid:"]), "mondo.parents"),
        ("consider", json!(["invalid:"]), "mondo.consider"),
        ("replaced_by", json!(["invalid:"]), "mondo.replaced_by"),
    ]
}
