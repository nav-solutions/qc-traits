use crate::{errors::QcFilterError, QcScope, QcSubset};

use super::operand::QcMaskOperand;

pub struct QcMaskFilter {
    pub scope: QcScope,
    pub subset: QcSubset,
    pub operand: QcMaskOperand,
}

/// Parse [QcMaskFilter] from readable string description.
/// Accepted values are:
/// - "nav>E10"
/// -
impl std::str::FromStr for QcMaskFilter {
    type Err = QcFilterError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim();

        let mut operand = QcMaskOperand::default();
        let mut subset = QcSubset::All;
        let mut scope = QcScope::All;

        if trimmed.len() < 2 {
            return Err(QcFilterError::InvalidMaskFilter);
        }
    }
}
