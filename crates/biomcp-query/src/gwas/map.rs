//! Keep adapter diagnostics beside mapped associations.
pub(crate) fn association(row: biomcp_adapters::Mapped<biomcp_types::GwasAssociation>, problems: &mut Vec<biomcp_types::Problem>) -> biomcp_types::GwasAssociation {
    problems.extend(row.problems);
    row.value
}
