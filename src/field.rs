use hifitime::prelude::{Epoch, HifitimeError};
use thiserror::Error;

/// [QcField] describes interesting data fields
#[derive(Debug, PartialEq, Clone)]
pub enum QcField {
    /// 3D ECEF coordinates (m)
    EcefCoordinates3d((f64, f64, f64)),
    /// 3D Geodetic coordinates (lat°, long°, alt(m))
    GeoCoordinates3d((f64, f64, f64)),
    /// Agency / publisher name
    Agency(String),
    /// Operator name
    Operator(String),
    /// Geodetic marker
    GeoMarker(String),
    /// First declared datetime
    StartDateTime(Epoch),
    /// Last declared datetime
    EndDateTime(Epoch),
    /// Scaling (static offset)
    Scaling(String),
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("missing field separator")]
    MissingSeparator,
    #[error("invalid data field")]
    InvalidField,
    #[error("datetime parsing: {0}")]
    DateTimeParsing(#[from] HifitimeError),
    #[error("coordinates parsing")]
    CoordinatesParsing,
}

fn parse_3d_csv(s: &str) -> Result<(f64, f64, f64), Error> {
    let mut index = 0;
    let (mut x, mut y, mut z) = (0.0, 0.0, 0.0);

    for item in s.split(',') {
        if index == 0 {
            x = item
                .trim()
                .parse::<f64>()
                .or(Err(Error::CoordinatesParsing))?;
        } else if index == 1 {
            y = item
                .trim()
                .parse::<f64>()
                .or(Err(Error::CoordinatesParsing))?;
        } else {
            z = item
                .trim()
                .parse::<f64>()
                .or(Err(Error::CoordinatesParsing))?;
        }
        index += 1;
    }

    if index == 3 {
        Ok((x, y, z))
    } else {
        Err(Error::MissingSeparator)
    }
}

impl std::str::FromStr for QcField {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim();
        if !trimmed.contains(':') {
            return Err(Error::MissingSeparator);
        }

        let index = trimmed.find(':').unwrap();
        let marker = &trimmed[..index];
        let content = trimmed[index + 1..].trim();

        if marker.eq("ecef") {
            let coords = parse_3d_csv(content)?;
            Ok(QcField::EcefCoordinates3d(coords))
        } else if marker.eq("geo") {
            let coords = parse_3d_csv(content)?;
            Ok(QcField::GeoCoordinates3d(coords))
        } else if marker.eq("agency") {
            Ok(QcField::Agency(content.to_string()))
        } else if marker.eq("operator") {
            Ok(QcField::Operator(content.to_string()))
        } else if marker.eq("marker") {
            Ok(QcField::GeoMarker(content.to_string()))
        } else if marker.eq("start") {
            let datetime = Epoch::from_str(content)?;
            Ok(QcField::StartDateTime(datetime))
        } else if marker.eq("end") {
            let datetime = Epoch::from_str(content)?;
            Ok(QcField::EndDateTime(datetime))
        } else if marker.eq("scaling") {
            Ok(QcField::Scaling(content.to_string()))
        } else {
            Err(Error::InvalidField)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::QcField;
    use hifitime::prelude::Epoch;
    use std::str::FromStr;

    #[test]
    fn qc_field_parsing() {
        let datetime = Epoch::from_str("2010-01-01T00:00:00 UTC").unwrap();
        for (content, expected) in [
            (
                "ecef:123,456,789",
                QcField::EcefCoordinates3d((123.0, 456.0, 789.0)),
            ),
            (
                "geo:123,456,789",
                QcField::GeoCoordinates3d((123.0, 456.0, 789.0)),
            ),
            ("agency:Some-Name", QcField::Agency("Some-Name".to_string())),
            (
                "operator:Some-One",
                QcField::Operator("Some-One".to_string()),
            ),
            (
                "marker:calibrated",
                QcField::GeoMarker("calibrated".to_string()),
            ),
            (
                "start:2010-01-01T00:00:00 UTC",
                QcField::StartDateTime(datetime),
            ),
            (
                "end:2010-01-01T00:00:00 UTC",
                QcField::EndDateTime(datetime),
            ),
            ("scaling:10", QcField::Scaling("10".to_string())),
        ] {
            let field = QcField::from_str(content).unwrap();
            assert_eq!(field, expected);
        }
    }
}
