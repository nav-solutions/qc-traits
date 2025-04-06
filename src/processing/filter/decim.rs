use hifitime::Duration;

/// Supported [QcDecimationFilter]s
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum QcDecimationFilter {
    /// Modulo decimation
    Modulo(u32),
    /// Duration decimation
    Duration(Duration),
}

impl QcDecimationFilter {
    /// Builds a [QcDecimationFilter::Duration] filter.
    pub fn from_duration(dt: Duration) -> Self {
        Self::Duration(dt)
    }

    /// Builds a [QcDecimationFilter::Modulo] filter.
    pub fn from_modulo(modulo: u32) -> Self {
        Self::Modulo(modulo)
    }
}

impl std::str::FromStr for QcDecimationFilter {
    type Err = QcFilterParsingError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim();
        if trimmed.starts_with('%') {
            let modulo = trimmed[..1]
                .parse::<u32>()
                .or(Err(QcFilterParsingError::InvalidModuloFilter))?;

            Ok(Self::Modulo(modulo))
        } else {
            // assumes duration / interval filter
            let dt = trimmed
                .parse::<Duration>()
                .or(Err(QcFilterParsingError::InvalidDuration))?;

            Ok(Self::Duration(dt))
        }
    }
}

#[cfg(test)]
mod test {
    use super::QcDecimationFilter;
    use crate::filter::{QcScope, QcSubset};
    use std::str::FromStr;

    #[test]
    fn decimation_filter_parsing() {
        let value = "%10";

        let filter = QcDecimationFilter::from_str(value).unwrap();
        assert_eq!()
    }
}
