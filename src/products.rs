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
    /// High Precision Orbital SP3 product
    HighPrecisionOrbit,
    /// High Precision Clock RINEX
    HighPrecisionClockRINEX,
}

impl std::fmt::Display for QcProductType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Observation => write!(f, "Observation RINEX"),
            Self::HighPrecisionClockRINEX => write!(f, "High Precision Clock RINEX"),
            Self::HighPrecisionOrbit => write!(f, "High Precision Orbit (SP3)"),
            Self::BroadcastNavigation => write!(f, "Broadcast Navigation RINEX (BRDC)"),
        }
    }
}

impl std::fmt::UpperHex for QcProductType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Observation => write!(f, "OBS RINEX"),
            Self::HighPrecisionClockRINEX => write!(f, "Clock RINEX"),
            Self::HighPrecisionOrbit => write!(f, "SP3"),
            Self::BroadcastNavigation => write!(f, "NAV RINEX"),
        }
    }
}

impl std::fmt::LowerExp for QcProductType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::HighPrecisionOrbit => write!(f, "SP3"),
            _ => write!(f, "RINEX"),
        }
    }
}

impl std::str::FromStr for QcProductType {
    type Err = QcScopeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim();
        let lowered = trimmed.to_lowercase();
        match lowered.as_str() {
            "sp3" => Ok(Self::HighPrecisionOrbit),
            "obs" | "observation" => Ok(Self::Observation),
            "nav" | "brdc" | "navigation" => Ok(Self::BroadcastNavigation),
            "clk" | "clock" => Ok(Self::HighPrecisionClockRINEX),
            _ => Err(QcScopeError::UnknownProductType),
        }
    }
}
