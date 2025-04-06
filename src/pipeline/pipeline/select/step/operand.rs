use crate::errors::QcSelectionError;

#[cfg(doc)]
use super::QcSelectionStep;

/// [QcSelectionStepOperand] attached a [QcSelectionStep].
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum QcSelectionStepOperand {
    /// [QcSelectionStepOperand::GreaterThan] symbolized by `>`
    GreaterThan,
    /// [QcSelectionStepOperand::GreaterEquals] symbolized by `>=`
    GreaterEquals,
    /// [QcSelectionStepOperand::LowerThan] symbolized by `<`
    LowerThan,
    /// [QcSelectionStepOperand::LowerEquals] symbolized by `<=`
    LowerEquals,
    #[default]
    /// [QcSelectionStepOperand::Equals] either implied when omitted, or symbolized by `=`
    Equals,
    /// [QcSelectionStepOperand::NotEquals] symbolized by `!=`
    NotEquals,
}

impl std::fmt::Display for QcSelectionStepOperand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Equals => write!(f, "="),
            Self::LowerThan => write!(f, "<"),
            Self::NotEquals => write!(f, "!="),
            Self::GreaterThan => write!(f, ">"),
            Self::LowerEquals => write!(f, "<="),
            Self::GreaterEquals => write!(f, ">="),
        }
    }
}

impl std::str::FromStr for QcSelectionStepOperand {
    type Err = QcSelectionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim();

        if trimmed.eq(">") {
            Ok(Self::GreaterThan)
        } else if trimmed.eq("!") {
            Ok(Self::NotEquals)
        } else if trimmed.eq("<") {
            Ok(Self::LowerThan)
        } else if trimmed.eq("=") {
            Ok(Self::Equals)
        } else if trimmed.eq(">=") {
            Ok(Self::GreaterEquals)
        } else if trimmed.eq("<=") {
            Ok(Self::LowerEquals)
        } else {
            Err(QcSelectionError::InvalidOperand)
        }
    }
}

impl std::ops::Not for QcSelectionStepOperand {
    type Output = Self;
    fn not(self) -> Self {
        match self {
            Self::Equals => Self::NotEquals,
            Self::NotEquals => Self::Equals,
            Self::GreaterEquals => Self::LowerEquals,
            Self::GreaterThan => Self::LowerThan,
            Self::LowerThan => Self::GreaterThan,
            Self::LowerEquals => Self::GreaterEquals,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn pipeline_selection_step_operand_parsing() {
        for (string, value) in [
            (">", QcSelectionStepOperand::GreaterThan),
            (">=", QcSelectionStepOperand::GreaterEquals),
            ("<", QcSelectionStepOperand::LowerThan),
            ("<=", QcSelectionStepOperand::LowerEquals),
            ("=", QcSelectionStepOperand::Equals),
            ("!", QcSelectionStepOperand::NotEquals),
        ] {
            let operand = QcSelectionStepOperand::from_str(string)
                .unwrap_or_else(|e| panic!("Failed to parse {}: {}", string, e));

            assert_eq!(operand, value);
        }
    }
}
