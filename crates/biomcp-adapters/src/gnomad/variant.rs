//! Source-native gnomAD population subsets; no frequency inference.
//! VA release/count/cohort fields: https://github.com/broadinstitute/gnomad-browser/blob/main/graphql-api/src/graphql/types/va.graphql
//! The resolver supplies an ALL cohort alongside ancestry rows; its constants are not observed releases.
use super::{Query, SPEC};
use biodata::Text;
use biomcp_fetch::{BadReply, Endpoint, Freshness, Leg, Reply, RequestError};
use serde::Deserialize;
use std::collections::BTreeMap;
const VARIANT: Endpoint = SPEC
    .post_json("/api")
    .idempotent()
    .fresh(Freshness::Days(7));
const QUERY: &str = "query($id:String!){variant(variantId:$id,dataset:gnomad_r4){variant_id exome{ac an filters \
    populations{id ac an}} genome{ac an filters populations{id ac an}} va{exome{derivedFrom{version} \
    focusAlleleCount locusAlleleCount cohort{id}} genome{derivedFrom{version} focusAlleleCount \
    locusAlleleCount cohort{id}}}}}";
/// Dataset selector sent unchanged to the source.
pub const DATASET: &str = "gnomad_r4";
#[derive(Debug, Deserialize)]
struct Response {
    data: Data,
    errors: Option<Vec<GraphError>>,
}
#[derive(Debug, Deserialize)]
struct GraphError {
    message: String,
}
#[derive(Debug, Deserialize)]
struct Data {
    #[serde(deserialize_with = "nullable")]
    variant: Option<Variant>,
}
/// One exact source allele; each dataset is independently nullable.
#[derive(Debug, Deserialize)]
pub struct Variant {
    /// Exact returned source allele identifier.
    pub variant_id: Text,
    /// Optional source release provenance; old retained replies can lack it.
    pub va: Option<Va>,
    /// Exome observation, absent when source says null.
    #[serde(deserialize_with = "nullable")]
    pub exome: Option<Subset>,
    /// Genome observation, absent when source says null.
    #[serde(deserialize_with = "nullable")]
    pub genome: Option<Subset>,
}
/// Source VA cohorts, kept separate for each sequencing dataset.
#[derive(Debug, Deserialize)]
pub struct Va {
    /// Exome cohort observations.
    pub exome: Option<Vec<CohortFrequency>>,
    /// Genome cohort observations.
    pub genome: Option<Vec<CohortFrequency>>,
}
/// Same-response release and count association, not a resolver constant.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CohortFrequency {
    /// Source dataset metadata.
    pub derived_from: Option<Derivation>,
    /// Source focus allele count.
    pub focus_allele_count: u64,
    /// Source locus allele count.
    pub locus_allele_count: u64,
    /// Source cohort identity.
    pub cohort: Cohort,
}
/// Source dataset derivation.
#[derive(Debug, Deserialize)]
pub struct Derivation {
    /// Nullable source release, never backfilled.
    pub version: Option<String>,
}
/// Source cohort identity, including the documented ALL total.
#[derive(Debug, Deserialize)]
pub struct Cohort {
    /// Native cohort key.
    pub id: Text,
}
/// Source counts and overlapping source groups, never summed or interpreted.
#[derive(Debug, Deserialize)]
pub struct Subset {
    /// Source allele count.
    pub ac: u64,
    /// Source allele number.
    pub an: u64,
    /// Source filters, including an explicitly empty list.
    pub filters: Vec<Text>,
    /// Every group in native source order, including sex-specific groups.
    pub populations: Vec<Group>,
}
/// Source group count without a derived allele frequency.
#[derive(Debug, Deserialize)]
pub struct Group {
    /// Native group identifier.
    pub id: Text,
    /// Source allele count.
    pub ac: u64,
    /// Source allele number.
    pub an: u64,
}
impl Reply for Response {
    fn check(&self) -> Result<(), BadReply> {
        let errors = self.errors.as_deref().unwrap_or_default();
        if self.data.variant.is_none() {
            return if matches!(errors, [error] if error.message == "Variant not found") {
                Ok(())
            } else {
                Err(BadReply)
            };
        }
        if !errors.is_empty() {
            return Err(BadReply);
        }
        let v = self.data.variant.as_ref().ok_or(BadReply)?;
        for subset in [&v.exome, &v.genome].into_iter().flatten() {
            if subset.ac > subset.an || subset.populations.iter().any(|g| g.ac > g.an) {
                return Err(BadReply);
            }
            let mut ids = std::collections::BTreeSet::new();
            if subset
                .populations
                .iter()
                .any(|g| !ids.insert(g.id.as_str()))
            {
                return Err(BadReply);
            }
        }
        Ok(())
    }
}
/// Read one provider-given GRCh38 allele using the shared checked cache transport.
pub fn population(leg: &Leg<'_>, id: &str) -> Result<Option<Variant>, RequestError> {
    let body = Query {
        query: QUERY.into(),
        variables: BTreeMap::from([("id".into(), id.into())]),
    };
    let (response, _) =
        leg.post_json_received_checked::<_, Response>(&VARIANT, &[], &body, |r| {
            if r.data
                .variant
                .as_ref()
                .is_none_or(|v| v.variant_id.as_str() == id)
            {
                Ok(())
            } else {
                Err(BadReply)
            }
        })?;
    Ok(response.data.variant)
}
fn nullable<'de, T: Deserialize<'de>, D: serde::Deserializer<'de>>(
    d: D,
) -> Result<Option<T>, D::Error> {
    Option::<T>::deserialize(d)
}
