//! 0942: inner pages and bounded collections state their page or total before their rows.
use super::Outcome;
use serde_json::json;

mod key_order {
    use serde::ser::{Impossible, Serialize, SerializeStruct, Serializer};
    type Error = serde_json::Error;
    fn none<T>() -> Result<T, Error> {
        Err(serde::ser::Error::custom("not a struct"))
    }
    #[derive(Debug)]
    pub struct Keys(pub Vec<&'static str>);
    impl SerializeStruct for &mut Keys {
        type Ok = ();
        type Error = Error;
        fn serialize_field<T: ?Sized + Serialize>(
            &mut self,
            key: &'static str,
            _: &T,
        ) -> Result<(), Error> {
            self.0.push(key);
            Ok(())
        }
        fn skip_field(&mut self, key: &'static str) -> Result<(), Error> {
            self.0.push(key);
            Ok(())
        }
        fn end(self) -> Result<(), Error> {
            Ok(())
        }
    }
    macro_rules! refuse {
        ($($name:ident($($arg:ty),*)),*) => {$(fn $name(self, $(_: $arg),*) -> Result<(), Error> { none() })*};
    }
    impl<'a> Serializer for &'a mut Keys {
        type Ok = ();
        type Error = Error;
        type SerializeSeq = Impossible<(), Error>;
        type SerializeTuple = Impossible<(), Error>;
        type SerializeTupleStruct = Impossible<(), Error>;
        type SerializeTupleVariant = Impossible<(), Error>;
        type SerializeMap = Impossible<(), Error>;
        type SerializeStruct = &'a mut Keys;
        type SerializeStructVariant = Impossible<(), Error>;
        refuse!(
            serialize_bool(bool),
            serialize_i8(i8),
            serialize_i16(i16),
            serialize_i32(i32),
            serialize_i64(i64),
            serialize_u8(u8),
            serialize_u16(u16),
            serialize_u32(u32),
            serialize_u64(u64),
            serialize_f32(f32),
            serialize_f64(f64),
            serialize_char(char),
            serialize_str(&str),
            serialize_bytes(&[u8]),
            serialize_none(),
            serialize_unit(),
            serialize_unit_struct(&'static str),
            serialize_unit_variant(&'static str, u32, &'static str)
        );
        fn serialize_some<T: ?Sized + Serialize>(self, _: &T) -> Result<(), Error> {
            none()
        }
        fn serialize_newtype_struct<T: ?Sized + Serialize>(
            self,
            _: &'static str,
            _: &T,
        ) -> Result<(), Error> {
            none()
        }
        fn serialize_newtype_variant<T: ?Sized + Serialize>(
            self,
            _: &'static str,
            _: u32,
            _: &'static str,
            _: &T,
        ) -> Result<(), Error> {
            none()
        }
        fn serialize_seq(self, _: Option<usize>) -> Result<Self::SerializeSeq, Error> {
            none()
        }
        fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple, Error> {
            none()
        }
        fn serialize_tuple_struct(
            self,
            _: &'static str,
            _: usize,
        ) -> Result<Self::SerializeTupleStruct, Error> {
            none()
        }
        fn serialize_tuple_variant(
            self,
            _: &'static str,
            _: u32,
            _: &'static str,
            _: usize,
        ) -> Result<Self::SerializeTupleVariant, Error> {
            none()
        }
        fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap, Error> {
            none()
        }
        fn serialize_struct(
            self,
            _: &'static str,
            _: usize,
        ) -> Result<Self::SerializeStruct, Error> {
            Ok(self)
        }
        fn serialize_struct_variant(
            self,
            _: &'static str,
            _: u32,
            _: &'static str,
            _: usize,
        ) -> Result<Self::SerializeStructVariant, Error> {
            none()
        }
    }
}

/// Decode synthetic wire JSON and require each `first` key declared before `rows`.
fn declared_first<T: serde::Serialize + serde::de::DeserializeOwned>(
    wire: serde_json::Value,
    first: &[&str],
    rows: &str,
) -> Outcome {
    let value: T = serde_json::from_value(wire)?;
    let mut keys = key_order::Keys(Vec::new());
    value.serialize(&mut keys)?;
    let at = |key: &str| {
        keys.0
            .iter()
            .position(|k| *k == key)
            .ok_or(format!("{key} missing: {:?}", keys.0))
    };
    let rows = at(rows)?;
    for key in first {
        assert!(at(key)? < rows, "{key} follows {:?}", keys.0);
    }
    Ok(())
}
fn page() -> serde_json::Value {
    json!({"offset":0,"returned":0,"total":0})
}
fn pathway_page() -> serde_json::Value {
    json!({"offset":0,"returned":0,"total":0,"total_kind":"source"})
}
fn pathway_header() -> serde_json::Value {
    json!({"id":{"namespace":"reactome","id":"R-HSA-5673001"},"name":"synthetic pathway","species":"Homo sapiens"})
}

#[test]
fn variant_pages_state_their_page_before_their_records() -> Outcome {
    use biomcp_types::{VariantHotspots, VariantOncokb};
    declared_first::<VariantHotspots>(json!({"page":page()}), &["page"], "records")?;
    declared_first::<VariantOncokb>(
        json!({"hgvsg":"7:g.140453136A>T","build":"GRCh37","tumor_type":"Melanoma","page":page()}),
        &["page"],
        "records",
    )?;
    declared_first::<biomcp_types::AlphaGenomeScorerPage>(
        json!({"request_ordinal":0,"requested_scorer":"gene-mask-lfc-rna-seq","data_type":"DATA_TYPE_FLOAT32",
        "shape":[1,1,1],"page":page()}),
        &["page"],
        "rows",
    )
}
#[test]
fn study_pages_state_their_page_before_their_items() -> Outcome {
    use biomcp_types::{StudyFiles, StudyPage, StudyProfiles, StudySampleLists, StudySamples};
    let study = "brca_tcga_pan_can_atlas_2018";
    let snapshot = json!({"revision":"synthetic","acquired":1793059200});
    declared_first::<StudyPage>(
        json!({"page":page(),"snapshot":snapshot}),
        &["page"],
        "items",
    )?;
    let scope = json!({"study_id":study,"page":page()});
    declared_first::<StudyProfiles>(scope.clone(), &["page"], "items")?;
    declared_first::<StudySampleLists>(scope.clone(), &["page"], "items")?;
    declared_first::<StudyFiles>(scope.clone(), &["page"], "items")?;
    declared_first::<StudySamples>(scope, &["page"], "items")
}
#[test]
fn gwas_pages_state_their_page_before_their_items() -> Outcome {
    declared_first::<biomcp_types::GwasPage>(
        json!({"page":page(),"qualifying_total":3}),
        &["page", "qualifying_total"],
        "items",
    )
}
#[test]
fn shared_and_pathway_pages_state_their_totals_before_their_items() -> Outcome {
    use biomcp_types::{DiagnosticGenes, PathwayEvents, PathwayGenes, PathwayPage};
    declared_first::<PathwayEvents>(
        json!({"header":pathway_header(),"page":pathway_page()}),
        &["page"],
        "items",
    )?;
    declared_first::<PathwayPage>(json!({"page":pathway_page()}), &["page"], "items")?;
    declared_first::<PathwayGenes>(
        json!({"header":pathway_header(),"page":pathway_page(),"reference_occurrences":0,"physical_entities":0}),
        &["page", "reference_occurrences", "physical_entities"],
        "items",
    )?;
    declared_first::<DiagnosticGenes>(
        json!({"distinct_id_total":2}),
        &["count_unit", "distinct_id_total"],
        "items",
    )
}
#[test]
fn other_bounded_collections_state_their_totals_before_their_rows() -> Outcome {
    use biomcp_types::{Capped, GenePage, TrialRecords};
    use biomcp_types::{ProteinComplexes, ProteinDomains};
    declared_first::<Capped<serde_json::Value>>(json!({"total":0}), &["total"], "items")?;
    declared_first::<TrialRecords<serde_json::Value>>(
        json!({"offset":0,"total":0}),
        &["offset", "total"],
        "items",
    )?;
    declared_first::<GenePage>(json!({"regions_total":60}), &["regions_total"], "regions")?;
    declared_first::<ProteinDomains>(json!({"native_total":0}), &["native_total"], "items")?;
    declared_first::<ProteinComplexes>(json!({"native_total":0}), &["native_total"], "items")
}
