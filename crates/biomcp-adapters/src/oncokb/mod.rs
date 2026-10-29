//! Keyed gene annotations, joined only by checked HGNC Entrez identity.
mod narrative;
use crate::mapped::push_problem;
use crate::mapped::record as record_text;
mod raw;
pub mod variant;
mod variant_groups;
pub mod variant_map;
mod variant_map_support;
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
        if let Some(value) = row.mapped(symbol, id, &mut problems) {
            accepted.push(value);
        }
    }
    let value = if accepted.len() == 1 {
        accepted.pop()
    } else {
        if accepted.len() > 1 {
            push_problem(
                &mut problems,
                Source::Oncokb,
                record_text(id),
                "entrezGeneId",
            );
        }
        None
    };
    Ok(Mapped { value, problems })
}
/// Read the data version once for the command's accepted gene data.
pub fn version(leg: &Leg<'_>) -> Result<Mapped<Option<Text>>, RequestError> {
    let info = leg.json::<raw::Info>(&INFO, &[])?;
    let mut cx = crate::mapped::Problems {
        source: Source::Oncokb,
        record: None,
        list: Vec::new(),
    };
    let value = cx
        .field("dataVersion", info.data_version)
        .and_then(|value| cx.field("dataVersion.version", value.version));
    Ok(Mapped {
        value,
        problems: cx.list,
    })
}
