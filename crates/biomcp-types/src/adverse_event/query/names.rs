//! FAERS drug-name widening, its opt-out and a checked retained continuation identity.
use super::{AdverseEventSearch, AdverseEventSource, QueryInvalid, SearchFields, Text};

pub(super) fn validate(fields: &SearchFields) -> Result<(), QueryInvalid> {
    if fields.expand_drug_names && fields.source != AdverseEventSource::Faers {
        return Err(QueryInvalid::new(&["source", "expand_drug_names"]));
    }
    if fields.expand_drug_names && fields.exact_term {
        return Err(QueryInvalid::new(&["expand_drug_names", "exact_term"]));
    }
    if fields.name_snapshot.is_some() && !widens(fields) {
        return Err(QueryInvalid::new(&["exact_term", "name_snapshot"]));
    }
    Ok(())
}

fn widens(fields: &SearchFields) -> bool {
    fields.source == AdverseEventSource::Faers && !fields.exact_term
}

impl AdverseEventSearch {
    /// Whether FAERS searches the typed drug names with the drugs' other names, as it does
    /// unless the caller asked for the typed names only.
    pub fn widens_drug_names(&self) -> bool {
        widens(&self.0)
    }
    /// Retained searched spellings and the resolution evidence that produced them.
    pub fn name_snapshot(&self) -> Option<&Text> {
        self.0.name_snapshot.as_ref()
    }
    /// Preserve the same name set in native offset continuations.
    pub fn with_name_snapshot(&self, snapshot: Text) -> Self {
        let mut query = self.clone();
        query.0.name_snapshot = Some(snapshot);
        query
    }
    /// Check the escaped whole native query after replacing only drug spellings.
    pub fn with_literal_drugs(&self, names: Vec<Text>) -> Result<Self, QueryInvalid> {
        let mut fields = self.0.clone();
        fields.drug = names
            .into_iter()
            .filter(|name| fields.text.as_ref().is_none_or(|text| text != name))
            .collect();
        fields.expand_drug_names = false;
        fields.exact_term = true;
        fields.name_snapshot = None;
        Self::try_from(fields)
    }
}
