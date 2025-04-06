//! Azimuth and elevation angles presentation
use crate::errors::QcAngleParsingError;

/// [QcAngle] to describe an angle in either radians or degrees.
/// Proposes a few formatting method and a parsing method.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct QcAngle {
    degrees: f64,
}

impl QcAngle {
    /// Create a [QcAngle] from value in degrees.
    pub fn from_degrees(deg: f64) -> Self {
        Self { degrees: deg }
    }

    /// Create a [QcAngle] from value in radians
    pub fn from_radians(rad: f64) -> Self {
        Self {
            degrees: rad.to_degrees(),
        }
    }
}

/// Formats this [QcAngle] in degrees
impl std::fmt::Display for QcAngle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.3}°", self.degrees)
    }
}

/// Formats this [QcAngle] in radians
impl std::fmt::LowerExp for QcAngle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.3}rad", self.degrees.to_radians())
    }
}

impl std::str::FromStr for QcAngle {
    type Err = QcAngleParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim();
        let alpha_pos = trimmed.find(|p: char| p.is_alphabetic());

        if let Some(alpha_pos) = alpha_pos {
            let val = trimmed[..alpha_pos]
                .trim()
                .parse::<f64>()
                .map_err(|_| QcAngleParsingError::InvalidValue)?;

            let unit_str = &trimmed[alpha_pos..];

            if unit_str.contains("deg") {
                Ok(Self::from_degrees(val))
            } else if unit_str.contains("rad") {
                Ok(Self::from_radians(val))
            } else {
                Err(QcAngleParsingError::InvalidUnit)
            }
        } else {
            let deg = trimmed
                .parse::<f64>()
                .map_err(|_| QcAngleParsingError::InvalidValue)?;

            Ok(Self::from_degrees(deg))
        }
    }
}

#[cfg(test)]
mod test {
    use super::QcAngle;
    use std::str::FromStr;

    #[test]
    fn angle_parsing() {
        for (value, expected) in [
            ("10", QcAngle::from_degrees(10.0)),
            ("10deg", QcAngle::from_degrees(10.0)),
            ("10.1 deg", QcAngle::from_degrees(10.1)),
            ("10.2rad", QcAngle::from_radians(10.2)),
            ("10.2 rad", QcAngle::from_radians(10.2)),
        ] {
            let angle = QcAngle::from_str(value)
                .unwrap_or_else(|e| panic!("Failed to parse angle from \"{}\" : {}", value, e));

            assert_eq!(angle, expected, "Failed to parse angle from \"{}\"", value);
        }
    }
}
