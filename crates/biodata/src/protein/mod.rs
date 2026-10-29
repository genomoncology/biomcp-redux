//! Neutral source-owned protein records with no I/O or clinical interpretation.
mod id;
pub use id::*;
mod coord;
pub use coord::{Bound, UncertainSpan0};
mod identity;
pub use identity::*;
mod domain;
pub use domain::*;
mod complex;
pub use complex::*;
mod sequence;
pub use sequence::*;
crate::wire_enum! {
    /// Coordinate convention on the native feature before conversion.
    ProteinCoordinateConvention {
        /// Native one-based inclusive residue boundaries.
        OneBasedInclusive => "one-based-inclusive"
    }
}
crate::wire_enum! {
    /// Native UniProt alternative-product sequence status.
    ProteinIsoformStatus {
        /// Native displayed sequence.
        Displayed => "Displayed",
        /// Native described sequence.
        Described => "Described",
        /// Native external entry reference.
        External => "External"
    }
}
crate::wire_enum! {
    /// Direct sequence availability is a retrieval fact, never constructed from another entry.
    ProteinSequenceAvailability {
        /// Exact direct sequence supplied.
        Available => "available",
        /// Source explicitly has no direct sequence.
        NotAvailable => "not-available",
        /// Direct sequence was not retrieved.
        NotChecked => "not-checked"
    }
}
