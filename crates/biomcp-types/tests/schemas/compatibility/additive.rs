//! Saved work is stored and read back by any 1.x build, so its schemas only add optional
//! fields (sdlc/planning/saved-work.md, Compatibility point 3).
use super::Shape;
use crate::policy::{ContractKind, Reviews};

/// The usual check, and for saved work also its reverse.
pub(super) fn compatible(
    saved: bool,
    old: &Shape,
    new: &Shape,
    kind: ContractKind,
    reviews: &Reviews,
    context: (&str, u64, &str),
) -> bool {
    super::compatible(old, new, kind, reviews, context)
        && (!saved || reverse(old, new, reviews, context))
}

/// `new` admits nothing `old` did not, apart from a longer open (`x-extensible-enum`) value
/// list, such as a new source, or a reviewed pattern widening the forward check allowed.
fn reverse(
    old: &Shape,
    new: &Shape,
    reviews: &Reviews,
    context: (&str, u64, &str),
) -> bool {
    let mut back = new.clone();
    back.pattern.clone_from(&old.pattern);
    if old.open_enum && new.open_enum {
        back.values.clone_from(&old.values);
    }
    super::compatible(&back, old, ContractKind::Reply, reviews, context)
}
