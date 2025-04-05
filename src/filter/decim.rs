use hifitime::Duration;

use crate::{
    errors::QcFilterError,
    filter::{QcScope, QcSubset},
};

#[derive(Clone, Debug)]
pub struct QcDecimationFilter {
    /// [QcScope] of the decimation filter
    pub scope: QcScope,
    /// [QcSubset] of which the decimation operation should apply
    pub subset: QcSubset,
    /// [QcDecimationFilterType]
    pub filter: QcDecimationFilterType,
}

impl QcDecimationFilter {
    /// Builds a new [QcDecimationFilter] targetting the entire Dataset
    /// to downsample using this modulo factor.
    pub fn from_modulo_decimation(&self, modulo: u32) -> QcDecimationFilter {
        QcDecimationFilter {
            scope: QcScope::All,
            subset: QcSubset::All,
            filter: QcDecimationFilterType::Modulo(modulo),
        }
    }

    /// Builds a new [QcDecimationFilter] targetting the entire Dataset
    /// to downsample to provided [Duration] interval.
    pub fn from_downsampling_interval(&self, dt: Duration) -> QcDecimationFilter {
        QcDecimationFilter {
            scope: QcScope::All,
            subset: QcSubset::All,
            filter: QcDecimationFilterType::Duration(dt),
        }
    }
}

impl std::str::FromStr for QcDecimationFilter {
    type Err = QcFilterError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut scope = QcScope::default();
        let mut subset = QcSubset::default();
        let mut filter = Option::<QcDecimationFilterType>::None;

        for item in s.split(':') {
            let trimmed = item.trim();
            if let Ok(parsed) = QcDecimationFilterType::from_str(trimmed) {
                filter = Some(parsed);
            } else if let Ok(parsed) = QcScope::from_str(trimmed) {
                scope = parsed;
            } else if let Ok(parsed) = QcSubset::from_str(trimmed) {
                subset = parsed;
            }
        }

        if let Some(filter) = filter {
            Ok({
                QcDecimationFilter {
                    scope,
                    subset,
                    filter,
                }
            })
        } else {
            Err(QcFilterError::InvalidDecimFilter)
        }
    }
}

/// Supported algorithms
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum QcDecimationFilterType {
    /// Simple modulo decimation
    Modulo(u32),
    /// Duration decimation
    Duration(Duration),
}

impl QcDecimationFilterType {
    /// Builds a [DecimationFilter::Duration].
    pub fn from_duration(dt: Duration) -> Self {
        Self::Duration(dt)
    }

    /// Builds a [DecimationFilter::Modulo].
    pub fn from_modulo(modulo: u32) -> Self {
        Self::Modulo(modulo)
    }
}

impl std::str::FromStr for QcDecimationFilterType {
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
