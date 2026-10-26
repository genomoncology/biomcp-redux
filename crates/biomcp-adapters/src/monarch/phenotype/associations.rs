//! Direct reverse associations share the disease row representation and mapper.
use biomcp_fetch::{Arg, BadReply, InputError, Leg, RequestError};
use biomcp_types::{PhenotypeGet, PhenotypeSection};
/// Read exactly one checked native page; wrong endpoints or taxa never enter cache.
pub fn get(leg: &Leg<'_>, query: &PhenotypeGet) -> Result<super::super::Page, RequestError> {
    crate::disease_sources::components::monarch(leg)?;
    let requested = query.id().as_str().parse::<biodata::Curie>().map_err(|_| InputError::Path)?;
    let genes = match query.section() {
        Some(PhenotypeSection::Genes) => true,
        Some(PhenotypeSection::Diseases) => false,
        _ => return Err(InputError::Path.into()),
    };
    let mut args = vec![
        ("object", Arg::Text(query.id().as_str())),
        ("subject_category", Arg::Text(if genes { "biolink:Gene" } else { "biolink:Disease" })),
    ];
    if genes { args.push(("subject_taxon", Arg::Text("NCBITaxon:9606"))); }
    args.extend([
        ("direct", Arg::Text("true")),
        ("limit", Arg::Num(query.section_limit().into())),
        ("offset", Arg::Num(query.section_offset().into())),
    ]);
    let (page, metadata) = leg.json_received_checked::<super::super::raw::Page>(&super::super::ASSOCIATIONS, &args, |page| {
        if page.limit != query.section_limit() || page.offset != query.section_offset()
            || page.items.iter().any(|row| row.object != requested
                || (genes && row.subject_taxon.as_ref().map(biodata::Curie::as_str) != Some("NCBITaxon:9606"))) {
            return Err(BadReply);
        }
        Ok(())
    })?;
    Ok(super::super::Page {
        items: page.items.into_iter().map(|row| super::super::map::row(row, &requested)).collect::<Result<_, _>>()?,
        components: super::super::components()?, total: page.total, metadata,
    })
}
