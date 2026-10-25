use biodata::{
    CompanionDiagnosticRelation, CompanionGroupCells, CompanionIndividualCells, DiagnosticCell,
};
use biomcp_fetch::{BadReply, KeyKind, Keys, Row, encode_bundle_row};
use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Native {
    pub relation: CompanionDiagnosticRelation,
    pub device_conflict: bool,
    pub drug_conflict: bool,
}
impl Row for Native {
    fn decode_document(body: &[u8]) -> Result<Vec<String>, BadReply> {
        let tables = super::html::decode(body)?;
        let mut rows = Vec::new();
        for (table, values) in tables.into_iter().enumerate() {
            for (index, cells) in values.into_iter().enumerate() {
                rows.push(encode_bundle_row(&native(table, index, cells)?)?);
            }
        }
        Ok(rows)
    }
    fn keys(&self) -> Keys {
        let mut exact = vec![(KeyKind::Band, "all".into())];
        let (device_ids, applications) = match &self.relation {
            CompanionDiagnosticRelation::Individual {
                cells,
                device_ids,
                drug_applications,
                ..
            } => {
                exact.push((KeyKind::Symbol, cells.biomarker.text.clone()));
                (device_ids, drug_applications)
            }
            CompanionDiagnosticRelation::Group {
                device_ids,
                drug_applications,
                ..
            } => (device_ids, drug_applications),
        };
        for id in device_ids {
            exact.push((KeyKind::Id, id.to_string().into()));
            if let Some((base, _)) = id.as_str().split_once('/') {
                exact.push((KeyKind::Id, base.into()));
            }
        }
        exact.extend(
            applications
                .iter()
                .map(|v| (KeyKind::Xref, v.id.to_string().into())),
        );
        Keys {
            exact,
            words: Vec::new(),
        }
    }
}
fn native(table: usize, index: usize, cells: Vec<DiagnosticCell>) -> Result<Native, BadReply> {
    let index = u32::try_from(index).map_err(|_| BadReply)?;
    let (device, drug) = match cells.as_slice() {
        [_, _, drug, _, _, device] if table == 0 => (device, drug),
        [_, _, device, drug] if table == 1 => (device, drug),
        _ => return Err(BadReply),
    };
    let (device_ids, device_conflict) = super::identity::devices(device);
    let (drug_applications, drug_conflict) = super::identity::drugs(drug);
    let relation = match cells.as_slice() {
        [diagnostic, indication, drug, biomarker, details, device] if table == 0 => {
            CompanionDiagnosticRelation::Individual {
                snapshot: "".into(),
                index,
                cells: CompanionIndividualCells {
                    diagnostic: diagnostic.clone(),
                    indication: indication.clone(),
                    drug: drug.clone(),
                    biomarker: biomarker.clone(),
                    details: details.clone(),
                    device: device.clone(),
                },
                device_ids,
                drug_applications,
            }
        }
        [diagnostic, indication, device, drug_group] if table == 1 => {
            CompanionDiagnosticRelation::Group {
                snapshot: "".into(),
                index,
                cells: CompanionGroupCells {
                    diagnostic: diagnostic.clone(),
                    indication: indication.clone(),
                    device: device.clone(),
                    drug_group: drug_group.clone(),
                },
                device_ids,
                drug_applications,
            }
        }
        _ => return Err(BadReply),
    };
    Ok(Native {
        relation,
        device_conflict,
        drug_conflict,
    })
}
