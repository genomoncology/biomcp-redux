use biodata::{EntrezGeneId, Text};
use biomcp_fetch::{BadReply, Reply};
use serde::{
    Deserialize,
    de::{MapAccess, Visitor},
};
#[derive(Debug, Deserialize)]
pub(super) struct Genes<const N: usize> {
    pub data: Aliases,
    #[serde(flatten)]
    extra: std::collections::BTreeMap<String, serde::de::IgnoredAny>,
}
#[derive(Debug)]
pub(super) struct Aliases(pub Vec<(String, Option<Gene>)>);
impl<'de> Deserialize<'de> for Aliases {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Map;
        impl<'de> Visitor<'de> for Map {
            type Value = Aliases;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("gene aliases")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Aliases, A::Error> {
                let mut aliases = Vec::new();
                while let Some(pair) = map.next_entry()? {
                    aliases.push(pair);
                }
                Ok(Aliases(aliases))
            }
        }
        d.deserialize_map(Map)
    }
}
impl<const N: usize> Reply for Genes<N> {
    fn check(&self) -> Result<(), BadReply> {
        if self.extra.contains_key("errors") || self.data.0.len() != N {
            return Err(BadReply);
        }
        for index in 0..N {
            if self
                .data
                .0
                .iter()
                .filter(|(name, _)| *name == format!("g{index}"))
                .count()
                != 1
            {
                return Err(BadReply);
            }
        }
        for (_, gene) in &self.data.0 {
            if let Some(gene) = gene {
                if gene.id == 0 || !gene.link.starts_with("/features/") {
                    return Err(BadReply);
                }
                Text::new(&gene.link).map_err(|_| BadReply)?;
            }
        }
        Ok(())
    }
}
#[derive(Debug, Deserialize)]
pub(super) struct Gene {
    pub id: u64,
    #[serde(rename = "entrezId")]
    #[serde(deserialize_with = "entrez")]
    pub entrez_id: EntrezGeneId,
    #[serde(deserialize_with = "nullable")]
    pub description: Option<String>,
    pub link: String,
    pub variants: Variants,
}
#[derive(Debug, Deserialize)]
pub(super) struct Variants {
    #[serde(rename = "totalCount")]
    pub total_count: u64,
}
fn nullable<'de, T: Deserialize<'de>, D: serde::Deserializer<'de>>(
    d: D,
) -> Result<Option<T>, D::Error> {
    Option::<T>::deserialize(d)
}

fn entrez<'de, D: serde::Deserializer<'de>>(d: D) -> Result<EntrezGeneId, D::Error> {
    EntrezGeneId::new(u32::deserialize(d)?).map_err(serde::de::Error::custom)
}
