// The future shared notation crate replaces this private reader-only parser.
use biodata::Text;
use std::str::FromStr;
const AMINO: &[(&str, &str)] = &[
    ("A", "Ala"),
    ("R", "Arg"),
    ("N", "Asn"),
    ("D", "Asp"),
    ("C", "Cys"),
    ("E", "Glu"),
    ("Q", "Gln"),
    ("G", "Gly"),
    ("H", "His"),
    ("I", "Ile"),
    ("L", "Leu"),
    ("K", "Lys"),
    ("M", "Met"),
    ("F", "Phe"),
    ("P", "Pro"),
    ("S", "Ser"),
    ("T", "Thr"),
    ("W", "Trp"),
    ("Y", "Tyr"),
    ("V", "Val"),
    ("U", "Sec"),
    ("O", "Pyl"),
    ("*", "Ter"),
];
/// Explicit protein substitution with checked residue codes and a positive position.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct ProteinChange {
    from: usize,
    position: u32,
    to: usize,
}
impl FromStr for ProteinChange {
    type Err = ();
    fn from_str(value: &str) -> Result<Self, ()> {
        let value = value.strip_prefix("p.").unwrap_or(value);
        let first = value
            .find(|c: char| c.is_ascii_digit())
            .ok_or(())?;
        let end = value
            .rfind(|c: char| c.is_ascii_digit())
            .ok_or(())?
            + 1;
        let residue = |v: &str| {
            AMINO
                .iter()
                .position(|(one, three)| v == *one || v == *three)
                .ok_or(())
        };
        let digits = value.get(first..end).ok_or(())?;
        if digits.starts_with('0') || !digits.bytes().all(|c| c.is_ascii_digit()) {
            return Err(());
        }
        let position = digits.parse().map_err(|_| ())?;
        Ok(Self {
            from: residue(value.get(..first).ok_or(())?)?,
            position,
            to: residue(value.get(end..).ok_or(())?)?,
        })
    }
}
impl ProteinChange {
    /// Literal spellings searched by readers, in stable order.
    pub(super) fn forms(&self) -> [String; 4] {
        let (from, from_three) = AMINO.get(self.from).copied().unwrap_or(("", ""));
        let (to, to_three) = AMINO.get(self.to).copied().unwrap_or(("", ""));
        let one = format!("{from}{}{to}", self.position);
        let three = format!("{from_three}{}{to_three}", self.position);
        [
            one.clone(),
            three.clone(),
            format!("p.{one}"),
            format!("p.{three}"),
        ]
    }
}
#[derive(Debug)]
pub(super) struct Selection {
    pub(super) forms: Option<[String; 4]>,
    pub(super) needles: Vec<String>,
}
impl Selection {
    pub(super) fn new(find: Option<&Text>, variant: Option<&Text>) -> Result<Self, ()> {
        let forms = variant.map(|v| v.as_str().parse::<ProteinChange>().map(|p| p.forms())).transpose()?;
        let needles = forms.clone().map(|v| v.to_vec()).unwrap_or_else(|| find.map(|v| vec![v.as_str().to_owned()]).unwrap_or_default());
        Ok(Self { forms, needles })
    }
}
