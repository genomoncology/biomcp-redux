//! Keyed gene annotations, joined only by checked HGNC Entrez identity.
use crate::mapped::push_problem;
use crate::mapped::record as record_text;
mod raw;
pub mod variant;
pub mod variant_map;
use crate::{Mapped, quotas};
use biodata::{GeneSymbol, NcbiGeneId, Text};

use biomcp_fetch::{
    Arg, Credential, Endpoint, Freshness, KeyPlace, KeyUse, Leg, RequestError, SecretName,
    SourceSpec,
};
use biomcp_types::{Oncokb, OncokbCard, Source};
use std::time::Duration;
/// Required-key source policy.
pub static SPEC: SourceSpec = SourceSpec {
    destinations: biomcp_fetch::Destinations::Primary,
    user_agent: None,
    terms: biomcp_fetch::Terms {
        name: "OncoKB license",
        url: "https://faq.oncokb.org/licensing",
        usage: biomcp_types::Use::Restricted,
    },
    source: Source::Oncokb,
    base: "https://www.oncokb.org",
    key: KeyUse::Required(Credential {
        secret: SecretName::OncokbApiKey,
        place: KeyPlace::Bearer,
    }),
    quota: &quotas::ONCOKB,
    deadline: Duration::from_secs(8),
};
const GENES: Endpoint = SPEC
    .get("/api/v1/utils/allCuratedGenes")
    .fresh(Freshness::Days(1));
pub(crate) const INFO: Endpoint = SPEC.get("/api/v1/info").fresh(Freshness::Days(1));
/// One accepted gene reply, shared by its card and section.
#[derive(Debug)]
pub struct GeneData {
    /// Short card annotation.
    pub card: OncokbCard,
    /// Whole source section.
    pub section: Oncokb,
}
/// Fetch one case-sensitive HGNC symbol and accept only its checked Entrez identity.
pub fn gene(
    leg: &Leg<'_>,
    symbol: &GeneSymbol,
    id: NcbiGeneId,
) -> Result<Mapped<Option<GeneData>>, RequestError> {
    let reply = leg.json_opt::<raw::Genes>(
        &GENES,
        &[
            ("hugoSymbol", Arg::Text(symbol.as_str())),
            ("includeEvidence", Arg::Text("true")),
        ],
    )?;
    let rows = reply.map(|reply| reply.0).unwrap_or_default();
    let mut problems = Vec::new();
    let mut accepted = Vec::new();
    for row in rows {
        if row.entrez_gene_id == id {
            accepted.push(row);
        } else {
            push_problem(&mut problems, Source::Oncokb, record_text(id), "entrez_id");
        }
    }
    let value = if accepted.len() == 1 {
        accepted.pop().map(|row| row.mapped(&mut problems))
    } else {
        if accepted.len() > 1 {
            push_problem(&mut problems, Source::Oncokb, record_text(id), "entrez_id");
        }
        None
    };
    Ok(Mapped { value, problems })
}
/// Read the data version once for the command's accepted gene data.
pub fn version(leg: &Leg<'_>) -> Result<Text, RequestError> {
    leg.json::<raw::Info>(&INFO, &[])
        .map(|info| info.data_version.version)
}
