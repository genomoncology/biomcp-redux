crate::macros::wire_enum! {
    /// Overall command outcome.
    Status {
        /// Every requested source answered.
        Complete => "complete",
        /// Some requested data failed or is stale.
        Partial => "partial",
        /// Every identity was confirmed absent.
        Absent => "absent",
        /// No requested identity could be answered.
        Failed => "failed",
    }
}
/// Outcome for one input.
pub type ItemStatus = Status;
