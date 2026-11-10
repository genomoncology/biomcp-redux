//! An explicit label-name union remains a bounded native selector, not label text.
use super::{DrugSearch, DrugSource, QueryInvalid, Text};
const LABEL_NAME_QUERY_MAX_BYTES: usize = 4096;

pub(super) fn validate(query: &DrugSearch) -> Result<(), QueryInvalid> {
    let fields = &query.0;
    if fields.expand_drug_names {
        let Some(name) = query.query() else {
            return Err(QueryInvalid::new(&["query", "expand_drug_names"]));
        };
        if query.source() != DrugSource::OpenfdaLabel
            || name.as_str().parse::<biodata::DrugId>().is_ok_and(|id| {
                DrugSource::Drugsfda.accepts(&id) || DrugSource::OpenfdaLabel.accepts(&id)
            })
        {
            return Err(QueryInvalid::new(&["source", "query", "expand_drug_names"]));
        }
        query.label_name_selector(std::slice::from_ref(name), true)?;
        if fields.offset != 0 && fields.name_snapshot.is_none() {
            return Err(QueryInvalid::new(&["offset", "name_snapshot"]));
        }
    } else if fields.name_snapshot.is_some() {
        return Err(QueryInvalid::new(&["expand_drug_names", "name_snapshot"]));
    }
    Ok(())
}

fn literal(value: &str) -> String {
    format!("\"{}\"", value.replace('\\', "\\\\").replace('"', "\\\""))
}

impl DrugSearch {
    /// Explicit label-name expansion, without changing the default lookup contract.
    pub fn expand_drug_names(&self) -> bool {
        self.0.expand_drug_names
    }
    /// Caller-authorized retained spellings and native matching tier.
    pub fn name_snapshot(&self) -> Option<&Text> {
        self.0.name_snapshot.as_ref()
    }
    /// Freeze the resolved names and matching tier in subsequent native pages.
    pub fn with_name_snapshot(&self, snapshot: Text) -> Self {
        let mut query = self.clone();
        query.0.name_snapshot = Some(snapshot);
        query
    }
    /// One escaped provider name union AND the unchanged literal indication phrase.
    pub fn label_name_selector(&self, names: &[Text], exact: bool) -> Result<Text, QueryInvalid> {
        let suffix = if exact { ".exact" } else { "" };
        let names: Vec<_> = names
            .iter()
            .map(|name| {
                let name = literal(name.as_str());
                format!(
                    "(openfda.brand_name{suffix}:{name} OR openfda.generic_name{suffix}:{name})"
                )
            })
            .collect();
        let mut selector = format!("({})", names.join(" OR "));
        if let Some(indication) = self.indication() {
            selector.push_str(&format!(
                " AND indications_and_usage:{}",
                literal(indication.as_str())
            ));
        }
        if names.is_empty() || selector.len() > LABEL_NAME_QUERY_MAX_BYTES {
            return Err(QueryInvalid::new(&[
                "query",
                "indication",
                "expand_drug_names",
            ]));
        }
        Text::new(&selector).map_err(|_| QueryInvalid::new(&["query", "indication"]))
    }
}
