use super::raw;
use biodata::{
    Attribution, PhenotypeHierarchyLink, PhenotypeHierarchyRelation, PhenotypeSynonym,
    PhenotypeSynonymClass, PhenotypeXref,
};
type Rows = (
    Option<Box<[PhenotypeSynonym]>>,
    Option<Box<[PhenotypeXref]>>,
    Option<Box<[PhenotypeHierarchyLink]>>,
);
pub(super) fn rows(node: &raw::Node, attribution: &Attribution) -> Rows {
    let mut synonyms = Vec::new();
    let mut available = false;
    for (rows, relationship) in [
        (&node.synonym, None),
        (&node.exact_synonym, Some(PhenotypeSynonymClass::Exact)),
        (&node.broad_synonym, Some(PhenotypeSynonymClass::Broad)),
        (&node.narrow_synonym, Some(PhenotypeSynonymClass::Narrow)),
        (&node.related_synonym, Some(PhenotypeSynonymClass::Related)),
    ] {
        if let Some(rows) = rows {
            available = true;
            synonyms.extend(rows.iter().map(|value| PhenotypeSynonym {
                value: value.clone(),
                relationship,
                attribution: attribution.clone(),
            }));
        }
    }
    let xrefs = node.xref.as_ref().map(|rows| {
        rows.iter()
            .map(|id| PhenotypeXref {
                id: id.clone(),
                predicate: None,
                attribution: attribution.clone(),
            })
            .collect()
    });
    let hierarchy = node.node_hierarchy.as_ref().and_then(|hierarchy| {
        if hierarchy.super_classes.is_none() && hierarchy.sub_classes.is_none() {
            return None;
        }
        let mut links = Vec::new();
        for (rows, relation) in [
            (&hierarchy.super_classes, PhenotypeHierarchyRelation::Parent),
            (&hierarchy.sub_classes, PhenotypeHierarchyRelation::Child),
        ] {
            links.extend(rows.as_deref().unwrap_or_default().iter().map(|row| {
                PhenotypeHierarchyLink {
                    id: row.id.clone(),
                    label: Some(row.name.clone()),
                    relation,
                    attribution: attribution.clone(),
                }
            }));
        }
        Some(links.into())
    });
    (available.then(|| synonyms.into()), xrefs, hierarchy)
}
