use serde::{Deserialize, Serialize};
use crate::{GtrTestId, GeneRef, Concept, Curie};
/// One complete GTR test-version row; repeated identities remain separate records.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct GtrTestVersion {
    /// Complete accession and version.
    pub test_accession_ver: GtrTestId,
    /// Native `name_of_laboratory` cell; an empty string stays explicit and pipe values stay whole.
    pub name_of_laboratory: Box<str>,
    /// Native `name_of_institution` cell; an empty string stays explicit and pipe values stay whole.
    pub name_of_institution: Box<str>,
    /// Native `facility_state` cell; an empty string stays explicit and pipe values stay whole.
    pub facility_state: Box<str>,
    /// Native `facility_postcode` cell; an empty string stays explicit and pipe values stay whole.
    pub facility_postcode: Box<str>,
    /// Native `facility_country` cell; an empty string stays explicit and pipe values stay whole.
    pub facility_country: Box<str>,
    /// Native `CLIA_number` cell; an empty string stays explicit and pipe values stay whole.
    #[serde(rename = "CLIA_number")]
    pub clia_number: Box<str>,
    /// Native `state_licenses` cell; an empty string stays explicit and pipe values stay whole.
    pub state_licenses: Box<str>,
    /// Native `state_license_numbers` cell; an empty string stays explicit and pipe values stay whole.
    pub state_license_numbers: Box<str>,
    /// Native `lab_test_id` cell; an empty string stays explicit and pipe values stay whole.
    pub lab_test_id: Box<str>,
    /// Native `last_touch_date` cell; an empty string stays explicit and pipe values stay whole.
    pub last_touch_date: Box<str>,
    /// Native `lab_test_name` cell; an empty string stays explicit and pipe values stay whole.
    pub lab_test_name: Box<str>,
    /// Native `manufacturer_test_name` cell; an empty string stays explicit and pipe values stay whole.
    pub manufacturer_test_name: Box<str>,
    /// Native `test_development` cell; an empty string stays explicit and pipe values stay whole.
    pub test_development: Box<str>,
    /// Native `lab_id` cell; an empty string stays explicit and pipe values stay whole.
    pub lab_id: Box<str>,
    /// Native `condition_identifiers` cell; an empty string stays explicit and pipe values stay whole.
    pub condition_identifiers: Box<str>,
    /// Native `indication_types` cell; an empty string stays explicit and pipe values stay whole.
    pub indication_types: Box<str>,
    /// Native `inheritances` cell; an empty string stays explicit and pipe values stay whole.
    pub inheritances: Box<str>,
    /// Native `method_categories` cell; an empty string stays explicit and pipe values stay whole.
    pub method_categories: Box<str>,
    /// Native `methods` cell; an empty string stays explicit and pipe values stay whole.
    pub methods: Box<str>,
    /// Native `platforms` cell; an empty string stays explicit and pipe values stay whole.
    pub platforms: Box<str>,
    /// Native `genes` cell; an empty string stays explicit and pipe values stay whole.
    pub genes: Box<str>,
    /// Native `drug_responses` cell; an empty string stays explicit and pipe values stay whole.
    pub drug_responses: Box<str>,
    /// Native `now_current` cell; an empty string stays explicit and pipe values stay whole.
    pub now_current: Box<str>,
    /// Native `test_currStat` cell; an empty string stays explicit and pipe values stay whole.
    #[serde(rename = "test_currStat")]
    pub test_current: Box<str>,
    /// Native `test_pubStat` cell; an empty string stays explicit and pipe values stay whole.
    #[serde(rename = "test_pubStat")]
    pub test_public: Box<str>,
    /// Native `lab_currStat` cell; an empty string stays explicit and pipe values stay whole.
    #[serde(rename = "lab_currStat")]
    pub laboratory_current: Box<str>,
    /// Native `lab_pubStat` cell; an empty string stays explicit and pipe values stay whole.
    #[serde(rename = "lab_pubStat")]
    pub laboratory_public: Box<str>,
    /// Native `test_create_date` cell; an empty string stays explicit and pipe values stay whole.
    pub test_create_date: Box<str>,
    /// Native `test_deletion_date` cell; an empty string stays explicit and pipe values stay whole.
    pub test_deletion_date: Box<str>,
    /// Native `test_order_code` cell; an empty string stays explicit and pipe values stay whole.
    pub test_order_code: Box<str>,
}
crate::wire_enum! {
    /// Native GTR relation test category, not an offering or authorization judgment.
    GtrTestType { Clinical => "Clinical", Research => "Research", ClinicalMicrobial => "Clinical microbial" }
}
/// Native relation membership; methods preserve independent lists rather than guessed pairs.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum GtrTestRelation {
    /// Native gene row; numeric source ID may be absent.
    Gene {
        /// Complete owning test identity.
        test_id: GtrTestId,
        /// Native test type.
        test_type: GtrTestType,
        /// Shared source-given gene reference.
        gene: GeneRef,
        /// Native GTR MedGen identity, independent of GeneID.
        native_medgen: Curie,
        /// Native MIM cell, including empty.
        mim_number: Box<str>,
        /// Complete native object label.
        object_name: Box<str>,
    },
    /// Native condition row; its SNOMED value never becomes a gene ID.
    Condition {
        /// Complete owning test identity.
        test_id: GtrTestId,
        /// Native test type.
        test_type: GtrTestType,
        /// Source MedGen concept with native label.
        condition: Concept,
        /// Native MIM cell, including empty.
        mim_number: Box<str>,
        /// Native SNOMED CT cell, including empty; never GeneID.
        snomed_ct_id: Box<str>,
    },
    /// One method-list entry owned by a test version.
    Method {
        /// Complete owning test identity.
        test_id: GtrTestId,
        /// Native independent list, never paired by position.
        list: GtrMethodList,
        /// Zero-based position within that native list.
        index: u32,
        /// Full source entry without clipping.
        value: Box<str>,
    },
}
crate::wire_enum! {
    /// Independent native GTR method lists.
    GtrMethodList { Categories => "method_categories", Methods => "methods", Platforms => "platforms" }
}
