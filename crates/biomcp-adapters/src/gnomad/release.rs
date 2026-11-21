//! Indexed, versioned gnomAD constraint tables.
use biodata::{Accession, AccessionBase, EnsemblFeature, FiniteNumber, Gene, Text};
use biomcp_fetch::{
    Bundle, Find, Freshness, KeyKind, Keys, Leg, RequestError, Row, RowFormat, SourceSpec,
};
use biomcp_types::{Constraint, ConstraintRelease, Source};
use serde::{Deserialize, Deserializer};
use std::{marker::PhantomData, time::Duration};

/// Publisher endpoint for the explicit constraint release tables.
pub static SPEC: SourceSpec = SourceSpec {
    base: "https://storage.googleapis.com/gcp-public-data--gnomad/release",
    source: Source::Gnomad,
    terms: super::SPEC.terms,
    destinations: super::SPEC.destinations,
    key: super::SPEC.key,
    quota: super::SPEC.quota,
    user_agent: super::SPEC.user_agent,
    deadline: Duration::from_secs(120),
    command_budget: None,
};

pub(crate) const V4_0_NAME: &str = "gnomad-constraint-v4.0";
pub(crate) const V4_1_NAME: &str = "gnomad-constraint-v4.1";

const fn bundle(release: ConstraintRelease) -> Bundle<Native> {
    let (name, path) = match release {
        ConstraintRelease::V4_0 => (
            V4_0_NAME,
            "/4.0/constraint/gnomad.v4.0.constraint_metrics.tsv",
        ),
        ConstraintRelease::V4_1 => (
            V4_1_NAME,
            "/4.1/constraint/gnomad.v4.1.constraint_metrics.tsv",
        ),
    };
    Bundle {
        name,
        path,
        spec: &SPEC,
        format: RowFormat::NativeTsv {
            gzip: false,
            max_decoded_bytes: 128 * 1024 * 1024,
        },
        fresh: Freshness::Days(365),
        row: PhantomData,
    }
}

/// Index one complete release through the standard bundle sync.
pub(crate) fn sync(
    fetch: &biomcp_fetch::Fetch,
    release: ConstraintRelease,
) -> Result<crate::Mapped<biomcp_types::SyncResult>, RequestError> {
    fetch
        .sync(&bundle(release))
        .map(|info| crate::mapped::bundle_sync(SPEC.source, info))
}

pub(crate) fn state(leg: &Leg<'_>) -> Result<biomcp_types::LocalBundleState, RequestError> {
    Ok(leg
        .stored_bundle_state(&bundle(ConstraintRelease::V4_1))?
        .with(leg.stored_bundle_state(&bundle(ConstraintRelease::V4_0))?))
}

/// Read only the requested identities from one release. The v4.0 table has no gene ID,
/// so its row must match the resolved gene's independently identified MANE transcript.
pub fn constraints(
    leg: &Leg<'_>,
    release: ConstraintRelease,
    genes: &[&Gene],
) -> Result<Vec<Option<Constraint>>, RequestError> {
    let keys: Vec<_> = genes
        .iter()
        .map(|gene| match release {
            ConstraintRelease::V4_0 => gene
                .mane
                .iter()
                .find(|pair| pair.status == biodata::ManeStatus::Select)
                .map(|pair| pair.ensembl().base().to_string()),
            ConstraintRelease::V4_1 => gene.ensembl_id.as_ref().map(|id| id.base().to_owned()),
        })
        .collect();
    let requested: Vec<_> = keys.iter().filter_map(|key| key.as_deref()).collect();
    let (rows, _) = leg.stored_rows(&bundle(release), Find::AnyExact(KeyKind::Id, &requested))?;
    let version = crate::mapped::text(release.as_str())
        .map_err(|_| biomcp_fetch::bad_reply(SPEC.source, None))?;
    Ok(keys
        .iter()
        .map(|key| {
            let key = key.as_deref()?;
            let mut candidates = rows
                .iter()
                .filter(|row| row.mane_select == "true" && row.key() == key);
            let row = candidates.next()?;
            candidates
                .next()
                .is_none()
                .then(|| row.constraint(version.clone()))
        })
        .collect())
}

#[derive(Debug, Deserialize)]
struct Native {
    gene_id: Option<String>,
    #[serde(deserialize_with = "transcript")]
    transcript: AccessionBase,
    mane_select: String,
    #[serde(rename = "lof.pLI", deserialize_with = "number")]
    pli: Option<FiniteNumber>,
    #[serde(rename = "lof.oe_ci.upper", deserialize_with = "number")]
    loeuf: Option<FiniteNumber>,
    #[serde(rename = "lof.oe", deserialize_with = "number")]
    oe_lof: Option<FiniteNumber>,
    #[serde(rename = "mis.z_score", deserialize_with = "number")]
    mis_z: Option<FiniteNumber>,
    #[serde(rename = "syn.z_score", deserialize_with = "number")]
    syn_z: Option<FiniteNumber>,
    #[serde(rename = "constraint_flags", deserialize_with = "flags")]
    flags: Box<[Text]>,
}

fn transcript<'de, D: Deserializer<'de>>(deserializer: D) -> Result<AccessionBase, D::Error> {
    let text = String::deserialize(deserializer)?;
    if text.contains('.') {
        text.parse::<Accession>()
            .map(Accession::base)
            .map_err(serde::de::Error::custom)
    } else {
        text.parse().map_err(serde::de::Error::custom)
    }
}

fn number<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Option<FiniteNumber>, D::Error> {
    let text = String::deserialize(deserializer)?;
    match text.as_str() {
        "" | "NA" => Ok(None),
        _ => {
            let value = text.parse::<f64>().map_err(serde::de::Error::custom)?;
            FiniteNumber::new(value)
                .map(Some)
                .map_err(serde::de::Error::custom)
        }
    }
}

fn flags<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Box<[Text]>, D::Error> {
    let value = String::deserialize(deserializer)?;
    if value.is_empty() || value == "NA" {
        return Ok(Box::default());
    }
    serde_json::from_str(&value).map_err(serde::de::Error::custom)
}

impl Native {
    fn key(&self) -> String {
        self.gene_id
            .as_deref()
            .filter(|id| !id.is_empty())
            .map_or_else(|| self.transcript.to_string(), str::to_owned)
    }

    fn constraint(&self, version: Text) -> Constraint {
        Constraint {
            pli: self.pli,
            loeuf: self.loeuf,
            oe_lof: self.oe_lof,
            mis_z: self.mis_z,
            syn_z: self.syn_z,
            flags: self.flags.clone(),
            transcript: self.transcript,
            version,
        }
    }
}
impl Row for Native {
    const REQUIRE_ACCEPTED_ROWS: bool = true;
    fn keys(&self) -> Keys {
        // v4.1 carries equivalent RefSeq rows under separate NCBI gene IDs.
        let exact = if self.transcript.ensembl_feature() == Some(EnsemblFeature::Transcript) {
            vec![(KeyKind::Id, self.key().into())]
        } else {
            // These valid native rows are retained without colliding with Ensembl keys.
            vec![(KeyKind::Xref, self.transcript.to_string().into())]
        };
        Keys {
            exact,
            words: Vec::new(),
        }
    }
}
