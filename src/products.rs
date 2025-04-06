//! (Input) Products we may have to manage
use crate::errors::QcScopeError;

/// [QcProductType] defines product types we may manage.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum QcProductType {
    /// GNSS carrier signal observation in the form
    /// of Observation RINEX data.
    Observation,
    /// Broadcast Navigation message as contained in
    /// Navigation RINEX files.
    BroadcastNavigation,
}

impl std::fmt::Display for QcProductType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Observation => write!(f, "Observation RINEX"),
            Self::BroadcastNavigation => write!(f, "Broadcast Navigation RINEX (BRDC)"),
        }
    }
}

impl std::str::FromStr for QcProductType {
    type Err = QcScopeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim();
        let lowered = trimmed.to_lowercase();
        match lowered.as_str() {
            "obs" | "observation" => Ok(Self::Observation),
            "nav" | "brdc" | "navigation" => Ok(Self::BroadcastNavigation),
            _ => Err(QcScopeError::UnknownProductType),
        }
    }
}
