use super::NativeRow;
use std::collections::BTreeMap;
/// An original native row's location in one published partition.
#[derive(Clone, Debug)]
pub struct Provenance {
    /// Native partition code, independent of local file paths.
    pub part: Box<str>,
    /// One-based logical CSV row, including the header.
    pub row: u64,
}
/// A checked row and its position in the complete replacement set.
#[derive(Clone, Debug)]
pub struct PartRow {
    /// Native partition code.
    pub part: Box<str>,
    /// One-based logical CSV row, including the header.
    pub row: u64,
    /// Original checked cells.
    pub value: NativeRow,
}
/// One native pair oriented to the selected source ID.
#[derive(Debug)]
pub struct Pair {
    /// The first complete native pair row, without merged wording.
    pub native: NativeRow,
    /// Every original row location contributing this same source pair.
    pub provenance: Vec<Provenance>,
    reversed: bool,
}
impl Pair {
    /// Exact native partner ID, without a name join.
    pub fn partner_id(&self) -> &str {
        if self.reversed {
            self.native.id_a()
        } else {
            self.native.id_b()
        }
    }
    /// Native partner display name from the retained row.
    pub fn partner_name(&self) -> &str {
        if self.reversed {
            self.native.name_a()
        } else {
            self.native.name_b()
        }
    }
    /// Native anchor name from the retained row.
    pub fn anchor_name(&self) -> &str {
        if self.reversed {
            self.native.name_b()
        } else {
            self.native.name_a()
        }
    }
}
/// Select exact source partners in partition/row order, retaining every duplicate origin.
pub fn partners(mut rows: Vec<PartRow>, anchor: &str) -> Vec<Pair> {
    rows.sort_by(|a, b| (&a.part, a.row).cmp(&(&b.part, b.row)));
    let mut pairs = BTreeMap::<String, Pair>::new();
    let mut order = Vec::new();
    for row in rows {
        let reversed = row.value.id_b() == anchor;
        if row.value.id_a() != anchor && !reversed {
            continue;
        }
        let partner = if reversed {
            row.value.id_a()
        } else {
            row.value.id_b()
        };
        let origin = Provenance {
            part: row.part,
            row: row.row,
        };
        if let Some(pair) = pairs.get_mut(partner) {
            pair.provenance.push(origin);
        } else {
            order.push(partner.to_owned());
            pairs.insert(
                partner.to_owned(),
                Pair {
                    native: row.value,
                    provenance: vec![origin],
                    reversed,
                },
            );
        }
    }
    order
        .into_iter()
        .filter_map(|id| pairs.remove(&id))
        .collect()
}
