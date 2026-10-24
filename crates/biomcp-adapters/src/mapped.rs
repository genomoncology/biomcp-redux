use biomcp_types::Problem;

/// A source value with rejected fields retained as problems.
#[derive(Debug)]
pub struct Mapped<T> {
    /// Accepted mapped value.
    pub value: T,
    /// Fields that could not be accepted.
    pub problems: Vec<Problem>,
}
