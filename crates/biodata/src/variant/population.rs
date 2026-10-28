//! Source population strata, without computed frequencies or classification.
use crate::{Attribution, Build, FiniteNumber, Text, VariantRef};
use serde::{Deserialize, Serialize};
/// One checked stratum of one source dataset release.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "Fields")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(with = "Fields"))]
pub struct PopulationFrequency(Fields);
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
struct Fields {
    subject: VariantRef,
    build: Build,
    dataset: Text,
    dataset_version: Text,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    data_type: Option<PopulationDataType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    ancestry_group: Option<Text>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    sex: Option<PopulationSex>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    subset: Option<Text>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    allele_count: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    allele_number: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(range(min = 0, max = 1)))]
    allele_frequency: Option<FiniteNumber>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    homozygote_count: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    hemizygote_count: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(range(min = 0, max = 1)))]
    faf95: Option<FiniteNumber>,
    #[serde(default, skip_serializing_if = "<[ _ ]>::is_empty")]
    filters: Box<[Text]>,
    attribution: Attribution,
}
impl TryFrom<Fields> for PopulationFrequency {
    type Error = crate::Error;
    fn try_from(fields: Fields) -> Result<Self, Self::Error> {
        if fields
            .allele_count
            .zip(fields.allele_number)
            .is_some_and(|(ac, an)| ac > an)
        {
            return Err(crate::Error::PopulationCountExceedsNumber);
        }
        if [fields.allele_frequency, fields.faf95]
            .into_iter()
            .flatten()
            .any(|n| !(0.0..=1.0).contains(&f64::from(n)))
        {
            return Err(crate::Error::PopulationFrequencyRange);
        }
        Ok(Self(fields))
    }
}
impl PopulationFrequency {
    /// Preserve one dataset identity without inventing observations.
    pub fn new(
        subject: VariantRef,
        build: Build,
        dataset: Text,
        dataset_version: Text,
        attribution: Attribution,
    ) -> Self {
        Self(Fields {
            subject,
            build,
            dataset,
            dataset_version,
            attribution,
            data_type: None,
            ancestry_group: None,
            sex: None,
            subset: None,
            allele_count: None,
            allele_number: None,
            allele_frequency: None,
            homozygote_count: None,
            hemizygote_count: None,
            faf95: None,
            filters: Box::default(),
        })
    }
    /// Preserve source stratum labels without combining or remapping groups.
    pub fn with_stratum(
        mut self,
        data_type: Option<PopulationDataType>,
        ancestry_group: Option<Text>,
        sex: Option<PopulationSex>,
        subset: Option<Text>,
    ) -> Self {
        self.0.data_type = data_type;
        self.0.ancestry_group = ancestry_group;
        self.0.sex = sex;
        self.0.subset = subset;
        self
    }
    /// Preserve source counts while refusing an allele count above its denominator.
    pub fn with_counts(
        mut self,
        allele_count: Option<u64>,
        allele_number: Option<u64>,
        homozygote_count: Option<u64>,
        hemizygote_count: Option<u64>,
    ) -> Result<Self, crate::Error> {
        self.0.allele_count = allele_count;
        self.0.allele_number = allele_number;
        self.0.homozygote_count = homozygote_count;
        self.0.hemizygote_count = hemizygote_count;
        self.0.try_into()
    }
    /// Preserve every source filter in native order.
    pub fn with_filters(mut self, filters: Box<[Text]>) -> Self {
        self.0.filters = filters;
        self
    }
}
crate::wire_enum! {
    /// Dataset sequencing sample set.
    PopulationDataType {
        /// Exome samples.
        Exomes => "exomes",
        /// Genome samples.
        Genomes => "genomes",
        /// Joint samples.
        Joint => "joint",
    }
}
crate::wire_enum! {
    /// Sex stratum stated by a population source.
    PopulationSex {
        /// Source XX stratum.
        Xx => "xx",
        /// Source XY stratum.
        Xy => "xy",
    }
}
