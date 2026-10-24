use biodata::{Cytoband, EnsemblGeneId, EntrezGeneId, GeneSymbol, HgncId};
use biomcp_fetch::{KeyKind, Keys, Row};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(super) struct Gene {
    #[serde(default)]
    pub hgnc_id: String,
    #[serde(default)]
    pub symbol: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub locus_group: String,
    #[serde(default)]
    pub locus_type: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub location: String,
    #[serde(default)]
    pub alias_symbol: String,
    #[serde(default)]
    pub prev_symbol: String,
    #[serde(default)]
    pub entrez_id: String,
    #[serde(default)]
    pub ensembl_gene_id: String,
    #[serde(default)]
    pub uniprot_ids: String,
    #[serde(default)]
    pub omim_id: String,
    #[serde(default)]
    pub mane_select: String,
}
pub(super) fn parts(value: &str) -> impl Iterator<Item = &str> {
    value.split('|').filter(|v| !v.is_empty())
}
impl Row for Gene {
    fn keys(&self) -> Keys {
        let mut keys = identity(&self.hgnc_id, &self.symbol);
        for (kind, values) in [
            (KeyKind::PreviousSymbol, &self.prev_symbol),
            (KeyKind::Alias, &self.alias_symbol),
        ] {
            keys.exact.extend(parts(values).map(|v| (kind, v.into())));
            keys.words.extend(parts(values).map(Into::into));
        }
        if let Ok(id) = self.entrez_id.parse::<EntrezGeneId>() {
            keys.exact
                .push((KeyKind::Xref, format!("entrez:{id}").into()));
        }
        if let Ok(id) = self.ensembl_gene_id.parse::<EnsemblGeneId>() {
            keys.exact
                .push((KeyKind::Xref, format!("ensembl:{}", id.base()).into()));
        }
        if Cytoband::new(&self.location).is_ok() {
            keys.exact
                .push((KeyKind::Band, self.location.clone().into()));
        }
        keys.words.push(self.name.clone().into());
        keys
    }
}
fn identity(id: &str, symbol: &str) -> Keys {
    let mut exact = Vec::new();
    if let Ok(id) = id.parse::<HgncId>() {
        exact.push((KeyKind::Id, id.to_string().into()));
    }
    if GeneSymbol::new(symbol).is_ok() {
        exact.push((KeyKind::Symbol, symbol.into()));
    }
    Keys {
        exact,
        words: vec![symbol.into()],
    }
}
#[derive(Debug, Deserialize)]
pub(super) struct Withdrawn {
    #[serde(default, rename = "HGNC_ID")]
    pub hgnc_id: String,
    #[serde(default, rename = "STATUS")]
    pub status: String,
    #[serde(default, rename = "WITHDRAWN_SYMBOL")]
    pub symbol: String,
    #[serde(default, rename = "MERGED_INTO_REPORT(S) (i.e HGNC_ID|SYMBOL|STATUS)")]
    pub merged: String,
}
impl Row for Withdrawn {
    fn keys(&self) -> Keys {
        identity(&self.hgnc_id, &self.symbol)
    }
}
