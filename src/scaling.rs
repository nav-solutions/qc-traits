#[derive(Debug, Copy, Clone, PartialEq)]
pub enum QcScaling {
    /// Offset (b)
    Offset(f64),
    /// a(x) + b scaling
    Scaling((f64, f64)),
}

use crate::errors::QcScalingParsingError;

impl std::str::FromStr for QcScaling {
    type Err = QcScalingParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim();

        let offset = trimmed.find(|c| c == 'x');

        if let Some(offset) = offset {
            let a = trimmed[..offset]
                .parse::<f64>()
                .map_err(|_| QcScalingParsingError::InvalidNumber)?;

            let b = trimmed[offset + 1..]
                .trim()
                .parse::<f64>()
                .map_err(|_| QcScalingParsingError::InvalidNumber)?;

            Ok(QcScaling::Scaling((a, b)))
        } else {
            let value = trimmed
                .parse::<f64>()
                .map_err(|_| QcScalingParsingError::InvalidNumber)?;

            Ok(QcScaling::Offset(value))
        }
    }
}

#[cfg(test)]
mod test {
    use super::QcScaling;
    use std::str::FromStr;

    #[test]
    fn scaling_parsing() {
        for (value, expected) in [
            ("10.0", QcScaling::Offset(10.0)),
            ("+10.0", QcScaling::Offset(10.0)),
            ("-10.0", QcScaling::Offset(-10.0)),
            ("-10.0x+10", QcScaling::Scaling((-10.0, 10.0))),
            ("-10.0x +10", QcScaling::Scaling((-10.0, 10.0))),
            ("-10x +10 ", QcScaling::Scaling((-10.0, 10.0))),
            ("-10x+10 ", QcScaling::Scaling((-10.0, 10.0))),
            ("-123x+0.001", QcScaling::Scaling((-123.0, 0.001))),
        ] {
            let scaling = QcScaling::from_str(value)
                .unwrap_or_else(|e| panic!("Failed to parse scaling from \"{}\": {}", value, e));

            assert_eq!(scaling, expected);
        }
    }
}
