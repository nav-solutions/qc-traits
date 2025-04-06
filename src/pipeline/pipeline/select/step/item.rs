use crate::{
    errors::QcSelectionError,
    prelude::{Constellation, Duration, Epoch, SV},
    QcAngle,
};

#[derive(Clone, Debug, PartialEq)]
pub enum QcSelectionStepItem {
    /// Applies to specific Datetime ([Epoch]) only
    Datetime(Epoch),
    /// [Duration]
    Duration(Duration),
    /// Elevation angle
    Elevation(QcAngle),
    /// Azimuth angle
    Azimuth(QcAngle),
    /// List of satellites described by [SV]
    Satellites(Vec<SV>),
    /// List of [Constellation]s described by [Constellation]
    Constellations(Vec<Constellation>),
    /// Readable CSV description we cannot interprate at this level.
    UninterpretedCsvString(String),
}

impl QcSelectionStepItem {
    /// Builds a [QcSelectionStepItem::Datetime]
    pub fn from_datetime(t: Epoch) -> Self {
        Self::Datetime(t)
    }

    /// Builds a [QcSelectionStepItem::Duration]
    pub fn from_duration(dt: Duration) -> Self {
        Self::Duration(dt)
    }

    /// Builds a [QcSelectionStepItem::Satellites] from this unique [SV]
    pub fn from_satellite(sv: SV) -> Self {
        Self::Satellites(vec![sv])
    }

    /// Builds a [QcSelectionStepItem::Satellites] from list of [SV]
    pub fn from_satellites(sv: &[SV]) -> Self {
        Self::Satellites(sv.to_vec())
    }

    /// Builds a [QcSelectionStepItem::Constellations] from this unique [Constellation]
    pub fn from_constellation(constellation: Constellation) -> Self {
        Self::Constellations(vec![constellation])
    }

    /// Builds a [QcSelectionStepItem::Constellations] from list of [Constellation]s
    pub fn from_constellations(constellations: &[Constellation]) -> Self {
        Self::Constellations(constellations.to_vec())
    }

    /// Builds a [QcSelectionStepItem::Elevation]
    pub fn from_elevation_deg(deg: f64) -> Self {
        Self::Elevation(QcAngle::from_degrees(deg))
    }

    /// Builds a [QcSelectionStepItem::Elevation] from elevation angle in radians
    pub fn from_elevation_rad(rad: f64) -> Self {
        Self::Elevation(QcAngle::from_radians(rad))
    }

    /// Builds a [QcSelectionStepItem::Azimuth] from azimuth angle in degrees
    pub fn from_azimuth_deg(deg: f64) -> Self {
        Self::Azimuth(QcAngle::from_degrees(deg))
    }

    /// Builds a [QcSelectionStepItem::Azimuth] from azimuth angle in radians
    pub fn from_azimuth_rad(rad: f64) -> Self {
        Self::Azimuth(QcAngle::from_radians(rad))
    }

    /// Builds a [QcSelectionStepItem::UninterpretedCsvString] we cannot interprate at this level.
    pub fn from_csv_string(s: &str) -> Self {
        Self::UninterpretedCsvString(s.to_string())
    }
}

impl std::str::FromStr for QcSelectionStepItem {
    type Err = QcSelectionError;

    fn from_str(content: &str) -> Result<Self, Self::Err> {
        let c = content.trim();
        let csv: Vec<&str> = c.split(',').collect();

        if csv.len() < 1 {
            return Err(QcSelectionError::EmptyStep);
        }

        let first_item = csv[0].trim();

        // type guessing
        if let Ok(t) = Epoch::from_str(first_item) {
            Ok(Self::Datetime(t))
        } else if let Ok(dt) = Duration::from_str(first_item) {
            Ok(Self::Duration(dt))
        } else {
            // known csv lists attempt
            let sv = csv
                .iter()
                .filter_map(|s| {
                    if let Ok(sv) = SV::from_str(s.trim()) {
                        Some(sv)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            let constellations = csv
                .iter()
                .filter_map(|s| {
                    if let Ok(c) = Constellation::from_str(s.trim()) {
                        Some(c)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            if !sv.is_empty() {
                Ok(Self::Satellites(sv))
            } else if !constellations.is_empty() {
                Ok(Self::Constellations(constellations))
            } else {
                Ok(Self::UninterpretedCsvString(c.to_string()))
            }
        }
    }
}

impl std::fmt::Display for QcSelectionStepItem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Datetime(t) => write!(f, "{}", t),
            Self::Duration(dt) => write!(f, "dt = {}", dt),
            Self::Azimuth(angle) => write!(f, "az {}", angle),
            Self::Elevation(angle) => write!(f, "el {}", angle),
            Self::Satellites(sv) => write!(f, "{:?}", sv),
            Self::Constellations(c) => write!(f, "{:?}", c),
            Self::UninterpretedCsvString(s) => write!(f, "{}", s),
        }
    }
}

#[cfg(test)]
mod test {
    use super::QcSelectionStepItem;
    use crate::prelude::{Constellation, Duration, Epoch, SV};
    use std::str::FromStr;

    #[test]
    fn pipeline_selection_step_item_parsing() {
        const GPS: Constellation = Constellation::GPS;

        const GAL: Constellation = Constellation::Galileo;
        let e07 = SV::new(GAL, 07);
        let e10 = SV::new(GAL, 10);

        let datetime_str = "2020-01-01T00:00:00 UTC";
        let datetime = Epoch::from_str(datetime_str).unwrap();

        let dt_30s = Duration::from_seconds(30.0);

        for (item_str, expected) in [
            ("30 s", QcSelectionStepItem::from_duration(dt_30s)),
            (datetime_str, QcSelectionStepItem::from_datetime(datetime)),
            ("E10", QcSelectionStepItem::from_satellite(e10)),
            ("E07,E10", QcSelectionStepItem::from_satellites(&[e07, e10])),
            ("GPS", QcSelectionStepItem::from_constellation(GPS)),
            (
                "GPS,Gal",
                QcSelectionStepItem::from_constellations(&[GPS, GAL]),
            ),
            (
                "this,is a test",
                QcSelectionStepItem::from_csv_string("this,is a test"),
            ),
        ] {
            let item = QcSelectionStepItem::from_str(item_str).unwrap_or_else(|e| {
                panic!(
                    "Failed to parse processing pipeline step item: \"{}\": {}",
                    item_str, e
                )
            });

            assert_eq!(item, expected, "invalid item parsed from \"{}\"", item_str);
        }
    }
}
