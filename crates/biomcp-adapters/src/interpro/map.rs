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
        let entry_type = crate::mapped::field_code_text(
            &mut problems.list,
            problems.source,
            problems.record.clone(),
            "type",
            entry.metadata.entry_type,
        );
        let signatures = signatures(entry.metadata.member_databases, &mut problems);
        for protein in entry.proteins {
            let protein_length = protein
                .protein_length
                .and_then(|v| problems.required("protein_length", v.value().filter(|n| *n > 0)));
            let locations = locations(
                protein.entry_protein_locations,
                protein_length,
                &mut problems,
            );
            rows.push(ProteinDomainMatch {
                id: entry.metadata.accession.clone(),
                name: name.clone(),
                entry_type: entry_type.clone(),
                protein: id.clone(),
                protein_length,
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

fn signatures(
    raw: Option<raw::MemberDatabases>,
    problems: &mut crate::mapped::Problems,
) -> Vec<ProteinMemberSignature> {
    let mut signatures = Vec::new();
    if let Some(field) = raw
        && let Some(databases) = problems.required("member_databases", field.value())
    {
        for (database, entries) in databases {
            let Some(database) =
                problems.checked_text("member_databases.database", Some(&database))
            else {
                continue;
            };
            let Some(entries) = problems.required("member_databases.entries", entries.value())
            else {
                continue;
            };
            for (accession, name) in entries {
                let Some(accession) =
                    problems.checked_text("member_databases.accession", Some(&accession))
                else {
                    continue;
                };
                signatures.push(ProteinMemberSignature {
                    namespace: database.clone(),
                    id: accession,
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
    signatures
}
fn locations(
    raw: Option<crate::raw::Field<Vec<crate::raw::Field<raw::Location>>>>,
    length: Option<u32>,
    problems: &mut crate::mapped::Problems,
) -> Box<[ProteinDomainLocation]> {
    let Some(raw) = raw else {
        return Box::new([]);
    };
    let Some(raw) = problems.required("entry_protein_locations", raw.value()) else {
        return Box::new([]);
    };
    raw.into_iter()
        .filter_map(|location| {
            let location = problems.required("entry_protein_locations", location.value())?;
            let fragments = location
                .fragments
                .and_then(|field| problems.required("fragments", field.value()))
                .unwrap_or_default()
                .into_iter()
                .filter_map(|f| {
                    let f = problems.required("fragments", f.value())?;
                    let start = problems.required(
                        "fragments.start",
                        f.start
                            .and_then(crate::raw::Field::value)
                            .filter(|n| *n > 0),
                    );
                    let end = problems.required(
                        "fragments.end",
                        f.end
                            .and_then(crate::raw::Field::value)
                            .filter(|n| *n > 0 && length.is_none_or(|len| *n <= len)),
                    );
                    let (start, end) = (start?, end?);
                    if end < start {
                        let _ = problems.required::<()>("fragments.end", None);
                        return None;
                    }
                    let span = problems.optional(
                        "fragments.span",
                        Some(Bound::new(Some(start), Some(start)).and_then(|start| {
                            Bound::new(Some(end), Some(end))
                                .and_then(|end| UncertainSpan0::from_inclusive(start, end))
                        })),
                    )?;
                    Some(ProteinDomainFragment {
                        span,
                        dc_status: crate::mapped::field_code_text(
                            &mut problems.list,
                            problems.source,
                            problems.record.clone(),
                            "dc-status",
                            f.dc_status,
                        ),
                    })
                })
                .collect();
            Some(ProteinDomainLocation { fragments })
        })
        .collect()
}
