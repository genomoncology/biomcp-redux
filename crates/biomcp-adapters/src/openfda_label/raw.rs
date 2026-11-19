use crate::openfda::{Envelope, Item};
pub(super) type Response = Envelope<Record>;
use crate::native_text::NativeText;
use biodata::{DrugId, Text};
use biomcp_fetch::BadReply;
use serde::Deserialize;

/// A source SPL revision; the label fields retain their native array boundaries.
#[derive(Debug, Deserialize)]
#[serde(from = "super::wire::Record")]
pub(super) struct Record {
    /// Native revision identifier, distinct from its SPL set.
    pub id: DrugId,
    /// Native SPL set identifier.
    pub set_id: DrugId,
    /// Source-stated revision number.
    pub version: Option<Text>,
    /// Native YYYYMMDD revision date.
    pub effective_time: Option<biodata::PartialDate>,
    /// Rejected native nonidentity fields.
    pub problems: Vec<biomcp_types::Problem>,
    pub(super) openfda: Names,
    pub(super) sections: std::collections::BTreeMap<String, Vec<String>>,
}
#[derive(Debug, Deserialize)]
pub(super) struct Names {
    /// FDA application numbers, such as NDA022406 or ANDA208544.
    pub application_number: Option<Vec<NativeText>>,
    pub brand_name: Option<Vec<NativeText>>,
    pub generic_name: Option<Vec<NativeText>>,
    pub spl_set_id: Option<Vec<DrugId>>,
    /// `[true]` on a label filed by its product's original packager; openFDA omits it on a
    /// repackager's copy, which instead names the original product NDC.
    pub is_original_packager: Option<Vec<bool>>,
}
impl Record {
    /// Only published native sections survive admission; absent remains absent.
    pub(super) fn parts(&self, heading: &str) -> Option<&[String]> {
        self.sections
            .get(heading)
            .map(Vec::as_slice)
            .filter(|parts| !parts.is_empty())
    }
    /// Check product-data text without changing the original section strings.
    pub(super) fn product_name(&self) -> (Option<Text>, Vec<biomcp_types::Problem>) {
        let mut problems = crate::mapped::Problems {
            source: biomcp_types::Source::OpenfdaLabel,
            record: crate::mapped::record(&self.id),
            list: Vec::new(),
        };
        let mut name = None;
        for value in self
            .parts("spl_product_data_elements")
            .iter()
            .copied()
            .flatten()
        {
            let value = problems.optional(
                "spl_product_data_elements",
                Some(crate::native_text::admit(value)),
            );
            if name.is_none() {
                name = value;
            }
        }
        (name, problems.list)
    }
    /// The product name in the label's own text, for a label without harmonized names: the
    /// longest run of leading product-data words that the display panel or indications section
    /// also prints, word for word in any letter case; `None` when neither prints the first word.
    pub(super) fn text_name(&self) -> Option<Text> {
        let product = self.product_name().0?;
        let printed: Vec<Vec<String>> = [
            "package_label_principal_display_panel",
            "indications_and_usage",
        ]
        .into_iter()
        .filter_map(|heading| self.parts(heading))
        .flatten()
        .map(|part| words(part).filter(|word| !word.is_empty()).collect())
        .collect();
        let leading: Vec<String> = words(product.as_str()).collect();
        let shown = (1..=leading.len())
            .take_while(|n| {
                leading.get(..*n).is_some_and(|run| {
                    printed
                        .iter()
                        .any(|text| text.windows(*n).any(|w| w == run))
                })
            })
            .last()?;
        let name = product
            .as_str()
            .split_whitespace()
            .take(shown)
            .collect::<Vec<_>>();
        crate::native_text::admit(&name.join(" ")).ok()
    }
    /// Admit each recent major changes element, naming any that cannot be admitted.
    pub(super) fn recent_major_changes(&self) -> (Box<[Text]>, Vec<biomcp_types::Problem>) {
        let mut problems = crate::mapped::Problems {
            source: biomcp_types::Source::OpenfdaLabel,
            record: crate::mapped::record(&self.id),
            list: Vec::new(),
        };
        let changes = self
            .parts("recent_major_changes")
            .iter()
            .copied()
            .flatten()
            .filter_map(|value| {
                problems.optional(
                    "recent_major_changes",
                    Some(crate::native_text::admit(value)),
                )
            })
            .collect();
        (changes, problems.list)
    }
    pub(super) fn check(&self) -> Result<(), BadReply> {
        use biomcp_types::DrugSource;
        if !DrugSource::OpenfdaLabel.accepts(&self.id)
            || !DrugSource::OpenfdaLabel.accepts(&self.set_id)
            || self
                .openfda
                .spl_set_id
                .iter()
                .flatten()
                .any(|id| id != &self.set_id)
        {
            return Err(BadReply);
        }
        Ok(())
    }
}

impl Item for Record {
    fn check(&self) -> Result<(), BadReply> {
        Record::check(self)
    }
}
/// Each whitespace-separated word in lower case, without trademark marks or trailing
/// punctuation, so `IBRANCE,` and `Itovebi™` read as the product data's `Ibrance` and `Itovebi`.
fn words(text: &str) -> impl Iterator<Item = String> + '_ {
    text.split_whitespace().map(|word| {
        word.trim_matches(['™', '®', ',', ';', ':', '.'])
            .to_lowercase()
    })
}
