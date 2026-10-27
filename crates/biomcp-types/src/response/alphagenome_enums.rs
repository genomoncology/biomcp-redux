//! Preserve publisher enum names in AlphaGenome metadata.
biodata::wire_enum! {
    /// Publisher Strand vocabulary.
    AlphaGenomeStrand {
        /// Publisher STRAND_UNSPECIFIED.
        Unspecified => "STRAND_UNSPECIFIED",
        /// Publisher STRAND_POSITIVE.
        Positive => "STRAND_POSITIVE",
        /// Publisher STRAND_NEGATIVE.
        Negative => "STRAND_NEGATIVE",
        /// Publisher STRAND_UNSTRANDED.
        Unstranded => "STRAND_UNSTRANDED",
    }
}
biodata::wire_enum! {
    /// Publisher OntologyType vocabulary.
    AlphaGenomeOntologyType {
        /// Publisher ONTOLOGY_TYPE_UNSPECIFIED.
        Unspecified => "ONTOLOGY_TYPE_UNSPECIFIED",
        /// Publisher ONTOLOGY_TYPE_CLO.
        Clo => "ONTOLOGY_TYPE_CLO",
        /// Publisher ONTOLOGY_TYPE_UBERON.
        Uberon => "ONTOLOGY_TYPE_UBERON",
        /// Publisher ONTOLOGY_TYPE_CL.
        Cl => "ONTOLOGY_TYPE_CL",
        /// Publisher ONTOLOGY_TYPE_EFO.
        Efo => "ONTOLOGY_TYPE_EFO",
        /// Publisher ONTOLOGY_TYPE_NTR.
        Ntr => "ONTOLOGY_TYPE_NTR",
    }
}
biodata::wire_enum! {
    /// Publisher BiosampleType vocabulary.
    AlphaGenomeBiosampleType {
        /// Publisher BIOSAMPLE_TYPE_UNSPECIFIED.
        Unspecified => "BIOSAMPLE_TYPE_UNSPECIFIED",
        /// Publisher BIOSAMPLE_TYPE_PRIMARY_CELL.
        PrimaryCell => "BIOSAMPLE_TYPE_PRIMARY_CELL",
        /// Publisher BIOSAMPLE_TYPE_IN_VITRO_DIFFERENTIATED_CELLS.
        InVitroDifferentiatedCells => "BIOSAMPLE_TYPE_IN_VITRO_DIFFERENTIATED_CELLS",
        /// Publisher BIOSAMPLE_TYPE_CELL_LINE.
        CellLine => "BIOSAMPLE_TYPE_CELL_LINE",
        /// Publisher BIOSAMPLE_TYPE_TISSUE.
        Tissue => "BIOSAMPLE_TYPE_TISSUE",
        /// Publisher BIOSAMPLE_TYPE_TECHNICAL_SAMPLE.
        TechnicalSample => "BIOSAMPLE_TYPE_TECHNICAL_SAMPLE",
        /// Publisher BIOSAMPLE_TYPE_ORGANOID.
        Organoid => "BIOSAMPLE_TYPE_ORGANOID",
    }
}
biodata::wire_enum! {
    /// Publisher Endedness vocabulary.
    AlphaGenomeEndedness {
        /// Publisher ENDEDNESS_UNSPECIFIED.
        Unspecified => "ENDEDNESS_UNSPECIFIED",
        /// Publisher ENDEDNESS_SINGLE.
        Single => "ENDEDNESS_SINGLE",
        /// Publisher ENDEDNESS_PAIRED.
        Paired => "ENDEDNESS_PAIRED",
    }
}
