//! A widened name group searched at the step its typed name selects, with exact-name hits in
//! any letter case first and the step kept on later pages.
use super::{Search, candidates, names};
use crate::openfda_label::literal;
use biodata::Text;
use biomcp_fetch::{Leg, RequestError};
use serde::{Deserialize, Serialize};

/// BioMCP's own bound on one native label request, so a name group stays one request.
const NAME_QUERY_MAX_BYTES: usize = 4096;

/// Which discovery step a widened search pages, chosen on its first page as the typed name's
/// literal search would choose it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum NameStep {
    /// Brand and generic names: exact hits in any letter case, then word hits.
    Names,
    /// The label's Indications and Usage text.
    Indication,
    /// Any label text.
    Text,
}

/// The native selectors for one group of names, each name a literal term.
#[derive(Debug)]
pub struct NameGroup {
    exact: String,
    words: String,
    indication: String,
    text: String,
    scope: Option<String>,
}

impl NameGroup {
    /// The selectors for these names, narrowed by a literal indication; `None` when the longest
    /// request would pass the bound. One name gives the same requests as its literal search.
    pub fn new(names: &[Text], indication: Option<&Text>) -> Option<Self> {
        if names.is_empty() {
            return None;
        }
        let scope = indication.map(|i| format!("indications_and_usage:{}", literal(i.as_str())));
        let join = |each: fn(&str) -> String| {
            names
                .iter()
                .map(|name| each(name.as_str()))
                .collect::<Vec<_>>()
                .join(" OR ")
        };
        let mut group = Self {
            exact: join(names::exact),
            words: join(words),
            indication: join(|name| format!("indications_and_usage:{}", literal(name))),
            text: join(literal),
            scope,
        };
        group.exact = group.scoped(&group.exact);
        group.words = group.scoped(&group.words);
        group.indication = group.scoped(&group.indication);
        // The word-only request holds both name selectors, so it is the longest one sent.
        let longest = group.exact.len() + group.words.len() + "() AND NOT ()".len();
        (longest <= NAME_QUERY_MAX_BYTES).then_some(group)
    }

    fn scoped(&self, selector: &str) -> String {
        match &self.scope {
            Some(scope) => format!("({selector}) AND {scope}"),
            None => selector.to_owned(),
        }
    }

    /// The selector a reply shows for the step searched.
    pub fn term(&self, step: NameStep) -> &str {
        match step {
            NameStep::Names => &self.words,
            NameStep::Indication => &self.indication,
            NameStep::Text => &self.text,
        }
    }

    fn page(
        &self,
        leg: &Leg<'_>,
        step: NameStep,
        limit: u32,
        offset: u32,
    ) -> Result<super::Page, RequestError> {
        match step {
            NameStep::Names => names::union(leg, &self.exact, &self.words, limit, offset),
            NameStep::Indication => candidates(leg, &self.indication, limit, offset),
            NameStep::Text => candidates(leg, &self.scoped(&self.text), limit, offset),
        }
    }

    /// Whether any brand or generic name matches, exactly or by word.
    fn names_match(&self, leg: &Leg<'_>) -> Result<bool, RequestError> {
        let selector = format!("({}) OR ({})", self.exact, self.words);
        Ok(candidates(leg, &selector, 1, 0)?
            .window
            .total()
            .is_some_and(|total| total > 0))
    }
}

fn words(name: &str) -> String {
    let name = literal(name);
    format!("openfda.brand_name:{name} OR openfda.generic_name:{name}")
}

/// Search one bounded name group in one native request per step; never combine per-name totals.
/// The first page takes the step the typed name alone would reach, so the group finds at least
/// what the typed name finds; `typed` is `None` when the group is the typed name alone.
pub fn search_expanded(
    leg: &Leg<'_>,
    group: &NameGroup,
    typed: Option<&NameGroup>,
    (limit, offset): (u32, u32),
    selected: Option<NameStep>,
) -> (Search, NameStep) {
    let found = |step: NameStep, page: Result<super::Page, RequestError>| {
        if step == NameStep::Names {
            return (
                Search {
                    page,
                    free_text: None,
                },
                step,
            );
        }
        let search = match group.term(step).parse() {
            Ok(term) => Search {
                page,
                free_text: Some(term),
            },
            Err(_) => Search {
                page: Err(biomcp_fetch::bad_reply(
                    biomcp_types::Source::OpenfdaLabel,
                    Some(200),
                )),
                free_text: None,
            },
        };
        (search, step)
    };
    if let Some(step) = selected {
        return found(step, group.page(leg, step, limit, offset));
    }
    let page = group.page(leg, NameStep::Names, limit, offset);
    let names_found = match &page {
        Err(_) => return found(NameStep::Names, page),
        Ok(page) => page.window.total().is_some_and(|total| total > 0),
    };
    if names_found {
        match typed.map_or(Ok(true), |typed| typed.names_match(leg)) {
            Ok(true) => return found(NameStep::Names, page),
            Ok(false) => {}
            Err(error) => return found(NameStep::Names, Err(error)),
        }
    }
    let page = group.page(leg, NameStep::Indication, limit, offset);
    let settled = match &page {
        Err(_) => true,
        Ok(page) => page.window.total().is_some_and(|total| total > 0),
    };
    if settled {
        return found(NameStep::Indication, page);
    }
    found(
        NameStep::Text,
        group.page(leg, NameStep::Text, limit, offset),
    )
}
