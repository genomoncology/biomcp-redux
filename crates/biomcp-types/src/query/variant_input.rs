use crate::{Error, text_value};
use std::{borrow::Cow, fmt, str::FromStr};
/// Published rsID, CAid, clinvar:N, VCV accession, reference-qualified HGVS,
/// chromosome genomic HGVS, or VCF-style chromosome-position-reference-alternate coordinates.
/// Ambiguous candidates remain separate.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VariantInput {
    text: crate::Text,
    kind: InputKind,
}
#[derive(Clone, Debug, PartialEq, Eq)]
enum InputKind {
    Id(crate::VariantId),
    ClinvarAssertion(crate::AssertionId),
    Hgvs(crate::HgvsText),
    Chromosome,
    Coordinates { hgvs: String, single_base: bool },
    Protein(Box<(crate::GeneSymbol, super::ProteinQuery)>),
    CivicProfile,
}
/// Namespace selected solely from the published identifier form.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VariantNamespace {
    /// dbSNP reference SNP identifier.
    Rs,
    /// ClinGen canonical allele identifier.
    Ca,
    /// ClinVar numeric Variation ID.
    ClinVar,
    /// Reference-qualified opaque HGVS description.
    Hgvs,
    /// Chromosome HGVS description requiring an explicit build.
    Chromosome,
    /// Checked gene and protein substitution, admitted only by variant get.
    Protein,
    /// Native CIViC molecular-profile ID or exact name.
    CivicProfile,
    /// Identifier vocabulary is recognized but this get slice has no source route.
    Unsupported,
}
impl VariantInput {
    pub(crate) const COORDINATE_PATTERN: &'static str =
        r"^(chr)?([1-9]|1[0-9]|2[0-2]|X|Y|M|MT)-[1-9][0-9]*-[ACGT]+-[ACGT]+$";
    pub(crate) const CHROMOSOME_PATTERN: &'static str =
        r"^chr([1-9]|1[0-9]|2[0-2]|X|Y|M|MT):g\.[\x21-\x7e]*[0-9][\x21-\x7e]*$";

    /// Maximum bytes in one published identifier.
    pub const MAX_BYTES: usize = 2048;
    /// Resolver forms shared by schema descriptions and recurrence guidance.
    pub const FORMS: &str = concat!(
        "Accepted forms: rsID, CAid, clinvar:N, VCV accession, reference-qualified HGVS, ",
        "chromosome genomic HGVS, or VCF-style chromosome-position-reference-alternate coordinates"
    );
    /// Shared assembly and reference-validation guidance for variant input schemas.
    pub const ASSEMBLY: &str = "defaulting to GRCh38. Reference bases are validated by the source.";
    /// Check the identifier spelling and canonicalize the ClinVar namespace prefix.
    pub fn new(value: &str) -> Result<Self, Error> {
        let value = if value
            .get(..8)
            .is_some_and(|prefix| prefix.eq_ignore_ascii_case("clinvar:"))
        {
            Cow::Owned(format!("clinvar:{}", &value[8..]))
        } else {
            Cow::Borrowed(value)
        };
        let text = input_text(value.as_ref())?;
        let kind = if let Ok(id) = value.parse::<crate::VariantId>() {
            InputKind::Id(id)
        } else if positive(&value) && value.parse::<u64>().is_ok() {
            InputKind::Id(
                format!("clinvar:{value}")
                    .parse()
                    .map_err(|_| Error::VariantInputInvalid)?,
            )
        } else if let Some(coordinates) = coordinates(value.as_ref()) {
            coordinates
        } else if let Ok(hgvs) = value.parse::<crate::HgvsText>() {
            InputKind::Hgvs(hgvs)
        } else if crate::Expression::new(crate::ExpressionSyntax::HgvsG, text.clone(), None).is_ok()
        {
            InputKind::Chromosome
        } else {
            return Err(Error::VariantInputInvalid);
        };
        Ok(Self { text, kind })
    }
    pub(super) fn lookup(value: &str) -> Result<Self, Error> {
        let assertion = value
            .parse::<crate::RcvId>()
            .map(crate::AssertionId::Rcv)
            .or_else(|_| value.parse::<crate::ScvId>().map(crate::AssertionId::Scv));
        if let Ok(assertion) = assertion {
            return Ok(Self {
                text: input_text(value)?,
                kind: InputKind::ClinvarAssertion(assertion),
            });
        }
        if let Some(pair) = super::variant_search::parse_shorthand(value) {
            return Ok(Self {
                text: crate::Text::new(value).map_err(|_| Error::VariantInputInvalid)?,
                kind: InputKind::Protein(Box::new(pair)),
            });
        }
        Self::new(value).or_else(|_| Self::civic_profile(value))
    }

    fn civic_profile(value: &str) -> Result<Self, Error> {
        let text = crate::Text::new(value).map_err(|_| Error::VariantInputInvalid)?;
        if text.as_str() != value || value.len() > Self::MAX_BYTES {
            return Err(Error::VariantInputInvalid);
        }
        Ok(Self {
            text,
            kind: InputKind::CivicProfile,
        })
    }

    pub(super) fn for_get(mut self, civic_only: bool) -> Result<Self, Error> {
        let numeric = self.as_str().bytes().all(|b| b.is_ascii_digit());
        if civic_only && (numeric || self.protein().is_some()) {
            if numeric && self.as_str().parse::<i32>().ok().is_none_or(|id| id <= 0) {
                return Err(Error::VariantInputInvalid);
            }
            self.kind = InputKind::CivicProfile;
        }
        if !civic_only && matches!(self.kind, InputKind::CivicProfile) {
            return Err(Error::VariantInputInvalid);
        }
        Ok(self)
    }

    pub(super) fn protein(&self) -> Option<&(crate::GeneSymbol, super::ProteinQuery)> {
        match &self.kind {
            InputKind::Protein(pair) => Some(pair),
            _ => None,
        }
    }
    /// Preserve a parsed candidate identity and apply the input text bounds once.
    pub fn from_identity(id: crate::VariantId) -> Result<Self, Error> {
        let text = input_text(&id.to_string())?;
        Ok(Self {
            text,
            kind: InputKind::Id(id),
        })
    }
    /// Coordinates use GRCh38 unless the caller supplies another assembly.
    pub fn coordinate_form(&self) -> bool {
        matches!(self.kind, InputKind::Coordinates { .. })
    }
    /// A VCF-style coordinate with one reference base and one alternate base.
    pub fn single_base_coordinate(&self) -> bool {
        matches!(
            self.kind,
            InputKind::Coordinates {
                single_base: true,
                ..
            }
        )
    }
    /// Full VCF REF/ALT for source validation, before shared padding is removed.
    pub fn coordinate_validation(&self, build: crate::Build) -> Option<String> {
        if !matches!(
            self.kind,
            InputKind::Coordinates {
                single_base: false,
                ..
            }
        ) {
            return None;
        }
        let parts: Vec<_> = self.as_str().split('-').collect();
        let [chromosome, position, reference, alternate] = parts.as_slice() else {
            return None;
        };
        let chromosome = chromosome.strip_prefix("chr").unwrap_or(chromosome);
        let chromosome: biodata::Chromosome = (if chromosome == "M" { "MT" } else { chromosome })
            .parse()
            .ok()?;
        let start: u64 = position.parse().ok()?;
        let end = start.checked_add(reference.len() as u64 - 1)?;
        let range = if start == end {
            start.to_string()
        } else {
            format!("{start}_{end}")
        };
        Some(format!(
            "{}:g.{range}del{reference}ins{alternate}",
            chromosome.accession(build)
        ))
    }
    /// Source lookup identity; VCV archive versions do not change the Variation ID.
    pub fn source_id(&self) -> Cow<'_, str> {
        if let Some(crate::VariantId::Vcv(id)) = self.identity() {
            let value = id.to_string();
            let digits = value
                .strip_prefix("VCV")
                .unwrap_or_default()
                .split('.')
                .next()
                .unwrap_or_default();
            return Cow::Owned(format!("clinvar:{}", digits.trim_start_matches('0')));
        }
        if let Some(id @ crate::VariantId::ClinVarVariation(_)) = self.identity() {
            return Cow::Owned(id.to_string());
        }
        self.native_hgvs()
    }
    /// Return the checked identifier with a canonical ClinVar prefix.
    pub fn as_str(&self) -> &str {
        self.text.as_str()
    }
    /// Borrow the already checked input text without reparsing or trimming it.
    pub fn text(&self) -> &crate::Text {
        &self.text
    }
    /// Checked parenthetical gene and its exact reference accession, when supplied.
    pub fn reference_gene(&self) -> Option<(&str, &str)> {
        let (reference, _) = self.as_str().split_once(':')?;
        let (accession, gene) = reference.split_once('(')?;
        let gene = gene.strip_suffix(')')?;
        if gene.parse::<biodata::Accession>().is_ok() || gene.parse::<crate::GeneSymbol>().is_err()
        {
            return None;
        }
        Some((accession, gene))
    }
    /// Native resolver input omits a checked parenthetical gene label.
    pub fn native_hgvs(&self) -> Cow<'_, str> {
        if let InputKind::Coordinates { hgvs, .. } = &self.kind {
            return Cow::Borrowed(hgvs);
        }
        if let Some(body) = self.as_str().strip_prefix("chrM:") {
            return Cow::Owned(format!("chrMT:{body}"));
        }
        if let Some((accession, _)) = self.reference_gene()
            && let Some((_, body)) = self.as_str().split_once(':')
        {
            Cow::Owned(format!("{accession}:{body}"))
        } else {
            Cow::Borrowed(self.as_str())
        }
    }
    /// Borrow the checked reference-qualified notation.
    pub fn hgvs(&self) -> Option<&crate::HgvsText> {
        match &self.kind {
            InputKind::Hgvs(value) => Some(value),
            _ => None,
        }
    }
    /// Retain the source identity parsed at the input boundary.
    pub fn identity(&self) -> Option<&crate::VariantId> {
        match &self.kind {
            InputKind::Id(id) => Some(id),
            _ => None,
        }
    }
    /// Accessions that need a checked ClinVar archive rather than an allele resolver.
    pub fn clinvar_archive_lookup(&self) -> bool {
        self.clinvar_assertion().is_some() || self.versioned_vcv().is_some()
    }
    /// An RCV or SCV accession admitted by variant get.
    pub fn clinvar_assertion(&self) -> Option<&crate::AssertionId> {
        match &self.kind {
            InputKind::ClinvarAssertion(id) => Some(id),
            _ => None,
        }
    }
    /// A VCV accession with a caller-specified archive version.
    pub fn versioned_vcv(&self) -> Option<crate::VcvId> {
        match self.identity() {
            Some(crate::VariantId::Vcv(id)) if id.version().is_some() => Some(*id),
            _ => None,
        }
    }
    /// Classify a published namespace, without parsing molecular meaning.
    pub fn namespace(&self) -> VariantNamespace {
        match &self.kind {
            InputKind::Id(crate::VariantId::Rs(_)) => VariantNamespace::Rs,
            InputKind::Id(crate::VariantId::Ca(_)) => VariantNamespace::Ca,
            InputKind::Id(crate::VariantId::ClinVarVariation(_) | crate::VariantId::Vcv(_)) => {
                VariantNamespace::ClinVar
            }
            InputKind::Id(_) | InputKind::ClinvarAssertion(_) => VariantNamespace::Unsupported,
            InputKind::Hgvs(_) => VariantNamespace::Hgvs,
            InputKind::Protein(_) => VariantNamespace::Protein,
            InputKind::CivicProfile => VariantNamespace::CivicProfile,
            InputKind::Chromosome | InputKind::Coordinates { .. } => VariantNamespace::Chromosome,
        }
    }
}
fn coordinates(value: &str) -> Option<InputKind> {
    let mut parts = value.split('-');
    let chromosome = parts
        .next()?
        .strip_prefix("chr")
        .unwrap_or(value.split('-').next()?);
    let chromosome = if chromosome == "M" { "MT" } else { chromosome };
    let _: biodata::Chromosome = chromosome.parse().ok()?;
    let position = parts.next()?;
    let start: u64 = position.parse().ok()?;
    let reference = parts.next()?;
    let alternate = parts.next()?;
    if parts.next().is_some()
        || position.starts_with('0')
        || start == 0
        || reference.is_empty()
        || alternate.is_empty()
        || !reference
            .bytes()
            .chain(alternate.bytes())
            .all(|b| b"ACGT".contains(&b))
    {
        return None;
    }
    let single_base = reference.len() == 1 && alternate.len() == 1;
    let change = if single_base {
        format!("{start}{reference}>{alternate}")
    } else {
        coordinate_change(start, reference, alternate)?
    };
    let expression = format!("chr{chromosome}:g.{change}");
    crate::Expression::new(
        crate::ExpressionSyntax::HgvsG,
        crate::Text::new(&expression).ok()?,
        None,
    )
    .ok()?;
    Some(InputKind::Coordinates {
        hgvs: expression,
        single_base,
    })
}
// VCF 4.4 REF/ALT padding and HGVS DNA deletion/insertion/delins grammar.
// Source validation retains the original REF, including shared bases.
fn coordinate_change(position: u64, reference: &str, alternate: &str) -> Option<String> {
    let original_reference = reference;
    let original_alternate = alternate;
    let mut start = position;
    let mut reference = reference;
    let mut alternate = alternate;
    while !reference.is_empty()
        && !alternate.is_empty()
        && reference.as_bytes().first() == alternate.as_bytes().first()
    {
        reference = &reference[1..];
        alternate = &alternate[1..];
        start = start.checked_add(1)?;
    }
    while !reference.is_empty()
        && !alternate.is_empty()
        && reference.as_bytes().last() == alternate.as_bytes().last()
    {
        reference = &reference[..reference.len() - 1];
        alternate = &alternate[..alternate.len() - 1];
    }
    let end = start.checked_add(reference.len().saturating_sub(1) as u64)?;
    let range = if start == end {
        start.to_string()
    } else {
        format!("{start}_{end}")
    };
    Some(match (reference.is_empty(), alternate.is_empty()) {
        (true, true) => return None,
        // HGVS has no position zero: keep right padding at the contig start.
        (true, false) if start == 1 => {
            let end = original_reference.len();
            let range = if end == 1 {
                "1".to_owned()
            } else {
                format!("1_{end}")
            };
            format!("{range}delins{original_alternate}")
        }
        (true, false) => format!("{}_{}ins{alternate}", start.checked_sub(1)?, start),
        (false, true) => format!("{range}del"),
        (false, false) if reference.len() == 1 && alternate.len() == 1 => {
            format!("{start}{reference}>{alternate}")
        }
        _ => format!("{range}delins{alternate}"),
    })
}
fn input_text(value: &str) -> Result<crate::Text, Error> {
    if value.is_empty()
        || value.len() > VariantInput::MAX_BYTES
        || !value.bytes().all(|byte| byte.is_ascii_graphic())
    {
        return Err(Error::VariantInputInvalid);
    }
    crate::Text::new(value).map_err(|_| Error::VariantInputInvalid)
}
fn positive(value: &str) -> bool {
    !value.is_empty() && !value.starts_with('0') && value.bytes().all(|b| b.is_ascii_digit())
}
impl FromStr for VariantInput {
    type Err = Error;
    fn from_str(value: &str) -> Result<Self, Error> {
        Self::new(value)
    }
}
impl fmt::Display for VariantInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
text_value!(
    VariantInput,
    "VariantInput",
    Self::schema_pattern(
        crate::VariantId::pattern(),
        crate::HgvsText::pattern(),
        biodata::ClinVarVariationId::pattern()
    ),
    format!(
        "{} (SNVs, deletions, insertions and delins), {}",
        Self::FORMS,
        Self::ASSEMBLY
    )
);

impl VariantInput {
    /// Combine source-owned schema domains with the shared spelling and length bounds.
    pub fn schema_pattern(identifiers: String, hgvs: String, variation: String) -> String {
        format!(
            "^(?=[\\x21-\\x7e]{{1,{}}}$)({}|{}|{}|{}|{})",
            Self::MAX_BYTES,
            identifiers.replace("clinvar:", "[cC][lL][iI][nN][vV][aA][rR]:"),
            hgvs,
            variation.replace("clinvar:", ""),
            Self::CHROMOSOME_PATTERN,
            Self::COORDINATE_PATTERN
        )
    }
}
/// Command input domains routed by the shared variant resolver. Source identities retain their full vocabulary.
#[cfg(feature = "schema")]
pub(super) struct LookupInput;
#[cfg(feature = "schema")]
impl schemars::JsonSchema for LookupInput {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "VariantLookupInput".into()
    }
    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        let mut schema = VariantInput::json_schema(generator);
        schema.insert(
            "pattern".into(),
            VariantInput::schema_pattern(
                format!(
                    "({}|{}|{}|{})",
                    biodata::RsId::pattern(),
                    biodata::CaId::pattern(),
                    biodata::ClinVarVariationId::pattern(),
                    biodata::VcvId::pattern()
                ),
                crate::HgvsText::pattern(),
                biodata::ClinVarVariationId::pattern(),
            )
            .into(),
        );
        schema
    }
}

/// Variant get also accepts exact source profile names containing spaces.
#[cfg(feature = "schema")]
pub(super) struct GetLookupInput;
#[cfg(feature = "schema")]
impl schemars::JsonSchema for GetLookupInput {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "VariantGetLookupInput".into()
    }
    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({
            "type": "string",
            "pattern": format!(r"^[^\r\n]{{1,{}}}$", VariantInput::MAX_BYTES),
        })
    }
}

/// Recurrence shares routed forms and advertises only its supported allele comparison.
#[cfg(feature = "schema")]
pub(super) struct RecurrenceInput;
#[cfg(feature = "schema")]
impl schemars::JsonSchema for RecurrenceInput {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "RecurrenceInput".into()
    }
    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        let mut schema = LookupInput::json_schema(generator);
        schema.insert(
            "description".into(),
            format!(
                "{}, {} Requires {}.",
                VariantInput::FORMS,
                VariantInput::ASSEMBLY,
                crate::VariantRecurrenceQuery::INPUT_REQUIREMENT
            )
            .into(),
        );
        schema
    }
}
