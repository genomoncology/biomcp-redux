use super::{bad, raw};
use biodata::{
    Attribution, Bound, ProteinDomainFragment, ProteinDomainLocation, ProteinDomainMatch,
    ProteinMemberSignature, UncertainSpan0, UniProtIsoform,
};
use biomcp_fetch::RequestError;
pub(super) fn rows(
    raw: Vec<raw::Entry>,
    id: &UniProtIsoform,
) -> Result<crate::Mapped<Vec<ProteinDomainMatch>>, RequestError> {
    let mut rows = Vec::new();
    let mut all_problems = Vec::new();
    for entry in raw {
        let mut problems = crate::mapped::Problems {
            source: biomcp_types::Source::InterPro,
            record: crate::mapped::record(&entry.metadata.accession),
            list: Vec::new(),
        };
        let name = crate::mapped::field_text(
            &mut problems.list,
            problems.source,
            problems.record.clone(),
            "name",
            entry.metadata.name,
        );
        let entry_type = crate::mapped::field_text(
            &mut problems.list,
            problems.source,
            problems.record.clone(),
            "type",
            entry.metadata.entry_type,
        );
        let mut signatures = Vec::new();
        if let Some(field) = entry.metadata.member_databases {
            if let Some(databases) = problems.required("member_databases", field.value()) {
                for (database, entries) in databases {
                    let Some(database) =
                        problems.text("member_databases.database", Some(&database))
                    else {
                        continue;
                    };
                    let Some(entries) =
                        problems.required("member_databases.entries", entries.value())
                    else {
                        continue;
                    };
                    for (accession, name) in entries {
                        let Some(accession) =
                            problems.text("member_databases.accession", Some(&accession))
                        else {
                            continue;
                        };
                        signatures.push(ProteinMemberSignature {
                            database: database.clone(),
                            accession,
                            name: crate::mapped::field_text(
                                &mut problems.list,
                                problems.source,
                                problems.record.clone(),
                                "member_databases.name",
                                Some(name),
                            ),
                        });
                    }
                }
            }
        }
        for protein in entry.proteins {
            let locations = protein
                .entry_protein_locations
                .into_iter()
                .map(|location| {
                    let fragments = location
                        .fragments
                        .into_iter()
                        .map(|f| {
                            let start =
                                Bound::new(Some(f.start), Some(f.start)).map_err(|_| bad())?;
                            let end = Bound::new(Some(f.end), Some(f.end)).map_err(|_| bad())?;
                            Ok(ProteinDomainFragment {
                                span: UncertainSpan0::from_inclusive(start, end)
                                    .map_err(|_| bad())?,
                                dc_status: crate::mapped::field_text(
                                    &mut problems.list,
                                    problems.source,
                                    problems.record.clone(),
                                    "dc-status",
                                    f.dc_status,
                                ),
                            })
                        })
                        .collect::<Result<Box<[_]>, RequestError>>()?;
                    Ok(ProteinDomainLocation { fragments })
                })
                .collect::<Result<Box<[_]>, RequestError>>()?;
            rows.push(ProteinDomainMatch {
                id: entry.metadata.accession.clone(),
                name: name.clone(),
                entry_type: entry_type.clone(),
                protein: id.clone(),
                protein_length: protein.protein_length,
                taxon: Some(9606),
                locations,
                member_signatures: signatures.clone().into(),
                attribution: Attribution {
                    source: crate::mapped::text("interpro").map_err(|_| bad())?,
                    record: Some(
                        crate::mapped::text(entry.metadata.accession.as_str())
                            .map_err(|_| bad())?,
                    ),
                    url: Some(
                        crate::mapped::text(&format!(
                            "https://www.ebi.ac.uk/interpro/entry/InterPro/{}",
                            entry.metadata.accession
                        ))
                        .map_err(|_| bad())?,
                    ),
                    version: None,
                    release: None,
                    submitter: None,
                    evaluated: None,
                },
            });
        }
        all_problems.extend(problems.list);
    }
    Ok(crate::Mapped {
        value: rows,
        problems: all_problems,
    })
}
