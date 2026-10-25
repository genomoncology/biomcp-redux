//! Native annotation facts before response-specific paging.
use biodata::{Curie, Text};

/// One exact received annotation with independently attributed component facts.
#[derive(Debug)]
pub struct Record {
    /// Exact caller-selected identifier.
    pub requested_id: Curie,
    /// Native top-level identifier, checked against the request or its own alias.
    pub returned_id: Curie,
    /// Native record revision, independent of ontology release.
    pub version: Option<u64>,
    /// Present, admitted components; no component is fetched separately.
    pub components: Box<[Component]>,
    /// Original transport retrieval time for this record.
    pub metadata: biomcp_fetch::ReplyMetadata,
}
/// A single component with its own native identity and complete annotation.
#[derive(Debug)]
pub struct Component {
    /// Declared component name used by the shared terms registry.
    pub name: &'static str,
    /// Native component identity, distinct from the aggregate identity.
    pub id: Curie,
    /// Complete source primary label.
    pub label: Text,
    /// Complete optional source definition, without borrowing another component's text.
    pub definition: Option<Text>,
    /// Native synonym scope and text, preserving component-local multiplicity.
    pub synonyms: Box<[Value]>,
    /// Native namespace field and original code, without merging namespaces.
    pub xrefs: Box<[Value]>,
    /// Native direct parents; ancestors and consider are never added.
    pub parents: Box<[Curie]>,
    /// Publisher obsolete state, absent when not supplied.
    pub obsolete: Option<bool>,
    /// Publisher alternatives, never parent relations.
    pub consider: Option<Box<[Curie]>>,
    /// Publisher replacements, never selected automatically.
    pub replaced_by: Option<Box<[Curie]>>,
}
/// One original component field value.
#[derive(Debug)]
pub struct Value {
    /// Native scope or namespace spelling.
    pub field: String,
    /// Complete source text or code.
    pub value: Text,
}
pub(super) fn component(name: &'static str, raw: super::raw::Component) -> Component {
    Component {
        name,
        id: raw.id,
        label: raw.label,
        definition: raw.definition,
        synonyms: values(raw.synonym),
        xrefs: values(raw.xrefs),
        parents: raw.parents.unwrap_or_default(),
        obsolete: raw.is_obsolete,
        consider: raw.consider,
        replaced_by: raw.replaced_by,
    }
}
fn values(
    fields: Option<std::collections::BTreeMap<String, crate::raw::OneOrMany<Text>>>,
) -> Box<[Value]> {
    fields
        .unwrap_or_default()
        .into_iter()
        .flat_map(|(field, values)| {
            values.values().into_iter().map(move |value| Value {
                field: field.clone(),
                value,
            })
        })
        .collect()
}
