//! Orphanet-to-OMIM relations as published in Orphadata product 1.
use super::raw::{Availability, Label, Term, label, term};
use crate::mapped::{Mapped, Problems};
use biodata::{Attribution, Curie, DiseaseEndpoint, DiseaseRelation};
use biomcp_fetch::{
    BadReply, Bundle, DocumentMedia, Fetch, Find, Freshness, KeyKind, Keys, Leg, RequestError, Row,
    RowFormat, encode_bundle_row,
};
use serde::{Deserialize, Serialize};

pub(super) const NAME: &str = "orphadata-xrefs";
const FILE: Bundle<Native> = Bundle {
    spec: &super::SPEC,
    name: NAME,
    path: "/data/xml/en_product1.xml",
    fresh: Freshness::Days(30),
    row: std::marker::PhantomData,
    format: RowFormat::Document {
        media: DocumentMedia::Xml,
    },
};

pub(super) fn sync(fetch: &Fetch) -> Result<Mapped<biomcp_types::SyncResult>, RequestError> {
    fetch
        .sync(&FILE)
        .map(|info| crate::mapped::bundle_sync(FILE.spec.source, info))
}

pub(super) fn state(leg: &Leg<'_>) -> Result<biomcp_types::LocalBundleState, RequestError> {
    leg.stored_bundle_state(&FILE)
}

#[derive(Deserialize)]
struct Jdbor {
    #[serde(rename = "@date")]
    date: String,
    #[serde(rename = "@version")]
    version: Option<String>,
    #[serde(rename = "Availability")]
    availability: Availability,
    #[serde(rename = "DisorderList")]
    disorders: Disorders,
}

#[derive(Deserialize)]
struct Disorders {
    #[serde(rename = "Disorder", default)]
    items: Vec<Disorder>,
}

#[derive(Deserialize)]
struct Disorder {
    #[serde(rename = "OrphaCode")]
    code: Option<Label>,
    #[serde(rename = "Name")]
    name: Option<Label>,
    #[serde(rename = "ExternalReferenceList")]
    references: Option<References>,
}

#[derive(Deserialize)]
struct References {
    #[serde(rename = "ExternalReference", default)]
    items: Vec<Reference>,
}

#[derive(Deserialize)]
struct Reference {
    #[serde(rename = "Source")]
    source: Option<Label>,
    #[serde(rename = "Reference")]
    id: Option<Label>,
    #[serde(rename = "DisorderMappingRelation")]
    relation: Option<Term>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Native {
    code: Option<String>,
    name: Option<String>,
    date: String,
    version: Option<String>,
    omim: Vec<Omim>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Omim {
    id: Option<String>,
    relation: Option<String>,
}

impl Row for Native {
    const REQUIRE_ACCEPTED_ROWS: bool = true;

    fn decode_document(body: &[u8]) -> Result<Vec<String>, BadReply> {
        let text = std::str::from_utf8(body).map_err(|_| BadReply)?;
        let document: Jdbor = quick_xml::de::from_str(text).map_err(|_| BadReply)?;
        document.availability.check()?;
        if document.date.trim().is_empty() || document.disorders.items.is_empty() {
            return Err(BadReply);
        }
        document
            .disorders
            .items
            .into_iter()
            .map(|disorder| {
                let omim = disorder
                    .references
                    .into_iter()
                    .flat_map(|list| list.items)
                    .filter_map(|row| {
                        (label(row.source).as_deref() == Some("OMIM")).then(|| Omim {
                            id: label(row.id),
                            relation: term(row.relation),
                        })
                    })
                    .collect();
                encode_bundle_row(&Self {
                    code: label(disorder.code),
                    name: label(disorder.name),
                    date: document.date.clone(),
                    version: document.version.clone(),
                    omim,
                })
            })
            .collect()
    }

    fn keys(&self) -> Keys {
        let exact = self
            .code
            .as_deref()
            .filter(|code| digits(code))
            .map(|code| (KeyKind::Id, code.into()))
            .into_iter()
            .collect();
        Keys {
            exact,
            words: Vec::new(),
        }
    }

    fn data_date(&self) -> Option<&str> {
        Some(&self.date)
    }
}

fn digits(value: &str) -> bool {
    !value.is_empty() && value.bytes().all(|byte| byte.is_ascii_digit())
}

/// Read OMIM mappings for the requested ORPHA code, keeping the publisher's direction.
pub fn omim_xrefs(leg: &Leg<'_>, id: &Curie) -> Result<Mapped<Vec<DiseaseRelation>>, RequestError> {
    let code = super::orpha_code(id.as_str()).ok_or(biomcp_fetch::InputError::Path)?;
    let subject_id: Curie = format!("ORPHA:{code}")
        .parse()
        .map_err(|_| biomcp_fetch::InputError::Serialization)?;
    let (rows, info) = leg.stored_rows(&FILE, Find::Exact(KeyKind::Id, code))?;
    let mut mapped = Mapped {
        value: Vec::new(),
        problems: crate::mapped::bundle_problems(FILE.spec.source, &info),
        refusals: crate::bundle_mapping::bundle_refusals(&info),
    };
    for row in rows {
        let mut cx = Problems {
            source: FILE.spec.source,
            record: crate::mapped::record(&subject_id),
            list: Vec::new(),
        };
        let subject = DiseaseEndpoint {
            id: subject_id.clone(),
            label: cx.text("Name", row.name.as_deref()),
        };
        let attribution = Attribution {
            source: FILE.spec.source.text(),
            record: cx.record.clone(),
            release: cx.text("JDBOR.date", Some(&row.date)),
            version: cx.text("JDBOR.version", row.version.as_deref()),
            url: cx.text("url", Some(&format!("{}{}", FILE.spec.base, FILE.path))),
            approved: None,
            published: None,
            evaluated: None,
            submitter: None,
        };
        for mapping in row.omim {
            if let Some(relation) = relation(mapping, &subject, &attribution, &mut cx) {
                mapped.value.push(relation);
            } else {
                mapped.refusals = crate::Refusal::Refused;
            }
        }
        mapped.problems.extend(cx.list);
    }
    Ok(mapped)
}

fn relation(
    mapping: Omim,
    subject: &DiseaseEndpoint,
    attribution: &Attribution,
    cx: &mut Problems,
) -> Option<DiseaseRelation> {
    let id = cx.required(
        "ExternalReference.Reference",
        mapping
            .id
            .as_deref()
            .filter(|value| digits(value))
            .and_then(|value| format!("OMIM:{value}").parse().ok()),
    )?;
    let code = mapping
        .relation
        .as_deref()
        .and_then(|text| text.split_whitespace().next())
        .unwrap_or("ExternalReference");
    let predicate = cx.required(
        "DisorderMappingRelation",
        format!("orphadata:{code}").parse().ok(),
    )?;
    Some(DiseaseRelation {
        subject: subject.clone(),
        predicate,
        object: DiseaseEndpoint { id, label: None },
        attribution: attribution.clone(),
        description: mapping.relation.map(Into::into),
    })
}
