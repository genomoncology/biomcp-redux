//! CIViC source grammar: https://github.com/griffithlab/civic-v2/blob/main/client/src/app/core/utilities/molecular-profile-parser.ts
//! Source display formatter: https://github.com/griffithlab/civic-v2/blob/main/server/app/tsv_formatters/molecular_profile_tsv_formatter.rb
use crate::{Curie, Error, Text, VariantRef};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

/// Source-stated Boolean relations. No node implies same-allele phase.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "operator", rename_all = "snake_case")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum ProfileNode {
    /// One source-native variant reference.
    Member {
        /// Source-native variant identifier.
        record: Curie,
    },
    /// Every operand is required by the source expression.
    And {
        /// Source operands in expression order.
        operands: Box<[Self]>,
    },
    /// Alternatives in the source expression.
    Or {
        /// Source alternatives in expression order.
        operands: Box<[Self]>,
    },
    /// Source negation of one member.
    Not {
        /// Negated source member.
        operand: Box<Self>,
    },
}

/// Exact source text together with its parsed Boolean tree.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "ExpressionFields")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(with = "ExpressionFields"))]
pub struct ProfileExpression {
    source: Text,
    tree: ProfileNode,
}
#[derive(Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
struct ExpressionFields {
    source: Text,
    tree: ProfileNode,
}
impl TryFrom<ExpressionFields> for ProfileExpression {
    type Error = Error;
    fn try_from(value: ExpressionFields) -> Result<Self, Error> {
        value.tree.records(0, &mut BTreeSet::new())?;
        Ok(Self {
            source: value.source,
            tree: value.tree,
        })
    }
}
impl ProfileExpression {
    /// Parse CIViC rawName, whose leaves are #VID identifiers.
    pub fn civic(source: Text) -> Result<Self, Error> {
        Self::parse(source, &[])
    }
    /// Parse CIViC display text using exact source Feature-Variant labels.
    /// Ambiguous labels are refused; the member list never supplies relations.
    pub fn civic_display(source: Text, labels: &[(Curie, Text)]) -> Result<Self, Error> {
        Self::parse(source, labels)
    }
    fn parse(source: Text, labels: &[(Curie, Text)]) -> Result<Self, Error> {
        let mut parser = Parser {
            rest: source.as_str().trim(),
            labels,
            count: 0,
        };
        let tree = parser.section(0)?;
        if !parser.rest.is_empty() {
            return Err(Error::ProfileExpressionInvalid);
        }
        Ok(Self { source, tree })
    }
    /// Borrow the exact source expression.
    pub fn source(&self) -> &Text {
        &self.source
    }
    /// Borrow the parsed source relations.
    pub fn tree(&self) -> &ProfileNode {
        &self.tree
    }
    pub(super) fn check(&self, members: &[VariantRef]) -> Result<(), Error> {
        let mut declared = BTreeSet::new();
        for member in members {
            let record = member
                .source_record()
                .ok_or(Error::ProfileExpressionInvalid)?;
            if !declared.insert(record.as_str()) {
                return Err(Error::ProfileMemberDuplicate);
            }
        }
        let mut referenced = BTreeSet::new();
        self.tree.records(0, &mut referenced)?;
        if referenced.iter().any(|record| !declared.contains(record)) {
            return Err(Error::ProfileMemberMissing);
        }
        if declared != referenced {
            return Err(Error::ProfileExpressionInvalid);
        }
        Ok(())
    }
}
impl ProfileNode {
    fn records<'a>(&'a self, depth: usize, output: &mut BTreeSet<&'a str>) -> Result<(), Error> {
        if depth > 64 {
            return Err(Error::ProfileExpressionInvalid);
        }
        match self {
            Self::Member { record } => {
                output.insert(record.as_str());
            }
            Self::And { operands } | Self::Or { operands } => {
                if operands.len() < 2 {
                    return Err(Error::ProfileExpressionInvalid);
                }
                for operand in operands {
                    operand.records(depth + 1, output)?;
                }
            }
            Self::Not { operand } => {
                if !matches!(operand.as_ref(), Self::Member { .. }) {
                    return Err(Error::ProfileExpressionInvalid);
                }
                operand.records(depth + 1, output)?;
            }
        }
        Ok(())
    }
}
struct Parser<'a> {
    rest: &'a str,
    labels: &'a [(Curie, Text)],
    count: usize,
}
impl Parser<'_> {
    fn section(&mut self, depth: usize) -> Result<ProfileNode, Error> {
        if depth > 64 {
            return Err(Error::ProfileExpressionInvalid);
        }
        let mut operands = vec![self.atom(depth)?];
        let mut operator = None;
        loop {
            let next = if self.take("AND") {
                Some(true)
            } else if self.take("OR") {
                Some(false)
            } else {
                None
            };
            let Some(next) = next else {
                break;
            };
            if operator.is_some_and(|previous| previous != next) {
                return Err(Error::ProfileExpressionInvalid);
            }
            operator = Some(next);
            operands.push(self.atom(depth)?);
        }
        match operator {
            Some(true) => Ok(ProfileNode::And {
                operands: operands.into(),
            }),
            Some(false) => Ok(ProfileNode::Or {
                operands: operands.into(),
            }),
            None => operands.pop().ok_or(Error::ProfileExpressionInvalid),
        }
    }
    fn atom(&mut self, depth: usize) -> Result<ProfileNode, Error> {
        self.count += 1;
        if self.count > 4096 {
            return Err(Error::ProfileExpressionInvalid);
        }
        // Match entire known labels before treating their parentheses or words as syntax.
        if let Some(record) = self.display_member()? {
            return Ok(ProfileNode::Member { record });
        }
        if self.take("(") {
            let node = self.section(depth + 1)?;
            if !self.take(")") {
                return Err(Error::ProfileExpressionInvalid);
            }
            return Ok(node);
        }
        let negated = self.take("NOT");
        let record = if let Some(record) = self.display_member()? {
            record
        } else {
            if !self.labels.is_empty() {
                return Err(Error::ProfileMemberMissing);
            }
            let rest = self
                .rest
                .strip_prefix("#VID")
                .ok_or(Error::ProfileExpressionInvalid)?;
            let end = rest.bytes().take_while(u8::is_ascii_digit).count();
            let digits = rest.get(..end).ok_or(Error::ProfileExpressionInvalid)?;
            let remaining = rest.get(end..).ok_or(Error::ProfileExpressionInvalid)?;
            let id = digits
                .parse::<u32>()
                .map_err(|_| Error::ProfileExpressionInvalid)?;
            if id == 0 || digits.starts_with('0') || !boundary(remaining) {
                return Err(Error::ProfileExpressionInvalid);
            }
            self.rest = remaining.trim_start();
            format!("civic:{id}").parse()?
        };
        let node = ProfileNode::Member { record };
        Ok(if negated {
            ProfileNode::Not {
                operand: Box::new(node),
            }
        } else {
            node
        })
    }
    fn display_member(&mut self) -> Result<Option<Curie>, Error> {
        let mut found = None;
        for (record, label) in self.labels {
            if let Some(rest) = self.rest.strip_prefix(label.as_str()) {
                if boundary(rest) {
                    if found.is_some() {
                        return Err(Error::ProfileExpressionInvalid);
                    }
                    found = Some((record.clone(), rest));
                }
            }
        }
        if let Some((record, rest)) = found {
            self.rest = rest.trim_start();
            Ok(Some(record))
        } else {
            Ok(None)
        }
    }
    fn take(&mut self, token: &str) -> bool {
        if let Some(rest) = self.rest.strip_prefix(token) {
            if token == "(" || token == ")" || boundary(rest) {
                self.rest = rest.trim_start();
                return true;
            }
        }
        false
    }
}
fn boundary(rest: &str) -> bool {
    rest.is_empty() || rest.starts_with(|c: char| c.is_whitespace() || c == '(' || c == ')')
}
