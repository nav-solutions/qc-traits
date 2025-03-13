use crate::errors::QcFilterError;
use hifitime::Duration;

/// Supported algorithms
#[derive(Clone, Debug, PartialEq)]
pub enum DecimationFilter {
    /// Simple modulo decimation
    Modulo(u32),
    /// Duration decimation
    Duration(Duration),
}

impl DecimationFilter {
    /// Builds a [DecimationFilter::Duration].
    pub fn from_duration(dt: Duration) -> Self {
        Self::Duration(dt)
    }

    /// Builds a [DecimationFilter::Modulo].
    pub fn from_modulo(modulo: u32) -> Self {
        Self::Modulo(modulo)
    }
}

impl std::str::FromStr for DecimationFilter {
    type Err = QcFilterError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim();
        if trimmed.starts_with('%') {
            let modulo = trimmed[..1]
                .parse::<u32>()
                .or(Err(QcFilterError::InvalidModuloFilter))?;

            Ok(Self::Modulo(modulo))
        } else {
            // assumes duration / interval filter
            let dt = trimmed
                .parse::<Duration>()
                .or(Err(QcFilterError::InvalidDuration))?;

            Ok(Self::Duration(dt))
        }
    }
}
