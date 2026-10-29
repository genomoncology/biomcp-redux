//! Select exact organization sites without changing native recruitment claims.
use crate::{Mapped, mapped::Problems, raw::Field};
use biodata::Text;
use biomcp_types::{NciMatchedSite, NciTrial, Source};
use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub(super) struct Site {
    org_name: Option<Field<Text>>,
    org_city: Option<Field<Text>>,
    org_state_or_province: Option<Field<Text>>,
    recruitment_status: Option<Field<Text>>,
}
impl super::Trial {
    /// Compact facility search card and problems for malformed or unequal source sites.
    pub fn facility_card(self, name: &Text) -> Mapped<NciTrial> {
        let mut card = self.card(false);
        let mut p = Problems {
            source: Source::Nci,
            record: Some(self.nci_id.clone()),
            list: Vec::new(),
        };
        let expected = name.as_str().to_lowercase();
        let mut count = 0;
        for row in p.fields("sites", self.sites) {
            let Some(org_name) = p.required_field("sites.org_name", row.org_name) else {
                continue;
            };
            if org_name.as_str().to_lowercase() != expected {
                continue;
            }
            count += 1;
            if card.matched_site.is_none() {
                card.matched_site = Some(NciMatchedSite {
                    org_name,
                    org_city: p.field("sites.org_city", row.org_city),
                    org_state_or_province: p
                        .field("sites.org_state_or_province", row.org_state_or_province),
                    recruitment_status: p.field("sites.recruitment_status", row.recruitment_status),
                });
            }
        }
        card.matching_sites = Some(count);
        if count == 0 {
            p.required::<Text>("sites.org_name", None);
        }
        Mapped {
            value: card,
            problems: p.list,
        }
    }
}
