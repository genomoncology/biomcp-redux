//! OBO term identity and the transitive closure of asserted is_a links.
use biodata::HpId;
use biomcp_fetch::BadReply;
use std::collections::{HashMap, HashSet};

pub(super) fn has_version(body: &str, date: &str) -> bool {
    body.lines()
        .take_while(|line| !line.starts_with('['))
        .any(|line| {
            line.strip_prefix("data-version: ")
                .and_then(|version| version.strip_prefix("hp/releases/"))
                == Some(date)
        })
}

#[derive(Default)]
struct Graph {
    children: HashMap<HpId, Vec<HpId>>,
    aliases: HashMap<HpId, HpId>,
    terms: HashSet<HpId>,
}

impl Graph {
    fn parse(body: &str) -> Result<Self, BadReply> {
        let mut graph = Self::default();
        for stanza in body.split("\n[Term]").skip(1) {
            let lines = || stanza.lines().take_while(|line| !line.starts_with('['));
            if lines().any(|line| line == "is_obsolete: true") {
                continue;
            }
            let id = lines()
                .find_map(|line| line.strip_prefix("id: "))
                .ok_or(BadReply)?;
            if !id.starts_with("HP:") {
                continue;
            }
            let id: HpId = id.parse().map_err(|_| BadReply)?;
            graph.terms.insert(id.clone());
            for line in lines() {
                if let Some(parent) = line.strip_prefix("is_a: ") {
                    let parent = parent.split_whitespace().next().ok_or(BadReply)?;
                    if parent.starts_with("HP:") {
                        graph
                            .children
                            .entry(parent.parse().map_err(|_| BadReply)?)
                            .or_default()
                            .push(id.clone());
                    }
                }
                if let Some(alias) = line.strip_prefix("alt_id: ") {
                    graph
                        .aliases
                        .insert(alias.parse().map_err(|_| BadReply)?, id.clone());
                }
            }
        }
        if graph.terms.is_empty() {
            return Err(BadReply);
        }
        Ok(graph)
    }
}

type Descendants = HashMap<HpId, Vec<usize>>;

pub(super) fn descendants(
    body: &str,
    requested: &[HpId],
) -> Result<(Descendants, Vec<HpId>), BadReply> {
    let graph = Graph::parse(body)?;
    let mut descendants = Descendants::new();
    let mut unknown = Vec::new();
    for (index, term) in requested.iter().enumerate() {
        let root = graph.aliases.get(term).unwrap_or(term);
        if !graph.terms.contains(root) {
            unknown.push(term.clone());
            continue;
        }
        let mut pending = vec![root];
        let mut seen = HashSet::new();
        while let Some(id) = pending.pop() {
            if !seen.insert(id) {
                continue;
            }
            descendants.entry(id.clone()).or_default().push(index);
            if let Some(children) = graph.children.get(id) {
                pending.extend(children);
            }
        }
    }
    for (alias, id) in graph.aliases {
        if let Some(indices) = descendants.get(&id) {
            descendants.insert(alias, indices.clone());
        }
    }
    Ok((descendants, unknown))
}
