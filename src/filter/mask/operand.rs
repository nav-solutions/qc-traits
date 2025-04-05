use crate::errors::QcFilterError;

/// [QcMaskOperand] describes how to apply a given mask
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum QcMaskOperand {
    /// Greater than, is symbolized by ">".
    GreaterThan,
    /// Greater Equals, symbolized by ">=".
    GreaterEquals,
    /// Lower than, symbolized by "<"."
    LowerThan,
    /// Lower Equals, symbolized by "<=".
    LowerEquals,
    #[default]
    /// Equals, symbolized by "=".
    /// Equals operand is implied anytime the operand is omitted in the description.
    Equals,
    /// Not Equals, symbolized by "!=".
    NotEquals,
}

impl std::str::FromStr for QcMaskOperand {
    type Err = QcFilterError;

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
            Err(QcFilterError::InvalidOperand)
        }
    }
}

impl std::ops::Not for QcMaskOperand {
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
    fn mask_operand() {
        for (string, value) in [
            (">", QcMaskOperand::GreaterThan),
            (">=", QcMaskOperand::GreaterEquals),
            ("<", QcMaskOperand::LowerThan),
            ("<=", QcMaskOperand::LowerEquals),
            ("=", QcMaskOperand::Equals),
            ("!", QcMaskOperand::NotEquals),
        ] {
            let operand = QcMaskOperand::from_str(string)
                .unwrap_or_else(|e| panic!("Failed to parse {}: {}", string, e));

            assert_eq!(operand, value);
        }
    }
}
