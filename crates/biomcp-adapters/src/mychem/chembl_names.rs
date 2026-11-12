//! ChEMBL's molecule names as MyChem copies them, for drug-name lookups that must not wait on
//! ChEMBL's own API.
use biomcp_fetch::{Arg, Leg, Read, RequestError};
use biomcp_types::DrugNameMatch;
/// One ChEMBL molecule stating a name, from MyChem's copy of ChEMBL.
#[derive(Debug)]
pub struct ChemblRecord {
    /// ChEMBL molecule identifier.
    pub id: String,
    /// The preferred name, then the synonyms, each name once.
    pub names: Vec<biodata::Text>,
    /// ChEMBL's preferred name.
    pub preferred: Vec<String>,
    /// The synonyms ChEMBL types as official.
    pub official: Vec<String>,
}
/// The ChEMBL molecules stating one name, with the native window they came from.
#[derive(Debug)]
pub struct ChemblNames {
    /// Native matching count, including candidates outside this window.
    pub total: u64,
    /// Native rows consumed before refusal or exact-name filtering.
    pub received: u32,
    /// Molecules whose preferred name or a synonym is the name.
    pub molecules: Vec<ChemblRecord>,
}
/// ChEMBL's name fields in MyChem: the ones ChEMBL's exact-name filters read.
const CHEMBL_NAME_FIELDS: &str = "chembl.pref_name,chembl.molecule_synonyms.molecule_synonym";
const CHEMBL_READ_FIELDS: &str =
    "chembl.molecule_chembl_id,chembl.pref_name,chembl.molecule_synonyms";
/// Read the ChEMBL molecules stating one name from MyChem's copy of ChEMBL, which answers in one
/// request where ChEMBL's own API takes two searches and a molecule read, each taking seconds.
pub fn chembl_names(
    leg: &Leg<'_>,
    query: &str,
    limit: u32,
) -> Result<crate::Mapped<ChemblNames>, RequestError> {
    let phrase = format!("\"{}\"", query.replace('\\', "\\\\").replace('"', "\\\""));
    let q = crate::drug_sources::components::allowed_fields(leg, CHEMBL_NAME_FIELDS)
        .split(',')
        .filter(|field| !field.is_empty())
        .map(|field| format!("{field}:{phrase}"))
        .collect::<Vec<_>>()
        .join(" OR ");
    if q.is_empty() {
        return Ok(crate::Mapped {
            refusals: crate::Refusal::None,
            value: ChemblNames {
                total: 0,
                received: 0,
                molecules: Vec::new(),
            },
            problems: Vec::new(),
        });
    }
    let fields = crate::drug_sources::components::allowed_fields(leg, CHEMBL_READ_FIELDS);
    let reply = leg
        .call(
            &super::SEARCH,
            biomcp_fetch::Input::Params(&[
                ("q", Arg::Text(&q)),
                ("size", Arg::Num(limit as u64)),
                ("fields", Arg::Text(&fields)),
            ]),
            Read::<super::raw::Page>::json()
                .check(&|_| Ok(()))
                .cache_if(&super::raw::Page::identities_admitted),
        )?
        .value;
    let total = reply.total;
    let received = reply.hits.len() as u32;
    let mapped = chembl_records(reply, leg, query);
    Ok(crate::Mapped {
        refusals: mapped.refusals,
        value: ChemblNames {
            total,
            received,
            molecules: mapped.value,
        },
        problems: mapped.problems,
    })
}
/// Keep the ChEMBL molecules whose preferred name or a synonym is the name, as ChEMBL's own
/// exact-name filters would, with each molecule's names in ChEMBL's order.
fn chembl_records(
    page: super::raw::Page,
    leg: &biomcp_fetch::Leg<'_>,
    query: &str,
) -> crate::Mapped<Vec<ChemblRecord>> {
    let mut problems = crate::mapped::Problems {
        source: biomcp_types::Source::Mychem,
        record: None,
        list: Vec::new(),
    };
    let mut records: Vec<ChemblRecord> = Vec::new();
    let mut refusals = crate::Refusal::None;
    let exact = |s: &str| DrugNameMatch::same_name(s, query);
    for hit in page.hits {
        problems.record = None;
        let Some(hit) = problems.field("_id", Some(hit)) else {
            refusals = crate::Refusal::Refused;
            continue;
        };
        problems.record = crate::mapped::record(&hit.id);
        if problems.optional("_id", Some(hit.identity())).is_none() {
            refusals = crate::Refusal::Refused;
            continue;
        }
        let (_, _, rows) = super::map::components(hit, leg, &mut problems);
        for (_, c) in rows.iter().filter(|(component, _)| *component == "chembl") {
            let synonyms = || {
                c.molecule_synonyms
                    .iter()
                    .flatten()
                    .filter_map(|s| s.molecule_synonym.as_deref())
            };
            if !c.pref_name.as_deref().is_some_and(exact) && !synonyms().any(exact) {
                continue;
            }
            let Some(id) =
                problems.required("chembl.molecule_chembl_id", c.molecule_chembl_id.clone())
            else {
                refusals = crate::Refusal::Refused;
                continue;
            };
            if records.iter().any(|record| record.id == id) {
                continue;
            }
            let mut names: Vec<biodata::Text> = Vec::new();
            for value in c.pref_name.as_deref().into_iter().chain(synonyms()) {
                if let Some(name) = problems.text("chembl.molecule_synonyms", Some(value))
                    && !names
                        .iter()
                        .any(|seen| DrugNameMatch::same_name(seen.as_str(), value))
                {
                    names.push(name);
                }
            }
            records.push(ChemblRecord {
                id,
                names,
                preferred: c.pref_name.clone().into_iter().collect(),
                official: super::names::official(c),
            });
        }
    }
    crate::Mapped {
        refusals,
        value: records,
        problems: problems.list,
    }
}
