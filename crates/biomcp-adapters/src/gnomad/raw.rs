use crate::raw::Field;
use biodata::{EnsemblGeneId, EnsemblTranscript, FiniteNumber};
use biomcp_fetch::{BadReply, Reply};
use serde::{
    Deserialize,
    de::{MapAccess, Visitor},
};

#[derive(Debug, Deserialize)]
pub(super) struct Genes<const N: usize> {
    pub data: Aliases,
    pub errors: Option<Vec<GraphError>>,
}
#[derive(Debug, Deserialize)]
pub(super) struct GraphError {
    message: String,
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
        if self.data.0.len() != N {
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
        let missing = self
            .data
            .0
            .iter()
            .filter(|(_, gene)| gene.is_none())
            .count();
        let errors = self.errors.as_deref().unwrap_or_default();
        if errors.len() != missing || errors.iter().any(|error| error.message != "Gene not found") {
            return Err(BadReply);
        }
        for (_, gene) in &self.data.0 {
            if let Some(gene) = gene {
                gene.gene_id
                    .parse::<EnsemblGeneId>()
                    .map_err(|_| BadReply)?;
                if gene.gnomad_constraint.is_some() {
                    gene.transcript().ok_or(BadReply)?;
                }
            }
        }
        Ok(())
    }
}
#[derive(Debug, Deserialize)]
pub(super) struct Gene {
    pub gene_id: String,
    pub canonical_transcript_id: Option<String>,
    pub mane_select_transcript: Option<Mane>,
    pub gnomad_constraint: Option<Constraint>,
}
#[derive(Debug, Deserialize)]
pub(super) struct Mane {
    pub ensembl_id: String,
}
impl Gene {
    pub fn transcript(&self) -> Option<EnsemblTranscript> {
        let text = self
            .mane_select_transcript
            .as_ref()
            .map(|mane| mane.ensembl_id.as_str())
            .or(self.canonical_transcript_id.as_deref())?;
        let id: EnsemblTranscript = text.parse().ok()?;
        id.base().parse().ok()
    }
}
#[derive(Debug, Deserialize)]
pub(super) struct Constraint {
    pub pli: FiniteNumber,
    pub oe_lof: FiniteNumber,
    pub oe_lof_upper: FiniteNumber,
    pub mis_z: FiniteNumber,
    pub syn_z: FiniteNumber,
    pub flags: Option<Field<Vec<Field<String>>>>,
}
