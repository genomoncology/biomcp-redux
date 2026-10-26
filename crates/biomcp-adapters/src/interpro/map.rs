use super::{bad, raw};
use biodata::{Attribution, Bound, ProteinDomainFragment, ProteinDomainLocation, ProteinDomainMatch, ProteinMemberSignature, Text, UncertainSpan0, UniProtIsoform};
use biomcp_fetch::RequestError;
pub(super) fn rows(raw: Vec<raw::Entry>, id: &UniProtIsoform) -> Result<Vec<ProteinDomainMatch>, RequestError> {
    let mut rows = Vec::new();
    for entry in raw {
        let mut signatures = Vec::new();
        for (database, entries) in entry.metadata.member_databases.into_iter().flatten() {
            for (accession, name) in entries {
                signatures.push(ProteinMemberSignature { database: text(&database)?, accession: text(&accession)?, name: Some(text(&name)?) });
            }
        }
        for protein in entry.proteins {
            let locations = protein.entry_protein_locations.into_iter().map(|location| {
                let fragments = location.fragments.into_iter().map(|f| {
                    let start = Bound::new(Some(f.start),Some(f.start)).map_err(|_|bad())?;
                    let end = Bound::new(Some(f.end),Some(f.end)).map_err(|_|bad())?;
                    Ok(ProteinDomainFragment { span: UncertainSpan0::from_inclusive(start,end).map_err(|_|bad())?, dc_status:f.dc_status })
                }).collect::<Result<Box<[_]>,RequestError>>()?;
                Ok(ProteinDomainLocation {fragments})
            }).collect::<Result<Box<[_]>,RequestError>>()?;
            rows.push(ProteinDomainMatch {
                id:entry.metadata.accession.clone(),name:entry.metadata.name.clone(),entry_type:entry.metadata.entry_type.clone(),
                protein:id.clone(),protein_length:protein.protein_length,taxon:Some(9606),locations,member_signatures:signatures.clone().into(),
                attribution:Attribution {source:text("interpro")?,record:Some(text(entry.metadata.accession.as_str())?),url:Some(text(&format!("https://www.ebi.ac.uk/interpro/entry/InterPro/{}",entry.metadata.accession))?),version:None,release:None,submitter:None,evaluated:None}
            });
        }
    }
    Ok(rows)
}
fn text(v:&str)->Result<Text,RequestError>{Text::new(v).map_err(|_|bad())}
